use asic_rs_core::data::{board::MinerControlBoard, collector::FromValue, device::MinerHardware};
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::models::ElphapexModel;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display)]
pub enum ElphapexControlBoard {
    DGHome1,
}

impl ElphapexControlBoard {
    pub fn parse(s: &str) -> Option<Self> {
        if s.contains("DG-Home1") {
            Some(Self::DGHome1)
        } else {
            None
        }
    }
}

impl FromValue for ElphapexControlBoard {
    fn from_value(value: &serde_json::Value) -> Option<Self> {
        Self::parse(value.as_str()?)
    }
}

impl From<ElphapexControlBoard> for MinerControlBoard {
    fn from(cb: ElphapexControlBoard) -> Self {
        MinerControlBoard::known(cb.to_string())
    }
}

impl From<ElphapexModel> for MinerHardware {
    fn from(value: ElphapexModel) -> Self {
        match value {
            ElphapexModel::DG1 => Self {
                fans: Some(4),
                boards: Some(vec![Some(144); 4]),
            },
            ElphapexModel::DG1Plus => Self {
                fans: Some(4),
                boards: Some(vec![Some(204); 4]),
            },
            ElphapexModel::DG1Home => Self {
                fans: Some(4),
                boards: Some(vec![Some(120); 4]),
            },
            ElphapexModel::Unknown(_) => Default::default(),
        }
    }
}
