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
pub enum VolcMinerModel {
    #[serde(alias = "VOLCMINER D1")]
    #[strum(props(algo = "Scrypt"))]
    D1,
    #[strum(to_string = "{0}")]
    #[strum(props(algo = "Scrypt"))]
    Unknown(String),
}

impl FromStr for VolcMinerModel {
    type Err = ModelSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .or_else(|_| Ok(Self::Unknown(s.to_string())))
    }
}

impl MinerModel for VolcMinerModel {
    fn make_name(&self) -> String {
        "VolcMiner".to_string()
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
        for model in VolcMinerModel::iter() {
            let declared = model.get_str("algo").expect("property declared");
            let expected = declared.parse::<HashAlgorithm>().expect("valid algorithm");
            assert_eq!(model.hash_algorithm(), expected, "{model}");
        }
    }

    #[test]
    fn known_model_parses() {
        let result = VolcMinerModel::from_str("VOLCMINER D1").unwrap();

        assert_eq!(result, VolcMinerModel::D1);
    }

    #[test]
    fn unknown_model_falls_back() {
        let result = VolcMinerModel::from_str("VOLCMINER DX").unwrap();

        assert_eq!(result, VolcMinerModel::Unknown("VOLCMINER DX".to_string()));
    }
}
