use crate::birdeye::fetch_trending;
use crate::jup::get_quote;
use crate::graph::build_graph;
use crate::pools::raydium::fetch_all_pairs;
use anyhow::Result;
use dotenv::dotenv;
use jupiter_swap_api_client::JupiterSwapApiClient;
use tokio;
use anchor_client::{Client, Cluster};
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
mod birdeye;
mod graph;
mod jup;
mod pools;

#[tokio::main]
async fn main() -> Result<()> {
    // load in configuration
    dotenv().ok();

    // setup the cluster
    let cluster = match std::env::var("CLUSTER")?.as_str() {
        "localnet" => Cluster::Localnet,
        "mainnet" => Cluster::Mainnet,
        _ => panic!("invalid cluster type")
    };

    let node_url = "https://api.devnet.solana.com";
    let connection = RpcClient::new_with_commitment(node_url, CommitmentConfig::confirmed());
    let send_tx_client = RpcClient::new_with_commitment(cluster.url(), CommitmentConfig::confirmed());



    






    //let _ = fetch_all_pairs().await?;

    //  find the latests 24hour trending on birdeye
    /*
    let trending = fetch_trending().await?;

    let client =
        JupiterSwapApiClient::new("https://quote-api.jup.ag/v6".to_string());
    let arb_graph = build_graph(&client, &trending, 1).await?;
    println!("{:?}", arb_graph);
    */
    /*
    let sol_to_boden = get_quote(
        &jupiter_swap_api_client,
        trending["USDT"].clone(),
        trending["jeo boden"].clone(),
        100_000,
    )
    .await?;
    let boden_to_sol = get_quote(
        &jupiter_swap_api_client,
        trending["jeo boden"].clone(),
        trending["USDT"].clone(),
        sol_to_boden as u64,
    )
    .await?;
    println!("{}, {}", sol_to_boden, boden_to_sol);
    */







    Ok(())
}
