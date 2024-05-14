use std::env;
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Token {
        pub address: String,
        pub name: String
}

#[derive(Deserialize, Debug)]
struct TokenList {
        tokens: Vec<Token>
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
        data: TokenList
}


pub async fn fetch_trending() -> Result<HashMap<String, String>> {
        // load in api and key
        let birdeye_url = format!("{}{}", 
                std::env::var("BIRDEYE_URL")?, 
                "/defi/tokenlist?sort_by=v24hUSD&sort_type=desc");
        let birdeye_key = std::env::var("BIRDEYE_KEY")?;

        let client = Client::new();
        let response = client
                .get(birdeye_url)
                .header("X-API-KEY", birdeye_key)
                .send()
                .await?
                .json::<ApiResponse>()
                .await?;

        let mut token_map: HashMap<String, String> = HashMap::new();

        for token in response.data.tokens {
                token_map.insert(token.name, token.address);
        }

        for (name, address) in &token_map {
                println!("Name: {}, Address: {}", name, address);
        }


        Ok(token_map)
}
