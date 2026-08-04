use std::sync::Arc;

use asic_rs::core::{
    data::{device::DeviceInfo, hashrate::HashRate},
    traits::miner::{Miner as MinerTrait, MinerAuth},
};
use measurements::Power;
use napi::{Error, Result, Status};
use napi_derive::napi;
use tokio::sync::RwLock;

use crate::{to_js_value, to_napi_error};

#[napi(js_name = "Miner")]
pub struct JsMiner {
    inner: Arc<RwLock<Box<dyn MinerTrait>>>,
}

impl JsMiner {
    pub(crate) fn new(inner: Box<dyn MinerTrait>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    fn with_miner<T>(&self, f: impl FnOnce(&dyn MinerTrait) -> T) -> T {
        let inner = self.inner.blocking_read();
        f(inner.as_ref())
    }
}

#[napi]
impl JsMiner {
    #[napi(getter)]
    pub fn ip(&self) -> String {
        self.with_miner(|miner| miner.get_ip().to_string())
    }

    #[napi(getter)]
    pub fn model(&self) -> String {
        self.with_miner(|miner| miner.get_device_info().model)
    }

    #[napi(getter)]
    pub fn make(&self) -> String {
        self.with_miner(|miner| miner.get_device_info().make)
    }

    #[napi(getter)]
    pub fn firmware(&self) -> String {
        self.with_miner(|miner| miner.get_device_info().firmware)
    }

    #[napi(getter, js_name = "deviceInfo")]
    pub fn device_info(&self) -> DeviceInfo {
        self.with_miner(|miner| miner.get_device_info())
    }

    #[napi(getter, js_name = "supportsSetFaultLight")]
    pub fn supports_set_fault_light(&self) -> bool {
        self.with_miner(|miner| miner.supports_set_fault_light())
    }

    #[napi(getter, js_name = "supportsSetPowerLimit")]
    pub fn supports_set_power_limit(&self) -> bool {
        self.with_miner(|miner| miner.supports_set_power_limit())
    }

    #[napi(getter, js_name = "supportsSetTuningPercent")]
    pub fn supports_set_tuning_percent(&self) -> bool {
        self.with_miner(|miner| miner.supports_set_tuning_percent())
    }

    #[napi(getter, js_name = "supportsRestart")]
    pub fn supports_restart(&self) -> bool {
        self.with_miner(|miner| miner.supports_restart())
    }

    #[napi(getter, js_name = "supportsPause")]
    pub fn supports_pause(&self) -> bool {
        self.with_miner(|miner| miner.supports_pause())
    }

    #[napi(getter, js_name = "supportsResume")]
    pub fn supports_resume(&self) -> bool {
        self.with_miner(|miner| miner.supports_resume())
    }

    #[napi(js_name = "setAuth")]
    pub async fn set_auth(&self, username: String, password: String) {
        self.inner
            .write()
            .await
            .set_auth(MinerAuth::new(username, password));
    }

    #[napi(js_name = "setToken")]
    pub async fn set_token(&self, token: String) {
        self.inner
            .write()
            .await
            .set_auth(MinerAuth::from_token(token));
    }

    #[napi(js_name = "getData")]
    pub async fn get_data(&self) -> Result<serde_json::Value> {
        let data = self.inner.read().await.get_data().await;
        to_js_value(data)
    }

    #[napi(js_name = "getHashrate")]
    pub async fn get_hashrate(&self) -> Option<HashRate> {
        self.inner.read().await.get_hashrate().await
    }

    #[napi(js_name = "getExpectedHashrate")]
    pub async fn get_expected_hashrate(&self) -> Option<HashRate> {
        self.inner.read().await.get_expected_hashrate().await
    }

    #[napi(js_name = "getFans")]
    pub async fn get_fans(&self) -> Result<serde_json::Value> {
        let data = self.inner.read().await.get_fans().await;
        to_js_value(data)
    }

    #[napi(js_name = "getPsuFans")]
    pub async fn get_psu_fans(&self) -> Result<serde_json::Value> {
        let data = self.inner.read().await.get_psu_fans().await;
        to_js_value(data)
    }

