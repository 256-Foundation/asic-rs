use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use macaddr::MacAddr;
use measurements::{AngularVelocity, Power, Temperature};
use serde_json::Value;

use super::api::AntminerAPI;
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

#[derive(Debug)]
pub struct AntminerModern {
    pub ip: IpAddr,
    pub api: AntminerAPI,
    pub device_info: DeviceInfo,
}

impl AntminerModern {
    pub fn new(ip: IpAddr, model: MinerModel, firmware: MinerFirmware) -> Self {
        AntminerModern {
            ip,
            api: AntminerAPI::new(ip, Some(4028), Some(80)),
            device_info: DeviceInfo::new(
                MinerMake::AntMiner,
                model,
                firmware,
                HashAlgorithm::SHA256,
            ),
        }
    }

    pub fn with_ports(
        ip: IpAddr,
        model: MinerModel,
        firmware: MinerFirmware,
        rpc_port: Option<u16>,
        web_port: Option<u16>,
    ) -> Self {
        AntminerModern {
            ip,
            api: AntminerAPI::new(ip, rpc_port, web_port),
            device_info: DeviceInfo::new(
                MinerMake::AntMiner,
                model,
                firmware,
                HashAlgorithm::SHA256,
            ),
        }
    }

    pub fn with_auth(
        ip: IpAddr,
        model: MinerModel,
        firmware: MinerFirmware,
        username: String,
        password: String,
    ) -> Self {
        AntminerModern {
            ip,
            api: AntminerAPI::with_auth(ip, Some(4098), Some(80), username, password),
            device_info: DeviceInfo::new(
                MinerMake::AntMiner,
                model,
                firmware,
                HashAlgorithm::SHA256,
            ),
        }
    }
}

impl GetDataLocations for AntminerModern {
    fn get_locations(&self, data_field: DataField) -> Vec<DataLocation> {
        match data_field {
            DataField::Mac => vec![(
                MinerCommand::WebAPI {
                    command: "get_system_info",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/macaddr"),
                    tag: None,
                },
            )],
            DataField::ApiVersion => vec![(
                MinerCommand::RPC {
                    command: "version",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/VERSION/0/API"),
                    tag: None,
                },
            )],
            DataField::FirmwareVersion => vec![(
                MinerCommand::RPC {
                    command: "version",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/VERSION/0/CompileTime"),
                    tag: None,
                },
            )],
            DataField::Hostname => vec![(
                MinerCommand::WebAPI {
                    command: "get_system_info",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/hostname"),
                    tag: Some("hostname"),
                },
            )],
            DataField::Hashrate => vec![(
                MinerCommand::RPC {
                    command: "summary",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/SUMMARY/0/GHS 5s"),
                    tag: Some("hashrate"),
                },
            )],
            DataField::ExpectedHashrate => vec![(
                MinerCommand::RPC {
                    command: "stats",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/1/total_rateideal"),
                    tag: Some("expected_hashrate"),
                },
            )],
            DataField::Fans => vec![(
                MinerCommand::RPC {
                    command: "stats",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/1"),
                    tag: Some("fans"),
                },
            )],
            DataField::Hashboards => vec![(
                MinerCommand::RPC {
                    command: "stats",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/0/chain"),
                    tag: None,
                },
            )],
            DataField::LightFlashing => vec![(
                MinerCommand::WebAPI {
                    command: "get_blink_status",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/blink"),
                    tag: Some("fault_light"),
                },
            )],
            DataField::IsMining => vec![(
                MinerCommand::WebAPI {
                    command: "get_miner_conf",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/bitmain-work-mode"),
                    tag: None,
                },
            )],
            DataField::Uptime => vec![(
                MinerCommand::RPC {
                    command: "stats",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/1/Elapsed"),
                    tag: None,
                },
            )],
            DataField::Pools => vec![(
                MinerCommand::RPC {
                    command: "pools",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/POOLS"),
                    tag: None,
                },
            )],
            DataField::Wattage => vec![(
                MinerCommand::RPC {
                    command: "stats",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/STATS/1"),
                    tag: None,
                },
            )],
            DataField::WattageLimit => vec![(
                MinerCommand::RPC {
                    command: "summary",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/SUMMARY/0/Power Limit"),
                    tag: None,
                },
            )],
            DataField::SerialNumber => vec![(
                MinerCommand::WebAPI {
                    command: "get_system_info",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/serial_no"),
                    tag: None,
                },
            )],
            DataField::Messages => vec![(
                MinerCommand::WebAPI {
                    command: "summary",
                    parameters: None,
                },
                DataExtractor {
                    func: get_by_pointer,
                    key: Some("/SUMMARY/0/status"),
                    tag: None,
                },
            )],
            _ => vec![],
        }
    }
}

