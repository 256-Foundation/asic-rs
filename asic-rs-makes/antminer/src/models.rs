use std::str::FromStr;

use asic_rs_core::data::device::HashAlgorithm;
use asic_rs_core::errors::ModelSelectionError;
use asic_rs_core::traits::model::MinerModel;
use serde::{Deserialize, Serialize};
use strum::Display;
use ts_rs::TS;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display, TS)]
pub enum AntMinerModel {
    #[serde(alias = "ANTMINER D3")]
    D3,
    #[serde(alias = "ANTMINER HS3")]
    HS3,
    #[serde(alias = "ANTMINER L3+")]
    L3Plus,
    #[serde(alias = "ANTMINER L3++")]
    L3PlusPlus,
    #[serde(alias = "ANTMINER KA3")]
    KA3,
    #[serde(alias = "ANTMINER KS3")]
    KS3,
    #[serde(alias = "ANTMINER DR5")]
    DR5,
    #[serde(alias = "ANTMINER KS5")]
    KS5,
    #[serde(alias = "ANTMINER KS5 PRO")]
    KS5Pro,
    #[serde(alias = "ANTMINER L7")]
    L7,
    #[serde(alias = "ANTMINER K7")]
    K7,
    #[serde(alias = "ANTMINER D7")]
    D7,
    #[serde(alias = "ANTMINER E9 PRO")]
    E9Pro,
    #[serde(alias = "ANTMINER D9")]
    D9,
    #[serde(alias = "ANTMINER S9")]
    S9,
    #[serde(alias = "ANTMINER S9I")]
    S9i,
    #[serde(alias = "ANTMINER S9J")]
    S9j,
    #[serde(alias = "ANTMINER T9")]
    T9,
    #[serde(alias = "ANTMINER L9")]
    L9,
    #[serde(alias = "ANTMINER L11")]
    L11,
    #[serde(alias = "ANTMINER Z15")]
    Z15,
    #[serde(alias = "ANTMINER Z15 PRO")]
    Z15Pro,
    #[serde(alias = "ANTMINER S17")]
    S17,
    #[serde(alias = "ANTMINER S17+")]
    S17Plus,
    #[serde(alias = "ANTMINER S17 PRO")]
    S17Pro,
    #[serde(alias = "ANTMINER S17E")]
    S17e,
    #[serde(alias = "ANTMINER T17")]
    T17,
    #[serde(alias = "ANTMINER T17+")]
    T17Plus,
    #[serde(alias = "ANTMINER T17E")]
    T17e,
    #[serde(alias = "ANTMINER S19")]
    S19,
    #[serde(alias = "ANTMINER S19L")]
    S19L,
    #[serde(alias = "ANTMINER S19 PRO")]
    S19Pro,
    #[serde(alias = "ANTMINER S19J")]
    S19j,
    #[serde(alias = "ANTMINER S19I")]
    S19i,
    #[serde(alias = "ANTMINER S19+")]
    S19Plus,
    #[serde(alias = "ANTMINER S19J88NOPIC")]
    S19jNoPIC,
    #[serde(alias = "ANTMINER S19PRO+")]
    S19ProPlus,
    #[serde(alias = "ANTMINER S19J PRO")]
    S19jPro,
    #[serde(alias = "ANTMINER S19J PRO+")]
    S19jProPlus,
    #[serde(alias = "ANTMINER S19 XP")]
    S19XP,
    #[serde(alias = "ANTMINER S19A")]
    S19a,
    #[serde(alias = "ANTMINER S19A PRO")]
    S19aPro,
    #[serde(alias = "ANTMINER S19 HYDRO")]
    S19Hydro,
    #[serde(alias = "ANTMINER S19 PRO HYD.")]
    #[serde(alias = "ANTMINER S19 PRO HYDRO")]
    S19ProHydro,
    #[serde(alias = "ANTMINER S19 PRO+ HYD.")]
    #[serde(alias = "ANTMINER S19 PRO+ HYDRO")]
    S19ProPlusHydro,
    #[serde(alias = "ANTMINER S19K PRO")]
    S19KPro,
    #[serde(alias = "ANTMINER S19J XP")]
    S19jXP,
    #[serde(alias = "ANTMINER T19")]
    T19,
    #[serde(alias = "ANTMINER S21")]
    #[serde(alias = "ANTMINER BHB68601")]
    #[serde(alias = "ANTMINER BHB68606")]
    S21,
    #[serde(alias = "ANTMINER S21 PRO")]
    S21Pro,
    #[serde(alias = "ANTMINER S21 PRO+")]
    S21ProPlus,
    #[serde(alias = "ANTMINER S21 XP")]
    S21XP,
    #[serde(alias = "ANTMINER S21+")]
    S21Plus,
    #[serde(alias = "ANTMINER S21 HYD.")]
    #[serde(alias = "ANTMINER S21 HYDRO")]
    S21Hydro,
    #[serde(alias = "ANTMINER S21+ HYD.")]
    #[serde(alias = "ANTMINER S21+ HYDRO")]
    S21PlusHydro,
    #[serde(alias = "ANTMINER S21E XP HYD.")]
    #[serde(alias = "ANTMINER S21E XP HYDRO")]
    S21eXPHydro,
    #[serde(alias = "ANTMINER T21")]
    T21,
    #[strum(to_string = "{0}")]
    Unknown(String),
}

