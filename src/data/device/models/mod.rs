use super::{MinerFirmware, MinerMake};
use antminer::AntMinerModel;
use bitaxe::BitaxeModel;
use braiins::BraiinsModel;
use serde::Serialize;
use std::{fmt::Display, str::FromStr};
use whatsminer::WhatsMinerModel;
use crate::data::device::MinerMake::AvalonMiner;
use crate::data::device::models::avalonminer::AvalonMinerModel;

pub mod antminer;
pub mod bitaxe;
pub mod braiins;
pub mod whatsminer;
pub mod avalonminer;

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

impl FromStr for AvalonMinerModel {
    type Err = ModelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_owned())).map_err(|_| ModelParseError)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum MinerModel {
    AntMiner(AntMinerModel),
    WhatsMiner(WhatsMinerModel),
    AvalonMiner(AvalonMinerModel),
    Braiins(BraiinsModel),
    Bitaxe(BitaxeModel),
}

pub(crate) struct MinerModelFactory {
    make: Option<MinerMake>,
    firmware: Option<MinerFirmware>,
}

impl MinerModelFactory {
    pub fn new() -> Self {
        MinerModelFactory {
            make: None,
            firmware: None,
        }
    }

    pub(crate) fn with_make(&mut self, make: MinerMake) -> &Self {
        self.make = Some(make);
        self
    }
    pub(crate) fn with_firmware(&mut self, firmware: MinerFirmware) -> &Self {
        self.firmware = Some(firmware);
        self
    }

    pub(crate) fn parse_model(&self, model_str: &str) -> Option<MinerModel> {
        match self.make {
            Some(MinerMake::AntMiner) => {
                let model = AntMinerModel::from_str(model_str).ok();
                model.map(MinerModel::AntMiner)
            }
            Some(MinerMake::WhatsMiner) => {
                let model = WhatsMinerModel::from_str(model_str).ok();
                model.map(MinerModel::WhatsMiner)
            }
            None => match self.firmware {
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
}
