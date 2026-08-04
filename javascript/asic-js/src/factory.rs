use std::{net::IpAddr, str::FromStr, sync::Arc};

use asic_rs::MinerFactory as MinerFactoryBase;
use napi::Result;
use napi_derive::napi;

use crate::{miner::JsMiner, to_napi_error};

#[napi(js_name = "MinerFactory")]
pub struct JsMinerFactory {
    inner: Arc<MinerFactoryBase>,
}

#[napi]
impl JsMinerFactory {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(MinerFactoryBase::new()),
        }
    }

    #[napi(factory, js_name = "fromSubnet")]
    pub fn from_subnet(subnet: String) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(MinerFactoryBase::from_subnet(&subnet).map_err(to_napi_error)?),
        })
    }

    #[napi(factory, js_name = "fromRange")]
    pub fn from_range(range: String) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(MinerFactoryBase::from_range(&range).map_err(to_napi_error)?),
        })
    }

    #[napi(factory, js_name = "fromOctets")]
    pub fn from_octets(
        octet1: String,
        octet2: String,
        octet3: String,
        octet4: String,
    ) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(
                MinerFactoryBase::from_octets(&octet1, &octet2, &octet3, &octet4)
                    .map_err(to_napi_error)?,
            ),
        })
    }

    #[napi(js_name = "withSubnet")]
    pub fn with_subnet(&mut self, subnet: String) -> Result<&Self> {
        let inner = Arc::<MinerFactoryBase>::make_mut(&mut self.inner).clone();
        self.inner = Arc::new(inner.with_subnet(&subnet).map_err(to_napi_error)?);
        Ok(self)
    }

    #[napi(js_name = "withRange")]
    pub fn with_range(&mut self, range: String) -> Result<&Self> {
        let inner = Arc::<MinerFactoryBase>::make_mut(&mut self.inner).clone();
        self.inner = Arc::new(inner.with_range(&range).map_err(to_napi_error)?);
        Ok(self)
    }

    #[napi(js_name = "withOctets")]
    pub fn with_octets(
        &mut self,
        octet1: String,
        octet2: String,
        octet3: String,
        octet4: String,
    ) -> Result<&Self> {
        let inner = Arc::<MinerFactoryBase>::make_mut(&mut self.inner).clone();
        self.inner = Arc::new(
            inner
                .with_octets(&octet1, &octet2, &octet3, &octet4)
                .map_err(to_napi_error)?,
        );
        Ok(self)
    }

    #[napi(js_name = "withConcurrentLimit")]
    pub fn with_concurrent_limit(&mut self, concurrent: u32) -> &Self {
        let inner = Arc::<MinerFactoryBase>::make_mut(&mut self.inner).clone();
        self.inner = Arc::new(inner.with_concurrent_limit(concurrent as usize));
        self
    }

    #[napi(js_name = "getMiner")]
    pub async fn get_miner(&self, ip: String) -> Result<Option<JsMiner>> {
        let ip = IpAddr::from_str(&ip).map_err(to_napi_error)?;
        let miner = self
            .inner
            .get_miner(ip)
            .await
            .map_err(to_napi_error)?
            .map(JsMiner::new);
        Ok(miner)
    }

    #[napi]
    pub async fn scan(&self) -> Result<Vec<JsMiner>> {
        Ok(self
            .inner
            .scan()
            .await
            .map_err(to_napi_error)?
            .into_iter()
            .map(JsMiner::new)
            .collect())
    }
}
