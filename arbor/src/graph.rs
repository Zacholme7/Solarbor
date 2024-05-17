use crate::Pool;
use std::collections::HashMap;

// Graph to interconnect all of the pools
#[derive(Debug)]
pub struct Graph {
    pub graph: HashMap<usize, Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    pub fn add_pool(&mut self, mint_a: usize, mint_b: usize, pool: Pool) {
        // Add pool for base -> quote direction
        self.graph
            .entry(mint_a)
            .or_insert_with(|| Edge {
                edge: HashMap::new(),
            })
            .edge
            .entry(mint_b)
            .or_insert_with(Vec::new)
            .push(pool.clone());

        // Add pool for quote -> base direction with swapped base and quote
        let reversed_pool = Pool {
            id: pool.id.clone(),
            base: pool.quote.clone(),
            quote: pool.base.clone(),
            base_vault: pool.quote_vault.clone(),
            quote_vault: pool.base_vault.clone(),
            base_decimals: pool.quote_decimals,
            quote_decimals: pool.base_decimals,
            pool_type: pool.pool_type.clone(),
            base_vault_total: pool.quote_vault_total.clone(),
            quote_vault_total: pool.base_vault_total.clone()
        };

        self.graph
            .entry(mint_b)
            .or_insert_with(|| Edge {
                edge: HashMap::new(),
            })
            .edge
            .entry(mint_a)
            .or_insert_with(Vec::new)
            .push(reversed_pool);
    }


    pub fn get_pool(&self, base_idx: usize, quote_idx: usize) -> Option<&Vec<Pool>> {
        self.graph.get(&base_idx)?.edge.get(&quote_idx)
    }
}

// Edges from each index in
#[derive(Debug)]
pub struct Edge {
    pub edge: HashMap<usize, Vec<Pool>>,
}
