use crate::birdeye::fetch_trending;
use crate::graph::build_graph;
use crate::jup::get_quote;
use crate::pools::pool::Pool;
use crate::pools::raydium::fetch_all_pairs;
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::{Client, Cluster};
use anyhow::Result;
use dotenv::dotenv;
use env_logger;
use jupiter_swap_api_client::JupiterSwapApiClient;
use log::{debug, info, warn};
use std::collections::{HashMap, HashSet};
use tokio;
mod birdeye;
mod graph;
mod jup;
mod pools;
use anchor_client::solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() -> Result<()> {
    // load in configuration
    dotenv().ok();

    env_logger::init();

    // setup the cluster
    let cluster = match std::env::var("CLUSTER")?.as_str() {
        "localnet" => Cluster::Localnet,
        "mainnet" => Cluster::Mainnet,
        _ => panic!("invalid cluster type"),
    };

    let node_url = "https://api.devnet.solana.com";
    let connection = RpcClient::new_with_commitment(node_url, CommitmentConfig::confirmed());
    let send_tx_client =
        RpcClient::new_with_commitment(cluster.url(), CommitmentConfig::confirmed());

    info!("loading in raydium pools");
    let raydium_pools: Vec<Pool> = fetch_all_pairs().await?;
    info!("fetched {} pools from raydium", raydium_pools.len());

    let mut token_mints = vec![];
    /*
    let mut pools = vec![];

    let mut update_pks = vec![];
    let mut update_pks_lengths = vec![];
    let mut all_mint_idxs = vec![];

    //let mut mint_to_index = HashMap::new();
    */
    //let mut graph_edges = vec![];
    let mut mint_to_index = HashMap::new();
    let mut graph_edges: Vec<_> = vec![];

    for pool in raydium_pools {
        //let mut mint_idxs = vec![];

        let mut mint_idx = vec![];

        for mint in [pool.mint_a, pool.mint_b] {
            let idx;
            if !token_mints.contains(&mint) {
                idx = token_mints.len();
                mint_to_index.insert(mint.clone(), idx);
                token_mints.push(mint.clone());
                graph_edges.push(HashSet::new());
            } else {
                idx = *mint_to_index.get(&mint).unwrap();
            }
            mint_idx.push(idx);
        }

        //let update_accounts = pool.get_update_accounts();
        let mint0_idx = mint_idx[0];
        let mint1_idx = mint_idx[1];

        if !graph_edges[mint0_idx].contains(&mint1_idx) {
            graph_edges[mint0_idx].insert(mint1_idx);
        }
        if !graph_edges[mint1_idx].contains(&mint0_idx) {
            graph_edges[mint1_idx].insert(mint0_idx);
        }
    }

    info!("added {:?} mints", token_mints.len());

    /*
    let usdc_mint = Pubkey::new("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();
    let start_mint = usdc_mint;
    let start_mint_idx = *mint_to_index.get(&start_mint).unwrap();
    */

    Ok(())
}
