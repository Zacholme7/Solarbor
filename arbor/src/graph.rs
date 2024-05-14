use crate::birdeye::Token;
use crate::jup::get_quote;
use anyhow::Result;
use jupiter_swap_api_client::JupiterSwapApiClient;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Edge {
    pub to: String,
    pub rate: f64,
}

type Graph = HashMap<String, Vec<Edge>>;

pub async fn build_graph(
    client: &JupiterSwapApiClient,
    nodes: &HashMap<String, String>,
    amount: u64,
) -> Result<Graph> {
    let mut graph: Graph = HashMap::new();

    for (from_name, from_address) in nodes {
        for (to_name, to_address) in nodes {
            if from_name != to_name {
                if let Ok(rate) =
                    get_quote(client, from_address.clone(), to_address.clone(), amount).await
                {
                    let edge = Edge {
                        to: to_name.clone(),
                        rate,
                    };
                    graph
                        .entry(from_name.clone())
                        .or_insert(Vec::new())
                        .push(edge);
                } else {
                    println!("err");
                }
            }
        }
    }
    Ok(graph)
}

