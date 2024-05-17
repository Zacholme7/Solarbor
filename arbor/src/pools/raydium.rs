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
    #[serde(rename = "marketBaseVault")]
    pub market_base_vault: String,
    #[serde(rename = "marketQuoteVault")]
    pub market_quote_vault: String
}

#[derive(Debug, Deserialize, Clone)]
struct RaydiumPools {
    official: Vec<RaydiumPool>,
    #[serde(rename = "unOfficial")]
    unofficial: Vec<RaydiumPool>,
}

// fetch all the pools and deserialize them into a vector of pools
pub async fn fetch_all_pairs() -> Result<Vec<Pool>> {
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
            pool_type: PoolType::Raydium,
            market_base_vault: p.market_base_vault,
            market_quote_vault: p.market_quote_vault
        })
        .collect();
    Ok(pools)
}

















