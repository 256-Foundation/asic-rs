use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use macaddr::MacAddr;
use measurements::{AngularVelocity, Frequency, Power, Temperature, Voltage};
use serde_json::Value;

use crate::data::board::{BoardData, ChipData};
use crate::data::device::MinerMake;
use crate::data::device::{DeviceInfo, HashAlgorithm, MinerFirmware, MinerModel};
use crate::data::fan::FanData;
use crate::data::hashrate::{HashRate, HashRateUnit};
use crate::data::pool::{PoolData, PoolURL};
use crate::miners::backends::traits::*;
use crate::miners::commands::MinerCommand;
use crate::miners::data::{
    DataCollector, DataExtensions, DataExtractor, DataField, DataLocation, get_by_pointer,
};
use web::EPicWebAPI;

pub mod web;

#[derive(Debug)]
pub struct EPic {
    ip: IpAddr,
    web: EPicWebAPI,
    device_info: DeviceInfo,
}

impl EPic {
    pub fn new(ip: IpAddr, make: MinerMake, model: MinerModel) -> Self {
        EPic {
            ip,
            web: EPicWebAPI::new(ip, 4028),
            device_info: DeviceInfo::new(make, model, MinerFirmware::EPic, HashAlgorithm::SHA256),
        }
    }
}

impl GetDataLocations for EPic {
    fn get_locations(&self, data_field: DataField) -> Vec<DataLocation> {
        fn cmd(endpoint: &'static str) -> MinerCommand {
            MinerCommand::WebAPI {
                command: endpoint,
                parameters: None,
            }
        }

        let summary_cmd = cmd("summary");
        let network_cmd = cmd("network");
        let _capabilities_cmd = cmd("capabilities");

        match data_field {
            DataField::Mac => vec![(
                network_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some(""),
                },
            )],
            DataField::Hostname => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Hostname"),
                },
            )],
            //DataField::SerialNumber => vec![
            //    (
            //        factory_info_cmd,
            //        DataExtractor {
            //            func: get_by_pointer,
            //            key: Some("/psu_serial"),
            //        },
            //    ),
            //    (
            //        info_cmd,
            //        DataExtractor {
            //            func: get_by_pointer,
            //            key: Some("/serial"),
            //        },
            //    ),
            //],
            //DataField::ApiVersion => vec![(
            //    info_cmd,
            //    DataExtractor {
            //        func: get_by_pointer,
            //        key: Some("/fw_version"),
            //    },
            //)],
            //DataField::FirmwareVersion => vec![(
            //    info_cmd,
            //    DataExtractor {
            //        func: get_by_pointer,
            //        key: Some("/fw_version"),
            //    },
            //)],
            //DataField::ControlBoardVersion => vec![(
            //    info_cmd,
            //    DataExtractor {
            //        func: get_by_pointer,
            //        key: Some("/platform"),
            //    },
            //)],
            DataField::Uptime => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Session/Uptime"),
                },
            )],
            //DataField::Hashrate => vec![(
            //    summary_cmd,
            //    DataExtractor {
            //        func: get_by_pointer,
            //        key: Some("/miner/hr_realtime"),
            //    },
            //)],
            //DataField::ExpectedHashrate => vec![
            //    (
            //        factory_info_cmd,
            //        DataExtractor {
            //            func: get_by_pointer,
            //            key: Some("/hr_stock"),
            //        },
            //    ),
            //    (
            //        summary_cmd,
            //        DataExtractor {
            //            func: get_by_pointer,
            //            key: Some("/miner/hr_stock"),
            //        },
            //    ),
            //],
            DataField::Wattage => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Power Supply Stats/Input Power"),
                },
            )],
            DataField::Fans => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Fans Rpm"),
                },
            )],
            //DataField::Hashboards => vec![
            //    (
            //        summary_cmd,
            //        DataExtractor {
            //            func: get_by_pointer,
            //            key: Some("/miner/chains"),
            //        },
            //    ),
            //    (
            //        chains_cmd,
            //        DataExtractor {
            //            func: get_by_pointer,
            //            key: Some(""),
            //        },
            //    ),
            //],
            DataField::Pools => vec![
                (
                    summary_cmd.clone(),
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some("/Stratum"),
                    },
                ),
                (
                    summary_cmd,
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some("/Session"),
                    },
                ),
            ],
            DataField::IsMining => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Status/Operating State"),
                },
            )],
            //DataField::Efficiency => vec![(
            //    summary_cmd,
            //    DataExtractor {
            //        func: get_by_pointer,
            //        key: Some("/Power Supply Stats/Input Power"),
            //    },
            //)],
            DataField::LightFlashing => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Misc/Locate Miner State"),
                },
            )],
            _ => vec![],
        }
    }
}

