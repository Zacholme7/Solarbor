use crate::pools::pool::PoolType;
use anyhow::Result;

use anchor_client::solana_sdk::pubkey::Pubkey;

// get amount out for raydium
pub async fn get_amount_out(
    base: Pubkey,
    quote: Pubkey,
    amount_in: f64,
    pool_type: PoolType,
) -> Result<f64> {
    let amount_out = match pool_type {
        PoolType::Raydium => get_amount_out_raydium(base, quote, amount_in).await?,
        PoolType::Meteora => get_amount_out_meteora(base, quote, amount_in).await?,
    };
    Ok(amount_out)
}

// Calculate the amount out for a raydium pool
async fn get_amount_out_raydium(
    base: Pubkey,
    quote: Pubkey,
    amount_in: f64
) -> Result<f64> {
    /*
    let fee = 0.0025;
    let adjusted_amount_in = amount_in * (1.0 - fee);
    let amount_out = (quote_resv * adjusted_amount_in) / (base_resv + adjusted_amount_in);

    Ok(amount_out)
    */
    todo!()
}

// calculate the amount out for a meteora pool
async fn get_amount_out_meteora(
    base: Pubkey,
    quote: Pubkey,
    amount_in: f64
) -> Result<f64> {
    todo!()
}
