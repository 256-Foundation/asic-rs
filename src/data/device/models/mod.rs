use std::{fmt::Display, str::FromStr};

use super::{MinerFirmware, MinerMake};
use antminer::AntMinerModel;
use braiins::BraiinsModel;
use whatsminer::WhatsMinerModel;

pub mod antminer;
pub mod braiins;
pub mod whatsminer;

#[derive(Debug, Clone)]
pub struct ModelParseError;

impl Display for ModelParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse model")
    }
}

impl std::error::Error for ModelParseError {}

impl FromStr for WhatsMinerModel {
    type Err = ModelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .map_err(|_| ModelParseError)
    }
}
impl FromStr for AntMinerModel {
    type Err = ModelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .map_err(|_| ModelParseError)
    }
}

impl FromStr for BraiinsModel {
    type Err = ModelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
            .map_err(|_| ModelParseError)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MinerModel {
    AntMiner(AntMinerModel),
    WhatsMiner(WhatsMinerModel),
    Braiins(BraiinsModel),
}

impl MinerModel {
    pub fn from_string(
        make: Option<MinerMake>,
        firmware: Option<MinerFirmware>,
        model_str: &str,
    ) -> Option<Self> {
        match make {
            Some(MinerMake::AntMiner) => {
                let model = AntMinerModel::from_str(model_str).ok();
                match model {
                    Some(model) => Some(MinerModel::AntMiner(model)),
                    None => None,
                }
            }
            Some(MinerMake::WhatsMiner) => {
                let model = WhatsMinerModel::from_str(model_str).ok();
                match model {
                    Some(model) => Some(MinerModel::WhatsMiner(model)),
                    None => None,
                }
            }
            None => match firmware {
                Some(MinerFirmware::BraiinsOS) => {
                    if let Ok(model) = AntMinerModel::from_str(model_str) {
                        return Some(MinerModel::AntMiner(model));
                    }
                    if let Ok(model) = BraiinsModel::from_str(model_str) {
                        return Some(MinerModel::Braiins(model));
                    }
                    None
                }
                Some(MinerFirmware::LuxOS) => {
                    if let Ok(model) = AntMinerModel::from_str(model_str) {
                        return Some(MinerModel::AntMiner(model));
                    }
                    None
                }
                None => None,
                _ => None,
            },
            _ => None,
        }
    }
    pub fn get_make(&self) -> MinerMake {
        match self {
            MinerModel::AntMiner(_) => MinerMake::AntMiner,
            MinerModel::WhatsMiner(_) => MinerMake::WhatsMiner,
            MinerModel::Braiins(_) => MinerMake::Braiins,
        }
    }
}
