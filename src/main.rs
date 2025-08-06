use asic_rs::MinerFactory;

#[tokio::main]
async fn main() {
    let factory = MinerFactory::new()
        // .with_timeout_secs(10)
        // .with_concurrent_limit(5096)
        .with_range("192.3.1-8.1-50")
        .unwrap();

    println!("Starting 2-phase scan...");

    match factory.scan_two_phase().await {
        Ok(alive_hosts) => {
            println!("2-phase scan completed");
            println!("Total IPs scanned: {}", factory.len());
            println!("Found {} active miners", alive_hosts.len());

            for (ip, _miner) in &alive_hosts {
                println!("  Active miner at: {}", ip);
            }
        }
        Err(e) => {
            eprintln!("2-phase scan failed: {}", e);
        }
    }
}
