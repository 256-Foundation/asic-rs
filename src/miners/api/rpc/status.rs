use crate::miners::api::rpc::errors::RPCError;

pub enum RPCCommandStatus {
    Success,
    Information,
    Error(String),
    Unknown,
}

impl RPCCommandStatus {
    pub fn into_result(self) -> Result<(), RPCError> {
        match self {
            RPCCommandStatus::Success => Ok(()),
            RPCCommandStatus::Information => Ok(()),
            RPCCommandStatus::Error(msg) => Err(RPCError::StatusCheckFailed(msg)),
            RPCCommandStatus::Unknown => {
                Err(RPCError::StatusCheckFailed(String::from("Unknown status")))
            }
        }
    }

    pub fn from_str(response: &str, message: Option<&str>) -> Self {
        match response {
            "S" => RPCCommandStatus::Success,
            "I" => RPCCommandStatus::Information,
            "E" => RPCCommandStatus::Error(message.unwrap_or("Unknown error").to_string()),
            _ => RPCCommandStatus::Unknown,
        }
    }

    pub fn from_luxminer(response: &str) -> Result<Self, RPCError> {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(response)
            && let Some(status_array) = json.get("STATUS").and_then(|s| s.as_array())
            && let Some(status_obj) = status_array.first()
            && let Some(status) = status_obj.get("STATUS").and_then(|s| s.as_str())
        {
            let message = status_obj.get("Msg").and_then(|m| m.as_str());
            return Ok(Self::from_str(status, message));
        }
        Err(RPCError::StatusCheckFailed(
            "Failed to parse status from LuxMiner response".to_string(),
        ))
    }
}
