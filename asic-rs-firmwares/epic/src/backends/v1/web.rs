use std::{net::IpAddr, time::Duration};

use once_cell::sync::OnceCell;

use anyhow::{self, Context, bail};
use asic_rs_core::{
    data::{
        command::MinerCommand,
        firmware::{FirmwareImage, FirmwareUpgradeOptions},
    },
    traits::miner::*,
};
use async_trait::async_trait;
use reqwest::{Client, Method, Response, header, multipart};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tracing::warn;

/// ePIC PowerPlay WebAPI client
#[derive(Debug)]
pub struct PowerPlayWebAPI {
    client: OnceCell<Client>,
    pub ip: IpAddr,
    port: u16,
    timeout: Duration,
    auth: MinerAuth,
}

#[async_trait]
impl APIClient for PowerPlayWebAPI {
    async fn get_api_result(&self, command: &MinerCommand) -> anyhow::Result<Value> {
        match command {
            MinerCommand::WebAPI {
                command,
                parameters,
            } => self
                .send_command(command, false, parameters.clone(), Method::GET)
                .await
                .map_err(|e| anyhow::anyhow!(e.to_string())),
            _ => Err(anyhow::anyhow!("Cannot send non web command to web API")),
        }
    }
}

#[async_trait]
impl WebAPIClient for PowerPlayWebAPI {
    /// Send a command to the EPic miner API
    async fn send_command(
        &self,
        command: &str,
        _privileged: bool,
        parameters: Option<Value>,
        method: Method,
    ) -> anyhow::Result<Value> {
        let url = format!("http://{}:{}/{}", self.ip, self.port, command);

        let response = self
            .execute_request(&url, &method, parameters.clone())
            .await?;

        let status = response.status();
        if status.is_success() {
            let json_data = response
                .json()
                .await
                .map_err(|e| PowerPlayError::ParseError(e.to_string()))?;
            Ok(json_data)
        } else {
            Err(PowerPlayError::HttpError(status.as_u16()))?
        }
    }
}

impl PowerPlayWebAPI {
    fn firmware_upgrade_timeout(&self, options: FirmwareUpgradeOptions) -> Duration {
        options
            .timeout
            .unwrap_or_else(|| self.timeout.max(Duration::from_secs(300)))
    }

    async fn sha256_hex(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        for chunk in bytes.chunks(64 * 1024) {
            hasher.update(chunk);
            tokio::task::yield_now().await;
        }
        format!("{:x}", hasher.finalize())
    }

    /// Create a new EPic WebAPI client
    pub fn new(ip: IpAddr, port: u16, auth: MinerAuth) -> Self {
        Self {
            client: OnceCell::new(),
            ip,
            port,
            timeout: Duration::from_secs(5),
            auth,
        }
    }

    pub fn set_auth(&mut self, auth: MinerAuth) {
        self.auth = auth;
    }

    pub fn username(&self) -> &str {
        self.auth.username()
    }

