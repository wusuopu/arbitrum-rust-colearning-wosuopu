use alloy::primitives::{utils};
use alloy::{self, providers::Provider};
use tokio;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;

    let provider = alloy::providers::ProviderBuilder::new().connect_http(rpc_url); 
    
    let price = provider.get_gas_price().await?;

    let gas_limit = 21000 as u128;
    let result = price * gas_limit;
    println!(
        "Gas Price: {}Gwei;\nGas Limit: {}Gwei;\nGas Fees: {}Gwei;",
        utils::format_units(price, "gwei").unwrap(),
        utils::format_units(gas_limit, "gwei").unwrap(),
        utils::format_units(result, "gwei").unwrap()
    );

    return Ok(());
}
