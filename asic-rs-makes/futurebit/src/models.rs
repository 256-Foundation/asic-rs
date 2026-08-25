use std::str::FromStr;

use asic_rs_core::errors::ModelSelectionError;
use asic_rs_core::traits::model::MinerModel;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumProperty};
use ts_rs::TS;

#[derive(
    Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display, EnumIter, EnumProperty, TS,
)]
pub enum FutureBitModel {
    #[serde(
        alias = "Apollo",
        alias = "Apollo BTC",
        alias = "Apollo-BTC",
        alias = "Apollo I",
        alias = "Apollo 1"
    )]
    #[strum(props(algo = "SHA256"))]
    Apollo1,
    #[serde(
        alias = "Apollo II",
        alias = "Apollo 2",
        alias = "Apollo-2",
        alias = "Apollo-BTC II",
        alias = "Apollo BTC II"
    )]
    #[strum(props(algo = "SHA256"))]
    Apollo2,
    #[strum(to_string = "{0}")]
    #[strum(props(algo = "SHA256"))]
    Unknown(String),
}

impl FromStr for FutureBitModel {
    type Err = ModelSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .or_else(|_| Ok(Self::Unknown(s.to_string())))
    }
}

impl MinerModel for FutureBitModel {
    fn make_name(&self) -> String {
        "FutureBit".to_string()
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
        for model in FutureBitModel::iter() {
            let declared = model.get_str("algo").expect("property declared");
            let expected = declared.parse::<HashAlgorithm>().expect("valid algorithm");
            assert_eq!(model.hash_algorithm(), expected, "{model}");
        }
    }
}
