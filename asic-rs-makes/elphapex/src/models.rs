use std::str::FromStr;

use asic_rs_core::{
    data::device::HashAlgorithm, errors::ModelSelectionError, traits::model::MinerModel,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumProperty};
use ts_rs::TS;

#[derive(
    Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display, EnumIter, EnumProperty, TS,
)]
pub enum ElphapexModel {
    #[serde(alias = "DG1")]
    #[strum(props(algo = "Scrypt"))]
    DG1,
    #[serde(alias = "DG1+", alias = "DG1Plus")]
    #[strum(props(algo = "Scrypt"))]
    DG1Plus,
    #[serde(alias = "DG-Home1")]
    #[strum(props(algo = "Scrypt"))]
    DG1Home,
    #[strum(to_string = "{0}")]
    #[strum(props(algo = "Scrypt"))]
    Unknown(String),
}

impl FromStr for ElphapexModel {
    type Err = ModelSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let model = s.trim().to_string();
        serde_json::from_value(serde_json::Value::String(model.clone()))
            .or(Ok(Self::Unknown(model)))
    }
}

impl MinerModel for ElphapexModel {
    fn make_name(&self) -> String {
        "Elphapex".to_string()
    }

    fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }

    fn hash_algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::Scrypt
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn every_model_declares_a_valid_algorithm() {
        for model in ElphapexModel::iter() {
            let declared = model.get_str("algo").expect("property declared");
            let expected = declared.parse::<HashAlgorithm>().expect("valid algorithm");
            assert_eq!(model.hash_algorithm(), expected, "{model}");
        }
    }

    #[test]
    fn known_models_parse() {
        assert_eq!(ElphapexModel::from_str("DG1").unwrap(), ElphapexModel::DG1);
        assert_eq!(
            ElphapexModel::from_str("DG1+").unwrap(),
            ElphapexModel::DG1Plus
        );
        assert_eq!(
            ElphapexModel::from_str("DG-Home1").unwrap(),
            ElphapexModel::DG1Home
        );
    }

    #[test]
    fn unknown_model_falls_back_to_raw_name() {
        assert_eq!(
            ElphapexModel::from_str("dg-x").unwrap(),
            ElphapexModel::Unknown("dg-x".to_string())
        );
    }
}
