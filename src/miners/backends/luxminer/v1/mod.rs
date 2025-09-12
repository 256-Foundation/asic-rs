use anyhow::{Result, anyhow};
use async_trait::async_trait;
use macaddr::MacAddr;
use measurements::{AngularVelocity, Frequency, Power, Temperature};
use serde_json::Value;
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use crate::data::board::BoardData;
use crate::data::device::{DeviceInfo, HashAlgorithm, MinerFirmware, MinerMake, MinerModel};
use crate::data::fan::FanData;
use crate::data::hashrate::{HashRate, HashRateUnit};
use crate::data::message::{MessageSeverity, MinerMessage};
use crate::data::pool::{PoolData, PoolURL};
use crate::miners::backends::traits::*;
use crate::miners::commands::MinerCommand;
use crate::miners::data::{
    DataCollector, DataExtensions, DataExtractor, DataField, DataLocation, get_by_pointer,
};

use rpc::LUXMinerRPCAPI;

mod rpc;

#[derive(Debug)]
pub struct LuxMinerV1 {
    pub ip: IpAddr,
    pub rpc: LUXMinerRPCAPI,
    pub device_info: DeviceInfo,
}

impl LuxMinerV1 {
    pub fn new(ip: IpAddr, model: MinerModel) -> Self {
        LuxMinerV1 {
            ip,
            rpc: LUXMinerRPCAPI::new(ip),
            device_info: DeviceInfo::new(
                MinerMake::AntMiner,
                model,
                MinerFirmware::LuxOS,
                HashAlgorithm::SHA256,
            ),
        }
    }

    fn parse_temp_string(temp_str: &str) -> Option<Temperature> {
        let temps: Vec<f64> = temp_str
            .split('-')
            .filter_map(|s| s.parse().ok())
            .filter(|&temp| temp > 0.0)
            .collect();

        if !temps.is_empty() {
            let avg = temps.iter().sum::<f64>() / temps.len() as f64;
            Some(Temperature::from_celsius(avg))
        } else {
            None
        }
    }
}

#[async_trait]
impl APIClient for LuxMinerV1 {
    async fn get_api_result(&self, command: &MinerCommand) -> Result<Value> {
        match command {
            MinerCommand::RPC { .. } => self.rpc.get_api_result(command).await,
            _ => Err(anyhow!("Unsupported command type for LuxMiner API")),
        }
    }
}

impl GetDataLocations for LuxMinerV1 {
    fn get_locations(&self, data_field: DataField) -> Vec<DataLocation> {
        let version_cmd = MinerCommand::RPC {
            command: "version",
            parameters: None,
        };

        let stats_cmd = MinerCommand::RPC {
            command: "stats",
            parameters: None,
        };

        let summary_cmd = MinerCommand::RPC {
            command: "summary",
            parameters: None,
        };

        let pools_cmd = MinerCommand::RPC {
            command: "pools",
            parameters: None,
        };

        let config_cmd = MinerCommand::RPC {
            command: "config",
            parameters: None,
        };

        let fans_cmd = MinerCommand::RPC {
            command: "fans",
            parameters: None,
        };

        let power_cmd = MinerCommand::RPC {
            command: "power",
            parameters: None,
        };

        let profiles_cmd = MinerCommand::RPC {
            command: "profiles",
            parameters: None,
        };

        match data_field {
            DataField::Mac => vec![(
                config_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/CONFIG/0/MACAddr"),
                    tag: None,
                },
            )],
            DataField::ApiVersion => vec![(
                version_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/VERSION/0/API"),
                    tag: None,
                },
            )],
            DataField::FirmwareVersion => vec![(
                version_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/VERSION/0/Miner"),
                    tag: None,
                },
            )],
            DataField::Hostname => vec![(
                config_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/CONFIG/0/hostname"), // May need adjustment based on actual LuxOS response
                    tag: None,
                },
            )],
            DataField::Hashrate => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/SUMMARY/0/GHS 5s"),
                    tag: None,
                },
            )],
            DataField::ExpectedHashrate => vec![(
                stats_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/1/total_rateideal"),
                    tag: None,
                },
            )],
            DataField::Fans => vec![(
                fans_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/FANS"),
                    tag: None,
                },
            )],
            DataField::Hashboards => vec![(
                stats_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/1"),
                    tag: None,
                },
            )],
            DataField::LightFlashing => vec![(
                config_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/CONFIG/0/RedLed"),
                    tag: None,
                },
            )],
            DataField::IsMining => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/SUMMARY/0/GHS 5s"),
                    tag: None,
                },
            )],
            DataField::Uptime => vec![(
                stats_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/1/Elapsed"),
                    tag: None,
                },
            )],
            DataField::Pools => vec![(
                pools_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/POOLS"),
                    tag: None,
                },
            )],
            DataField::Wattage => vec![(
                power_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/POWER/0/Watts"),
                    tag: None,
                },
            )],
            DataField::WattageLimit => vec![(
                profiles_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/PROFILES"),
                    tag: None,
                },
            )],
            DataField::SerialNumber => vec![(
                config_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/CONFIG/0/serial_no"),
                    tag: None,
                },
            )],
            DataField::Messages => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATUS"),
                    tag: None,
                },
            )],
            _ => vec![],
        }
    }
}

