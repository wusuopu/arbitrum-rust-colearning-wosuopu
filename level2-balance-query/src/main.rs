use alloy::primitives::{Address, utils};
use alloy::{self, providers::Provider};
use tokio;
use std::env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("请输入地址参数".into());
    }

    let address = args[1].parse::<Address>();
    if address.is_err() {
        return Err("地址格式错误".into());
    }

    let address = address.unwrap();
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;

    let provider = alloy::providers::ProviderBuilder::new().connect_http(rpc_url); 

    let balance = provider.get_balance(address).await?;
    
    println!("查询地址： {}", format!("https://sepolia.arbiscan.io/address/{address}"));
    println!("地址： {};\n余额： {}ETH", address, utils::format_units(balance, "eth").unwrap());

    return Ok(());
}
