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
use web::PowerPlayWebAPI;

pub mod web;

#[derive(Debug)]
pub struct PowerPlay {
    ip: IpAddr,
    web: PowerPlayWebAPI,
    device_info: DeviceInfo,
}

impl PowerPlay {
    pub fn new(ip: IpAddr, make: MinerMake, model: MinerModel) -> Self {
        PowerPlay {
            ip,
            web: PowerPlayWebAPI::new(ip, 4028),
            device_info: DeviceInfo::new(make, model, MinerFirmware::EPic, HashAlgorithm::SHA256),
        }
    }
}

impl GetDataLocations for PowerPlay {
    fn get_locations(&self, data_field: DataField) -> Vec<DataLocation> {
        fn cmd(endpoint: &'static str) -> MinerCommand {
            MinerCommand::WebAPI {
                command: endpoint,
                parameters: None,
            }
        }

        let summary_cmd = cmd("summary");
        let network_cmd = cmd("network");
        let capabilities_cmd = cmd("capabilities");
        let chip_temps_cmd = cmd("temps/chip");
        let chip_voltages_cmd = cmd("voltages");
        let chip_hashrates_cmd = cmd("hashrate");
        let chip_clocks_cmd = cmd("clocks");
        let temps_cmd = cmd("temps");

        match data_field {
            DataField::Mac => vec![(
                network_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some(""),
                    tag: None,
                },
            )],
            DataField::Hostname => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Hostname"),
                    tag: None,
                },
            )],
            DataField::Uptime => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Session/Uptime"),
                    tag: None,
                },
            )],
            DataField::Wattage => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Power Supply Stats/Input Power"),
                    tag: None,
                },
            )],
            DataField::Fans => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Fans Rpm"),
                    tag: None,
                },
            )],
            DataField::Hashboards => vec![
                (
                    temps_cmd,
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some(""),
                        tag: Some("Board Temps"),
                    },
                ),
                (
                    summary_cmd.clone(),
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some("/HBStatus"),
                        tag: Some("HBStatus"),
                    },
                ),
                (
                    summary_cmd,
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some("/HBs"),
                        tag: Some("HBs"),
                    },
                ),
                (
                    chip_temps_cmd,
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some(""),
                        tag: Some("Chip Temps"),
                    },
                ),
                (
                    chip_voltages_cmd,
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some(""),
                        tag: Some("Chip Voltages"),
                    },
                ),
                (
                    chip_hashrates_cmd,
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some(""),
                        tag: Some("Chip Hashrates"),
                    },
                ),
                (
                    chip_clocks_cmd,
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some(""),
                        tag: Some("Chip Clocks"),
                    },
                ),
                (
                    capabilities_cmd,
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some(""),
                        tag: Some("Capabilities"),
                    },
                ),
            ],
            DataField::Pools => vec![
                (
                    summary_cmd.clone(),
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some("/Stratum"),
                        tag: None,
                    },
                ),
                (
                    summary_cmd,
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some("/Session"),
                        tag: None,
                    },
                ),
            ],
            DataField::IsMining => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Status/Operating State"),
                    tag: None,
                },
            )],
            DataField::LightFlashing => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Misc/Locate Miner State"),
                    tag: None,
                },
            )],
            DataField::ControlBoardVersion => vec![(
                capabilities_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Control Board Version/cpuHardware"),
                    tag: None,
                },
            )],
            DataField::SerialNumber => vec![(
                capabilities_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Control Board Version/cpuSerial"),
                    tag: None,
                },
            )],
            DataField::ExpectedHashrate => vec![(
                capabilities_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Default Hashrate"),
                    tag: None,
                },
            )],
            DataField::FirmwareVersion => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Software"),
                    tag: None,
                },
            )],
            DataField::Hashrate => vec![(
                summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/Session/Average MHs"),
                    tag: None,
                },
            )],
            _ => vec![],
        }
    }
}

impl GetIP for PowerPlay {
    fn get_ip(&self) -> IpAddr {
        self.ip
    }
}

impl GetDeviceInfo for PowerPlay {
    fn get_device_info(&self) -> DeviceInfo {
        self.device_info
    }
}

impl CollectData for PowerPlay {
    fn get_collector(&self) -> DataCollector<'_> {
        DataCollector::new(self, &self.web)
    }
}

impl GetMAC for PowerPlay {
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

impl GetSerialNumber for PowerPlay {
    fn parse_serial_number(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::SerialNumber)
    }
}

