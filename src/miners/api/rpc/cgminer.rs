use crate::miners::api::ApiClient;
use crate::miners::util::send_rpc_command;
use async_trait::async_trait;
use serde_json::Value;
use std::net::IpAddr;

pub struct CGMinerRPC {
    ip: IpAddr,
}

impl CGMinerRPC {
    pub fn new(ip: IpAddr) -> Self {
        Self { ip }
    }
}

impl CGMinerRPC {}


#[async_trait]
impl ApiClient for CGMinerRPC {
    async fn send_command(&self, command: &'static str) -> Result<Value, String> {
        match send_rpc_command(&self.ip, command).await {
            Some(data) => Ok(data),
            None => Err(String::from("Failed to send command")),
        }
    }
}
