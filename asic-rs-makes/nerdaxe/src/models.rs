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
pub enum NerdAxeModel {
    #[serde(alias = "BM1368")]
    #[algorithm(HashAlgorithm::SHA256)]
    NerdAxe,
    #[serde(alias = "BM1370", alias = "nerdqaxe++", alias = "NerdQAxe++")]
    #[algorithm(HashAlgorithm::SHA256)]
    NerdQAxe,
    #[serde(alias = "BM1397")]
    #[algorithm(HashAlgorithm::SHA256)]
    NerdMiner,
    #[serde(alias = "BM1366")]
    #[algorithm(HashAlgorithm::SHA256)]
    NerdAxeUltra,
    #[strum(to_string = "{0}")]
    #[algorithm(HashAlgorithm::Unknown)]
    Unknown(String),
}

impl FromStr for NerdAxeModel {
    type Err = ModelSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .or_else(|_| Ok(Self::Unknown(s.to_string())))
    }
}

impl MinerModel for NerdAxeModel {
    fn make_name(&self) -> String {
        "Nerdaxe".to_string()
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
    fn parsing() {
        #[track_caller]
        fn case(s: &str, expected: NerdAxeModel) {
            assert_eq!(NerdAxeModel::from_str(s).unwrap(), expected);
        }

        case("NerdAxe", NerdAxeModel::NerdAxe);
        case("NerdQAxe", NerdAxeModel::NerdQAxe);
        case("NerdMiner", NerdAxeModel::NerdMiner);
        case("NerdAxeUltra", NerdAxeModel::NerdAxeUltra);
    }

    #[test]
    fn unknown_model_falls_back() {
        // Act
        let result = NerdAxeModel::from_str("NerdAxeXXX").unwrap();

        // Assert
        assert_eq!(result, NerdAxeModel::Unknown("NerdAxeXXX".to_string()));
    }
}
