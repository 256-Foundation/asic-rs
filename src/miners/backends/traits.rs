use async_trait::async_trait;
use crate::data::miner::MinerData;

#[async_trait]
pub trait GetMinerData {
    async fn get_data(&self) -> MinerData;
}