impl GetHostname for PowerPlay {
    fn parse_hostname(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::Hostname)
    }
}

impl GetApiVersion for PowerPlay {
    fn parse_api_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::ApiVersion)
    }
}

impl GetFirmwareVersion for PowerPlay {
    fn parse_firmware_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::FirmwareVersion)
    }
}

impl GetControlBoardVersion for PowerPlay {
    fn parse_control_board_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::ControlBoardVersion)
    }
}

impl GetHashboards for PowerPlay {
    fn parse_hashboards(&self, data: &HashMap<DataField, Value>) -> Vec<BoardData> {
        let mut hashboards: Vec<BoardData> = Vec::new();
        let info = data.get(&DataField::Hashboards);
        let combined_hbs = Self::combine_by_index(info.unwrap());
        let capabilities = match info.and_then(|v| v.get("Capabilities")) {
            Some(caps) => caps,
            None => return hashboards,
        };
        for hb in combined_hbs {
            let mut hashrate = None;
            let mut frequency = None;
            let mut voltage = None;
            let mut performance = 0.0;
            let serial_number = None;
            let mut expected_hashrate = HashRate {
                value: 0.0,
                unit: HashRateUnit::MegaHash,
                algo: String::from("SHA256"),
            };

            let mut chips = vec![];

            if hb
                .pointer("/HBStatus/Enabled")
                .and_then(|v| v.as_bool())
                .expect("HBStatus/Enabled should be present")
                && hb
                    .pointer("/HBStatus/Detected")
                    .and_then(|v| v.as_bool())
                    .expect("HBStatus/Enabled should be present")
            {
                hashrate = hb
                    .pointer("/HBs/Hashrate")
                    .and_then(|v| v.as_array())
                    .and_then(|v| v.first().and_then(|f| f.as_f64()))
                    .map(|h| HashRate {
                        value: h,
                        unit: HashRateUnit::MegaHash,
                        algo: String::from("SHA256"),
                    });

                frequency = hb
                    .pointer("/HBs/Core Clock Avg")
                    .and_then(|v| v.as_f64())
                    .map(Frequency::from_megahertz);

                voltage = hb
                    .pointer("/HBs/Input Voltage")
                    .and_then(|v| v.as_f64())
                    .map(Voltage::from_volts);

                performance = hb
                    .pointer("/HBs/Hashrate")
                    .and_then(|v| v.as_array())
                    .and_then(|v| v.get(1).and_then(|f| f.as_f64()))
                    .unwrap_or(0.0);

                let chip_hashrates: Vec<HashRate> = hb
                    .pointer("/Chip Hashrates/Data")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|inner| inner.as_array())
                            .filter_map(|inner| inner.first().and_then(|v| v.as_f64()))
                            .map(|hr| HashRate {
                                value: hr,
                                unit: HashRateUnit::MegaHash,
                                algo: String::from("SHA256"),
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                let chip_temps: Vec<Temperature> = hb
                    .pointer("/Chip Temps/Data")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_f64())
                            .map(Temperature::from_celsius)
                            .collect()
                    })
                    .unwrap_or_default();

                let chip_voltages: Vec<Voltage> = hb
                    .pointer("/Chip Voltages/Data")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_f64())
                            .map(Voltage::from_millivolts)
                            .collect()
                    })
                    .unwrap_or_default();

                let chip_clocks: Vec<Frequency> = hb
                    .pointer("/Chip Clocks/Data")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_f64())
                            .map(Frequency::from_megahertz)
                            .collect()
                    })
                    .unwrap_or_default();

                let chip_data: Vec<ChipData> = (0..chip_hashrates.len())
                    .map(|i| ChipData {
                        position: i as u16,
                        hashrate: Some(chip_hashrates[i].clone()),
                        temperature: Some(chip_temps[i]),
                        voltage: Some(chip_voltages[i]),
                        frequency: Some(chip_clocks[i]),
                        tuned: None,
                        working: Some(true),
                    })
                    .collect();
                chips.extend(chip_data);
            }

            let board_temperature = hb
                .pointer("/Board Temps/Data")
                .and_then(|v| v.as_array())
                .and_then(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_f64())
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                })
                .map(Temperature::from_celsius);
            let intake_temperature = hb
                .pointer("/Board Temps/Data")
                .and_then(|v| v.as_array())
                .and_then(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_f64())
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                })
                .map(Temperature::from_celsius);
            let outlet_temperature = hb
                .pointer("/Board Temps/Data")
                .and_then(|v| v.as_array())
                .and_then(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_f64())
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                })
                .map(Temperature::from_celsius);
            let expected_chips = capabilities
                .pointer("/Performance Estimator/Chip Count")
                .and_then(|v| v.as_u64().map(|v| v as u16));

            if let Some(hashrate) = hashrate.clone() {
                let calculated_hr = hashrate.value / performance;
                expected_hashrate = HashRate {
                    value: calculated_hr,
                    unit: HashRateUnit::MegaHash,
                    algo: String::from("SHA256"),
                };
            }

            let bd = BoardData {
                position: hb
                    .get("Index")
                    .and_then(|v| v.as_u64().map(|v| v as u8))
                    .unwrap_or(0),
                active: hb.pointer("/HBStatus/Enabled").and_then(|v| v.as_bool()),
                hashrate,
                expected_hashrate: Some(expected_hashrate),
                intake_temperature,
                outlet_temperature,
                expected_chips,
                board_temperature,
                working_chips: Some(chips.len() as u16),
                serial_number,
                chips,
                voltage,
                frequency,
                tuned: None,
            };
            hashboards.push(bd);
        }

        // convert info to array

        hashboards
    }
}

