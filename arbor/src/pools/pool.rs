use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pool {
    pub id: String,
    pub mint_a: String,
    pub mint_b: String,
    pub pool_type: PoolType,
}

#[derive(Debug, Deserialize)]
pub enum PoolType {
    Raydium,
}
