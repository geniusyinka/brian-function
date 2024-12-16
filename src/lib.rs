use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;

#[derive(Debug, Deserialize)]
pub struct ApiRequest {
    pub prompt: String,
    pub address: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub output: serde_json::Value,
    pub message: String,
}

pub struct ApiClient {
    api_key: String,
    client: Client,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn process_transaction(&self, request: &ApiRequest) -> Result<ApiResponse> {
        let response = self.client
            .post("https://api.brianknows.org/api/v0/agent/transaction")
            .header("Content-Type", "application/json")
            .header("x-brian-api-key", &self.api_key)
            .json(&serde_json::json!({
                "prompt": &request.prompt,
                "address": &request.address
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("API request failed: {}", response.status()));
        }

        let output = response.json().await?;
        
        Ok(ApiResponse {
            output,
            message: "Transaction processed successfully".to_string(),
        })
    }
}
