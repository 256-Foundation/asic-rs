use std::net::IpAddr;

use asic_rs_core::traits::{
    miner::{Miner, MinerConstructor},
    model::MinerModel,
};
use v1::ElphapexV1;

pub mod v1;

pub struct Elphapex;

impl MinerConstructor for Elphapex {
    fn new(ip: IpAddr, model: impl MinerModel, _: Option<semver::Version>) -> Box<dyn Miner> {
        Box::new(ElphapexV1::new(ip, model))
    }
}
