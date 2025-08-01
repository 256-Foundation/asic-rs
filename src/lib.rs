use crate::miners::backends::traits::GetMinerData;
use crate::miners::factory::MinerFactory;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use anyhow::Result;
use futures::stream::{self, StreamExt};
use crate::data::device::{MinerFirmware, MinerMake};

pub mod data;
pub mod miners;

/// Constructs a single miner from a single IP address with specified miner makes and firmwares
pub async fn get_miner_with_options(ip: IpAddr, makes: Option<Vec<MinerMake>>, firmwares: Option<Vec<MinerFirmware>>) -> Result<Option<Box<dyn GetMinerData>>> {
    let mut factory = MinerFactory::new();

    if let Some(makes) = makes {
        factory.with_search_makes(makes);
    }
    if let Some(firmwares) = firmwares {
        factory.with_search_firmwares(firmwares);
    }

    factory.get_miner(ip).await
}

/// Constructs a single miner from a single IP address using default miner makes and firmwares
pub async fn get_miner(ip: IpAddr) -> Result<Option<Box<dyn GetMinerData>>> {
    get_miner_with_options(ip, None, None).await
}

/// Constructs a list of miners from an ip range (CIDR Notation) with specified miner makes and firmwares
pub async fn get_miners_with_options(ip_range: &str, makes: Option<Vec<MinerMake>>, firmwares: Option<Vec<MinerFirmware>>) -> Result<Vec<Box<dyn GetMinerData>>> {

    const MAX_CONCURRENT_TASKS: usize = 25;

    let range = ipnet::IpNet::from_str(ip_range)?;

    let mut factory = MinerFactory::new();

    if let Some(makes) = makes {
        factory.with_search_makes(makes);
    }
    if let Some(firmwares) = firmwares {
        factory.with_search_firmwares(firmwares);
    }

    let factory = Arc::new(factory);


    let miners: Vec<Box<dyn GetMinerData>> = stream::iter(range.hosts())
        .map(|ip| {
            let factory_clone = factory.clone();
            async move {
                factory_clone.get_miner(ip).await.ok().flatten()
            }
        })
        .buffer_unordered(MAX_CONCURRENT_TASKS)
        .filter_map(|miner_opt| async move { miner_opt })
        .collect()
        .await;

    Ok(miners)
}

/// Constructs a list of miners from an ip range (CIDR Notation) using default miner makes and firmwares
pub async fn get_miners(ip_range: &str) -> Result<Vec<Box<dyn GetMinerData>>> {
    get_miners_with_options(ip_range, None, None).await
}