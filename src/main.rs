use asic_rs::{get_miner, get_miners};
use std::net::IpAddr;

#[tokio::main]
async fn main() {

    let miner_ip = IpAddr::from([192, 168, 1, 199]);
    match get_miner(miner_ip).await {
        Ok(Some(miner)) => {
            println!(
                "{}", serde_json::to_string(&miner.get_data().await).unwrap()
            );
        },
        Ok(None) => println!("No miner found at {}", miner_ip),
        Err(e) => println!("Error getting miner: {}", e),
    }


    let subnet = "192.168.1.0/24";
    println!("\nSearching for miners in subnet {}", subnet);

    match get_miners(subnet).await {
        Ok(miners) => {
            println!("Found {} miners in subnet", miners.len());
            for (i, miner) in miners.iter().enumerate() {
                println!(
                    "Miner {}: {:?}",
                    i + 1,
                    serde_json::to_string(&miner.get_data().await).unwrap()
                );
            }
            if miners.is_empty() {
                println!("No miners found in subnet");
            }
        },
        Err(e) => println!("Error getting miners: {}", e),
    }
    // let miner = BTMinerV3Backend::new(miner_ip);
    // dbg!(miner.get_device_info().await.unwrap());
    // dbg!(miner.get_miner_status_summary().await.unwrap());
    // dbg!(miner.get_miner_status_pools().await.unwrap());
    // dbg!(miner.get_miner_status_edevs().await.unwrap());
}