impl GetIP for EPic {
    fn get_ip(&self) -> IpAddr {
        self.ip
    }
}

impl GetDeviceInfo for EPic {
    fn get_device_info(&self) -> DeviceInfo {
        self.device_info
    }
}

impl CollectData for EPic {
    fn get_collector(&self) -> DataCollector<'_> {
        DataCollector::new(self, &self.web)
    }
}

impl GetMAC for EPic {
    fn parse_mac(&self, data: &HashMap<DataField, Value>) -> Option<MacAddr> {
        match serde_json::from_value::<HashMap<String, Value>>(data.get(&DataField::Mac)?.clone())
            .ok()
            .and_then(|inner| inner.get("dhcp").or_else(|| inner.get("static")).cloned())
            .and_then(|obj| {
                obj.get("mac_address")
                    .and_then(|v| v.as_str())
                    .map(String::from)
            }) {
            Some(mac_str) => MacAddr::from_str(&mac_str).ok(),
            None => None,
        }
    }
}

impl GetSerialNumber for EPic {
    fn parse_serial_number(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::SerialNumber)
    }
}

impl GetHostname for EPic {
    fn parse_hostname(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::Hostname)
    }
}

impl GetApiVersion for EPic {
    fn parse_api_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::ApiVersion)
    }
}

impl GetFirmwareVersion for EPic {
    fn parse_firmware_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::FirmwareVersion)
    }
}

impl GetControlBoardVersion for EPic {
    fn parse_control_board_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::ControlBoardVersion)
    }
}

impl GetHashboards for EPic {
    fn parse_hashboards(&self, data: &HashMap<DataField, Value>) -> Vec<BoardData> {
        let mut hashboards: Vec<BoardData> = Vec::new();

        let chains_data = data.get(&DataField::Hashboards).and_then(|v| v.as_array());

        if let Some(chains_array) = chains_data {
            for (idx, chain) in chains_array.iter().enumerate() {
                let hashrate = Self::extract_hashrate(chain, &["/hashrate_rt", "/hr_realtime"]);
                let expected_hashrate =
                    Self::extract_hashrate(chain, &["/hashrate_ideal", "/hr_nominal"]);

                let frequency = Self::extract_frequency(chain);
                let voltage = Self::extract_voltage(chain);
                let (board_temperature, chip_temperature) = Self::extract_temperatures(chain);

                let working_chips = Self::extract_working_chips(chain);
                let active = Self::extract_chain_active_status(chain, &hashrate);
                let serial_number = Self::extract_chain_serial(chain, data);
                let tuned = Self::extract_tuned_status(chain, data);
                let chips = Self::extract_chips(chain);

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
                    serial_number,
                    chips,
                    voltage,
                    frequency,
                    tuned,
                    active,
                });
            }
        }

        hashboards
    }
}

impl GetHashrate for EPic {
    fn parse_hashrate(&self, data: &HashMap<DataField, Value>) -> Option<HashRate> {
        data.extract_map::<f64, _>(DataField::Hashrate, |f| HashRate {
            value: f,
            unit: HashRateUnit::GigaHash,
            algo: String::from("SHA256"),
        })
    }
}

impl GetExpectedHashrate for EPic {
    fn parse_expected_hashrate(&self, data: &HashMap<DataField, Value>) -> Option<HashRate> {
        data.extract_map::<f64, _>(DataField::ExpectedHashrate, |f| HashRate {
            value: f,
            unit: HashRateUnit::GigaHash,
            algo: String::from("SHA256"),
        })
    }
}

