pub mod v2020;

use crate::data::device::MinerModel;
use crate::miners::backends::traits::GetMinerData;
use std::net::IpAddr;
use v2020::AntMinerV2020;

pub struct AntMiner;

impl AntMiner {
    pub fn new(
        ip: IpAddr,
        model: MinerModel,
        version: Option<semver::Version>,
    ) -> Box<dyn GetMinerData> {
        Box::new(AntMinerV2020::new(ip, model))
    }
}
