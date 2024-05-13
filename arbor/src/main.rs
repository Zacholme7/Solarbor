use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
struct Token {
        address: String,
        name: String
}

#[derive(Deserialize, Debug)]
struct TokenList {
        tokens: Vec<Token>
}


#[derive(Deserialize, Debug)]
struct ApiResponse {
        data: TokenList
}




#[tokio::main]
async fn main() -> Result<()> {
        let url = "https://public-api.birdeye.so/defi/tokenlist?sort_by=v24hUSD&sort_type=desc";
        let api_key = "59e9ffa005134bf193ddc73b3a36a51f";

        let client = Client::new();
        let response = client
                .get(url)
                .header("X-API-KEY", api_key)
                .send()
                .await?
                .json::<ApiResponse>()
                .await?;

        let mut token_map: HashMap<String, String> = HashMap::new();

        for token in response.data.tokens {
                token_map.insert(token.name, token.address);
        }

        // Print the token map
        for (name, address) in &token_map {
                println!("Name: {}, Address: {}", name, address);
        }

        Ok(())
}

/* 
async fn get_quote(client: &Client, input_mint: &str, output_mint: &str, amount: u64) -> Result<Value> {
        let url = format!("https://quote-api.jup.ag/v1/quote?inputMint={}&outputMint={}&amount={}", input_mint, output_mint, amount);
        let res = client.get(&url).send().await?.json::<Value>().await?;
        Ok(res)
}
*/




