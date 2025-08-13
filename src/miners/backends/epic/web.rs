use crate::miners::{
    api::{APIClient, WebAPIClient},
    commands::MinerCommand,
};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::{Client, Method, Response};
use serde_json::Value;
use std::{net::IpAddr, time::Duration};

/// VNish WebAPI client

#[derive(Debug)]
pub struct EPicWebAPI {
    client: Client,
    pub ip: IpAddr,
    port: u16,
    timeout: Duration,
    _password: Option<String>,
}

#[async_trait]
impl APIClient for EPicWebAPI {
    async fn get_api_result(&self, command: &MinerCommand) -> Result<Value> {
        match command {
            MinerCommand::WebAPI {
                command,
                parameters,
            } => self
                .send_command(command, false, parameters.clone(), Method::GET)
                .await
                .map_err(|e| anyhow!(e.to_string())),
            _ => Err(anyhow!("Cannot send non web command to web API")),
        }
    }
}

#[async_trait]
impl WebAPIClient for EPicWebAPI {
    /// Send a command to the EPic miner API
    async fn send_command(
        &self,
        command: &str,
        _privileged: bool,
        parameters: Option<Value>,
        method: Method,
    ) -> Result<Value> {
        let url = format!("http://{}:{}/{}", self.ip, self.port, command);

        let response = self
            .execute_request(&url, &method, parameters.clone())
            .await?;

        let status = response.status();
        if status.is_success() {
            let json_data = response
                .json()
                .await
                .map_err(|e| EPicError::ParseError(e.to_string()))?;
            Ok(json_data)
        } else {
            Err(EPicError::HttpError(status.as_u16()))?
        }
    }
}

impl EPicWebAPI {
    /// Create a new EPic WebAPI client
    pub fn new(ip: IpAddr, port: u16) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            ip,
            port,
            timeout: Duration::from_secs(5),
            _password: Some("admin".to_string()), // Default password
        }
    }

    /// Execute the actual HTTP request
    async fn execute_request(
        &self,
        url: &str,
        method: &Method,
        parameters: Option<Value>,
    ) -> Result<Response, EPicError> {
        let request_builder = match *method {
            Method::GET => self.client.get(url),
            Method::POST => {
                let mut builder = self.client.post(url);
                if let Some(params) = parameters {
                    builder = builder.json(&params);
                }
                builder
            }
            Method::PATCH => {
                let mut builder = self.client.patch(url);
                if let Some(params) = parameters {
                    builder = builder.json(&params);
                }
                builder
            }
            _ => return Err(EPicError::UnsupportedMethod(method.to_string())),
        };

        let request_builder = request_builder.timeout(self.timeout);

        let request = request_builder
            .build()
            .map_err(|e| EPicError::RequestError(e.to_string()))?;

        let response = self
            .client
            .execute(request)
            .await
            .map_err(|e| EPicError::NetworkError(e.to_string()))?;

        Ok(response)
    }
}

/// Error types for EPic WebAPI operations
#[derive(Debug, Clone)]
pub enum EPicError {
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

impl std::fmt::Display for EPicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EPicError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            EPicError::HttpError(code) => write!(f, "HTTP error: {code}"),
            EPicError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            EPicError::RequestError(msg) => write!(f, "Request error: {msg}"),
            EPicError::Timeout => write!(f, "Request timeout"),
            EPicError::UnsupportedMethod(method) => write!(f, "Unsupported method: {method}"),
            EPicError::MaxRetriesExceeded => write!(f, "Maximum retries exceeded"),
            EPicError::AuthenticationFailed => write!(f, "Authentication failed"),
            EPicError::Unauthorized => write!(f, "Unauthorized access"),
        }
    }
}

impl std::error::Error for EPicError {}
