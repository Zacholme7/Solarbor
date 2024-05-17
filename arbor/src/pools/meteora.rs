use super::pool::{Pool, PoolType};
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

pub async fn fetch_meteora_pools() -> Result<Vec<Pool>> {
    todo!()
}
