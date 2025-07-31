use std::net::IpAddr;
use crate::data::device::{MinerFirmware, MinerModel};
use crate::miners::backends::traits::GetMinerData;
use avalon_miner::AvalonMiner as AMiner;

mod avalon_miner;


struct AvalonMiner;

impl AvalonMiner {
    fn new(ip: IpAddr, model: MinerModel, firmware: MinerFirmware) -> Box<dyn GetMinerData> {
        Box::new(AMiner::new(ip, model, firmware))
    }
}