use std::str::FromStr;

use asic_rs_core::errors::ModelSelectionError;
use asic_rs_core::traits::model::MinerModel;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumProperty};
use ts_rs::TS;

#[derive(
    Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display, EnumIter, EnumProperty, TS,
)]
pub enum BitaxeModel {
    #[serde(alias = "BM1368")]
    #[strum(props(algo = "SHA256"))]
    Supra,
    #[serde(alias = "BM1370")]
    #[strum(props(algo = "SHA256"))]
    Gamma,
    #[serde(alias = "BM1397")]
    #[strum(props(algo = "SHA256"))]
    Max,
    #[serde(alias = "BM1366")]
    #[strum(props(algo = "SHA256"))]
    Ultra,
    #[strum(to_string = "{0}")]
    #[strum(props(algo = "SHA256"))]
    Unknown(String),
}

impl FromStr for BitaxeModel {
    type Err = ModelSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .or_else(|_| Ok(Self::Unknown(s.to_string())))
    }
}

impl MinerModel for BitaxeModel {
    fn make_name(&self) -> String {
        "Bitaxe".to_string()
    }
    fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use asic_rs_core::data::device::HashAlgorithm;
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn every_model_declares_a_valid_algorithm() {
        for model in BitaxeModel::iter() {
            let declared = model.get_str("algo").expect("property declared");
            let expected = declared.parse::<HashAlgorithm>().expect("valid algorithm");
            assert_eq!(model.hash_algorithm(), expected, "{model}");
        }
    }

    #[test]
    fn known_model_parses() {
        // Act
        let result = BitaxeModel::from_str("BM1370").unwrap();

        // Assert
        assert_eq!(result, BitaxeModel::Gamma);
    }

    #[test]
    fn unknown_model_falls_back() {
        // Act
        let result = BitaxeModel::from_str("BM9999").unwrap();

        // Assert
        assert_eq!(result, BitaxeModel::Unknown("BM9999".to_string()));
    }
}
