use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use macaddr::MacAddr;
use measurements::{AngularVelocity, Frequency, Power, Temperature};
use serde_json::{Value, json};

use crate::data::board::BoardData;
use crate::data::device::MinerMake;
use crate::data::device::{DeviceInfo, HashAlgorithm, MinerFirmware, MinerModel};
use crate::data::fan::FanData;
use crate::data::hashrate::{HashRate, HashRateUnit};
use crate::data::pool::{PoolData, PoolURL};
use crate::miners::backends::traits::*;
use crate::miners::commands::MinerCommand;
use crate::miners::data::{
    DataCollector, DataExtensions, DataExtractor, DataField, DataLocation, get_by_key,
    get_by_pointer,
};

pub use rpc::WhatsMinerRPCAPI;

mod rpc;

#[derive(Debug)]
pub struct WhatsMinerV3 {
    pub ip: IpAddr,
    pub rpc: WhatsMinerRPCAPI,
    pub device_info: DeviceInfo,
}

impl WhatsMinerV3 {
    pub fn new(ip: IpAddr, model: MinerModel) -> Self {
        WhatsMinerV3 {
            ip,
            rpc: WhatsMinerRPCAPI::new(ip, None),
            device_info: DeviceInfo::new(
                MinerMake::WhatsMiner,
                model,
                MinerFirmware::Stock,
                HashAlgorithm::SHA256,
            ),
        }
    }
}

impl GetDataLocations for WhatsMinerV3 {
    fn get_locations(&self, data_field: DataField) -> Vec<DataLocation> {
        let get_device_info_cmd: MinerCommand = MinerCommand::RPC {
            command: "get.device.info",
            parameters: None,
        };
        let get_miner_status_summary_cmd: MinerCommand = MinerCommand::RPC {
            command: "get.miner.status",
            parameters: Some(json!("summary")),
        };
        let get_miner_status_pools_cmd: MinerCommand = MinerCommand::RPC {
            command: "get.miner.status",
            parameters: Some(json!("pools")),
        };
        let get_miner_status_edevs_cmd: MinerCommand = MinerCommand::RPC {
            command: "get.miner.status",
            parameters: Some(json!("edevs")),
        };

        match data_field {
            DataField::Mac => vec![(
                get_device_info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/network/mac"),
                    tag: None,
                },
            )],
            DataField::ApiVersion => vec![(
                get_device_info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/system/api"),
                    tag: None,
                },
            )],
            DataField::FirmwareVersion => vec![(
                get_device_info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/system/fwversion"),
                    tag: None,
                },
            )],
            DataField::ControlBoardVersion => vec![(
                get_device_info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/system/platform"),
                    tag: None,
                },
            )],
            DataField::SerialNumber => vec![(
                get_device_info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/miner/miner-sn"),
                    tag: None,
                },
            )],
            DataField::Hostname => vec![(
                get_device_info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/network/hostname"),
                    tag: None,
                },
            )],
            DataField::LightFlashing => vec![(
                get_device_info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/system/ledstatus"),
                    tag: None,
                },
            )],
            DataField::WattageLimit => vec![(
                get_device_info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/miner/power-limit-set"),
                    tag: None,
                },
            )],
            DataField::Fans => vec![(
                get_miner_status_summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/summary"),
                    tag: None,
                },
            )],
            DataField::PsuFans => vec![(
                get_device_info_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/power/fanspeed"),
                    tag: None,
                },
            )],
            DataField::Hashboards => vec![
                (
                    get_device_info_cmd,
                    DataExtractor {
                        func: get_by_pointer,
                        key: Some("/msg/miner"),
                        tag: None,
                    },
                ),
                (
                    get_miner_status_edevs_cmd,
                    DataExtractor {
                        func: get_by_key,
                        key: Some("msg"),
                        tag: None,
                    },
                ),
            ],
            DataField::Pools => vec![(
                get_miner_status_pools_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/pools"),
                    tag: None,
                },
            )],
            DataField::Uptime => vec![(
                get_miner_status_summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/summary/elapsed"),
                    tag: None,
                },
            )],
            DataField::Wattage => vec![(
                get_miner_status_summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/summary/power-realtime"),
                    tag: None,
                },
            )],
            DataField::Hashrate => vec![(
                get_miner_status_summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/summary/hash-realtime"),
                    tag: None,
                },
            )],
            DataField::ExpectedHashrate => vec![(
                get_miner_status_summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/summary/factory-hash"),
                    tag: None,
                },
            )],
            DataField::FluidTemperature => vec![(
                get_miner_status_summary_cmd,
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/msg/summary/environment-temperature"),
                    tag: None,
                },
            )],
            _ => vec![],
        }
    }
}

