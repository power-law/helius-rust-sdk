use helius_sdk::config::Config;
use helius_sdk::error::HeliusError;
use helius_sdk::rpc_client::RpcClient;
use helius_sdk::types::*;
use helius_sdk::Helius;

use reqwest::Client;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), HeliusError> {
    let api_key: &str = "your_api_key";
    let cluster: Cluster = Cluster::MainnetBeta;

    let config: Arc<Config> = Arc::new(Config::new(api_key, cluster)?);
    let client: Client = Client::new();
    let rpc_client: Arc<RpcClient> = Arc::new(RpcClient::new(Arc::new(client.clone()), Arc::clone(&config)).unwrap());

    let helius: Helius = Helius {
        config,
        client,
        rpc_client,
    };

    let request: ParseTransactionsRequest = ParseTransactionsRequest {
        transactions: vec![
            "2sShYqqcWAcJiGc3oK74iFsYKgLCNiY2DsivMbaJGQT8pRzR8z5iBcdmTMXRobH8cZNZgeV9Ur9VjvLsykfFE2Li".to_string(),
        ],
    };

    let response: Result<Vec<EnhancedTransaction>, HeliusError> = helius.parse_transactions(request).await;
    println!("Assets: {:?}", response);

    Ok(())
}