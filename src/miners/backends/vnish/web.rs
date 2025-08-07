use crate::miners::{
    api::{APIClient, WebAPIClient},
    commands::MinerCommand,
};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::{Client, Method, Response};
use serde_json::Value;
use std::{net::IpAddr, time::Duration};

/// Vnish WebAPI client for communicating with VnishOS miners
pub struct VnishWebAPI {
    client: Client,
    pub ip: IpAddr,
    port: u16,
    timeout: Duration,
    retries: u32,
    api_key: Option<String>,
    bearer_token: Option<String>,
}

impl std::fmt::Debug for VnishWebAPI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VnishWebAPI")
            .field("ip", &self.ip)
            .field("port", &self.port)
            .field("timeout", &self.timeout)
            .field("retries", &self.retries)
            .field("api_key", &self.api_key.as_ref().map(|_| "***"))
            .field("bearer_token", &self.bearer_token.as_ref().map(|_| "***"))
            .finish()
    }
}

#[async_trait]
impl APIClient for VnishWebAPI {
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
impl WebAPIClient for VnishWebAPI {
    /// Send a command to the Vnish miner API
    async fn send_command(
        &self,
        command: &str,
        _privileged: bool,
        parameters: Option<Value>,
        method: Method,
    ) -> Result<Value> {
        let url = format!("http://{}:{}/api/v1/{}", self.ip, self.port, command);

        for attempt in 0..=self.retries {
            let result = self
                .execute_request(&url, &method, parameters.clone())
                .await;

            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json().await {
                            Ok(json_data) => return Ok(json_data),
                            Err(e) => {
                                if attempt == self.retries {
                                    return Err(VnishError::ParseError(e.to_string()))?;
                                }
                            }
                        }
                    } else if attempt == self.retries {
                        return Err(VnishError::HttpError(response.status().as_u16()))?;
                    }
                }
                Err(e) => {
                    if attempt == self.retries {
                        return Err(e)?;
                    }
                }
            }
        }

        Err(VnishError::MaxRetriesExceeded)?
    }
}

impl VnishWebAPI {
    /// Create a new Vnish WebAPI client
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
            retries: 2,
            api_key: None,
            bearer_token: None,
        }
    }

    /// Set the timeout for API requests
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the number of retries for failed requests
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    /// Set API key for authentication (x-api-key header)
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Set bearer token for authentication (Authorization: Bearer header)
    pub fn with_bearer_token(mut self, token: String) -> Self {
        self.bearer_token = Some(token);
        self
    }

    /// Execute the actual HTTP request
    async fn execute_request(
        &self,
        url: &str,
        method: &Method,
        parameters: Option<Value>,
    ) -> Result<Response, VnishError> {
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
            _ => return Err(VnishError::UnsupportedMethod(method.to_string())),
        };

        let mut request_builder = request_builder.timeout(self.timeout);

        // Add authentication headers if provided
        if let Some(ref api_key) = self.api_key {
            request_builder = request_builder.header("x-api-key", api_key);
        }
        if let Some(ref token) = self.bearer_token {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
        }

        let request = request_builder
            .build()
            .map_err(|e| VnishError::RequestError(e.to_string()))?;

        let response = self
            .client
            .execute(request)
            .await
            .map_err(|e| VnishError::NetworkError(e.to_string()))?;

        Ok(response)
    }
}

/// Error types for Vnish WebAPI operations
#[derive(Debug, Clone)]
pub enum VnishError {
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
}

impl std::fmt::Display for VnishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VnishError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            VnishError::HttpError(code) => write!(f, "HTTP error: {}", code),
            VnishError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            VnishError::RequestError(msg) => write!(f, "Request error: {}", msg),
            VnishError::Timeout => write!(f, "Request timeout"),
            VnishError::UnsupportedMethod(method) => write!(f, "Unsupported method: {}", method),
            VnishError::MaxRetriesExceeded => write!(f, "Maximum retries exceeded"),
        }
    }
}

impl std::error::Error for VnishError {}
