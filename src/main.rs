// src/main.rs
use brian_function::{ApiClient, ApiRequest};
use sgxkit::io::OutputWriter;
use std::io::Write;

fn main() {
    let mut writer = OutputWriter::new();

    // Get API key from environment
    let api_key = match std::env::var("BRIAN_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            writer
                .write_all(format!("API Key Error: {}\n", e).as_bytes())
                .unwrap();
            return;
        }
    };

    // Create API client
    let client = ApiClient::new(api_key);
    
    // Create test request
    let request = ApiRequest {
        prompt: "swap 0.001 ETH for USDC on polygon to 0x05CC0391045d49BE3DA54e49041Ad5c5eDB555Db".to_string(),
        address: "0x0eD6b59C8BFfd2DB1019Ea8938F11Ad19c8Be0a5".to_string(),
    };

    // Create a runtime for async operations
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Process the request
    match runtime.block_on(async {
        client.process_transaction(&request).await
    }) {
        Ok(response) => {
            writer
                .write_all(serde_json::to_string_pretty(&response).unwrap().as_bytes())
                .unwrap();
            writer
                .write_all(b"\nTransaction processed successfully\n")
                .unwrap();
        }
        Err(e) => {
            writer
                .write_all(format!("Error: {}\n", e).as_bytes())
                .unwrap();
        }
    }
}
