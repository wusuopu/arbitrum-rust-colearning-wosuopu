use alloy::{self, providers::Provider};
use tokio;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;

    let provider = alloy::providers::ProviderBuilder::new().connect_http(rpc_url); 
    
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {latest_block}");
    println!("Hello web3");

    return Ok(());
}