impl GetIP for LuxMinerV1 {
    fn get_ip(&self) -> IpAddr {
        self.ip
    }
}

impl GetDeviceInfo for LuxMinerV1 {
    fn get_device_info(&self) -> DeviceInfo {
        self.device_info
    }
}

impl CollectData for LuxMinerV1 {
    fn get_collector(&self) -> DataCollector<'_> {
        DataCollector::new(self)
    }
}

impl GetMAC for LuxMinerV1 {
    fn parse_mac(&self, data: &HashMap<DataField, Value>) -> Option<MacAddr> {
        data.extract::<String>(DataField::Mac)
            .and_then(|s| MacAddr::from_str(&s.to_uppercase()).ok())
    }
}

impl GetHostname for LuxMinerV1 {
    fn parse_hostname(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::Hostname)
    }
}

impl GetApiVersion for LuxMinerV1 {
    fn parse_api_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::ApiVersion)
    }
}

impl GetFirmwareVersion for LuxMinerV1 {
    fn parse_firmware_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::FirmwareVersion)
    }
}

impl GetHashboards for LuxMinerV1 {
    fn parse_hashboards(&self, data: &HashMap<DataField, Value>) -> Vec<BoardData> {
        let mut hashboards: Vec<BoardData> = Vec::new();
        let board_count = self.device_info.hardware.boards.unwrap_or(3);

        for idx in 0..board_count {
            hashboards.push(BoardData {
                hashrate: None,
                position: idx,
                expected_hashrate: None,
                board_temperature: None,
                intake_temperature: None,
                outlet_temperature: None,
                expected_chips: self.device_info.hardware.chips,
                working_chips: None,
                serial_number: None,
                chips: vec![],
                voltage: None,
                frequency: None,
                tuned: Some(false),
                active: Some(false),
            });
        }

        if let Some(stats_data) = data.get(&DataField::Hashboards) {
            for idx in 1..=board_count {
                let board_idx = (idx - 1) as usize;
                if board_idx >= hashboards.len() {
                    break;
                }

                // Parse chain rate
                if let Some(hashrate) = stats_data
                    .get(format!("chain_rate{}", idx))
                    .and_then(|v| v.as_f64())
                    .map(|f| {
                        HashRate {
                            value: f,
                            unit: HashRateUnit::GigaHash,
                            algo: String::from("SHA256"),
                        }
                        .as_unit(HashRateUnit::TeraHash)
                    })
                {
                    hashboards[board_idx].hashrate = Some(hashrate);
                }

                // Parse working chips
                if let Some(working_chips) = stats_data
                    .get(format!("chain_acn{}", idx))
                    .and_then(|v| v.as_u64())
                    .map(|u| u as u16)
                {
                    hashboards[board_idx].working_chips = Some(working_chips);
                }

                // Parse board temperature - LuxOS format
                if let Some(board_temp) = stats_data
                    .get(format!("temp_pcb{}", idx))
                    .and_then(|v| v.as_str())
                    .and_then(Self::parse_temp_string)
                {
                    hashboards[board_idx].board_temperature = Some(board_temp);
                }

                // Parse chip temperature
                if let Some(chip_temp) = stats_data
                    .get(format!("temp_chip{}", idx))
                    .and_then(|v| v.as_str())
                    .and_then(Self::parse_temp_string)
                {
                    hashboards[board_idx].intake_temperature = Some(chip_temp);
                }

                // Parse frequency if available
                if let Some(frequency) = stats_data
                    .get(format!("freq{}", idx))
                    .and_then(|v| v.as_u64())
                    .map(|f| Frequency::from_megahertz(f as f64))
                {
                    hashboards[board_idx].frequency = Some(frequency);
                }

                let has_hashrate = hashboards[board_idx]
                    .hashrate
                    .as_ref()
                    .map(|h| h.value > 0.0)
                    .unwrap_or(false);
                let has_chips = hashboards[board_idx]
                    .working_chips
                    .map(|chips| chips > 0)
                    .unwrap_or(false);

                hashboards[board_idx].active = Some(has_hashrate || has_chips);
                hashboards[board_idx].tuned = Some(has_hashrate || has_chips);
            }
        }

        hashboards
    }
}

