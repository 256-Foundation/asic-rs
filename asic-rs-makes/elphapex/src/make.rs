use std::{fmt::Display, str::FromStr};

use asic_rs_core::{
    data::board::MinerControlBoard, errors::ModelSelectionError, traits::make::MinerMake,
};

use crate::{hardware::ElphapexControlBoard, models::ElphapexModel};

#[derive(Default, Debug)]
pub struct ElphapexMake {}

impl Display for ElphapexMake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Elphapex")
    }
}

impl MinerMake for ElphapexMake {
    type Model = ElphapexModel;

    fn parse_model(model: String) -> Result<Self::Model, ModelSelectionError> {
        ElphapexModel::from_str(&model)
    }

    fn parse_control_board(&self, cb_type: &str) -> Option<MinerControlBoard> {
        Some(ElphapexControlBoard::parse(cb_type)?.into())
    }
}
