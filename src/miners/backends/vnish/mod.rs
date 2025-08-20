use semver;
use std::net::IpAddr;

pub use v1_2_0::VnishV120;

use crate::data::device::{MinerMake, MinerModel};
use crate::miners::backends::traits::GetMinerData;

pub mod v1_2_0;

pub struct Vnish;

impl Vnish {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        ip: IpAddr,
        make: MinerMake,
        model: MinerModel,
        _: Option<semver::Version>,
    ) -> Box<dyn GetMinerData> {
        Box::new(VnishV120::new(ip, make, model))
    }
}
