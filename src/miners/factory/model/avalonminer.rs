use crate::data::device::models::MinerModelFactory;
use crate::data::device::{MinerMake, MinerModel};
use crate::miners::util;
use std::net::IpAddr;

pub(crate) async fn get_model_avalonminer(ip: IpAddr) -> Option<MinerModel> {
    let response = util::send_rpc_command(&ip, "version").await;

    match response {
        Some(data) => {
            // Extract the model name from the response
            let mut miner_model = data.pointer("/VERSION/0/PROD")?.as_str()?.to_uppercase();

            // If the model name contains a hyphen, split it and take the first part
            if miner_model.contains('-') {
                if let Some(index) = miner_model.find('-') {
                    miner_model = miner_model[..index].to_string();
                }
            }

            // Handle special cases
            if miner_model == "AVALONNANO"
                || miner_model == "AVALON0O"
                || miner_model == "AVALONMINER 15"
            {
                if let Some(subtype) = data.pointer("/VERSION/0/MODEL")?.as_str() {
                    miner_model = format!("AVALONMINER {}", subtype.to_uppercase());
                }
            }

            MinerModelFactory::new()
                .with_make(MinerMake::AvalonMiner)
                .parse_model(&miner_model)
        }
        None => None,
    }
}
