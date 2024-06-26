use super::pool::{Pool, PoolType};
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct RaydiumPool {
    pub id: String,
    #[serde(rename = "baseMint")]
    pub base: String,
    #[serde(rename = "quoteMint")]
    pub quote: String,
    #[serde(rename = "baseVault")]
    pub base_vault: String,
    #[serde(rename = "quoteVault")]
    pub quote_vault: String,
    #[serde(rename = "baseDecimals")]
    pub base_decimals: usize, 
    #[serde(rename = "quoteDecimals")]
    pub quote_decimals: usize,
}

#[derive(Debug, Deserialize, Clone)]
struct RaydiumPools {
    official: Vec<RaydiumPool>,
    #[serde(rename = "unOfficial")]
    unofficial: Vec<RaydiumPool>,
}

// fetch all the pools and deserialize them into a vector of pools
pub async fn fetch_raydium_pools() -> Result<Vec<Pool>> {
    let url = "https://api.raydium.io/v2/sdk/liquidity/mainnet.json";
    let client = Client::new();
    let response = client.get(url).send().await?.json::<RaydiumPools>().await?;
    let combined_pools: Vec<RaydiumPool> = [response.official, response.unofficial].concat();

    let pools: Vec<Pool> = combined_pools
        .into_iter()
        .map(|p| Pool {
            id: p.id,
            base: p.base,
            quote: p.quote,
            base_vault: p.base_vault,
            quote_vault: p.quote_vault,
            base_decimals: p.base_decimals,
            quote_decimals: p.quote_decimals,
            base_vault_total: 0,
            quote_vault_total: 0,
            pool_type: PoolType::Raydium,

        })
        .collect();
    Ok(pools)
}

















