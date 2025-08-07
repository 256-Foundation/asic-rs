use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use macaddr::MacAddr;
use measurements::{AngularVelocity, Frequency, Power, Temperature};
use serde_json::Value;

use crate::data::board::BoardData;
use crate::data::device::MinerMake;
use crate::data::device::{DeviceInfo, HashAlgorithm, MinerFirmware, MinerModel};
use crate::data::fan::FanData;
use crate::data::hashrate::{HashRate, HashRateUnit};
use crate::data::pool::{PoolData, PoolScheme, PoolURL};
use crate::miners::backends::traits::*;
use crate::miners::commands::MinerCommand;
use crate::miners::data::{
    DataCollector, DataExtensions, DataExtractor, DataField, DataLocation, get_by_pointer,
};
use web::VnishWebAPI;

pub mod web;

#[derive(Debug)]
pub struct Vnish {
    ip: IpAddr,
    web: VnishWebAPI,
    device_info: DeviceInfo,
}

impl Vnish {
    pub fn new(ip: IpAddr, model: MinerModel, firmware: MinerFirmware) -> Self {
        Vnish {
            ip,
            web: VnishWebAPI::new(ip, 80), // Standard HTTP port for VnishOS
            device_info: DeviceInfo::new(
                MinerMake::AntMiner, // VnishOS typically runs on AntMiner hardware
                model,
                firmware,
                HashAlgorithm::SHA256,
            ),
        }
    }
}

impl GetDataLocations for Vnish {
    fn get_locations(&self, data_field: DataField) -> Vec<DataLocation> {
        let info_cmd: MinerCommand = MinerCommand::WebAPI {
            command: "info",
            parameters: None,
        };
        let status_cmd: MinerCommand = MinerCommand::WebAPI {
            command: "status",
            parameters: None,
        };
        let summary_cmd: MinerCommand = MinerCommand::WebAPI {
            command: "summary",
            parameters: None,
        };
        let chains_cmd: MinerCommand = MinerCommand::WebAPI {
            command: "chains",
            parameters: None,
        };

        match data_field {
            DataField::Mac => vec![(
                info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/system/network_status/mac"),
                },
            )],
            DataField::SerialNumber => vec![(
                info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/serial"),
                },
            )],
            DataField::Hostname => vec![(
                info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/system/network_status/hostname"),
                },
            )],
            DataField::ApiVersion => vec![(
                info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/fw_version"),
                },
            )],
            DataField::FirmwareVersion => vec![(
                info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/fw_version"),
                },
            )],
            DataField::ControlBoardVersion => vec![(
                info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/platform"),
                },
            )],
            DataField::Uptime => vec![(
                info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/system/uptime"),
                },
            )],
            DataField::Hashrate => vec![(
                summary_cmd.clone(),
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/miner/hr_realtime"),
                },
            )],
            DataField::ExpectedHashrate => vec![(
                summary_cmd.clone(),
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/miner/hr_nominal"),
                },
            )],
            DataField::Wattage => vec![(
                summary_cmd.clone(),
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/miner/power_consumption"),
                },
            )],
            DataField::Fans => vec![(
                summary_cmd.clone(),
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/miner/cooling/fans"),
                },
            )],
            DataField::Hashboards => vec![(
                chains_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some(""),
                },
            )],
            DataField::Pools => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/miner/pools"),
                },
            )],
            DataField::IsMining => vec![(
                status_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/miner_state"),
                },
            )],
            _ => vec![],
        }
    }
}

impl GetIP for Vnish {
    fn get_ip(&self) -> IpAddr {
        self.ip
    }
}

impl GetDeviceInfo for Vnish {
    fn get_device_info(&self) -> DeviceInfo {
        self.device_info.clone()
    }
}

impl CollectData for Vnish {
    fn get_collector(&self) -> DataCollector {
        DataCollector::new(self, &self.web)
    }
}

impl GetMAC for Vnish {
    fn parse_mac(&self, data: &HashMap<DataField, Value>) -> Option<MacAddr> {
        data.extract::<String>(DataField::Mac)
            .and_then(|s| MacAddr::from_str(&s).ok())
    }
}

