use std::net::IpAddr;

use anyhow;
use asic_rs_core::{
    data::command::{MinerCommand, RPCCommandStatus},
    errors::RPCError,
    traits::miner::*,
    util::{DEFAULT_RPC_TIMEOUT, connect_tcp_stream, read_stream_response, write_all_with_timeout},
};
use async_trait::async_trait;
use serde_json::{Value, json};

/// Build the JSON request for a cgminer-style RPC call.
///
/// Bitmain's API extensions — `new_api` being the one this backend uses — are
/// top-level flags alongside `command`, not values of cgminer's `parameter`
/// argument. Wrapping them makes the firmware ignore the flag and silently
/// answer with the legacy payload, which is indistinguishable from a
/// successful call.
///
/// An object is always one of those extensions; cgminer's own convention
/// passes a scalar (e.g. `switchpool`'s pool index), so scalars keep the
/// `parameter` wrapper.
fn build_rpc_request(command: &str, parameters: Option<Value>) -> Value {
    match parameters {
        Some(Value::Object(params)) => {
            let mut request = params;
            // Inserted last so a stray `command` key cannot displace it.
            request.insert("command".to_string(), Value::from(command));
            Value::Object(request)
        }
        Some(params) => json!({
            "command": command,
            "parameter": params
        }),
        None => json!({
            "command": command
        }),
    }
}

#[derive(Debug)]
pub struct AntMinerRPCAPI {
    ip: IpAddr,
    port: u16,
}

#[allow(dead_code)]
impl AntMinerRPCAPI {
    pub fn new(ip: IpAddr) -> Self {
        Self { ip, port: 4028 }
    }

    async fn send_rpc_command(
        &self,
        command: &str,
        _privileged: bool,
        parameters: Option<Value>,
    ) -> anyhow::Result<Value> {
        let request = build_rpc_request(command, parameters);

        let json_str = request.to_string();
        let message = format!("{}\n", json_str);

        let response = {
            let mut stream = connect_tcp_stream((self.ip, self.port), DEFAULT_RPC_TIMEOUT)
                .await
                .map_err(|_| RPCError::ConnectionFailed)?;

            write_all_with_timeout(&mut stream, message.as_bytes(), DEFAULT_RPC_TIMEOUT).await?;
            read_stream_response(&mut stream, DEFAULT_RPC_TIMEOUT).await
        };
        let response = response?;
        self.parse_rpc_result(&response)
    }

    fn parse_rpc_result(&self, response: &str) -> anyhow::Result<Value> {
        let status = RPCCommandStatus::from_antminer(response)?;
        match status.into_result() {
            Ok(_) => Ok(serde_json::from_str(response)?),
            Err(e) => Err(e)?,
        }
    }

    pub async fn stats(&self, new_api: bool) -> anyhow::Result<Value> {
        if new_api {
            self.send_rpc_command("stats", false, Some(json!({"new_api": true})))
                .await
        } else {
            self.send_rpc_command("stats", false, None).await
        }
    }

    pub async fn summary(&self, new_api: bool) -> anyhow::Result<Value> {
        if new_api {
            self.send_rpc_command("summary", false, Some(json!({"new_api": true})))
                .await
        } else {
            self.send_rpc_command("summary", false, None).await
        }
    }

    pub async fn pools(&self, new_api: bool) -> anyhow::Result<Value> {
        if new_api {
            self.send_rpc_command("pools", false, Some(json!({"new_api": true})))
                .await
        } else {
            self.send_rpc_command("pools", false, None).await
        }
    }

    pub async fn version(&self) -> anyhow::Result<Value> {
        self.send_rpc_command("version", false, None).await
    }

    pub async fn rate(&self) -> anyhow::Result<Value> {
        self.send_rpc_command("rate", false, Some(json!({"new_api": true})))
            .await
    }

    pub async fn warning(&self) -> anyhow::Result<Value> {
        self.send_rpc_command("warning", false, Some(json!({"new_api": true})))
            .await
    }

    pub async fn reload(&self) -> anyhow::Result<Value> {
        self.send_rpc_command("reload", false, Some(json!({"new_api": true})))
            .await
    }
}

#[async_trait]
impl APIClient for AntMinerRPCAPI {
    async fn get_api_result(&self, command: &MinerCommand) -> anyhow::Result<Value> {
        match command {
            MinerCommand::RPC {
                command,
                parameters,
            } => self
                .send_rpc_command(command, false, parameters.clone())
                .await
                .map_err(|e| anyhow::anyhow!(e.to_string())),
            _ => Err(anyhow::anyhow!("Unsupported command type for RPC client")),
        }
    }
}

#[async_trait]
impl RPCAPIClient for AntMinerRPCAPI {
    async fn send_command(
        &self,
        command: &str,
        privileged: bool,
        parameters: Option<Value>,
    ) -> anyhow::Result<Value> {
        self.send_rpc_command(command, privileged, parameters).await
    }
}

trait StatusFromAntMiner {
    fn from_antminer(response: &str) -> Result<Self, RPCError>
    where
        Self: Sized;
}

impl StatusFromAntMiner for RPCCommandStatus {
    fn from_antminer(response: &str) -> Result<Self, RPCError> {
        let value: Value = serde_json::from_str(response)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_parameters_become_top_level_flags() {
        // The firmware only honours `new_api` at the top level; nested under
        // `parameter` it is ignored and the legacy payload comes back.
        assert_eq!(
            build_rpc_request("stats", Some(json!({"new_api": true}))),
            json!({"command": "stats", "new_api": true})
        );
    }

    #[test]
    fn scalar_parameters_keep_the_cgminer_wrapper() {
        // cgminer's own convention, e.g. `switchpool` with a pool index.
        assert_eq!(
            build_rpc_request("switchpool", Some(json!("1"))),
            json!({"command": "switchpool", "parameter": "1"})
        );
    }

    #[test]
    fn absent_parameters_send_the_bare_command() {
        assert_eq!(
            build_rpc_request("version", None),
            json!({"command": "version"})
        );
    }

    #[test]
    fn command_wins_over_a_conflicting_parameter_key() {
        assert_eq!(
            build_rpc_request("stats", Some(json!({"command": "evil"}))),
            json!({"command": "stats"})
        );
    }
}
