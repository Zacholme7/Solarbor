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
        self.graph
            .entry(mint_a)
            .or_insert_with(|| Edge {
                edge: HashMap::new(),
            })
            .edge
            .entry(mint_b)
            .or_insert_with(Vec::new)
            .push(pool.clone());

        self.graph
            .entry(mint_b)
            .or_insert_with(|| Edge {
                edge: HashMap::new(),
            })
            .edge
            .entry(mint_a)
            .or_insert_with(Vec::new)
            .push(pool);
    }
}

// Edges from each index in
#[derive(Debug)]
pub struct Edge {
    pub edge: HashMap<usize, Vec<Pool>>,
}