impl GetFans for EPic {
    fn parse_fans(&self, data: &HashMap<DataField, Value>) -> Vec<FanData> {
        let mut fans: Vec<FanData> = Vec::new();
        if let Some(fans_data) = data.get(&DataField::Fans) {
            if let Some(obj) = fans_data.as_object() {
                for (key, value) in obj {
                    if let Some(num) = value.as_f64() {
                        // Extract the number from the key (e.g. "Fans Speed 3" -> 3)
                        if let Some(pos_str) = key.strip_prefix("Fans Speed ") {
                            if let Ok(pos) = pos_str.parse::<i16>() {
                                fans.push(FanData {
                                    position: pos,
                                    rpm: Some(AngularVelocity::from_rpm(num)),
                                });
                            }
                        }
                    }
                }
            }
        }

        fans
    }
}

impl GetPsuFans for EPic {}

impl GetFluidTemperature for EPic {}

impl GetWattage for EPic {
    fn parse_wattage(&self, data: &HashMap<DataField, Value>) -> Option<Power> {
        data.extract_map::<f64, _>(DataField::Wattage, |w| Power::from_watts(w))
    }
}

impl GetWattageLimit for EPic {}

impl GetLightFlashing for EPic {
    fn parse_light_flashing(&self, data: &HashMap<DataField, Value>) -> Option<bool> {
        data.extract::<bool>(DataField::LightFlashing)
    }
}

impl GetMessages for EPic {}

impl GetUptime for EPic {
    fn parse_uptime(&self, data: &HashMap<DataField, Value>) -> Option<Duration> {
        if let Some(time) = data.extract::<u64>(DataField::Uptime) {
            Some(Duration::from_secs(time))
        } else {
            None
        }
    }
}

impl GetIsMining for EPic {
    fn parse_is_mining(&self, data: &HashMap<DataField, Value>) -> bool {
        data.extract::<String>(DataField::IsMining)
            .map(|state| state != "Idling")
            .unwrap_or(false)
    }
}

impl GetPools for EPic {
    fn parse_pools(&self, data: &HashMap<DataField, Value>) -> Vec<PoolData> {
        let mut pools_vec: Vec<PoolData> = Vec::new();

        if let Some(pools_data) = data.get(&DataField::Pools) {
            if let Some(pool) = pools_data.as_object() {
                let position = pool
                    .get("Config Id")
                    .and_then(|v| v.as_u64().map(|v| v as u16));
                let url = pool
                    .get("Current Pool")
                    .and_then(|v| v.as_str())
                    .and_then(|s| {
                        if s.is_empty() {
                            None
                        } else {
                            Some(Self::parse_pool_url(s))
                        }
                    });

                let user = pool
                    .get("Current User")
                    .and_then(|v| v.as_str())
                    .map(String::from);

                let accepted_shares = pool.get("Accepted").and_then(|v| v.as_u64());
                let rejected_shares = pool.get("Rejected").and_then(|v| v.as_u64());
                let alive = pool.get("IsPoolConnected").and_then(|v| v.as_bool());

                pools_vec.push(PoolData {
                    position,
                    url,
                    accepted_shares,
                    rejected_shares,
                    active: Some(true),
                    alive,
                    user,
                });
            }
        }
        pools_vec
    }
}

// Helper methods for data extraction
impl EPic {
    fn extract_hashrate(chain: &Value, paths: &[&str]) -> Option<HashRate> {
        paths
            .iter()
            .find_map(|&path| chain.pointer(path).and_then(|v| v.as_f64()))
            .map(|f| HashRate {
                value: f,
                unit: HashRateUnit::GigaHash,
                algo: String::from("SHA256"),
            })
    }

    fn extract_frequency(chain: &Value) -> Option<Frequency> {
        chain
            .pointer("/frequency")
            .or_else(|| chain.pointer("/freq"))
            .and_then(|v| v.as_f64())
            .map(Frequency::from_megahertz)
    }

