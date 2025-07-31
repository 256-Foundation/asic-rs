
use crate::data::device::{DeviceInfo, HashAlgorithm, MinerFirmware, MinerModel};
use crate::data::miner::MinerData;
use crate::miners::api::rpc::cgminer::CGMinerRPC;
use crate::miners::backends::traits::GetMinerData;
use crate::miners::commands::MinerCommand;
use crate::miners::data::{
    DataCollector, DataExtensions, DataExtractor, DataField, DataLocation, get_by_pointer,
};
use async_trait::async_trait;
use macaddr::MacAddr;
use measurements::Power;
use regex::Regex;
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::SystemTime;
use crate::data::device::MinerMake;

#[derive(Debug)]
pub struct AvalonMiner {
    model: MinerModel,
    rpc: CGMinerRPC,
    ip: IpAddr,
    miner_firmware: MinerFirmware
}

impl AvalonMiner {
    pub fn new(ip: IpAddr, model: MinerModel, miner_firmware: MinerFirmware) -> Self {
        Self {
            model,
            rpc: CGMinerRPC::new(ip),
            ip,
            miner_firmware
        }
    }
}

pub fn parse_stats(response: &str) -> HashMap<&str, Value> {
    let re = Regex::new(r".+?\[.*?]").expect("Failed to compile regex");
    let mut stats_dict: HashMap<&str, Value> = HashMap::new();

    for item_match in re.find_iter(response) {
        let item = item_match.as_str();
        let key_part: &str;
        let value_json: Value;

        if item.contains(": ") {
            let parts: Vec<&str> = item.splitn(2, '[').collect();
            if parts.len() != 2 {
                continue;
            }

            key_part = parts[0].trim();
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

            key_part = key_components.remove(0);

            let mut value_vec: Vec<Value> = key_components.iter().map(|s| json!(s)).collect();
            value_vec.push(json!(val_str));
            value_json = json!(value_vec);
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

        let device_info = DeviceInfo::new(MinerMake::AvalonMiner, self.model.clone(), self.miner_firmware, HashAlgorithm::SHA256);

        MinerData {
            schema_version: env!("CARGO_PKG_VERSION").to_owned(),
            timestamp,
            ip: self.ip,
            mac,
            device_info,
            serial_number: None,
            hostname: None,
            api_version: None,
            firmware_version: None,
            control_board_version: None,
            expected_hashboards: None,
            hashboards: vec![],
            hashrate: None,
            expected_hashrate: None,
            expected_chips: None,
            total_chips: None,
            expected_fans: None,
            fans: vec![],
            psu_fans: vec![],
            average_temperature: None,
            fluid_temperature: None,
            wattage,
            wattage_limit: None,
            efficiency: None,
            light_flashing: None,
            messages: vec![],
            uptime: None,
            is_mining: false,
            pools: vec![],
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
                    key: Some("/STATS/0/MM ID0"),
                },
            )],
            DataField::Hashrate => vec![(
                devs_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/DEVS/0/MHS 1m"),
                },
            )],
            DataField::Fans | DataField::PsuFans => vec![(
                stats_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/0/MM IDO"),
                },
            )],
            DataField::AverageTemperature => vec![(
                stats_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/0/MM ID0"),
                },
            )],
            DataField::Wattage | DataField::WattageLimit | DataField::LightFlashing => vec![(
                stats_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/0/MM IDO"),
                },
            )],
            DataField::Messages => vec![],
            DataField::Uptime => vec![(
                stats_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/1/Elapsed"),
                },
            )],
            DataField::Pools => vec![(
                pools_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some(""),
                },
            )],
            _ => vec![],
        }
    }
}
