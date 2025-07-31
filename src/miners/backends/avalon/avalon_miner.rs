use crate::data::board::BoardData;
use crate::data::device::MinerMake;
use crate::data::device::{DeviceInfo, HashAlgorithm, MinerFirmware, MinerModel};
use crate::data::fan::FanData;
use crate::data::hashrate::{HashRate, HashRateUnit};
use crate::data::miner::MinerData;
use crate::data::pool::PoolData;
use crate::miners::api::rpc::cgminer::CGMinerRPC;
use crate::miners::backends::traits::GetMinerData;
use crate::miners::commands::MinerCommand;
use crate::miners::data::{
    DataCollector, DataExtensions, DataExtractor, DataField, DataLocation, get_by_pointer,
};
use async_trait::async_trait;
use macaddr::MacAddr;
use measurements::{AngularVelocity, Power, Temperature};
use regex::Regex;
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct AvalonMiner {
    model: MinerModel,
    rpc: CGMinerRPC,
    ip: IpAddr,
    miner_firmware: MinerFirmware,
}

impl AvalonMiner {
    pub fn new(ip: IpAddr, model: MinerModel, miner_firmware: MinerFirmware) -> Self {
        Self {
            model,
            rpc: CGMinerRPC::new(ip),
            ip,
            miner_firmware,
        }
    }
}

// Custom extractor for Avalon's unique 'stats' command output
fn extract_from_stats(response: &Value, key: Option<&str>) -> Option<Value> {
    let key = key?;
    // The response for 'stats' is a JSON object with a "STATS" key,
    // which contains an array, and the relevant stats string is under "MM ID1" in the first object.
    let stats_obj = response.get("STATS")?.get(0)?;
    let stats_str = stats_obj.get("MM ID1")?.as_str()?;
    let parsed_stats = parse_stats(stats_str);
    parsed_stats.get(key).cloned()
}

// Custom extractor for hashrate from the 'devs' command
fn extract_hashrate(response: &Value, _key: Option<&str>) -> Option<Value> {
    let mhs = response.get("DEVS")?.get(0)?.get("MHS 1m")?.as_f64()?;
    // Convert MHS to THS
    Some(json!(mhs / 1_000_000.0))
}

// Custom extractor for hashboards from the 'stats' command
fn extract_hashboards(response: &Value, _key: Option<&str>) -> Option<Value> {
    let stats_obj = response.get("STATS")?.get(0)?;
    let stats_str = stats_obj.get("MM ID1")?.as_str()?;
    let parsed_stats = parse_stats(stats_str);
    let mw_array = parsed_stats.get("MW")?.as_array()?;
    let hbs = mw_array.len() as u8;
    let chips_per_board = 104 / hbs as u16;

    // Temps: Try to get per board from MM Temp or PVT_T*
    let mut board_temps: Vec<f64> = vec![0.0; hbs as usize];
    if let Some(mm_temp) = parsed_stats.get("MM Temp") {
        if let Some(temp_str) = mm_temp.get(0).and_then(|v| v.as_str()) {
            let temps: Vec<f64> = temp_str.split('-').filter_map(|s| s.parse().ok()).collect();
            if temps.len() == hbs as usize {
                board_temps = temps;
            }
        }
    } else {
        // Fallback to single Temp
        if let Some(temp_val) = parsed_stats.get("Temp")?.get(0)?.as_str() {
            let temp = temp_val.parse::<f64>().ok()?;
            board_temps = vec![temp; hbs as usize];
        }
    }

    let mut boards = vec![];
    for i in 0..hbs {
        boards.push(json!({
            "position": i,
            "board_temperature": board_temps[i as usize],
            "working_chips": chips_per_board,
        }));
    }
    Some(json!(boards))
}

fn extract_fans(response: &Value, _key: Option<&str>) -> Option<Value> {
    let stats_obj = response.get("STATS")?.get(0)?;
    let stats_str = stats_obj.get("MM ID1")?.as_str()?;
    let parsed_stats = parse_stats(stats_str);
    let fan_speed_val = parsed_stats.get("Fan")?.get(0)?;
    let fan_speed = fan_speed_val.as_str()?.parse::<u32>().ok()?;
    Some(json!([{"position": 0, "rpm": fan_speed}]))
}

