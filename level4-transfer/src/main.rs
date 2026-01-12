use alloy::primitives::{Address, Bytes, utils};
use alloy::rpc::types::{TransactionInput, TransactionRequest};
use alloy::{self, providers::Provider};
use alloy::network::{ReceiptResponse, TransactionBuilder};
use alloy::signers::local::PrivateKeySigner;
use tokio;
use dotenv;
use std::env;
use std::str::FromStr;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("请输入收款地址参数".into());
    }

    let recipient_address = args[1].parse::<Address>();
    if recipient_address.is_err() {
        return Err("地址格式错误".into());
    }

    let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let sender_address = std::env::var("ACCOUT").expect("ACCOUT must be set");

    let recipient_address = recipient_address.unwrap();
    let sender_address = sender_address.parse::<Address>()?;
    let signer: PrivateKeySigner = private_key.parse()?;

    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;

    let provider = alloy::providers::ProviderBuilder::new()
        .wallet(signer)
        .connect_http(rpc_url);

    // 查询余额
    let balance = provider.get_balance(sender_address).await?;
    println!("原地址： {};\t余额： {}ETH", sender_address, utils::format_units(balance, "eth").unwrap());

    // 查询 gas 价格
    let gas_price = provider.get_gas_price().await?;

    // 发起转账
    let value = utils::parse_units("0.001", "ether").unwrap().into();

    println!("当前 gas 价格： {}Gwei；转账金额： {}ETH", utils::format_units(gas_price, "gwei").unwrap(), utils::format_ether(value));

    let input_bytes = Bytes::from("rust 转账测试");
    let tx = TransactionRequest::default()
        .with_from(sender_address)
        .with_to(recipient_address)
        .with_value(value)
        .with_gas_price(gas_price)
        .with_input(input_bytes);

    // 估算 gas 限制
    let gas_limit = provider.estimate_gas(tx.clone()).await?;
    println!("Estimated gas limit: {}", gas_limit);

    let tx = tx.with_gas_limit(gas_limit);

    let tx_result = provider.send_transaction(tx).await?;
    let tx_response = tx_result.get_receipt().await?;
    println!(
        "转账状态：{}\n交易哈希： {}\nGas used: {}",
        tx_response.status(),
        tx_response.transaction_hash(),
        tx_response.gas_used(),
    );

    return Ok(());
}