    fn extract_voltage(chain: &Value) -> Option<Voltage> {
        chain
            .pointer("/voltage")
            .and_then(|v| v.as_i64())
            .map(|v| Voltage::from_millivolts(v as f64))
    }

    fn extract_temperatures(chain: &Value) -> (Option<Temperature>, Option<Temperature>) {
        let board_temp = chain
            .pointer("/pcb_temp/max")
            .and_then(|v| v.as_i64())
            .map(|t| Temperature::from_celsius(t as f64));

        let chip_temp = chain
            .pointer("/chip_temp/max")
            .and_then(|v| v.as_i64())
            .map(|t| Temperature::from_celsius(t as f64));

        (board_temp, chip_temp)
    }

    fn extract_working_chips(chain: &Value) -> Option<u16> {
        chain
            .pointer("/chip_statuses")
            .map(|statuses| {
                let red = statuses
                    .pointer("/red")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                let orange = statuses
                    .pointer("/orange")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                (red + orange) as u16
            })
            .or_else(|| {
                chain
                    .pointer("/chips")
                    .and_then(|v| v.as_array())
                    .map(|chips| chips.len() as u16)
            })
    }

    fn extract_chain_active_status(chain: &Value, hashrate: &Option<HashRate>) -> Option<bool> {
        chain
            .pointer("/status/state")
            .and_then(|v| v.as_str())
            .map(|s| s == "mining")
            .or_else(|| hashrate.as_ref().map(|h| h.value > 0.0))
    }

    fn parse_pool_url(url_str: &str) -> PoolURL {
        // Convert host:port format to full UR
        let full_url = if url_str.starts_with("stratum") {
            url_str.to_string()
        } else {
            format!("stratum+tcp://{url_str}")
        };

        PoolURL::from(full_url)
    }

    fn extract_chain_serial(chain: &Value, data: &HashMap<DataField, Value>) -> Option<String> {
        // Try to get serial from chain-specific data first (factory-info)
        chain
            .pointer("/serial")
            .and_then(|v| v.as_str())
            .map(String::from)
            .or_else(|| {
                // Fallback to miner-wide serial number
                data.extract::<String>(DataField::SerialNumber)
            })
    }

    fn extract_tuned_status(_chain: &Value, data: &HashMap<DataField, Value>) -> Option<bool> {
        // Check miner state to determine tuning status
        if let Some(miner_state) = data.extract::<String>(DataField::IsMining) {
            match miner_state.as_str() {
                "auto-tuning" => Some(false), // Currently tuning, not yet tuned
                "mining" => Some(true),       // Tuned and mining
                _ => None,
            }
        } else {
            None
        }
    }

    fn extract_chips(chain: &Value) -> Vec<ChipData> {
        let mut chips: Vec<ChipData> = Vec::new();

        if let Some(chips_array) = chain.pointer("/chips").and_then(|v| v.as_array()) {
            for (idx, chip) in chips_array.iter().enumerate() {
                let hashrate = chip
                    .pointer("/hr")
                    .and_then(|v| v.as_f64())
                    .map(|f| HashRate {
                        value: f,
                        unit: HashRateUnit::GigaHash,
                        algo: String::from("SHA256"),
                    });

                let temperature = chip
                    .pointer("/temp")
                    .and_then(|v| v.as_f64())
                    .map(Temperature::from_celsius);

                let voltage = chip
                    .pointer("/volt")
                    .and_then(|v| v.as_i64())
                    .map(|v| Voltage::from_millivolts(v as f64));

                let frequency = chip
                    .pointer("/freq")
                    .and_then(|v| v.as_i64())
                    .map(|f| Frequency::from_megahertz(f as f64));

                let working = hashrate.as_ref().map(|hr| hr.value > 0.0);

                chips.push(ChipData {
                    position: chip
                        .pointer("/id")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(idx as u64) as u16,
                    hashrate,
                    temperature,
                    voltage,
                    frequency,
                    tuned: None,
                    working,
                });
            }
        }

        chips
    }
}
