
use anyhow::Result;


pub fn get_amount_out(base_resv: f64, quote_resv: f64, base_decimals: usize, quote_decimals: usize, amount_in: f64) -> Result<f64> {
    let base_normalized = base_resv * 10f64.powi(base_decimals as i32);
    let quote_normalized = quote_resv * 10f64.powi(quote_decimals as i32);
    let amount_in_normalized = amount_in * 10f64.powi(base_decimals as i32);

    let amount_out_normalized = (quote_normalized * amount_in_normalized) / (base_normalized + amount_in_normalized);
    Ok(amount_out_normalized / 10f64.powi(quote_decimals as i32))
}