    #[napi(js_name = "getPools")]
    pub async fn get_pools(&self) -> Result<serde_json::Value> {
        let data = self.inner.read().await.get_pools().await;
        to_js_value(data)
    }

    #[napi(js_name = "getMessages")]
    pub async fn get_messages(&self) -> Result<serde_json::Value> {
        let data = self.inner.read().await.get_messages().await;
        to_js_value(data)
    }

    #[napi(js_name = "getDeviceInfo")]
    pub fn get_device_info(&self) -> DeviceInfo {
        self.with_miner(|miner| miner.get_device_info())
    }

    #[napi(js_name = "getMac")]
    pub async fn get_mac(&self) -> Option<String> {
        self.inner
            .read()
            .await
            .get_mac()
            .await
            .map(|value| value.to_string())
    }

    #[napi(js_name = "getSerialNumber")]
    pub async fn get_serial_number(&self) -> Option<String> {
        self.inner.read().await.get_serial_number().await
    }

    #[napi(js_name = "getHostname")]
    pub async fn get_hostname(&self) -> Option<String> {
        self.inner.read().await.get_hostname().await
    }

    #[napi(js_name = "getApiVersion")]
    pub async fn get_api_version(&self) -> Option<String> {
        self.inner.read().await.get_api_version().await
    }

    #[napi(js_name = "getFirmwareVersion")]
    pub async fn get_firmware_version(&self) -> Option<String> {
        self.inner.read().await.get_firmware_version().await
    }

    #[napi(js_name = "getWattage")]
    pub async fn get_wattage(&self) -> Option<f64> {
        self.inner
            .read()
            .await
            .get_wattage()
            .await
            .map(|p| p.as_watts())
    }

    #[napi(js_name = "getFluidTemperature")]
    pub async fn get_fluid_temperature(&self) -> Option<f64> {
        self.inner
            .read()
            .await
            .get_fluid_temperature()
            .await
            .map(|t| t.as_celsius())
    }

    #[napi(js_name = "getOutletFluidTemperature")]
    pub async fn get_outlet_fluid_temperature(&self) -> Option<f64> {
        self.inner
            .read()
            .await
            .get_outlet_fluid_temperature()
            .await
            .map(|t| t.as_celsius())
    }

    #[napi(js_name = "getTuningPercent")]
    pub async fn get_tuning_percent(&self) -> Option<u32> {
        self.inner
            .read()
            .await
            .get_tuning_percent()
            .await
            .map(u32::from)
    }

    #[napi(js_name = "getIsMining")]
    pub async fn get_is_mining(&self) -> bool {
        self.inner.read().await.get_is_mining().await
    }

    #[napi(js_name = "restart")]
    pub async fn restart(&self) -> Result<bool> {
        self.inner
            .read()
            .await
            .restart()
            .await
            .map_err(to_napi_error)
    }

    #[napi(js_name = "pause")]
    pub async fn pause(&self) -> Result<bool> {
        self.inner
            .read()
            .await
            .pause(None)
            .await
            .map_err(to_napi_error)
    }

    #[napi(js_name = "resume")]
    pub async fn resume(&self) -> Result<bool> {
        self.inner
            .read()
            .await
            .resume(None)
            .await
            .map_err(to_napi_error)
    }

    #[napi(js_name = "setFaultLight")]
    pub async fn set_fault_light(&self, fault: bool) -> Result<bool> {
        self.inner
            .read()
            .await
            .set_fault_light(fault)
            .await
            .map_err(to_napi_error)
    }

    #[napi(js_name = "setPowerLimit")]
    pub async fn set_power_limit(&self, watts: f64) -> Result<bool> {
        self.inner
            .read()
            .await
            .set_power_limit(Power::from_watts(watts))
            .await
            .map_err(to_napi_error)
    }

    #[napi(js_name = "setTuningPercent")]
    pub async fn set_tuning_percent(&self, percent: u32) -> Result<bool> {
        let percent = u8::try_from(percent)
            .map_err(|_| Error::new(Status::InvalidArg, "percent must fit in u8"))?;
        self.inner
            .read()
            .await
            .set_tuning_percent(percent)
            .await
            .map_err(to_napi_error)
    }
}
