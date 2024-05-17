use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Pool {
    pub id: String,
    pub base: String,
    pub quote: String,
    pub base_vault: String,
    pub quote_vault: String,
    pub base_decimals: usize,
    pub quote_decimals: usize,
    pub pool_type: PoolType,
    pub market_base_vault: String,
    pub market_quote_vault: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum PoolType {
    Raydium,
}
