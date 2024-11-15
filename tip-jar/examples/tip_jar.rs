use dotenv::dotenv;
use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
    utils::WEI_IN_ETHER,
};
use eyre::eyre;
use std::str::FromStr;
use std::sync::Arc;

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

    abigen!(
        TipJar,
        r#"[
            function getBalance(address _address) external view returns (uint256)
            function tip(address to) external payable
            function withdraw(address user) external
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = contract_address.parse()?;
    let user1_address: Address = "0x00A2895816e64F152FF81c8A931DC1bd9F5c3ce3".parse()?;

    let wallet = LocalWallet::from_str(&private_key).unwrap();
    let wallet_address: Address = wallet.address();
    let chain_id: u64 = provider.get_chainid().await?.as_u64();

    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let client_cloned = client.clone();

    let tip_jar = TipJar::new(address, client);
    let mut tip_balance: U256 = tip_jar.get_balance(wallet_address).call().await?;
    println!("Initial User 1 Tip Balance = {:?}", tip_balance);

    let mut balance = client_cloned.get_balance(user1_address, None).await?;
    println!("Initial User 1 Balance = {:?}", balance);

    let value = U256::from(WEI_IN_ETHER);

    tip_jar
        .tip(user1_address)
        .value(value)
        .send()
        .await?
        .await?;

    tip_balance = tip_jar.get_balance(user1_address).call().await?;
    println!("After Tip User 1 Tip Balance = {:?}", tip_balance);

    balance = client_cloned.get_balance(user1_address, None).await?;
    println!("After Tip User 1 Balance = {:?}", balance);

    balance = client_cloned.get_balance(address, None).await?;
    println!("After Tip Contract Balance = {:?}", balance);

    tip_jar.withdraw(user1_address).send().await?.await?;

    tip_balance = tip_jar.get_balance(user1_address).call().await?;
    println!("After Withdraw User 1 Tip Balance = {:?}", tip_balance);
    balance = client_cloned.get_balance(user1_address, None).await?;
    println!("After Withdraw User 1 Balance = {:?}", balance);
    balance = client_cloned.get_balance(address, None).await?;
    println!("After Withdraw Contract Balance = {:?}", balance);
    Ok(())
}