    fn build_client() -> Result<Client, PowerPlayError> {
        Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| PowerPlayError::RequestError(format!("failed to create HTTP client: {e}")))
    }

    fn client(&self) -> Result<&Client, PowerPlayError> {
        self.client.get_or_try_init(Self::build_client)
    }

    pub async fn upgrade_firmware(&self, image: FirmwareImage) -> anyhow::Result<bool> {
        self.upgrade_firmware_with_options(image, FirmwareUpgradeOptions::default())
            .await
    }

    pub async fn upgrade_firmware_with_options(
        &self,
        image: FirmwareImage,
        options: FirmwareUpgradeOptions,
    ) -> anyhow::Result<bool> {
        let url = format!("http://{}:{}{}", self.ip, self.port, "/systemupdate");
        let FirmwareImage { filename, bytes } = image;
        let checksum = Self::sha256_hex(&bytes).await;

        let form = multipart::Form::new()
            .text("password", self.auth.password().to_string())
            .text("checksum", checksum)
            .text("keepsettings", "true")
            .part(
                "update.zip",
                multipart::Part::bytes(bytes)
                    .file_name(filename)
                    .mime_str("application/zip")
                    .context("failed to set firmware part mime type")?,
            );

        let timeout = self.firmware_upgrade_timeout(options);
        let response = self
            .client()?
            .post(url)
            .header(header::ACCEPT, "application/json")
            .timeout(timeout)
            .multipart(form)
            .send()
            .await
            .with_context(|| {
                format!(
                    "firmware upload HTTP request failed (request timeout: {} seconds)",
                    timeout.as_secs_f64()
                )
            })?;

        let status = response.status();
        let body = response
            .text()
            .await
            .context("failed to read firmware upload response body")?;

        if !status.is_success() {
            bail!(
                "Firmware upload failed with status code {}: {}",
                status,
                body
            );
        }

        let payload: Value = serde_json::from_str(&body).with_context(|| {
            format!(
                "Invalid {} response body from {}: {}",
                "/systemupdate", self.ip, body
            )
        })?;
        let result = payload
            .get("result")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        if !result && let Some(error) = payload.get("error").and_then(Value::as_str) {
            warn!(
                miner_ip = %self.ip,
                endpoint = "/systemupdate",
                error = error,
                "ePIC firmware update API returned result=false"
            );
        }

        Ok(result)
    }

    pub async fn change_password(&self, password: &str) -> anyhow::Result<bool> {
        self.send_command(
            "password",
            true,
            Some(json!({ "param": password })),
            Method::POST,
        )
        .await
        .map(|v| v.get("result").and_then(Value::as_bool).unwrap_or(false))
    }

    async fn read_text(&self, command: &str) -> anyhow::Result<String> {
        let url = format!("http://{}:{}/{}", self.ip, self.port, command);
        let response = self.execute_request(&url, &Method::GET, None).await?;
        let status = response.status();
        if !status.is_success() {
            Err(PowerPlayError::HttpError(status.as_u16()))?;
        }

        response
            .text()
            .await
            .map_err(|e| anyhow::anyhow!("failed to read {command} response body: {e}"))
    }

    pub async fn read_logs(&self) -> anyhow::Result<String> {
        let main_log = self.read_text("log").await?;
        let error_log = self.read_text("log/error").await?;

        Ok(format!(
            "== log ==\n{}\n== log/error ==\n{}",
            main_log, error_log
        ))
    }

    /// Execute the actual HTTP request
    async fn execute_request(
        &self,
        url: &str,
        method: &Method,
        parameters: Option<Value>,
    ) -> anyhow::Result<Response, PowerPlayError> {
        let client = self.client()?;

        let request_builder = match *method {
            Method::GET => client.get(url),
            Method::POST => client.post(url).json(&{
                let mut p = parameters.unwrap_or_else(|| json!({}));
                p.as_object_mut().map(|m| {
                    m.insert(
                        "password".into(),
                        Value::String(self.auth.password().to_string()),
                    )
                });
                p
            }),
            _ => return Err(PowerPlayError::UnsupportedMethod(method.to_string())),
        };

        let request_builder = request_builder.timeout(self.timeout);

        let request = request_builder
            .build()
            .map_err(|e| PowerPlayError::RequestError(e.to_string()))?;

        let response = client
            .execute(request)
            .await
            .map_err(|e| PowerPlayError::NetworkError(e.to_string()))?;

        Ok(response)
    }
}

/// Error types for EPic WebAPI operations
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PowerPlayError {
    /// Network error (connection issues, DNS resolution, etc.)
    NetworkError(String),
    /// HTTP error with status code
    HttpError(u16),
    /// JSON parsing error
    ParseError(String),
    /// Request building error
    RequestError(String),
    /// Timeout error
    Timeout,
    /// Unsupported HTTP method
    UnsupportedMethod(String),
    /// Maximum retries exceeded
    MaxRetriesExceeded,
    /// Authentication failed
    AuthenticationFailed,
    /// Unauthorized (401)
    Unauthorized,
}

impl std::fmt::Display for PowerPlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerPlayError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            PowerPlayError::HttpError(code) => write!(f, "HTTP error: {code}"),
            PowerPlayError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            PowerPlayError::RequestError(msg) => write!(f, "Request error: {msg}"),
            PowerPlayError::Timeout => write!(f, "Request timeout"),
            PowerPlayError::UnsupportedMethod(method) => write!(f, "Unsupported method: {method}"),
            PowerPlayError::MaxRetriesExceeded => write!(f, "Maximum retries exceeded"),
            PowerPlayError::AuthenticationFailed => write!(f, "Authentication failed"),
            PowerPlayError::Unauthorized => write!(f, "Unauthorized access"),
        }
    }
}

impl std::error::Error for PowerPlayError {}

#[cfg(test)]
mod tests {
    use std::{net::Ipv4Addr, time::Instant};

    use tokio::net::TcpListener;

    use super::*;

    #[test]
    fn firmware_upgrade_timeout_preserves_default_and_accepts_override() {
        let api = PowerPlayWebAPI::new(
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            80,
            MinerAuth::new("root", "password"),
        );

        assert_eq!(
            api.firmware_upgrade_timeout(FirmwareUpgradeOptions::default()),
            Duration::from_secs(300)
        );
        assert_eq!(
            api.firmware_upgrade_timeout(FirmwareUpgradeOptions::with_timeout(
                Duration::from_millis(25)
            )),
            Duration::from_millis(25)
        );
    }

    #[tokio::test]
    async fn firmware_upgrade_honors_request_timeout() {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move {
            let (_socket, _) = listener.accept().await.unwrap();
            tokio::time::sleep(Duration::from_secs(2)).await;
        });
        let api = PowerPlayWebAPI::new(
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            port,
            MinerAuth::new("root", "password"),
        );
        let start = Instant::now();

        let error = api
            .upgrade_firmware_with_options(
                FirmwareImage::new("firmware.zip".to_string(), vec![1, 2, 3]),
                FirmwareUpgradeOptions::with_timeout(Duration::from_millis(50)),
            )
            .await
            .unwrap_err();

        assert!(start.elapsed() < Duration::from_secs(1));
        assert!(format!("{error:#}").contains("request timeout: 0.05 seconds"));
        server.abort();
    }
}
