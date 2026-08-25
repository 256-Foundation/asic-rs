use std::str::FromStr;

use asic_rs_core::errors::ModelSelectionError;
use asic_rs_core::traits::model::MinerModel;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumProperty};
use ts_rs::TS;

#[derive(
    Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display, EnumIter, EnumProperty, TS,
)]
pub enum AuradineModel {
    #[serde(alias = "AI2500")]
    #[strum(props(algo = "SHA256"))]
    AI2500,
    #[serde(alias = "AT1500")]
    #[strum(props(algo = "SHA256"))]
    AT1500,
    #[serde(alias = "AI3680")]
    #[strum(props(algo = "SHA256"))]
    AI3680,
    #[serde(alias = "AT2880")]
    #[strum(props(algo = "SHA256"))]
    AT2880,
    #[serde(alias = "AH3880")]
    #[strum(props(algo = "SHA256"))]
    AH3880,
    #[strum(to_string = "{0}")]
    #[strum(props(algo = "SHA256"))]
    Unknown(String),
}

impl FromStr for AuradineModel {
    type Err = ModelSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_ascii_uppercase();
        serde_json::from_value(serde_json::Value::String(normalized.clone()))
            .or(Ok(Self::Unknown(normalized)))
    }
}

impl MinerModel for AuradineModel {
    fn make_name(&self) -> String {
        "Auradine".to_string()
    }
    fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }
}

#[cfg(test)]
mod tests {
    use asic_rs_core::data::device::HashAlgorithm;
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn every_model_declares_a_valid_algorithm() {
        for model in AuradineModel::iter() {
            let declared = model.get_str("algo").expect("property declared");
            let expected = declared.parse::<HashAlgorithm>().expect("valid algorithm");
            assert_eq!(model.hash_algorithm(), expected, "{model}");
        }
    }

    #[test]
    fn known_model_parses() {
        let model = AuradineModel::from_str("at1500").expect("model parses");
        assert_eq!(model, AuradineModel::AT1500);
    }

    #[test]
    fn at2880_model_parses() {
        let model = AuradineModel::from_str("AT2880").expect("model parses");
        assert_eq!(model, AuradineModel::AT2880);
    }

    #[test]
    fn unknown_model_falls_back() {
        let model = AuradineModel::from_str("AX9999").expect("model parses");
        assert_eq!(model, AuradineModel::Unknown("AX9999".to_string()));
    }
}
