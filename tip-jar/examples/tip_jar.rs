use std::str::FromStr;

use dotenv::dotenv;

use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::eyre;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TipJar,
    "artifacts/TipJar.json"
);

/// Your private key file path.
const PRIV_KEY_PATH: &str = "PRIVATE_KEY";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "RPC_URL";

/// Deployed contract address.
const STYLUS_CONTRACT_ADDRESS: &str = "STYLUS_CONTRACT_ADDRESS";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let private_key =
        std::env::var(PRIV_KEY_PATH).map_err(|_| eyre!("No {} env var set", PRIV_KEY_PATH))?;
    let rpc_url = std::env::var(RPC_URL).map_err(|_| eyre!("No {} env var set", RPC_URL))?;
    let contract_address = std::env::var(STYLUS_CONTRACT_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", STYLUS_CONTRACT_ADDRESS))?;

    println!("Private key: {}", private_key);
    println!("RPC URL: {}", rpc_url);
    println!("Contract Address: {}", contract_address);

    let signer: PrivateKeySigner = PrivateKeySigner::from_str(&private_key).unwrap();
    let wallet = EthereumWallet::from(signer.clone());

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url.parse()?);

    let address: Address = contract_address.parse()?;
    let user1_address: Address = "0x00A2895816e64F152FF81c8A931DC1bd9F5c3ce3".parse()?;

    let tip_jar = TipJar::new(contract_address.parse()?, provider.clone());
    let mut tip_balance: U256 = tip_jar.getBalance(user1_address).call().await?._0;
    println!("Initial User 1 Tip Balance = {:?}", tip_balance);

    let mut balance = provider.get_balance(user1_address).await?;
    println!("Initial User 1 Balance = {:?}", balance);

    let value = U256::from(1e18);

    let mut tx_hash = tip_jar
        .tip(user1_address)
        .value(value)
        .send()
        .await?
        .watch()
        .await?;

    println!("Tip Success, TxHash = {:?}", tx_hash.to_string());

    tip_balance = tip_jar.getBalance(user1_address).call().await?._0;
    println!("After Tip User 1 Tip Balance = {:?}", tip_balance);

    balance = provider.get_balance(user1_address).await?;
    println!("After Tip User 1 Balance = {:?}", balance);

    balance = provider.get_balance(address).await?;
    println!("After Tip Contract Balance = {:?}", balance);

    tx_hash = tip_jar
        .withdraw(user1_address)
        .send()
        .await?
        .watch()
        .await?;

    println!("Withdraw Success, TxHash = {:?}", tx_hash.to_string());

    tip_balance = tip_jar.getBalance(user1_address).call().await?._0;
    println!("After Withdraw User 1 Tip Balance = {:?}", tip_balance);
    balance = provider.get_balance(user1_address).await?;
    println!("After Withdraw User 1 Balance = {:?}", balance);
    balance = provider.get_balance(address).await?;
    println!("After Withdraw Contract Balance = {:?}", balance);
    Ok(())
}
