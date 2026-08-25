use std::str::FromStr;

use asic_rs_core::errors::ModelSelectionError;
use asic_rs_core::traits::model::MinerModel;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumProperty};
use ts_rs::TS;

#[derive(
    Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display, EnumIter, EnumProperty, TS,
)]
pub enum EPicModel {
    #[serde(alias = "BLOCKMINER 520i")]
    #[strum(props(algo = "SHA256"))]
    BM520i,
    #[serde(alias = "ANTMINER S19J PRO DUAL")]
    #[strum(props(algo = "SHA256"))]
    S19JProDual,
    #[strum(to_string = "{0}")]
    #[strum(props(algo = "SHA256"))]
    Unknown(String),
}

impl FromStr for EPicModel {
    type Err = ModelSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .or_else(|_| Ok(Self::Unknown(s.to_string())))
    }
}

impl MinerModel for EPicModel {
    fn make_name(&self) -> String {
        "ePIC".to_string()
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
        for model in EPicModel::iter() {
            let declared = model.get_str("algo").expect("property declared");
            let expected = declared.parse::<HashAlgorithm>().expect("valid algorithm");
            assert_eq!(model.hash_algorithm(), expected, "{model}");
        }
    }

    #[test]
    fn known_model_parses() {
        // Act
        let result = EPicModel::from_str("BLOCKMINER 520i").unwrap();

        // Assert
        assert_eq!(result, EPicModel::BM520i);
    }

    #[test]
    fn unknown_model_falls_back() {
        // Act
        let result = EPicModel::from_str("BLOCKMINER 999").unwrap();

        // Assert
        assert_eq!(result, EPicModel::Unknown("BLOCKMINER 999".to_string()));
    }
}
