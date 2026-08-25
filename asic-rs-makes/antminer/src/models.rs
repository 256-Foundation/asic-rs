use std::str::FromStr;

use asic_rs_core::data::device::HashAlgorithm;
use asic_rs_core::errors::ModelSelectionError;
use asic_rs_core::traits::model::MinerModel;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumProperty};
use ts_rs::TS;

#[derive(
    Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display, EnumIter, EnumProperty, TS,
)]
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
    #[strum(props(algo = "SHA256"))]
    S9,
    #[serde(alias = "ANTMINER S9I")]
    #[strum(props(algo = "SHA256"))]
    S9i,
    #[serde(alias = "ANTMINER S9J")]
    #[strum(props(algo = "SHA256"))]
    S9j,
    #[serde(alias = "ANTMINER T9")]
    #[strum(props(algo = "SHA256"))]
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
    #[strum(props(algo = "SHA256"))]
    S17,
    #[serde(alias = "ANTMINER S17+")]
    #[strum(props(algo = "SHA256"))]
    S17Plus,
    #[serde(alias = "ANTMINER S17 PRO")]
    #[strum(props(algo = "SHA256"))]
    S17Pro,
    #[serde(alias = "ANTMINER S17E")]
    #[strum(props(algo = "SHA256"))]
    S17e,
    #[serde(alias = "ANTMINER T17")]
    #[strum(props(algo = "SHA256"))]
    T17,
    #[serde(alias = "ANTMINER T17+")]
    #[strum(props(algo = "SHA256"))]
    T17Plus,
    #[serde(alias = "ANTMINER T17E")]
    #[strum(props(algo = "SHA256"))]
    T17e,
    #[serde(alias = "ANTMINER S19")]
    #[strum(props(algo = "SHA256"))]
    S19,
    #[serde(alias = "ANTMINER S19L")]
    #[strum(props(algo = "SHA256"))]
    S19L,
    #[serde(alias = "ANTMINER S19 PRO")]
    #[strum(props(algo = "SHA256"))]
    S19Pro,
    #[serde(alias = "ANTMINER S19J")]
    #[strum(props(algo = "SHA256"))]
    S19j,
    #[serde(alias = "ANTMINER S19I")]
    #[strum(props(algo = "SHA256"))]
    S19i,
    #[serde(alias = "ANTMINER S19+")]
    #[strum(props(algo = "SHA256"))]
    S19Plus,
    #[serde(alias = "ANTMINER S19J88NOPIC")]
    #[strum(props(algo = "SHA256"))]
    S19jNoPIC,
    #[serde(alias = "ANTMINER S19PRO+")]
    #[strum(props(algo = "SHA256"))]
    S19ProPlus,
    #[serde(alias = "ANTMINER S19J PRO")]
    #[strum(props(algo = "SHA256"))]
    S19jPro,
    #[serde(alias = "ANTMINER S19J PRO+")]
    #[strum(props(algo = "SHA256"))]
    S19jProPlus,
    #[serde(alias = "ANTMINER S19 XP")]
    #[strum(props(algo = "SHA256"))]
    S19XP,
    #[serde(alias = "ANTMINER S19A")]
    #[strum(props(algo = "SHA256"))]
    S19a,
    #[serde(alias = "ANTMINER S19A PRO")]
    #[strum(props(algo = "SHA256"))]
    S19aPro,
    #[serde(alias = "ANTMINER S19 HYDRO")]
    #[strum(props(algo = "SHA256"))]
    S19Hydro,
    #[serde(alias = "ANTMINER S19 PRO HYD.")]
    #[serde(alias = "ANTMINER S19 PRO HYDRO")]
    #[strum(props(algo = "SHA256"))]
    S19ProHydro,
    #[serde(alias = "ANTMINER S19 PRO+ HYD.")]
    #[serde(alias = "ANTMINER S19 PRO+ HYDRO")]
    #[strum(props(algo = "SHA256"))]
    S19ProPlusHydro,
    #[serde(alias = "ANTMINER S19K PRO")]
    #[strum(props(algo = "SHA256"))]
    S19KPro,
    #[serde(alias = "ANTMINER S19J XP")]
    #[strum(props(algo = "SHA256"))]
    S19jXP,
    #[serde(alias = "ANTMINER T19")]
    #[strum(props(algo = "SHA256"))]
    T19,
    #[serde(alias = "ANTMINER S21")]
    #[serde(alias = "ANTMINER BHB68601")]
    #[serde(alias = "ANTMINER BHB68606")]
    #[strum(props(algo = "SHA256"))]
    S21,
    #[serde(alias = "ANTMINER S21 PRO")]
    #[strum(props(algo = "SHA256"))]
    S21Pro,
    #[serde(alias = "ANTMINER S21 PRO+")]
    #[strum(props(algo = "SHA256"))]
    S21ProPlus,
    #[serde(alias = "ANTMINER S21 XP")]
    #[strum(props(algo = "SHA256"))]
    S21XP,
    #[serde(alias = "ANTMINER S21+")]
    #[strum(props(algo = "SHA256"))]
    S21Plus,
    #[serde(alias = "ANTMINER S21++")]
    #[strum(props(algo = "SHA256"))]
    S21PlusPlus,
    #[serde(alias = "ANTMINER S21 HYD.")]
    #[serde(alias = "ANTMINER S21 HYDRO")]
    #[strum(props(algo = "SHA256"))]
    S21Hydro,
    #[serde(alias = "ANTMINER S21+ HYD.")]
    #[serde(alias = "ANTMINER S21+ HYDRO")]
    #[strum(props(algo = "SHA256"))]
    S21PlusHydro,
    #[serde(alias = "ANTMINER S21E XP HYD.")]
    #[serde(alias = "ANTMINER S21E XP HYDRO")]
    #[strum(props(algo = "SHA256"))]
    S21eXPHydro,
    #[serde(alias = "ANTMINER T21")]
    #[strum(props(algo = "SHA256"))]
    T21,
    #[strum(to_string = "{0}")]
    #[strum(props(algo = "SHA256"))]
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
    /// hard to miss. Every variant declares the property; the fallback is a
    /// defensive guard against an invalid or accidentally omitted value.
    fn hash_algorithm(&self) -> HashAlgorithm {
        self.get_str("algo")
            .and_then(|algo| HashAlgorithm::from_str(algo).ok())
            .unwrap_or(HashAlgorithm::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use strum::IntoEnumIterator;

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

    /// A missing property or one naming something that is not a real
    /// [`HashAlgorithm`] would silently fall back to SHA-256. Iterating the
    /// enum makes this test cover newly added variants automatically.
    #[test]
    fn every_model_declares_a_valid_algorithm() {
        for model in AntMinerModel::iter() {
            let declared = model.get_str("algo").expect("property declared");
            let expected = HashAlgorithm::from_str(declared).unwrap_or_else(|_| {
                panic!("{model} declares {declared:?}, which is not a HashAlgorithm")
            });
            assert_eq!(
                model.hash_algorithm(),
                expected,
                "{model} does not resolve to its declared algorithm"
            );
        }
    }

    #[test]
    fn unknown_model_defaults_to_sha256() {
        let model = AntMinerModel::from_str("ANTMINER S99").unwrap();

        assert_eq!(model.hash_algorithm(), HashAlgorithm::SHA256);
    }
}