impl GetSerialNumber for Vnish {
    fn parse_serial_number(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::SerialNumber)
    }
}

impl GetHostname for Vnish {
    fn parse_hostname(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::Hostname)
    }
}

impl GetApiVersion for Vnish {
    fn parse_api_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::ApiVersion)
    }
}

impl GetFirmwareVersion for Vnish {
    fn parse_firmware_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::FirmwareVersion)
    }
}

impl GetControlBoardVersion for Vnish {
    fn parse_control_board_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::ControlBoardVersion)
    }
}

impl GetHashboards for Vnish {
    fn parse_hashboards(&self, data: &HashMap<DataField, Value>) -> Vec<BoardData> {
        let mut hashboards: Vec<BoardData> = Vec::new();

        if let Some(chains_data) = data.get(&DataField::Hashboards) {
            if let Some(chains_array) = chains_data.as_array() {
                for (idx, chain) in chains_array.iter().enumerate() {
                    // Use correct paths from AntmChainChips schema
                    let hashrate =
                        chain
                            .pointer("/hr_realtime")
                            .and_then(|v| v.as_f64())
                            .map(|f| HashRate {
                                value: f,
                                unit: HashRateUnit::TeraHash, // VnishOS returns TH/s
                                algo: String::from("SHA256"),
                            });

                    let expected_hashrate = chain
                        .pointer("/hr_nominal")
                        .and_then(|v| v.as_f64())
                        .map(|f| HashRate {
                            value: f,
                            unit: HashRateUnit::TeraHash,
                            algo: String::from("SHA256"),
                        });

                    let frequency = chain
                        .pointer("/freq")
                        .and_then(|v| v.as_f64())
                        .map(Frequency::from_megahertz);

                    // Extract temperature sensors data properly
                    let sensors_data = chain.pointer("/sensors").and_then(|v| v.as_array());
                    let (board_temperature, chip_temperature) = if let Some(sensors) = sensors_data
                    {
                        let mut pcb_temps = Vec::new();
                        let mut chip_temps = Vec::new();

                        for sensor in sensors {
                            if let Some(pcb_temp) =
                                sensor.pointer("/pcb_temp").and_then(|v| v.as_i64())
                            {
                                pcb_temps.push(pcb_temp as f64);
                            }
                            if let Some(chip_temp) =
                                sensor.pointer("/chip_temp").and_then(|v| v.as_i64())
                            {
                                chip_temps.push(chip_temp as f64);
                            }
                        }

                        let board_temp = if !pcb_temps.is_empty() {
                            pcb_temps
                                .iter()
                                .max_by(|a, b| {
                                    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                                })
                                .map(|&temp| Temperature::from_celsius(temp))
                        } else {
                            None
                        };

                        let chip_temp = if !chip_temps.is_empty() {
                            chip_temps
                                .iter()
                                .max_by(|a, b| {
                                    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                                })
                                .map(|&temp| Temperature::from_celsius(temp))
                        } else {
                            None
                        };

                        (board_temp, chip_temp)
                    } else {
                        (None, None)
                    };

                    // Count working chips from individual chip data
                    let chips_array = chain.pointer("/chips").and_then(|v| v.as_array());
                    let working_chips = chips_array.map(|chips| chips.len() as u16);

                    let active = hashrate.as_ref().map(|h| h.value > 0.0);

                    hashboards.push(BoardData {
                        position: chain
                            .pointer("/id")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(idx as u64) as u8,
                        hashrate,
                        expected_hashrate,
                        board_temperature,
                        intake_temperature: chip_temperature,
                        outlet_temperature: chip_temperature,
                        expected_chips: self.device_info.hardware.chips,
                        working_chips,
                        serial_number: None, // Not provided in AntmChainChips schema
                        chips: vec![],       // Could be populated from /chips array if needed
                        voltage: None,       // Not directly provided
                        frequency,
                        tuned: Some(true),
                        active,
                    });
                }
            }
        }

        hashboards
    }
}

impl GetHashrate for Vnish {
    fn parse_hashrate(&self, data: &HashMap<DataField, Value>) -> Option<HashRate> {
        data.extract_map::<f64, _>(DataField::Hashrate, |f| HashRate {
            value: f,
            unit: HashRateUnit::TeraHash,
            algo: String::from("SHA256"),
        })
    }
}