impl GetIP for WhatsMinerV3 {
    fn get_ip(&self) -> IpAddr {
        self.ip
    }
}
impl GetDeviceInfo for WhatsMinerV3 {
    fn get_device_info(&self) -> DeviceInfo {
        self.device_info
    }
}

impl CollectData for WhatsMinerV3 {
    fn get_collector(&self) -> DataCollector<'_> {
        DataCollector::new(self, &self.rpc)
    }
}

impl GetMAC for WhatsMinerV3 {
    fn parse_mac(&self, data: &HashMap<DataField, Value>) -> Option<MacAddr> {
        data.extract::<String>(DataField::Mac)
            .and_then(|s| MacAddr::from_str(&s).ok())
    }
}

impl GetSerialNumber for WhatsMinerV3 {}
impl GetHostname for WhatsMinerV3 {
    fn parse_hostname(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::Hostname)
    }
}
impl GetApiVersion for WhatsMinerV3 {
    fn parse_api_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::ApiVersion)
    }
}
impl GetFirmwareVersion for WhatsMinerV3 {
    fn parse_firmware_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::FirmwareVersion)
    }
}
impl GetControlBoardVersion for WhatsMinerV3 {
    fn parse_control_board_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::ControlBoardVersion)
    }
}
impl GetHashboards for WhatsMinerV3 {
    fn parse_hashboards(&self, data: &HashMap<DataField, Value>) -> Vec<BoardData> {
        let mut hashboards: Vec<BoardData> = Vec::new();
        let board_count = self.device_info.hardware.boards.unwrap_or(3);
        for idx in 0..board_count {
            let hashrate = data
                .get(&DataField::Hashboards)
                .and_then(|val| val.pointer(&format!("/edevs/{idx}/hash-average")))
                .and_then(|val| val.as_f64())
                .map(|f| HashRate {
                    value: f,
                    unit: HashRateUnit::TeraHash,
                    algo: String::from("SHA256"),
                });
            let expected_hashrate = data
                .get(&DataField::Hashboards)
                .and_then(|val| val.pointer(&format!("/edevs/{idx}/factory-hash")))
                .and_then(|val| val.as_f64())
                .map(|f| HashRate {
                    value: f,
                    unit: HashRateUnit::TeraHash,
                    algo: String::from("SHA256"),
                });
            let board_temperature = data
                .get(&DataField::Hashboards)
                .and_then(|val| val.pointer(&format!("/edevs/{idx}/chip-temp-min")))
                .and_then(|val| val.as_f64())
                .map(Temperature::from_celsius);
            let intake_temperature = data
                .get(&DataField::Hashboards)
                .and_then(|val| val.pointer(&format!("/edevs/{idx}/chip-temp-min")))
                .and_then(|val| val.as_f64())
                .map(Temperature::from_celsius);
            let outlet_temperature = data
                .get(&DataField::Hashboards)
                .and_then(|val| val.pointer(&format!("/edevs/{idx}/chip-temp-max")))
                .and_then(|val| val.as_f64())
                .map(Temperature::from_celsius);
            let serial_number =
                data.extract_nested::<String>(DataField::Hashboards, &format!("pcbsn{idx}"));

            let working_chips = data
                .get(&DataField::Hashboards)
                .and_then(|val| val.pointer(&format!("/edevs/{idx}/effective-chips")))
                .and_then(|val| val.as_u64())
                .map(|u| u as u16);
            let frequency = data
                .get(&DataField::Hashboards)
                .and_then(|val| val.pointer(&format!("/edevs/{idx}/freq")))
                .and_then(|val| val.as_f64())
                .map(Frequency::from_megahertz);

            let active = Some(hashrate.clone().map(|h| h.value).unwrap_or(0f64) > 0f64);
            hashboards.push(BoardData {
                hashrate,
                position: idx,
                expected_hashrate,
                board_temperature,
                intake_temperature,
                outlet_temperature,
                expected_chips: self.device_info.hardware.chips,
                working_chips,
                serial_number,
                chips: vec![],
                voltage: None, // TODO
                frequency,
                tuned: Some(true),
                active,
            });
        }
        hashboards
    }
}
impl GetHashrate for WhatsMinerV3 {
    fn parse_hashrate(&self, data: &HashMap<DataField, Value>) -> Option<HashRate> {
        data.extract_map::<f64, _>(DataField::Hashrate, |f| HashRate {
            value: f,
            unit: HashRateUnit::TeraHash,
            algo: String::from("SHA256"),
        })
    }
}
impl GetExpectedHashrate for WhatsMinerV3 {
    fn parse_expected_hashrate(&self, data: &HashMap<DataField, Value>) -> Option<HashRate> {
        data.extract_map::<f64, _>(DataField::ExpectedHashrate, |f| HashRate {
            value: f,
            unit: HashRateUnit::TeraHash,
            algo: String::from("SHA256"),
        })
    }
}
impl GetFans for WhatsMinerV3 {
    fn parse_fans(&self, data: &HashMap<DataField, Value>) -> Vec<FanData> {
        let mut fans: Vec<FanData> = Vec::new();
        for (idx, direction) in ["in", "out"].iter().enumerate() {
            let fan = data.extract_nested_map::<f64, _>(
                DataField::Fans,
                &format!("fan-speed-{direction}"),
                |rpm| FanData {
                    position: idx as i16,
                    rpm: Some(AngularVelocity::from_rpm(rpm)),
                },
            );
            if fan.is_some() {
                fans.push(fan.unwrap());
            }
        }
        fans
    }
}
impl GetPsuFans for WhatsMinerV3 {
    fn parse_psu_fans(&self, data: &HashMap<DataField, Value>) -> Vec<FanData> {
        let mut psu_fans: Vec<FanData> = Vec::new();

        let psu_fan = data.extract_map::<f64, _>(DataField::PsuFans, |rpm| FanData {
            position: 0i16,
            rpm: Some(AngularVelocity::from_rpm(rpm)),
        });
        if psu_fan.is_some() {
            psu_fans.push(psu_fan.unwrap());
        }
        psu_fans
    }
}
impl GetFluidTemperature for WhatsMinerV3 {
    fn parse_fluid_temperature(&self, data: &HashMap<DataField, Value>) -> Option<Temperature> {
        data.extract_map::<f64, _>(DataField::FluidTemperature, Temperature::from_celsius)
    }
}
impl GetWattage for WhatsMinerV3 {
    fn parse_wattage(&self, data: &HashMap<DataField, Value>) -> Option<Power> {
        data.extract_map::<f64, _>(DataField::Wattage, Power::from_watts)
    }
}
impl GetWattageLimit for WhatsMinerV3 {
    fn parse_wattage_limit(&self, data: &HashMap<DataField, Value>) -> Option<Power> {
        data.extract_map::<String, _>(DataField::WattageLimit, |p| p.parse::<f64>().ok())?
            .map(Power::from_watts)
    }
}
impl GetLightFlashing for WhatsMinerV3 {
    fn parse_light_flashing(&self, data: &HashMap<DataField, Value>) -> Option<bool> {
        data.extract_map::<String, _>(DataField::LightFlashing, |l| l != "auto")
    }
}
impl GetMessages for WhatsMinerV3 {}
impl GetUptime for WhatsMinerV3 {
    fn parse_uptime(&self, data: &HashMap<DataField, Value>) -> Option<Duration> {
        data.extract_map::<u64, _>(DataField::Uptime, Duration::from_secs)
    }
}
impl GetIsMining for WhatsMinerV3 {}
impl GetPools for WhatsMinerV3 {
    fn parse_pools(&self, data: &HashMap<DataField, Value>) -> Vec<PoolData> {
        let mut pools: Vec<PoolData> = Vec::new();
        let pools_raw = data.get(&DataField::Pools);
        if pools_raw.is_some() {
            let pools_response = pools_raw.unwrap();
            for (idx, _) in pools_response
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .enumerate()
            {
                let user = data
                    .get(&DataField::Pools)
                    .and_then(|val| val.pointer(&format!("/{idx}/account")))
                    .map(|val| String::from(val.as_str().unwrap_or("")));

                let alive = data
                    .get(&DataField::Pools)
                    .and_then(|val| val.pointer(&format!("/{idx}/status")))
                    .map(|val| val.as_str())
                    .map(|val| val == Some("alive"));

                let active = data
                    .get(&DataField::Pools)
                    .and_then(|val| val.pointer(&format!("/{idx}/stratum-active")))
                    .and_then(|val| val.as_bool());

                let url = data
                    .get(&DataField::Pools)
                    .and_then(|val| val.pointer(&format!("/{idx}/url")))
                    .map(|val| PoolURL::from(String::from(val.as_str().unwrap_or(""))));

                pools.push(PoolData {
                    position: Some(idx as u16),
                    url,
                    accepted_shares: None,
                    rejected_shares: None,
                    active,
                    alive,
                    user,
                });
            }
        }
        pools
    }
}