impl GetHashrate for PowerPlay {
    fn parse_hashrate(&self, data: &HashMap<DataField, Value>) -> Option<HashRate> {
        data.extract_map::<f64, _>(DataField::Hashrate, |f| HashRate {
            value: f,
            unit: HashRateUnit::MegaHash,
            algo: String::from("SHA256"),
        })
    }
}

impl GetExpectedHashrate for PowerPlay {
    fn parse_expected_hashrate(&self, data: &HashMap<DataField, Value>) -> Option<HashRate> {
        data.extract_map::<f64, _>(DataField::ExpectedHashrate, |f| HashRate {
            value: f,
            unit: HashRateUnit::TeraHash,
            algo: String::from("SHA256"),
        })
    }
}

impl GetFans for PowerPlay {
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

impl GetPsuFans for PowerPlay {}

impl GetFluidTemperature for PowerPlay {}

impl GetWattage for PowerPlay {
    fn parse_wattage(&self, data: &HashMap<DataField, Value>) -> Option<Power> {
        data.extract_map::<f64, _>(DataField::Wattage, Power::from_watts)
    }
}

impl GetWattageLimit for PowerPlay {}

impl GetLightFlashing for PowerPlay {
    fn parse_light_flashing(&self, data: &HashMap<DataField, Value>) -> Option<bool> {
        data.extract::<bool>(DataField::LightFlashing)
    }
}

impl GetMessages for PowerPlay {}

impl GetUptime for PowerPlay {
    fn parse_uptime(&self, data: &HashMap<DataField, Value>) -> Option<Duration> {
        data.extract::<u64>(DataField::Uptime)
            .map(Duration::from_secs)
    }
}

impl GetIsMining for PowerPlay {
    fn parse_is_mining(&self, data: &HashMap<DataField, Value>) -> bool {
        data.extract::<String>(DataField::IsMining)
            .map(|state| state != "Idling")
            .unwrap_or(false)
    }
}

impl GetPools for PowerPlay {
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
impl PowerPlay {
    fn parse_pool_url(url_str: &str) -> PoolURL {
        PoolURL::from(url_str.to_string())
    }
    fn combine_by_index(data: &Value) -> Vec<Value> {
        let mut combined: HashMap<u64, serde_json::Map<String, Value>> = HashMap::new();

        let keys = [
            "Board Temps",
            "Chip Temps",
            "Chip Voltages",
            "HBStatus",
            "HBs",
            "Chip Hashrates",
            "Chip Clocks",
        ];

        for key in keys {
            if let Some(arr) = data.get(key).and_then(|v| v.as_array()) {
                for obj in arr {
                    if let Some(index) = obj.get("Index").and_then(|i| i.as_u64()) {
                        let entry = combined.entry(index).or_default();
                        entry.insert(key.to_string(), obj.clone());
                    }
                }
            }
        }

        // Convert the map into a sorted Vec<Value>
        let mut result: Vec<Value> = combined
            .into_iter()
            .map(|(index, mut map)| {
                map.insert("Index".to_string(), Value::Number(index.into()));
                Value::Object(map)
            })
            .collect();

        result.sort_by_key(|v| v.get("Index").and_then(|i| i.as_u64()).unwrap_or(0));
        result
    }
}