impl FromStr for AntMinerModel {
    type Err = ModelSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .or_else(|_| Ok(Self::Unknown(s.to_string())))
    }
}

impl MinerModel for AntMinerModel {
    fn make_name(&self) -> String {
        "Antminer".to_string()
    }
    fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }

    /// AntMiner is a mixed-algorithm make: the L-series mines Scrypt and the
    /// D-series X11, while the S/T-series mines SHA-256. Without this the make
    /// inherits the trait's SHA-256 default and every L9/L11 reports itself as
    /// a SHA-256 miner regardless of firmware.
    ///
    /// Models whose algorithm [`HashAlgorithm`] cannot yet name — KS3/KS5
    /// (kHeavyHash), K7 (Eaglesong), E9 Pro (Ethash), Z15 (Equihash), HS3
    /// (Handshake), DR5 (Blake256R14) — keep reporting SHA-256. That is still
    /// wrong for them, but it is unchanged from today; naming those algorithms
    /// needs new `HashAlgorithm` variants, which is a public API change and is
    /// left to a follow-up.
    fn hash_algorithm(&self) -> HashAlgorithm {
        match self {
            Self::L3Plus | Self::L3PlusPlus | Self::L7 | Self::L9 | Self::L11 => {
                HashAlgorithm::Scrypt
            }
            Self::D3 | Self::D7 | Self::D9 => HashAlgorithm::X11,
            Self::KA3 => HashAlgorithm::Kadena,
            _ => HashAlgorithm::SHA256,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn known_model_parses() {
        // Act
        let result = AntMinerModel::from_str("ANTMINER S21").unwrap();

        // Assert
        assert_eq!(result, AntMinerModel::S21);
    }

    #[test]
    fn unknown_model_falls_back() {
        // Act
        let result = AntMinerModel::from_str("ANTMINER S99").unwrap();

        // Assert
        assert_eq!(result, AntMinerModel::Unknown("ANTMINER S99".to_string()));
    }

    #[test]
    fn l_series_is_scrypt() {
        for model in [
            AntMinerModel::L3Plus,
            AntMinerModel::L3PlusPlus,
            AntMinerModel::L7,
            AntMinerModel::L9,
            AntMinerModel::L11,
        ] {
            assert_eq!(
                model.hash_algorithm(),
                HashAlgorithm::Scrypt,
                "{model} should be Scrypt"
            );
        }
    }

    #[test]
    fn d_series_is_x11() {
        for model in [AntMinerModel::D3, AntMinerModel::D7, AntMinerModel::D9] {
            assert_eq!(
                model.hash_algorithm(),
                HashAlgorithm::X11,
                "{model} should be X11"
            );
        }
    }

    #[test]
    fn sha256_models_are_unchanged() {
        for model in [
            AntMinerModel::S9,
            AntMinerModel::S19,
            AntMinerModel::S21,
            AntMinerModel::T21,
        ] {
            assert_eq!(
                model.hash_algorithm(),
                HashAlgorithm::SHA256,
                "{model} should be SHA256"
            );
        }
    }

    #[test]
    fn unknown_model_defaults_to_sha256() {
        let model = AntMinerModel::from_str("ANTMINER S99").unwrap();

        assert_eq!(model.hash_algorithm(), HashAlgorithm::SHA256);
    }
}
