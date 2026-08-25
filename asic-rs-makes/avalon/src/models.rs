use asic_rs_core::traits::model::MinerModel;
use std::str::FromStr;

use asic_rs_core::errors::ModelSelectionError;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumProperty};
use ts_rs::TS;

#[derive(
    Debug, Display, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, EnumIter, EnumProperty, TS,
)]
pub enum AvalonMinerModel {
    #[serde(alias = "721")]
    #[strum(props(algo = "SHA256"))]
    Avalon721,
    #[serde(alias = "741")]
    #[strum(props(algo = "SHA256"))]
    Avalon741,
    #[serde(alias = "761")]
    #[strum(props(algo = "SHA256"))]
    Avalon761,
    #[serde(alias = "821")]
    #[strum(props(algo = "SHA256"))]
    Avalon821,
    #[serde(alias = "841")]
    #[strum(props(algo = "SHA256"))]
    Avalon841,
    #[serde(alias = "851")]
    #[strum(props(algo = "SHA256"))]
    Avalon851,
    #[serde(alias = "921")]
    #[strum(props(algo = "SHA256"))]
    Avalon921,
    #[serde(alias = "1026")]
    #[strum(props(algo = "SHA256"))]
    Avalon1026,
    #[serde(alias = "1047")]
    #[strum(props(algo = "SHA256"))]
    Avalon1047,
    #[serde(alias = "1066")]
    #[strum(props(algo = "SHA256"))]
    Avalon1066,
    #[serde(alias = "1166PRO")]
    #[strum(props(algo = "SHA256"))]
    Avalon1166Pro,
    #[serde(alias = "1126PRO")]
    #[strum(props(algo = "SHA256"))]
    Avalon1126Pro,
    #[serde(alias = "1246")]
    #[strum(props(algo = "SHA256"))]
    Avalon1246,
    #[serde(alias = "1466")]
    #[strum(props(algo = "SHA256"))]
    Avalon1466,
    #[serde(alias = "15")]
    #[serde(alias = "15PRO")]
    #[strum(props(algo = "SHA256"))]
    Avalon15,
    #[serde(alias = "1566")]
    #[strum(props(algo = "SHA256"))]
    Avalon1566,
    #[serde(alias = "1566HA")]
    #[strum(props(algo = "SHA256"))]
    Avalon1566Ha,
    #[serde(alias = "1566HU")]
    #[strum(props(algo = "SHA256"))]
    Avalon1566Hu,
    #[serde(alias = "NANO3")]
    #[strum(props(algo = "SHA256"))]
    AvalonNano3,
    #[serde(alias = "NANO3S")]
    #[strum(props(algo = "SHA256"))]
    AvalonNano3s,
    #[serde(alias = "Q")]
    #[strum(props(algo = "SHA256"))]
    AvalonHomeQ,
    #[strum(to_string = "{0}")]
    #[strum(props(algo = "SHA256"))]
    Unknown(String),
}

impl FromStr for AvalonMinerModel {
    type Err = ModelSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .or_else(|_| Ok(Self::Unknown(s.to_string())))
    }
}

impl MinerModel for AvalonMinerModel {
    fn make_name(&self) -> String {
        "Avalonminer".to_string()
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
        for model in AvalonMinerModel::iter() {
            let declared = model.get_str("algo").expect("property declared");
            let expected = declared.parse::<HashAlgorithm>().expect("valid algorithm");
            assert_eq!(model.hash_algorithm(), expected, "{model}");
        }
    }

    #[test]
    fn known_model_parses() {
        // Act
        let result = AvalonMinerModel::from_str("1466").unwrap();

        // Assert
        assert_eq!(result, AvalonMinerModel::Avalon1466);
    }

    #[test]
    fn unknown_model_falls_back() {
        // Act
        let result = AvalonMinerModel::from_str("9999").unwrap();

        // Assert
        assert_eq!(result, AvalonMinerModel::Unknown("9999".to_string()));
    }

    #[test]
    fn recent_model_aliases_parse() {
        assert_eq!(
            AvalonMinerModel::from_str("15PRO").unwrap(),
            AvalonMinerModel::Avalon15
        );
        assert_eq!(
            AvalonMinerModel::from_str("1566HA").unwrap(),
            AvalonMinerModel::Avalon1566Ha
        );
        assert_eq!(
            AvalonMinerModel::from_str("1566HU").unwrap(),
            AvalonMinerModel::Avalon1566Hu
        );
    }
}
