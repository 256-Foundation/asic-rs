use std::net::IpAddr;

use semver;
pub use v1::BTMiner1;
pub use v2::BTMiner2;
pub use v3::BTMiner3;

use crate::data::device::{MinerFirmware, MinerModel};
use crate::miners::backends::traits::GetMinerData;

pub mod v1;
pub mod v2;
pub mod v3;

pub struct BTMiner;

impl BTMiner {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        ip: IpAddr,
        model: MinerModel,
        firmware: MinerFirmware,
        version: semver::Version,
    ) -> Box<dyn GetMinerData> {
        if semver::VersionReq::parse(">=2024.11.0")
            .unwrap()
            .matches(&version)
        {
            Box::new(BTMiner3::new(ip, model, firmware))
        } else if semver::VersionReq::parse(">= 2022.9.20")
            .unwrap()
            .matches(&version)
        {
            Box::new(BTMiner2::new(ip, model, firmware))
        } else {
            Box::new(BTMiner1::new(ip, model, firmware))
        }
    }
}
