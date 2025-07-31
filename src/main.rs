use asic_rs::get_miner;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    let miner_ip = IpAddr::from([192, 168, 1, 199]);

    let miner = get_miner(miner_ip).await.unwrap();
    if miner.is_some() {
        println!(
            "{:?}",
            serde_json::to_string(&miner.unwrap().get_data().await).unwrap()
        );
    } else {
        println!("No miner found");
    }

    // let miner = BTMinerV3Backend::new(miner_ip);
    // dbg!(miner.get_device_info().await.unwrap());
    // dbg!(miner.get_miner_status_summary().await.unwrap());
    // dbg!(miner.get_miner_status_pools().await.unwrap());
    // dbg!(miner.get_miner_status_edevs().await.unwrap());
}
