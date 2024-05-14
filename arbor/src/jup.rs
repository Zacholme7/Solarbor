use anyhow::Result;
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_sdk::{pubkey, transaction::VersionedTransaction};
use solana_sdk::{pubkey::Pubkey, signature::NullSigner};
use std::str::FromStr;

pub async fn get_quote(
    client: &JupiterSwapApiClient,
    input_mint: String,
    output_mint: String,
    amount: u64,
) -> Result<f64> {
    let input_mint = Pubkey::from_str(input_mint.as_str())?;
    let output_mint = Pubkey::from_str(output_mint.as_str())?;
    let quote_request = QuoteRequest {
        amount,
        input_mint,
        output_mint,
        slippage_bps: 50,
        ..QuoteRequest::default()
    };

    // GET /quote
    match client.quote(&quote_request).await {
        Ok(quote_response) => {
            println!("success {} -> {}", input_mint, output_mint);
            Ok(quote_response.out_amount as f64)
        },
        Err(e) => {
            println!("failed {} -> {} with {}", input_mint, output_mint, e);
            Ok(0 as f64)
        }
    }

    /*
    let quote_response = client.quote(&quote_request).await.unwrap();
    println!("{quote_response:#?}");
    //Ok(quote_response.out_amount as f64 / quote_response.in_amount as f64)
    Ok(quote_response.out_amount as f64)
    */
}