pub fn parse_stats(response: &str) -> HashMap<String, Value> {
    let re = Regex::new(r".+?\[.*?]").expect("Failed to compile regex");
    let mut stats_dict: HashMap<String, Value> = HashMap::new();

    for item_match in re.find_iter(response) {
        let item = item_match.as_str();
        let key_part: String;
        let value_json: Value;

        if item.contains(": ") {
            let parts: Vec<&str> = item.splitn(2, '[').collect();
            if parts.len() != 2 {
                continue;
            }

            key_part = parts[0].trim().to_string();
            let inner_content = parts[1].trim_end_matches(']');

            let mut data_dict = Map::new();
            let pairs: Vec<&str> = inner_content.split(',').map(|s| s.trim()).collect();

            let is_key_value = pairs.iter().all(|p| p.contains(": "));

            if is_key_value {
                for pair in pairs {
                    if let Some((key, val)) = pair.split_once(": ") {
                        data_dict.insert(key.to_string(), json!(val));
                    }
                }
            } else {
                let all_args: Vec<&str> = inner_content.split_whitespace().collect();
                for chunk in all_args.chunks(2) {
                    if chunk.len() == 2 {
                        data_dict.insert(chunk[0].to_string(), json!(chunk[1]));
                    }
                }
            }

            value_json = json!([data_dict]);
        } else {
            let parts: Vec<&str> = item.splitn(2, '[').collect();
            if parts.len() != 2 {
                continue;
            }

            let keys_str = parts[0].trim();
            let val_str = parts[1].trim_end_matches(']');

            let mut key_components: Vec<&str> = keys_str.split_whitespace().collect();
            if key_components.is_empty() {
                continue;
            }

            key_part = key_components.remove(0).to_string();

            // Fixed: Remove unused variable and unnecessary mutability
            let vals: Vec<Value> = val_str.split_whitespace().map(|s| json!(s)).collect();
            value_json = json!(vals);
        }

        stats_dict.insert(key_part, value_json);
    }

    stats_dict
}

