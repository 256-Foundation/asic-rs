use std::str::FromStr;

use asic_rs_core::data::device::HashAlgorithm;
use asic_rs_core::errors::ModelSelectionError;
use asic_rs_core::traits::model::MinerModel;
use asic_rs_macros::ModelAlgorithm;
use serde::{Deserialize, Serialize};
use strum::Display;
use ts_rs::TS;

#[derive(
    Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display, ModelAlgorithm, TS,
)]
pub enum BraiinsModel {
    #[serde(alias = "BRAIINS MINI MINER BMM 100")]
    #[algorithm(HashAlgorithm::SHA256)]
    BMM100,
    #[serde(alias = "BRAIINS MINI MINER BMM 101")]
    #[algorithm(HashAlgorithm::SHA256)]
    BMM101,
    #[strum(to_string = "{0}")]
    #[algorithm(HashAlgorithm::Unknown)]
    Unknown(String),
}

impl FromStr for BraiinsModel {
    type Err = ModelSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .or_else(|_| Ok(Self::Unknown(s.to_string())))
    }
}

impl MinerModel for BraiinsModel {
    fn make_name(&self) -> String {
        "Braiins".to_string()
    }
    fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn known_model_parses() {
        // Act
        let result = BraiinsModel::from_str("BRAIINS MINI MINER BMM 100").unwrap();

        // Assert
        assert_eq!(result, BraiinsModel::BMM100);
    }

    #[test]
    fn unknown_model_falls_back() {
        // Act
        let result = BraiinsModel::from_str("BRAIINS MINI MINER BMM 999").unwrap();

        // Assert
        assert_eq!(
            result,
            BraiinsModel::Unknown("BRAIINS MINI MINER BMM 999".to_string())
        );
    }
}
