use serde::{Deserialize, Serialize};
use anyhow::Result;

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

#[derive(Debug, Serialize)]
pub struct ApiMetadata {
    pub icon: String,
    pub title: String,
    pub description: String,
    pub label: String,
}

pub struct ApiClient {
    api_key: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_metadata(&self) -> Result<ApiMetadata> {
        println!("Getting API metadata...");

        Ok(ApiMetadata {
            icon: "https://t3.ftcdn.net/jpg/05/59/27/48/360_F_559274893_O9iSRQwTKIkAooNTglilMgx2yMcXK9Or.jpg".to_string(),
            title: "Brian Knows API".to_string(),
            description: "Invoke the Brian Knows API for transactions Main".to_string(),
            label: "Activate Transaction".to_string(),
        })
    }

    pub async fn process_transaction(&self, request: &ApiRequest) -> Result<ApiResponse> {
        println!("Making API request with:");
        println!("Prompt: {}", request.prompt);
        println!("Address: {}", request.address);

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

        let status = response.status();
        println!("Response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            println!("Error response: {}", error_text);
            return Err(anyhow::anyhow!("API request failed: {} - {}", status, error_text));
        }

        let output = response.json().await?;
        println!("Received response: {}", serde_json::to_string_pretty(&output)?);
        
        Ok(ApiResponse {
            output,
            message: "Transaction processed successfully".to_string(),
        })
    }
}