impl GetIP for AntminerModern {
    fn get_ip(&self) -> IpAddr {
        self.ip
    }
}

impl GetDeviceInfo for AntminerModern {
    fn get_device_info(&self) -> DeviceInfo {
        self.device_info
    }
}

impl CollectData for AntminerModern {
    fn get_collector(&self) -> DataCollector<'_> {
        DataCollector::new(self, &self.api)
    }
}

impl GetMAC for AntminerModern {
    fn parse_mac(&self, data: &HashMap<DataField, Value>) -> Option<MacAddr> {
        data.extract::<String>(DataField::Mac)
            .and_then(|s| MacAddr::from_str(&s).ok())
    }
}

impl GetHostname for AntminerModern {
    fn parse_hostname(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::Hostname)
    }
}

impl GetApiVersion for AntminerModern {
    fn parse_api_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::ApiVersion)
    }
}

impl GetFirmwareVersion for AntminerModern {
    fn parse_firmware_version(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::FirmwareVersion)
    }
}

impl GetHashboards for AntminerModern {
    fn parse_hashboards(&self, data: &HashMap<DataField, Value>) -> Vec<BoardData> {
        let mut hashboards: Vec<BoardData> = Vec::new();
        let board_count = self.device_info.hardware.boards.unwrap_or(3);

        if let Some(chain_data) = data.get(&DataField::Hashboards) {
            if let Some(chains) = chain_data.as_array() {
                for (idx, chain) in chains.iter().enumerate() {
                    let hashrate = chain.get("rate_real").and_then(|v| v.as_f64()).map(|f| {
                        HashRate {
                            value: f,
                            unit: HashRateUnit::GigaHash,
                            algo: String::from("SHA256"),
                        }
                        .as_unit(HashRateUnit::TeraHash)
                    });

                    let working_chips = chain
                        .get("asic_num")
                        .and_then(|v| v.as_u64())
                        .map(|u| u as u16);

                    let serial_number = chain.get("sn").and_then(|v| v.as_str()).map(String::from);

                    // Temperature handling - check for S21+ Hyd vs regular models
                    let (board_temp, _chip_temp, inlet_temp, outlet_temp) =
                        if self.device_info.model.to_string().contains("S21+ Hyd") {
                            let inlet = chain
                                .get("temp_pcb")
                                .and_then(|v| v.as_array())
                                .and_then(|arr| arr.get(0))
                                .and_then(|v| v.as_f64())
                                .map(Temperature::from_celsius);

                            let outlet = chain
                                .get("temp_pcb")
                                .and_then(|v| v.as_array())
                                .and_then(|arr| arr.get(2))
                                .and_then(|v| v.as_f64())
                                .map(Temperature::from_celsius);

                            let chip = chain
                                .get("temp_pic")
                                .and_then(|v| v.as_array())
                                .and_then(|arr| arr.get(0))
                                .and_then(|v| v.as_f64())
                                .map(Temperature::from_celsius);

                            // Calculate average temp from various sensors
                            let board = Self::calculate_average_temp_s21_hyd(chain);

                            (board, chip, inlet, outlet)
                        } else {
                            let board = Self::calculate_average_temp_pcb(chain);
                            let chip = Self::calculate_average_temp_chip(chain);
                            (board, chip, None, None)
                        };

                    let position = chain
                        .get("index")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(idx as u64) as u16;

                    let active = Some(hashrate.as_ref().map(|h| h.value > 0.0).unwrap_or(false));

                    hashboards.push(BoardData {
                        hashrate,
                        position: position as u8,
                        expected_hashrate: None, // TODO
                        board_temperature: board_temp,
                        intake_temperature: inlet_temp,
                        outlet_temperature: outlet_temp,
                        expected_chips: self.device_info.hardware.chips,
                        working_chips,
                        serial_number,
                        chips: vec![],
                        voltage: None,
                        frequency: None, // TODO
                        tuned: Some(true),
                        active,
                    });
                }
                return hashboards;
            }
        }

        // Fallback to default empty hashboards if no chain data
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

        hashboards
    }
}

