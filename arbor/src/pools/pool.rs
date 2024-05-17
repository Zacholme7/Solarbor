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
    pub base_vault_total: u64,
    pub quote_vault_total: u64,
    pub pool_type: PoolType,
}

#[derive(Debug, Deserialize, Clone)]
pub enum PoolType {
    Raydium,
    Meteora
}
