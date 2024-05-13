use jupiter_swap_api_client::{
        quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
        JupiterSwapApiClient,
    };
use solana_sdk::{pubkey, transaction::VersionedTransaction};
use solana_sdk::{pubkey::Pubkey, signature::NullSigner};
use anyhow::Result;
use std::str::FromStr;


pub async fn get_quote(client: &JupiterSwapApiClient, input_mint: String, output_mint: String, amount: u64) -> Result<()>{
        let input_mint = Pubkey::from_str(input_mint.as_str())?;
        let output_mint = Pubkey::from_str(output_mint.as_str())?;
        let quote_request = QuoteRequest {
                amount: amount,
                input_mint,
                output_mint,
                slippage_bps: 50,
                ..QuoteRequest::default()
        };

        // GET /quote
        let quote_response = client.quote(&quote_request).await.unwrap();
        println!("{quote_response:#?}");
        Ok(())
}