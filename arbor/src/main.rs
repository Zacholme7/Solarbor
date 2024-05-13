use anyhow::Result;
use dotenv::dotenv;
use jupiter_swap_api_client::JupiterSwapApiClient;
use tokio;
use crate::birdeye::fetch_trending;
use crate::jup::get_quote;

mod birdeye;
mod jup;


#[tokio::main]
async fn main() -> Result<()> {
        // load in configuration
        dotenv().ok();

        //  find the latests 24hour trending on birdeye
        let trending = fetch_trending().await?;

        let jupiter_swap_api_client = JupiterSwapApiClient::new("https://quote-api.jup.ag/v6".to_string());
        let _ = get_quote(
                &jupiter_swap_api_client,
                trending["USDT"].clone(),
                trending["POPCAT"].clone(),
                100_000
        ).await?;

        Ok(())
}




