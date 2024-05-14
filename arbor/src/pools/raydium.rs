use super::pool::{Pool, PoolType};
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RaydiumPool {
    pub id: String,
    #[serde(rename = "mintA")]
    pub mint_a: String,
    #[serde(rename = "mintB")]
    pub mint_b: String,
}

#[derive(Debug, Deserialize)]
struct RaydiumPools {
    data: Vec<RaydiumPool>,
}

// fetch all the pools and deserialize them into a vector of pools
pub async fn fetch_all_pairs() -> Result<Vec<Pool>> {
    let url = "https://api.raydium.io/v2/ammV3/ammPools";
    let client = Client::new();
    let response = client.get(url).send().await?.json::<RaydiumPools>().await?;

    let pools: Vec<Pool> = response
        .data
        .into_iter()
        .map(|p| Pool {
            id: p.id,
            mint_a: p.mint_a,
            mint_b: p.mint_b,
            pool_type: PoolType::Raydium,
        })
        .collect();
    Ok(pools)
}
