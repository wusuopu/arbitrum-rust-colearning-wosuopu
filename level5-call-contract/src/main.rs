use alloy::sol;
use alloy::{
    primitives::{address, Address, U256},
};
use tokio;
use std::env;

// 使用 sol! 宏定义 ERC20 ABI 中的必要函数和事件[7](@ref)
sol!(
    #[sol(rpc)]
    IERC20,
    "src/bep20-abi.json"
);

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
    let erc20 = IERC20::new(address!("0xecf8b7e40c7b43fd565496f6f80f3060ad7be07f"), &provider);

    // 3. 构建调用消息并执行查询（balanceOf 是 view 函数，使用 call()）
    let balance: U256 = erc20.balanceOf(address).call().await?;

    println!("调用 TestUSDC({}) balanceOf 方法\n代币余额: {}", erc20.address(), balance);

    return Ok(());
}
