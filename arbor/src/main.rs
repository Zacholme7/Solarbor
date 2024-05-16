use anchor_client::solana_sdk::pubkey::Pubkey;
use crate::pools::pool::Pool;
use crate::pools::raydium::fetch_all_pairs;
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::{Client, Cluster};
use anyhow::Result;
use dotenv::dotenv;
use env_logger;
use log::{debug, info, warn};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::util::unpack_token_account;
use crate::graph::Graph;

mod birdeye;
mod graph;
mod jup;
mod pools;
mod util;

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

    // setup the rpc clients
    let node_url = std::env::var("NODE_URL")?;
    let connection = RpcClient::new_with_commitment(node_url, CommitmentConfig::confirmed());
    let send_tx_client = RpcClient::new_with_commitment(cluster.url(), CommitmentConfig::confirmed());

    // state 
    let mut pools: Vec<Pool> = vec![]; // track every pool available to swap
    let mut token_mints: Vec<String> = vec![]; // track every unique token mint
    let mut mint_to_index = HashMap::new(); // mint pubkey -> index in token_mints
    let mut graph_edges: Vec<HashSet<usize>> = vec![];  // graph representation, index in token_mint to all edges (possible swaps)
    let mut graph = Graph::new(); // construct a new graph to represent entire network

    // load in all of the pools that we want to arb
    info!("Loading in all pools");
    pools.extend(fetch_all_pairs().await?);

    // process all of the pools
    info!("Processing Pools and setting up graphs");
    for pool in &pools {
        let mut local_mint_idx = vec![]; // track the indicies of the current pool mints

        // process the mints for this pool
        for mint in [&pool.mint_a, &pool.mint_b] {
                let idx = match mint_to_index.get(mint) {
                        Some(&idx) => idx,
                        None => {
                                let idx = token_mints.len();
                                mint_to_index.insert(mint.clone(), idx);
                                token_mints.push(mint.clone());
                                graph_edges.push(HashSet::new());
                                idx
                        }
                };
                local_mint_idx.push(idx);
        }
        // mint --> [mint, ....]
        graph_edges[local_mint_idx[0]].insert(local_mint_idx[1]);
        graph_edges[local_mint_idx[1]].insert(local_mint_idx[0]);

        // add into the main graph
        graph.add_pool(local_mint_idx[0], local_mint_idx[1], pool.clone());


    }

    info!("Added {} pools", pools.len());
    info!("Added {} mints", token_mints.len());
    info!("{:?}", graph.graph[&0].edge[&10]);






    Ok(())
}


























