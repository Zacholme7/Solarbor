
use anyhow::Result;



pub fn get_amount_out(base_resv: f64, quote_resv: f64, amount_in: f64) -> Result<f64> {
    let fee = 0.0025;
    let adjusted_amount_in = amount_in * (1.0 - fee);
    let amount_out = (quote_resv * adjusted_amount_in) / (base_resv + adjusted_amount_in);

    Ok(amount_out)
}
