use crate::pools::pool::Pool;
use crate::pools::raydium::fetch_all_pairs;
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::{Client, Cluster};
use anyhow::Result;
use dotenv::dotenv;
use env_logger;
use log::{debug, info, warn};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::calculator::get_amount_out;
use crate::graph::Graph;
use crate::util::{decode_account_data, unpack_token_account, TokenAccount};

mod birdeye;
mod calculator;
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
    let send_tx_client =
        RpcClient::new_with_commitment(cluster.url(), CommitmentConfig::confirmed());

    // state
    let mut pools: Vec<Pool> = vec![]; // track every pool available to swap
    let mut token_mints: Vec<String> = vec![]; // track every unique token mint
    let mut mint_to_index = HashMap::new(); // mint pubkey -> index in token_mints
    let mut graph_edges: Vec<HashSet<usize>> = vec![]; // graph representation, index in token_mint to all edges (possible swaps)
    let mut graph = Graph::new(); // construct a new graph to represent entire network

    // load in all of the pools that we want to arb
    info!("Loading in all pools");
    pools.extend(fetch_all_pairs().await?);

    // process all of the pools
    info!("Processing Pools and setting up graphs");
    for pool in &pools {
        let mut local_mint_idx = vec![]; // track the indicies of the current pool mints

        // process the mints for this pool
        for mint in [&pool.base, &pool.quote] {
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

    // get the indicies and the pool
    let base = mint_to_index["HfYFjMKNZygfMC8LsQ8LtpPsPxEJoXJx4M6tqi75Hajo"];
    let quote = mint_to_index["EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"];
    let pool = graph.get_pool(base, quote).unwrap().get(0).unwrap();
    let id = Pubkey::from_str(&pool.id)?;
    let account = connection.get_account(&id).unwrap();
    let pool_state = decode_account_data(&account.data).unwrap();

    let base_vault_account = connection.get_account(&pool_state.base_vault).unwrap();
    let quote_vault_account = connection.get_account(&pool_state.quote_vault).unwrap();

    let base_vault_total = unpack_token_account(&base_vault_account.data).amount  - pool_state.base_need_take_pnl;
    let quote_vault_total = unpack_token_account(&quote_vault_account.data).amount - pool_state.quote_need_take_pnl;

    println!("Pool: {:?}", pool_state);
    println!(
        "base: {}, quote: {}, basePnl {}, quotePnl: {}",
        base_vault_total as f64,
        quote_vault_total as f64,
        pool_state.base_need_take_pnl,
        pool_state.quote_need_take_pnl
    );
    let res = get_amount_out(base_vault_total as f64, quote_vault_total as f64, 55.0);
    println!("output {:?}", res);

    Ok(())
}