#[async_trait]
impl GetMinerData for AvalonMiner {
    async fn get_data(&self) -> MinerData {
        let mut collector = DataCollector::new(self, &self.rpc);
        let data = collector.collect_all().await;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mac = data
            .extract::<String>(DataField::Mac)
            .and_then(|s| MacAddr::from_str(&s).ok());

        let wattage = data.extract_map::<f64, _>(DataField::Wattage, Power::from_watts);
        let firmware_version = data.extract::<String>(DataField::FirmwareVersion);
        let api_version = data.extract::<String>(DataField::ApiVersion);
        let uptime = data.extract_map::<u64, _>(DataField::Uptime, Duration::from_secs);

        let hashrate = data.extract_map::<f64, _>(DataField::Hashrate, |mhs| {
            let ths = mhs / 1_000_000.0;
            HashRate {
                value: ths,
                unit: HashRateUnit::TeraHash,
                algo: "SHA256".to_string(),
            }
        });

        let mut hashboards = vec![];
        let mut fans = vec![];
        let stats_value = data.get(&DataField::Hashboards);

        // Fixed: Moved parsed_stats declaration to broader scope
        let mut parsed_stats: HashMap<String, Value> = HashMap::new();
        let mut stats_parsed = false;

        if let Some(stats_value) = stats_value {
            if let Some(stats_array) = stats_value.get("STATS") {
                if let Some(stats_obj) = stats_array.get(0) {
                    if let Some(stats_str) = stats_obj.get("MM ID1").and_then(|v| v.as_str()) {
                        parsed_stats = parse_stats(stats_str);
                        stats_parsed = true;

                        let elapsed = parsed_stats
                            .get("Elapsed")
                            .and_then(|v| v.get(0))
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse::<f64>().ok())
                            .unwrap_or(1.0);

                        let mut mw_vec: Vec<f64> = vec![];
                        if let Some(mw_array) = parsed_stats.get("MW") {
                            if let Some(array) = mw_array.as_array() {
                                mw_vec = array
                                    .iter()
                                    .filter_map(|v| v.as_str().and_then(|s| s.parse::<f64>().ok()))
                                    .collect();
                            }
                        }
                        let hbs = mw_vec.len() as u8;

                        let total_chips = parsed_stats
                            .get("TA")
                            .and_then(|v| v.get(0))
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse::<u16>().ok())
                            .unwrap_or(0);
                        let chips_per_board = if hbs > 0 { total_chips / hbs as u16 } else { 0 };

                        let mut board_temps: Vec<f64> = vec![0.0; hbs as usize];
                        if let Some(mm_temp) = parsed_stats.get("MM Temp") {
                            if let Some(temp_str) = mm_temp.get(0).and_then(|v| v.as_str()) {
                                let temps: Vec<f64> =
                                    temp_str.split('-').filter_map(|s| s.parse().ok()).collect();
                                if temps.len() == hbs as usize {
                                    board_temps = temps;
                                }
                            }
                        } else if let Some(temp_val) = parsed_stats.get("Temp") {
                            if let Some(temp_str) = temp_val.get(0).and_then(|v| v.as_str()) {
                                let temp = temp_str.parse::<f64>().unwrap_or(0.0);
                                board_temps = vec![temp; hbs as usize];
                            }
                        }

                        for i in 0..hbs {
                            let mw = mw_vec[i as usize];
                            let board_hr = (mw * 4.294967296e9 / elapsed) / 1e12;
                            hashboards.push(BoardData {
                                position: i,
                                hashrate: Some(HashRate {
                                    value: board_hr,
                                    unit: HashRateUnit::TeraHash,
                                    algo: "SHA256".to_string(),
                                }),
                                expected_hashrate: None,
                                board_temperature: Some(Temperature::from_celsius(
                                    board_temps[i as usize],
                                )),
                                intake_temperature: None,
                                outlet_temperature: None,
                                expected_chips: Some(chips_per_board),
                                working_chips: Some(chips_per_board),
                                serial_number: None,
                                chips: vec![],
                                voltage: None,
                                frequency: None,
                                tuned: None,
                                active: Some(true),
                            });
                        }
                    }
                }
            }
        }

        // Fixed: Now parsed_stats is in scope here
        if stats_parsed {
            if let Some(fan_speed) = parsed_stats.get("Fan") {
                if let Some(speed_str) = fan_speed.get(0).and_then(|v| v.as_str()) {
                    if let Ok(rpm) = speed_str.parse::<u32>() {
                        fans.push(FanData {
                            position: 0,
                            rpm: Some(AngularVelocity::from_rpm(rpm as f64)),
                        });
                    }
                }
            }
        }

        let average_temperature = if !hashboards.is_empty() {
            let total_temp: f64 = hashboards
                .iter()
                .map(|b| b.board_temperature.map_or(0.0, |t| t.as_celsius()))
                .sum();
            Some(Temperature::from_celsius(
                total_temp / hashboards.len() as f64,
            ))
        } else {
            None
        };

        let pools_value = data.get(&DataField::Pools);
        let mut pools = vec![];
        if let Some(pools_array) = pools_value
            .and_then(|v| v.get("POOLS"))
            .and_then(|v| v.as_array())
        {
            for (i, pool) in pools_array.iter().enumerate() {
                if let Some(pool_obj) = pool.as_object() {
                    let url = pool_obj
                        .get("URL")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let user = pool_obj
                        .get("User")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let status = pool_obj
                        .get("Status")
                        .and_then(|v| v.as_str())
                        .map(|s| s == "Alive");
                    let accepted = pool_obj.get("Accepted").and_then(|v| v.as_u64());
                    let rejected = pool_obj.get("Rejected").and_then(|v| v.as_u64());

                    pools.push(PoolData {
                        position: Some(i as u16),
                        url: url.map(|u| u.into()),
                        accepted_shares: accepted,
                        rejected_shares: rejected,
                        active: Some(true),
                        alive: status,
                        user,
                    });
                }
            }
        }

        let device_info = DeviceInfo::new(
            MinerMake::AvalonMiner,
            self.model.clone(),
            self.miner_firmware,
            HashAlgorithm::SHA256,
        );
        let expected_hashboards = device_info.hardware.boards;
        let expected_fans = device_info.hardware.fans;
        let expected_chips = device_info.hardware.chips;

        let expected_hashrate = Some(HashRate {
            value: 90.0,
            unit: HashRateUnit::TeraHash,
            algo: "SHA256".to_string(),
        });

        let total_chips = Some(
            hashboards
                .iter()
                .map(|b| b.working_chips.unwrap_or(0))
                .sum::<u16>(),
        );

        MinerData {
            schema_version: env!("CARGO_PKG_VERSION").to_owned(),
            timestamp,
            ip: self.ip,
            mac,
            device_info,
            serial_number: None,
            hostname: None,
            api_version,
            firmware_version,
            control_board_version: None,
            expected_hashboards,
            hashboards,
            hashrate,
            expected_hashrate,
            expected_chips,
            total_chips,
            expected_fans,
            fans,
            psu_fans: vec![],
            average_temperature,
            fluid_temperature: None,
            wattage,
            wattage_limit: None,
            efficiency: None,
            light_flashing: None,
            messages: vec![],
            uptime,
            is_mining: !pools.is_empty() && pools.iter().any(|p| p.alive == Some(true)),
            pools,
        }
    }

    fn get_locations(&self, data_field: DataField) -> Vec<DataLocation> {
        let version_cmd: MinerCommand = MinerCommand::RPC {
            command: "version",
            parameters: None,
        };
        let stats_cmd: MinerCommand = MinerCommand::RPC {
            command: "stats",
            parameters: None,
        };
        let devs_cmd: MinerCommand = MinerCommand::RPC {
            command: "devs",
            parameters: None,
        };
        let pools_cmd: MinerCommand = MinerCommand::RPC {
            command: "pools",
            parameters: None,
        };

        match data_field {
            DataField::Mac => vec![(
                version_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/VERSION/0/MAC"),
                },
            )],
            DataField::ApiVersion => vec![(
                version_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/VERSION/0/API"),
                },
            )],
            DataField::FirmwareVersion => vec![(
                version_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/VERSION/0/CGMiner"),
                },
            )],
            DataField::Hashboards => vec![(
                stats_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS"),
                },
            )],
            DataField::Hashrate => vec![(
                devs_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/DEVS/0/MHS 1m"),
                },
            )],
            DataField::Fans => vec![(
                stats_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS"),
                },
            )],
            DataField::Wattage => vec![],
            DataField::Uptime => vec![(
                stats_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/0/Elapsed"),
                },
            )],
            DataField::Pools => vec![(
                pools_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/POOLS"),
                },
            )],
            _ => vec![],
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::device::models::avalonminer::AvalonMinerModel;
    use crate::data::device::{MinerFirmware, MinerModel};
    use crate::miners::api::ApiClient;
    use crate::miners::commands::MinerCommand;
    use async_trait::async_trait;
    use serde_json::json;
    use std::collections::HashMap;
    use std::net::IpAddr;
    use std::str::FromStr;

    // Mock API client that returns the test data you provided
    struct MockCGMinerAPI {
        responses: HashMap<String, serde_json::Value>,
    }

    impl MockCGMinerAPI {
        fn new() -> Self {
            let mut responses = HashMap::new();

            // VERSION response
            responses.insert(
                "version".to_string(),
                json!({
                    "STATUS": [
                        {
                            "STATUS": "S",
                            "When": 1662562633,
                            "Code": 11,
                            "Msg": "CGMiner versions",
                            "Description": "cgminer 4.9.0",
                        }
                    ],
                    "VERSION": [
                        {
                            "CGMiner": "4.9.0",
                            "API": "3.1",
                        }
                    ],
                    "id": 1,
                }),
            );

            // DEVS response
            responses.insert(
                "devs".to_string(),
                json!({
                    "STATUS": [
                        {
                            "STATUS": "S",
                            "When": 1662562633,
                            "Code": 70,
                            "Msg": "Summary",
                            "Description": "cgminer 4.9.0",
                        }
                    ],
                    "DEVS": [
                        {
                            "ASC": 0,
                            "Name": "SM",
                            "ID": 0,
                            "Enabled": "Y",
                            "Status": "Alive",
                            "Temperature": 60.5,
                            "MHS av": 110000000.00,
                            "MHS 5s": 110000000.00,
                            "MHS 1m": 110000000.00,
                            "MHS 5m": 110000000.00,
                            "MHS 15m": 110000000.00,
                            "Accepted": 10,
                            "Rejected": 1,
                            "Hardware Errors": 2,
                            "Utility": 1.0,
                            "Last Share Pool": 0,
                            "Last Share Time": 10,
                            "Total MH": 11000000000.0,
                            "Diff1 Work": 100,
                            "Difficulty Accepted": 100.0,
                            "Difficulty Rejected": 10.0,
                            "Last Share Difficulty": 10.0,
                            "No Device": false,
                            "Device Elapsed": 100,
                            "Device Hardware%": 0.0002,
                            "Device Rejected%": 10.0,
                        }
                    ],
                    "id": 1,
                }),
            );

            // POOLS response
            responses.insert(
                "pools".to_string(),
                json!({
                    "STATUS": [
                        {
                            "STATUS": "S",
                            "When": 1662562633,
                            "Code": 70,
                            "Msg": "Summary",
                            "Description": "cgminer 4.9.0",
                        }
                    ],
                    "POOLS": [
                        {
                            "POOL": 0,
                            "URL": "stratum+tcp://test.pool:3333",
                            "Status": "Alive",
                            "Priority": 0,
                            "Quota": 1,
                            "Long Poll": "N",
                            "Getworks": 10,
                            "Accepted": 10,
                            "Rejected": 1,
                            "Works": 100,
                            "Discarded": 100,
                            "Stale": 0,
                            "Get Failures": 0,
                            "Remote Failures": 0,
                            "User": "test.user",
                            "Last Share Time": "0:00:10",
                            "Diff": "1",
                            "Diff1 Shares": 10,
                            "Proxy Type": "",
                            "Proxy": "",
                            "Difficulty Accepted": 100.0,
                            "Difficulty Rejected": 10.0,
                            "Difficulty Stale": 0.0,
                            "Last Share Difficulty": 10.0,
                            "Has Stratum": true,
                            "Stratum Active": true,
                            "Stratum URL": "test.pool",
                            "Has GBT": false,
                            "Best Share": 100,
                            "Pool Rejected%": 10.0,
                            "Pool Stale%": 0.0,
                        }
                    ],
                    "id": 1,
                }),
            );

            // SUMMARY response
            responses.insert(
                "summary".to_string(),
                json!({
                    "STATUS": [
                        {
                            "STATUS": "S",
                            "When": 1662562633,
                            "Code": 70,
                            "Msg": "Summary",
                            "Description": "cgminer 4.9.0",
                        }
                    ],
                    "SUMMARY": [
                        {
                            "Elapsed": 100.0,
                            "MHS av": 110000000.00,
                            "Found Blocks": 0,
                            "Getworks": 1,
                            "Accepted": 10,
                            "Rejected": 1,
                            "Hardware Errors": 2,
                            "Utility": 1.0,
                            "Discarded": 100,
                            "Stale": 0,
                            "Get Failures": 0,
                            "Local Work": 100,
                            "Remote Failures": 0,
                            "Network Blocks": 1,
                            "Total MH": 11000000000.0,
                            "Work Utility": 100.0,
                            "Difficulty Accepted": 100.0,
                            "Difficulty Rejected": 10.0,
                            "Difficulty Stale": 0.0,
                            "Best Share": 100,
                            "Device Hardware%": 0.0002,
                            "Device Rejected%": 10.0,
                            "Pool Rejected%": 10.0,
                            "Pool Stale%": 0.0,
                        }
                    ],
                    "id": 1,
                }),
            );

            // STATS response (basic CGMiner response without Avalon-specific MM ID1)
            responses.insert(
                "stats".to_string(),
                json!({
                    "STATUS": [
                        {
                            "STATUS": "S",
                            "When": 1662562633,
                            "Code": 70,
                            "Msg": "Summary",
                            "Description": "cgminer 4.9.0",
                        }
                    ],
                    "STATS": [
                        {
                            "CGMiner": "4.9.0",
                            "Miner": "1.0.0",
                            "CompileTime": "Tue Nov 29 13:36:09 UTC 2016",
                            "Type": "Avalon6"
                        }
                    ],
                    "id": 1,
                }),
            );

            Self { responses }
        }

        fn with_avalon_stats(mut self) -> Self {
            // Add Avalon-specific stats response with MM ID1 field
            self.responses.insert("stats".to_string(), json!({
                "STATUS": [
                    {
                        "STATUS": "S",
                        "When": 1662562633,
                        "Code": 70,
                        "Msg": "Summary",
                        "Description": "cgminer 4.9.0",
                    }
                ],
                "STATS": [
                    {
                        "MM ID1": "Fan[5000] Temp[45-50-48] MW[1234.56 1245.67 1256.78] Elapsed[100]",
                        "CGMiner": "4.9.0",
                        "Miner": "1.0.0",
                        "CompileTime": "Tue Nov 29 13:36:09 UTC 2016",
                        "Type": "Avalon6"
                    }
                ],
                "id": 1,
            }));
            self
        }
    }

    #[async_trait]
    impl ApiClient for MockCGMinerAPI {
        async fn get_api_result(
            &self,
            command: &MinerCommand,
        ) -> Result<serde_json::Value, String> {
            match command {
                MinerCommand::RPC {
                    command,
                    parameters: _,
                } => {
                    if let Some(response) = self.responses.get(*command) {
                        Ok(response.clone())
                    } else {
                        Err(format!("Command {} not found in mock responses", command))
                    }
                }
                _ => Err("Unsupported command type".to_string()),
            }
        }
    }

    #[tokio::test]
    async fn test_avalon_miner_with_avalon_specific_stats() {
        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        let model = MinerModel::AvalonMiner(AvalonMinerModel::A721);
        let firmware = MinerFirmware::Stock;

        let miner = AvalonMiner::new(ip, model.clone(), firmware);

        // Use mock with Avalon-specific stats
        let mock_api = MockCGMinerAPI::new().with_avalon_stats();
        let _collector = DataCollector::new(&miner, &mock_api);

        let miner_data = miner.get_data().await;

        // Should have fan data from Avalon stats
        assert_eq!(miner_data.fans.len(), 1);
        let fan = &miner_data.fans[0];
        assert_eq!(fan.position, 0);
        // Fan speed should be 5000 RPM from the MM ID1 string
        assert!(fan.rpm.is_some());
        assert!((fan.rpm.unwrap().as_rpm() - 5000.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_stats_parsing() {
        // Test the parse_stats function with typical Avalon format
        let stats_string = "Fan[5000] Temp[45-50-48] MW[1234.56 1245.67 1256.78] Elapsed[100]";
        let parsed = parse_stats(stats_string);

        // Verify Fan parsing
        assert!(parsed.contains_key("Fan"));
        let fan_data = parsed.get("Fan").unwrap();
        assert_eq!(fan_data.get(0).unwrap().as_str().unwrap(), "5000");

        // Verify Temp parsing
        assert!(parsed.contains_key("Temp"));
        let temp_data = parsed.get("Temp").unwrap();
        assert_eq!(temp_data.get(0).unwrap().as_str().unwrap(), "45-50-48");

        // Verify MW parsing
        assert!(parsed.contains_key("MW"));
        let mw_data = parsed.get("MW").unwrap().as_array().unwrap();
        assert_eq!(mw_data.len(), 3);
        assert_eq!(mw_data[0].as_str().unwrap(), "1234.56");
        assert_eq!(mw_data[1].as_str().unwrap(), "1245.67");
        assert_eq!(mw_data[2].as_str().unwrap(), "1256.78");

        // Verify Elapsed parsing
        assert!(parsed.contains_key("Elapsed"));
        let elapsed_data = parsed.get("Elapsed").unwrap();
        assert_eq!(elapsed_data.get(0).unwrap().as_str().unwrap(), "100");
    }

    #[tokio::test]
    async fn test_missing_data_handling() {
        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        let model = MinerModel::AvalonMiner(AvalonMinerModel::A721);
        let firmware = MinerFirmware::Stock;

        let miner = AvalonMiner::new(ip, model.clone(), firmware);

        // Create empty mock API that returns no responses
        let mock_api = MockCGMinerAPI {
            responses: HashMap::new(),
        };
        let collector = DataCollector::new(&miner, &mock_api);

        let miner_data = miner.get_data().await;

        // Should handle missing data gracefully
        assert_eq!(miner_data.ip, ip);
        assert_eq!(miner_data.api_version, None);
        assert_eq!(miner_data.firmware_version, None);
        assert_eq!(miner_data.hashrate, None);
        assert_eq!(miner_data.uptime, None);
        assert!(miner_data.hashboards.is_empty());
        assert!(miner_data.pools.is_empty());
        assert!(!miner_data.is_mining); // Should be false with no pools
        assert!(miner_data.fans.is_empty());
        assert_eq!(miner_data.average_temperature, None);
    }
}
