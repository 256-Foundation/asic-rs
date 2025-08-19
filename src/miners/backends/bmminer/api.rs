use crate::miners::api::rpc::errors::RPCError;
use crate::miners::api::rpc::status::RPCCommandStatus;
use crate::miners::api::{APIClient, RPCAPIClient, WebAPIClient};
use crate::miners::commands::MinerCommand;
use anyhow::{Result, anyhow, bail};
use async_trait::async_trait;
use reqwest::{Client, Method, Response};
use serde_json::{Value, json};
use std::{net::IpAddr, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct AntminerAPI {
    ip: IpAddr,
    rpc_port: u16,
    web_port: u16,
    client: Client,
    timeout: Duration,
    username: String,
    password: String,
}

#[async_trait]
impl APIClient for AntminerAPI {
    async fn get_api_result(&self, command: &MinerCommand) -> Result<Value> {
        match command {
            MinerCommand::RPC {
                command,
                parameters,
            } => self
                .send_rpc_command(command, false, parameters.clone())
                .await
                .map_err(|e| anyhow!(e.to_string())),
            MinerCommand::WebAPI {
                command,
                parameters,
            } => self
                .send_web_command(command, false, parameters.clone(), Method::GET)
                .await
                .map_err(|e| anyhow!(e.to_string())),
            _ => Err(anyhow!("Unsupported command type for Antminer API")),
        }
    }
}

#[async_trait]
impl RPCAPIClient for AntminerAPI {
    async fn send_command(
        &self,
        command: &str,
        privileged: bool,
        parameters: Option<Value>,
    ) -> Result<Value> {
        self.send_rpc_command(command, privileged, parameters).await
    }
}

#[async_trait]
impl WebAPIClient for AntminerAPI {
    async fn send_command(
        &self,
        command: &str,
        privileged: bool,
        parameters: Option<Value>,
        method: Method,
    ) -> Result<Value> {
        self.send_web_command(command, privileged, parameters, method)
            .await
    }
}

impl AntminerAPI {
    pub fn new(ip: IpAddr, rpc_port: Option<u16>, web_port: Option<u16>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            ip,
            rpc_port: rpc_port.unwrap_or(4028),
            web_port: web_port.unwrap_or(80),
            client,
            timeout: Duration::from_secs(5),
            username: "root".to_string(),
            password: "root".to_string(),
        }
    }

    pub fn with_auth(
        ip: IpAddr,
        rpc_port: Option<u16>,
        web_port: Option<u16>,
        username: String,
        password: String,
    ) -> Self {
        let mut api = Self::new(ip, rpc_port, web_port);
        api.username = username;
        api.password = password;
        api
    }

    // RPC API methods
    async fn send_rpc_command(
        &self,
        command: &str,
        _privileged: bool,
        parameters: Option<Value>,
    ) -> Result<Value> {
        let mut stream = tokio::net::TcpStream::connect((self.ip, self.rpc_port))
            .await
            .map_err(|_| RPCError::ConnectionFailed)?;

        let request = if let Some(params) = parameters {
            json!({
                "command": command,
                "parameter": params
            })
        } else {
            json!({
                "command": command
            })
        };

        let json_str = request.to_string();
        let message = format!("{}\n", json_str);

        stream.write_all(message.as_bytes()).await?;

        let mut response = String::new();
        let mut buffer = [0; 8192];

        loop {
            let bytes_read = stream.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }

            let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
            response.push_str(&chunk);

            if response.contains('\0') || response.ends_with('\n') {
                break;
            }
        }

        let clean_response = response.trim_end_matches('\0').trim_end_matches('\n');
        self.parse_rpc_result(clean_response)
    }

    fn parse_rpc_result(&self, response: &str) -> Result<Value> {
        let status = RPCCommandStatus::from_antminer(response)?;
        match status.into_result() {
            Ok(_) => Ok(serde_json::from_str(response)?),
            Err(e) => Err(e)?,
        }
    }

    // Web API methods
    async fn send_web_command(
        &self,
        command: &str,
        _privileged: bool,
        parameters: Option<Value>,
        method: Method,
    ) -> Result<Value> {
        let url = format!(
            "http://{}:{}/cgi-bin/{}.cgi",
            self.ip, self.web_port, command
        );

        let response = self
            .execute_web_request(&url, &method, parameters.clone())
            .await?;

        let status = response.status();
        if status.is_success() {
            let json_data = response.json().await.map_err(|e| anyhow!(e.to_string()))?;
            Ok(json_data)
        } else {
            bail!("HTTP request failed with status code {}", status);
        }
    }

    async fn execute_web_request(
        &self,
        url: &str,
        method: &Method,
        parameters: Option<Value>,
    ) -> Result<Response> {
        let auth = (self.username.clone(), Some(self.password.clone()));

        let request_builder = match *method {
            Method::GET => self.client.get(url).basic_auth(auth.0, auth.1),
            Method::POST => {
                let data = parameters.unwrap_or_else(|| json!({}));
                self.client.post(url).json(&data).basic_auth(auth.0, auth.1)
            }
            _ => bail!("Unsupported method: {}", method),
        };

        let request_builder = request_builder.timeout(self.timeout);

        let request = request_builder
            .build()
            .map_err(|e| anyhow!(e.to_string()))?;

        let response = self
            .client
            .execute(request)
            .await
            .map_err(|e| anyhow!(e.to_string()))?;

        Ok(response)
    }

    pub async fn rpc_stats(&self, new_api: bool) -> Result<Value> {
        if new_api {
            self.send_rpc_command("stats", false, Some(json!({"new_api": true})))
                .await
        } else {
            self.send_rpc_command("stats", false, None).await
        }
    }

    pub async fn rpc_summary(&self, new_api: bool) -> Result<Value> {
        if new_api {
            self.send_rpc_command("summary", false, Some(json!({"new_api": true})))
                .await
        } else {
            self.send_rpc_command("summary", false, None).await
        }
    }

    pub async fn rpc_pools(&self, new_api: bool) -> Result<Value> {
        if new_api {
            self.send_rpc_command("pools", false, Some(json!({"new_api": true})))
                .await
        } else {
            self.send_rpc_command("pools", false, None).await
        }
    }

    pub async fn rpc_version(&self) -> Result<Value> {
        self.send_rpc_command("version", false, None).await
    }

    pub async fn rpc_rate(&self) -> Result<Value> {
        self.send_rpc_command("rate", false, Some(json!({"new_api": true})))
            .await
    }

    pub async fn rpc_warning(&self) -> Result<Value> {
        self.send_rpc_command("warning", false, Some(json!({"new_api": true})))
            .await
    }

    pub async fn rpc_reload(&self) -> Result<Value> {
        self.send_rpc_command("reload", false, Some(json!({"new_api": true})))
            .await
    }

    // Web API-specific convenience methods
    pub async fn get_miner_conf(&self) -> Result<Value> {
        self.send_web_command("get_miner_conf", false, None, Method::GET)
            .await
    }

    pub async fn set_miner_conf(&self, conf: Value) -> Result<Value> {
        self.send_web_command("set_miner_conf", false, Some(conf), Method::POST)
            .await
    }

    pub async fn blink(&self, blink: bool) -> Result<Value> {
        let param = if blink {
            json!({"blink": "true"})
        } else {
            json!({"blink": "false"})
        };
        self.send_web_command("blink", false, Some(param), Method::POST)
            .await
    }

    pub async fn reboot(&self) -> Result<Value> {
        self.send_web_command("reboot", false, None, Method::POST)
            .await
    }

    pub async fn get_system_info(&self) -> Result<Value> {
        self.send_web_command("get_system_info", false, None, Method::GET)
            .await
    }

    pub async fn get_network_info(&self) -> Result<Value> {
        self.send_web_command("get_network_info", false, None, Method::GET)
            .await
    }

    pub async fn web_summary(&self) -> Result<Value> {
        self.send_web_command("summary", false, None, Method::GET)
            .await
    }

    pub async fn get_blink_status(&self) -> Result<Value> {
        self.send_web_command("get_blink_status", false, None, Method::GET)
            .await
    }

    pub async fn set_network_conf(
        &self,
        ip: String,
        dns: String,
        gateway: String,
        subnet_mask: String,
        hostname: String,
        protocol: u8,
    ) -> Result<Value> {
        let config = json!({
            "ipAddress": ip,
            "ipDns": dns,
            "ipGateway": gateway,
            "ipHost": hostname,
            "ipPro": protocol,
            "ipSub": subnet_mask
        });
        self.send_web_command("set_network_conf", false, Some(config), Method::POST)
            .await
    }
}

impl RPCCommandStatus {
    fn from_antminer(response: &str) -> Result<Self, RPCError> {
        let value: serde_json::Value = serde_json::from_str(response)?;

        if let Some(status_array) = value.get("STATUS")
            && let Some(status_obj) = status_array.get(0)
            && let Some(status) = status_obj.get("STATUS").and_then(|v| v.as_str())
        {
            let message = status_obj.get("Msg").and_then(|v| v.as_str());

            return Ok(Self::from_str(status, message));
        }

        Ok(Self::Success)
    }
}
