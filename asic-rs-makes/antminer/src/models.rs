use std::str::FromStr;

use asic_rs_core::data::device::HashAlgorithm;
use asic_rs_core::errors::ModelSelectionError;
use asic_rs_core::traits::model::MinerModel;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumProperty};
use ts_rs::TS;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display, EnumProperty, TS)]
pub enum AntMinerModel {
    #[serde(alias = "ANTMINER D3")]
    #[strum(props(algo = "X11"))]
    D3,
    #[serde(alias = "ANTMINER HS3")]
    #[strum(props(algo = "Handshake"))]
    HS3,
    #[serde(alias = "ANTMINER L3+")]
    #[strum(props(algo = "Scrypt"))]
    L3Plus,
    #[serde(alias = "ANTMINER L3++")]
    #[strum(props(algo = "Scrypt"))]
    L3PlusPlus,
    #[serde(alias = "ANTMINER KA3")]
    #[strum(props(algo = "Kadena"))]
    KA3,
    #[serde(alias = "ANTMINER KS3")]
    #[strum(props(algo = "KHeavyHash"))]
    KS3,
    #[serde(alias = "ANTMINER DR5")]
    #[strum(props(algo = "Blake256R14"))]
    DR5,
    #[serde(alias = "ANTMINER KS5")]
    #[strum(props(algo = "KHeavyHash"))]
    KS5,
    #[serde(alias = "ANTMINER KS5 PRO")]
    #[strum(props(algo = "KHeavyHash"))]
    KS5Pro,
    #[serde(alias = "ANTMINER L7")]
    #[strum(props(algo = "Scrypt"))]
    L7,
    #[serde(alias = "ANTMINER K7")]
    #[strum(props(algo = "Eaglesong"))]
    K7,
    #[serde(alias = "ANTMINER D7")]
    #[strum(props(algo = "X11"))]
    D7,
    #[serde(alias = "ANTMINER E9 PRO")]
    #[strum(props(algo = "EtHash"))]
    E9Pro,
    #[serde(alias = "ANTMINER D9")]
    #[strum(props(algo = "X11"))]
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
    #[strum(props(algo = "Scrypt"))]
    L9,
    #[serde(alias = "ANTMINER L11")]
    #[strum(props(algo = "Scrypt"))]
    L11,
    #[serde(alias = "ANTMINER Z15")]
    #[strum(props(algo = "Equihash"))]
    Z15,
    #[serde(alias = "ANTMINER Z15 PRO")]
    #[strum(props(algo = "Equihash"))]
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
    /// The algorithm is declared on the variant itself via `strum`'s `algo`
    /// property rather than in a second `match`, so adding a model means
    /// touching one place and the value sits next to its siblings where it is
    /// hard to miss. An absent property means SHA-256, which covers the bulk
    /// of the make.
    ///
    /// Every model that is not SHA-256 carries a property, so the fallback
    /// now applies only to the S/T-series and to models this crate does not
    /// recognise.
    fn hash_algorithm(&self) -> HashAlgorithm {
        self.get_str("algo")
            .and_then(|algo| HashAlgorithm::from_str(algo).ok())
            .unwrap_or(HashAlgorithm::SHA256)
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
    fn non_sha256_models_declare_their_algorithm() {
        for (model, expected) in [
            (AntMinerModel::HS3, HashAlgorithm::Handshake),
            (AntMinerModel::DR5, HashAlgorithm::Blake256R14),
            (AntMinerModel::KA3, HashAlgorithm::Kadena),
            (AntMinerModel::KS3, HashAlgorithm::KHeavyHash),
            (AntMinerModel::KS5, HashAlgorithm::KHeavyHash),
            (AntMinerModel::KS5Pro, HashAlgorithm::KHeavyHash),
            (AntMinerModel::K7, HashAlgorithm::Eaglesong),
            (AntMinerModel::E9Pro, HashAlgorithm::EtHash),
            (AntMinerModel::Z15, HashAlgorithm::Equihash),
            (AntMinerModel::Z15Pro, HashAlgorithm::Equihash),
        ] {
            assert_eq!(model.hash_algorithm(), expected, "{model}");
        }
    }

    /// A property naming something that is not a real [`HashAlgorithm`] would
    /// silently fall back to SHA-256, so every declared property is checked to
    /// parse and to resolve to something other than the fallback.
    #[test]
    fn every_declared_property_resolves() {
        for model in [
            AntMinerModel::D3,
            AntMinerModel::HS3,
            AntMinerModel::L3Plus,
            AntMinerModel::L3PlusPlus,
            AntMinerModel::KA3,
            AntMinerModel::KS3,
            AntMinerModel::DR5,
            AntMinerModel::KS5,
            AntMinerModel::KS5Pro,
            AntMinerModel::L7,
            AntMinerModel::K7,
            AntMinerModel::D7,
            AntMinerModel::E9Pro,
            AntMinerModel::D9,
            AntMinerModel::L9,
            AntMinerModel::L11,
            AntMinerModel::Z15,
            AntMinerModel::Z15Pro,
        ] {
            let declared = model.get_str("algo").expect("property declared");
            assert!(
                HashAlgorithm::from_str(declared).is_ok(),
                "{model} declares {declared:?}, which is not a HashAlgorithm"
            );
            assert_ne!(
                model.hash_algorithm(),
                HashAlgorithm::SHA256,
                "{model} declares {declared:?} but resolved to the fallback"
            );
        }
    }

    #[test]
    fn unknown_model_defaults_to_sha256() {
        let model = AntMinerModel::from_str("ANTMINER S99").unwrap();

        assert_eq!(model.hash_algorithm(), HashAlgorithm::SHA256);
    }
}
