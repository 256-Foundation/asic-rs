use std::net::IpAddr;
use crate::data::device::MinerModel;
use crate::data::device::models::avalonminer::AvalonMinerModel;
use crate::miners::util;

pub(crate) async fn get_model_avalonminer(ip: IpAddr) -> Option<MinerModel> {
    let response = util::send_rpc_command(&ip, "version").await;

    match response {
        Some(data) => {
            // Extract the model from the VERSION/0/PROD field
            let mut miner_model = data.pointer("/VERSION/0/PROD")?.as_str()?.to_uppercase();
            
            // If model contains a hyphen, split it and take the first part
            if miner_model.contains('-') {
                miner_model = miner_model.split('-').next()?.to_string();
            }
            
            // Handle special cases
            if ["AVALONNANO", "AVALON0O", "AVALONMINER 15"].contains(&miner_model.as_str()) {
                if let Some(subtype) = data.pointer("/VERSION/0/MODEL").and_then(|v| v.as_str()) {
                    miner_model = format!("AVALONMINER {}", subtype.to_uppercase());
                }
            }
            
            // Map the model string to the appropriate AvalonMinerModel variant
            let model = match miner_model.as_str() {
                "AVALONMINER 721" => Some(AvalonMinerModel::A721),
                "AVALONMINER 741" => Some(AvalonMinerModel::A741),
                "AVALONMINER 761" => Some(AvalonMinerModel::A761),
                "AVALONMINER 821" => Some(AvalonMinerModel::A821),
                "AVALONMINER 841" => Some(AvalonMinerModel::A841),
                "AVALONMINER 851" => Some(AvalonMinerModel::A851),
                "AVALONMINER 921" => Some(AvalonMinerModel::A921),
                "AVALONMINER 1026" => Some(AvalonMinerModel::A1026),
                "AVALONMINER 1047" => Some(AvalonMinerModel::A1047),
                "AVALONMINER 1066" => Some(AvalonMinerModel::A1066),
                "AVALONMINER 1126" => Some(AvalonMinerModel::A1126),
                "AVALONMINER 1166" => Some(AvalonMinerModel::A1166),
                "AVALONMINER 1246" => Some(AvalonMinerModel::A1246),
                "AVALONMINER 1566" => Some(AvalonMinerModel::A1566),
                "AVALONMINER NANO3" => Some(AvalonMinerModel::Nano3),
                "AVALONMINER NANO3S" => Some(AvalonMinerModel::Nano3S),
                _ => None,
            };
            
            model.map(MinerModel::AvalonMiner)
        }
        None => None,
    }
}