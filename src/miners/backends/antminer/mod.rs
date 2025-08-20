pub mod Antminer2022;

use crate::data::device::MinerModel;
use crate::miners::backends::traits::GetMinerData;
use Antminer2022::Antminer2022 as AntminerV3;
use std::net::IpAddr;

pub struct AntMiner;

impl AntMiner {
    pub fn new(
        ip: IpAddr,
        model: MinerModel,
        version: Option<semver::Version>,
    ) -> Box<dyn GetMinerData> {
        if let Some(version) = version {
            match version.major {
                2022 => Box::new(AntminerV3::new(ip, model)),
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }
}
