use std::str::FromStr;

use asic_rs_core::errors::ModelSelectionError;
use asic_rs_core::traits::model::MinerModel;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumProperty};
use ts_rs::TS;

#[derive(
    Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display, EnumIter, EnumProperty, TS,
)]
pub enum NerdAxeModel {
    #[serde(alias = "BM1368")]
    #[strum(props(algo = "SHA256"))]
    NerdAxe,
    #[serde(alias = "BM1370", alias = "nerdqaxe++", alias = "NerdQAxe++")]
    #[strum(props(algo = "SHA256"))]
    NerdQAxe,
    #[serde(alias = "BM1397")]
    #[strum(props(algo = "SHA256"))]
    NerdMiner,
    #[serde(alias = "BM1366")]
    #[strum(props(algo = "SHA256"))]
    NerdAxeUltra,
    #[strum(to_string = "{0}")]
    #[strum(props(algo = "SHA256"))]
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

    use asic_rs_core::data::device::HashAlgorithm;
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn every_model_declares_a_valid_algorithm() {
        for model in NerdAxeModel::iter() {
            let declared = model.get_str("algo").expect("property declared");
            let expected = declared.parse::<HashAlgorithm>().expect("valid algorithm");
            assert_eq!(model.hash_algorithm(), expected, "{model}");
        }
    }

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
