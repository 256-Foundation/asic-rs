use crate::data::device::{MinerFirmware, MinerMake, MinerModel};
use crate::miners::util;
use diqwest::WithDigestAuth;
use reqwest::{Client, Response};
use std::net::IpAddr;

pub(crate) async fn get_model_antminer(ip: IpAddr) -> Option<MinerModel> {
    let response: Option<Response> = Client::new()
        .get(format!("http://{}/cgi-bin/get_system_info.cgi", ip))
        .send_with_digest_auth("root", "root")
        .await
        .ok();
    match response {
        Some(data) => {
            let json_data = data.json::<serde_json::Value>().await.ok()?;
            MinerModel::from_string(
                Some(MinerMake::AntMiner),
                None,
                &json_data["minertype"].as_str().unwrap_or("").to_uppercase(),
            )
        }
        None => None,
    }
}
pub(crate) async fn get_model_whatsminer(ip: IpAddr) -> Option<MinerModel> {
    let response = util::send_rpc_command(&ip, "devdetails").await;
    match response {
        Some(json_data) => {
            let model = json_data["DEVDETAILS"][0]["Model"].as_str();
            if model.is_none() {
                return None;
            }
            let mut model = model.unwrap().to_uppercase().replace("_", "");
            model.pop();
            model.push('0');

            MinerModel::from_string(Some(MinerMake::WhatsMiner), None, &model)
        }
        None => None,
    }
}
pub(crate) async fn get_model_luxos(ip: IpAddr) -> Option<MinerModel> {
    let response = util::send_rpc_command(&ip, "version").await;
    match response {
        Some(json_data) => {
            let model = json_data["VERSION"][0]["Type"].as_str();
            if model.is_none() {
                return None;
            }
            let model = model.unwrap().to_uppercase();

            MinerModel::from_string(None, Some(MinerFirmware::LuxOS), &model)
        }
        None => None,
    }
}