impl GetExpectedHashrate for Vnish {
    fn parse_expected_hashrate(&self, data: &HashMap<DataField, Value>) -> Option<HashRate> {
        data.extract_map::<f64, _>(DataField::ExpectedHashrate, |f| HashRate {
            value: f,
            unit: HashRateUnit::TeraHash,
            algo: String::from("SHA256"),
        })
    }
}

impl GetFans for Vnish {
    fn parse_fans(&self, data: &HashMap<DataField, Value>) -> Vec<FanData> {
        let mut fans: Vec<FanData> = Vec::new();

        if let Some(fans_data) = data.get(&DataField::Fans) {
            if let Some(fans_array) = fans_data.as_array() {
                for (idx, fan) in fans_array.iter().enumerate() {
                    if let Some(rpm) = fan.pointer("/rpm").and_then(|v| v.as_i64()) {
                        fans.push(FanData {
                            position: idx as i16,
                            rpm: Some(AngularVelocity::from_rpm(rpm as f64)),
                        });
                    }
                }
            }
        }

        fans
    }
}

impl GetPsuFans for Vnish {}

impl GetFluidTemperature for Vnish {}

impl GetWattage for Vnish {
    fn parse_wattage(&self, data: &HashMap<DataField, Value>) -> Option<Power> {
        data.extract_map::<i64, _>(DataField::Wattage, |w| Power::from_watts(w as f64))
    }
}

impl GetWattageLimit for Vnish {}

impl GetLightFlashing for Vnish {}

impl GetMessages for Vnish {}

impl GetUptime for Vnish {
    fn parse_uptime(&self, data: &HashMap<DataField, Value>) -> Option<Duration> {
        data.extract::<String>(DataField::Uptime)
            .and_then(|uptime_str| {
                // Parse uptime string format (e.g., "1 day, 2:30:45" or similar)
                // This is a simplified parser - you may need to adjust based on actual format
                if let Some(seconds_part) = uptime_str.split_whitespace().last() {
                    if let Ok(seconds) = seconds_part.parse::<u64>() {
                        return Some(Duration::from_secs(seconds));
                    }
                }
                None
            })
    }
}

impl GetIsMining for Vnish {
    fn parse_is_mining(&self, data: &HashMap<DataField, Value>) -> bool {
        data.extract::<String>(DataField::IsMining)
            .map(|state| state == "mining")
            .unwrap_or(false)
    }
}

impl GetPools for Vnish {
    fn parse_pools(&self, data: &HashMap<DataField, Value>) -> Vec<PoolData> {
        let mut pools: Vec<PoolData> = Vec::new();

        if let Some(pools_data) = data.get(&DataField::Pools) {
            if let Some(pools_array) = pools_data.as_array() {
                for (idx, pool) in pools_array.iter().enumerate() {
                    let url = pool
                        .pointer("/url")
                        .and_then(|v| v.as_str())
                        .map(|url_str| {
                            // Parse the URL - assume stratum format
                            PoolURL {
                                scheme: PoolScheme::StratumV1,
                                host: url_str.to_string(),
                                port: 4444, // Default stratum port
                                pubkey: None,
                            }
                        });

                    let user = pool
                        .pointer("/user")
                        .and_then(|v| v.as_str())
                        .map(String::from);

                    let accepted_shares = pool.pointer("/accepted").and_then(|v| v.as_u64());

                    let rejected_shares = pool.pointer("/rejected").and_then(|v| v.as_u64());

                    // Pool status according to spec: ["offline","working","disabled","active","rejecting","unknown"]
                    let pool_status = pool.pointer("/status").and_then(|v| v.as_str());

                    let active = pool_status.map(|status| matches!(status, "active" | "working"));

                    let alive = pool_status.map(|status| !matches!(status, "offline" | "disabled"));

                    pools.push(PoolData {
                        position: Some(idx as u16),
                        url,
                        accepted_shares,
                        rejected_shares,
                        active,
                        alive,
                        user,
                    });
                }
            }
        }

        pools
    }
}