impl GetHashrate for LuxMinerV1 {
    fn parse_hashrate(&self, data: &HashMap<DataField, Value>) -> Option<HashRate> {
        data.extract_map::<f64, _>(DataField::Hashrate, |f| {
            HashRate {
                value: f,
                unit: HashRateUnit::GigaHash,
                algo: String::from("SHA256"),
            }
            .as_unit(HashRateUnit::TeraHash)
        })
    }
}

impl GetExpectedHashrate for LuxMinerV1 {
    fn parse_expected_hashrate(&self, data: &HashMap<DataField, Value>) -> Option<HashRate> {
        data.extract_map::<f64, _>(DataField::ExpectedHashrate, |f| {
            HashRate {
                value: f,
                unit: HashRateUnit::GigaHash,
                algo: String::from("SHA256"),
            }
            .as_unit(HashRateUnit::TeraHash)
        })
    }
}

impl GetFans for LuxMinerV1 {
    fn parse_fans(&self, data: &HashMap<DataField, Value>) -> Vec<FanData> {
        data.get(&DataField::Fans)
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(idx, fan_info)| {
                let rpm = fan_info.get("RPM")?.as_f64()?;
                Some(FanData {
                    position: idx as i16,
                    rpm: Some(AngularVelocity::from_rpm(rpm)),
                })
            })
            .collect()
    }
}

impl GetLightFlashing for LuxMinerV1 {
    fn parse_light_flashing(&self, data: &HashMap<DataField, Value>) -> Option<bool> {
        data.extract::<String>(DataField::LightFlashing)
            .map(|s| s.to_lowercase() != "off")
    }
}

impl GetUptime for LuxMinerV1 {
    fn parse_uptime(&self, data: &HashMap<DataField, Value>) -> Option<Duration> {
        data.extract_map::<u64, _>(DataField::Uptime, Duration::from_secs)
    }
}

impl GetIsMining for LuxMinerV1 {
    fn parse_is_mining(&self, data: &HashMap<DataField, Value>) -> bool {
        data.extract::<f64>(DataField::IsMining)
            .map(|hr| hr > 0.0)
            .unwrap_or(false)
    }
}

impl GetPools for LuxMinerV1 {
    fn parse_pools(&self, data: &HashMap<DataField, Value>) -> Vec<PoolData> {
        data.get(&DataField::Pools)
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(idx, pool)| PoolData {
                position: Some(idx as u16),
                url: pool
                    .get("URL")?
                    .as_str()
                    .map(|s| PoolURL::from(s.to_string())),
                user: pool.get("User")?.as_str().map(String::from),
                alive: pool.get("Status")?.as_str().map(|s| s == "Alive"),
                active: pool.get("Stratum Active")?.as_bool(),
                accepted_shares: pool.get("Accepted")?.as_u64(),
                rejected_shares: pool.get("Rejected")?.as_u64(),
            })
            .collect()
    }
}

impl GetSerialNumber for LuxMinerV1 {
    fn parse_serial_number(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::SerialNumber)
    }
}

impl GetControlBoardVersion for LuxMinerV1 {}

impl GetWattage for LuxMinerV1 {
    fn parse_wattage(&self, data: &HashMap<DataField, Value>) -> Option<Power> {
        data.extract_map::<f64, _>(DataField::Wattage, Power::from_watts)
    }
}

impl GetWattageLimit for LuxMinerV1 {
    fn parse_wattage_limit(&self, data: &HashMap<DataField, Value>) -> Option<Power> {
        data.get(&DataField::WattageLimit)?
            .as_array()?
            .iter()
            .find(|prof| prof.get("Active")?.as_bool() == Some(true))
            .and_then(|prof| prof.get("Power")?.as_f64())
            .map(Power::from_watts)
    }
}

impl GetFluidTemperature for LuxMinerV1 {}

impl GetPsuFans for LuxMinerV1 {}

impl GetMessages for LuxMinerV1 {
    fn parse_messages(&self, data: &HashMap<DataField, Value>) -> Vec<MinerMessage> {
        data.get(&DataField::Messages)
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(idx, item)| {
                let status = item.get("STATUS")?.as_str()?;
                (status != "S").then(|| {
                    let text = item
                        .get("Msg")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error");
                    let severity = match status {
                        "E" => MessageSeverity::Error,
                        "W" => MessageSeverity::Warning,
                        _ => MessageSeverity::Info,
                    };
                    MinerMessage::new(0, idx as u64, text.to_string(), severity)
                })
            })
            .collect()
    }
}