impl AntminerModern {
    fn calculate_average_temp_s21_hyd(chain: &Value) -> Option<Temperature> {
        let mut temps = Vec::new();

        if let Some(temp_pic) = chain.get("temp_pic").and_then(|v| v.as_array()) {
            for i in 1..=3 {
                if let Some(temp) = temp_pic.get(i).and_then(|v| v.as_f64()) {
                    if temp != 0.0 {
                        temps.push(temp);
                    }
                }
            }
        }

        if let Some(temp_pcb) = chain.get("temp_pcb").and_then(|v| v.as_array()) {
            if let Some(temp) = temp_pcb.get(1).and_then(|v| v.as_f64()) {
                if temp != 0.0 {
                    temps.push(temp);
                }
            }
            if let Some(temp) = temp_pcb.get(3).and_then(|v| v.as_f64()) {
                if temp != 0.0 {
                    temps.push(temp);
                }
            }
        }

        if !temps.is_empty() {
            let avg = temps.iter().sum::<f64>() / temps.len() as f64;
            Some(Temperature::from_celsius(avg))
        } else {
            None
        }
    }

    fn calculate_average_temp_pcb(chain: &Value) -> Option<Temperature> {
        if let Some(temp_pcb) = chain.get("temp_pcb").and_then(|v| v.as_array()) {
            let temps: Vec<f64> = temp_pcb
                .iter()
                .filter_map(|v| v.as_f64())
                .filter(|&temp| temp != 0.0)
                .collect();

            if !temps.is_empty() {
                let avg = temps.iter().sum::<f64>() / temps.len() as f64;
                Some(Temperature::from_celsius(avg))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn calculate_average_temp_chip(chain: &Value) -> Option<Temperature> {
        if let Some(temp_chip) = chain.get("temp_chip").and_then(|v| v.as_array()) {
            let temps: Vec<f64> = temp_chip
                .iter()
                .filter_map(|v| v.as_f64())
                .filter(|&temp| temp != 0.0)
                .collect();

            if !temps.is_empty() {
                let avg = temps.iter().sum::<f64>() / temps.len() as f64;
                Some(Temperature::from_celsius(avg))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl GetHashrate for AntminerModern {
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

impl GetExpectedHashrate for AntminerModern {
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

impl GetFans for AntminerModern {
    fn parse_fans(&self, data: &HashMap<DataField, Value>) -> Vec<FanData> {
        let mut fans: Vec<FanData> = Vec::new();

        // Extract fan data from stats
        if let Some(stats_data) = data.get(&DataField::Fans) {
            // Look for fan speed fields in stats
            for i in 1..=self.device_info.hardware.fans.unwrap_or(1) {
                if let Some(fan_speed) = stats_data
                    .get(&format!("fan{}", i))
                    .or_else(|| stats_data.get(&format!("Fan{}", i)))
                    .and_then(|v| v.as_f64())
                {
                    if fan_speed > 0.0 {
                        fans.push(FanData {
                            position: (i - 1) as i16,
                            rpm: Some(AngularVelocity::from_rpm(fan_speed)),
                        });
                    }
                }
            }
        }

        fans
    }
}

impl GetLightFlashing for AntminerModern {
    fn parse_light_flashing(&self, data: &HashMap<DataField, Value>) -> Option<bool> {
        data.extract::<bool>(DataField::LightFlashing).or_else(|| {
            // Handle string response from get_blink_status
            data.extract::<String>(DataField::LightFlashing)
                .map(|s| s.to_lowercase() == "true" || s == "1")
        })
    }
}

impl GetUptime for AntminerModern {
    fn parse_uptime(&self, data: &HashMap<DataField, Value>) -> Option<Duration> {
        data.extract_map::<u64, _>(DataField::Uptime, Duration::from_secs)
    }
}

impl GetIsMining for AntminerModern {
    fn parse_is_mining(&self, data: &HashMap<DataField, Value>) -> bool {
        data.extract::<String>(DataField::IsMining)
            .map(|status| {
                let status_lower = status.to_lowercase();
                status_lower != "stopped" && status_lower != "idle" && status_lower != "sleep"
            })
            .or_else(|| {
                // Fallback: check if we have active hashrate
                data.extract::<f64>(DataField::Hashrate).map(|hr| hr > 0.0)
            })
            .unwrap_or(false)
    }
}

impl GetPools for AntminerModern {
    fn parse_pools(&self, data: &HashMap<DataField, Value>) -> Vec<PoolData> {
        let mut pools: Vec<PoolData> = Vec::new();

        if let Some(pools_data) = data.get(&DataField::Pools) {
            if let Some(pools_array) = pools_data.as_array() {
                for (idx, pool_info) in pools_array.iter().enumerate() {
                    let url = pool_info
                        .get("URL")
                        .and_then(|v| v.as_str())
                        .map(|s| PoolURL::from(s.to_string()));

                    let user = pool_info
                        .get("User")
                        .and_then(|v| v.as_str())
                        .map(String::from);

                    let alive = pool_info
                        .get("Status")
                        .and_then(|v| v.as_str())
                        .map(|s| s == "Alive");

                    let active = pool_info.get("Stratum Active").and_then(|v| v.as_bool());

                    let accepted_shares = pool_info.get("Accepted").and_then(|v| v.as_u64());

                    let rejected_shares = pool_info.get("Rejected").and_then(|v| v.as_u64());

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

impl GetSerialNumber for AntminerModern {
    fn parse_serial_number(&self, data: &HashMap<DataField, Value>) -> Option<String> {
        data.extract::<String>(DataField::SerialNumber)
    }
}

impl GetControlBoardVersion for AntminerModern {}

impl GetWattage for AntminerModern {
    fn parse_wattage(&self, data: &HashMap<DataField, Value>) -> Option<Power> {
        if let Some(stats_data) = data.get(&DataField::Wattage) {
            // Look for chain_power field (HiveOS style: "3250 W")
            if let Some(chain_power) = stats_data.get("chain_power") {
                if let Some(power_str) = chain_power.as_str() {
                    // Parse "3250 W" format
                    if let Some(watt_part) = power_str.split_whitespace().next() {
                        if let Ok(watts) = watt_part.parse::<f64>() {
                            return Some(Power::from_watts(watts));
                        }
                    }
                }
            }

            // Look for other power fields
            if let Some(power) = stats_data
                .get("power")
                .or_else(|| stats_data.get("Power"))
                .and_then(|v| v.as_f64())
            {
                return Some(Power::from_watts(power));
            }
        }
        None
    }
}

impl GetWattageLimit for AntminerModern {
    fn parse_wattage_limit(&self, data: &HashMap<DataField, Value>) -> Option<Power> {
        data.extract_map::<f64, _>(DataField::WattageLimit, |f| Power::from_watts(f))
    }
}

impl GetFluidTemperature for AntminerModern {
    fn parse_fluid_temperature(&self, data: &HashMap<DataField, Value>) -> Option<Temperature> {
        // For S21+ Hyd models, use inlet/outlet temperature average
        if self.device_info.model.to_string().contains("S21+ Hyd") {
            if let Some(hashboards_data) = data.get(&DataField::Hashboards) {
                if let Some(chains) = hashboards_data.as_array() {
                    let mut temps = Vec::new();

                    for chain in chains {
                        if let Some(temp_pcb) = chain.get("temp_pcb").and_then(|v| v.as_array()) {
                            // Inlet temp (index 0) and outlet temp (index 2)
                            if let Some(inlet) = temp_pcb.get(0).and_then(|v| v.as_f64()) {
                                if inlet != 0.0 {
                                    temps.push(inlet);
                                }
                            }
                            if let Some(outlet) = temp_pcb.get(2).and_then(|v| v.as_f64()) {
                                if outlet != 0.0 {
                                    temps.push(outlet);
                                }
                            }
                        }
                    }

                    if !temps.is_empty() {
                        let avg = temps.iter().sum::<f64>() / temps.len() as f64;
                        return Some(Temperature::from_celsius(avg));
                    }
                }
            }
        }
        None
    }
}

impl GetPsuFans for AntminerModern {}

impl GetMessages for AntminerModern {
    fn parse_messages(&self, data: &HashMap<DataField, Value>) -> Vec<MinerMessage> {
        let mut messages = Vec::new();

        if let Some(status_data) = data.get(&DataField::Messages) {
            if let Some(status_array) = status_data.as_array() {
                for (idx, item) in status_array.iter().enumerate() {
                    if let Some(status) = item.get("status").and_then(|v| v.as_str()) {
                        if status != "s" {
                            // 's' means success/ok
                            let message_text = item
                                .get("msg")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string();

                            let severity = match status {
                                "E" | "e" => MessageSeverity::Error,
                                "W" | "w" => MessageSeverity::Warning,
                                _ => MessageSeverity::Info,
                            };

                            messages.push(MinerMessage::new(0, idx as u64, message_text, severity));
                        }
                    }
                }
            }
        }

        messages
    }
}
