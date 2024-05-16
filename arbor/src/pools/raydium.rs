use super::pool::{Pool, PoolType};
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

/*
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
*/
#[derive(Debug, Deserialize, Clone)]
struct RaydiumPool {
    pub id: String,
    #[serde(rename = "baseMint")]
    pub base: String,
    #[serde(rename = "quoteMint")]
    pub quote: String,
    /*
    #[serde(rename = "baseVault")]
    pub base_vault: String,
    #[serde(rename = "quoteVault")]
    pub quote_vault: String,
    */
}

#[derive(Debug, Deserialize, Clone)]
struct RaydiumPools {
    official: Vec<RaydiumPool>,
    #[serde(rename = "unOfficial")]
    unofficial: Vec<RaydiumPool>,
}

// fetch all the pools and deserialize them into a vector of pools
pub async fn fetch_all_pairs() -> Result<Vec<Pool>> {
    //let url = "https://api.raydium.io/v2/ammV3/ammPools";
    let url = "https://api.raydium.io/v2/sdk/liquidity/mainnet.json";
    let client = Client::new();
    let response = client.get(url).send().await?.json::<RaydiumPools>().await?;
    println!("response len {}", response.official.len() + response.unofficial.len());

    let combined_pools: Vec<RaydiumPool> = [response.official, response.unofficial].concat();

    let pools: Vec<Pool> = combined_pools
        .into_iter()
        .map(|p| Pool {
            id: p.id,
            mint_a: p.base,
            mint_b: p.quote,
            pool_type: PoolType::Raydium,
        })
        .collect();
    Ok(pools)
}

















