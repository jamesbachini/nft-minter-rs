use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::signers::{LocalWallet, Signer};
use std::sync::Arc;
use std::convert::TryFrom;
use std::time::Duration;
use dotenv::dotenv;
use std::env;

abigen!(
    NFT,
    r#"[
        function mint(address _to) external
     ]"#
);

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    
    let infura_api_key = env::var("INFURA_API_KEY").expect("INFURA_API_KEY must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let provider = Provider::<Http>::try_from(format!(
        "https://sepolia.infura.io/v3/{}",
        infura_api_key
    ))?
    .interval(Duration::from_millis(10u64));
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?;
    let wallet = wallet.with_chain_id(11155111u64); // Sepolia
    let client = SignerMiddleware::new(provider.clone(), wallet);
    let client = Arc::new(client);
    println!("Account Connected: {}", client.address());
    let balance = provider.get_balance(client.address(), None).await?;
    println!("Account Balance: {} ETH", ethers::utils::format_ether(balance));
    let nft_address = "0xaF3148FE00021529cBdBd6129383119dc16bDF0C".parse::<Address>()?;
    let nft = NFT::new(nft_address, client.clone());   
    println!("Preparing transaction...");
    println!("NFT address: {:?}", nft_address);
    let tx = nft.mint(client.address()).gas(500000);
    println!("Sending transaction...");
    match tx.send().await {
        Ok(pending_tx) => {
            println!("Transaction sent successfully. Hash: {:?}", pending_tx.tx_hash());
            match pending_tx.await {
                Ok(receipt) => {
                    println!("Transaction successful!");
                    println!("Receipt: {:?}", receipt);
                },
                Err(e) => {
                    println!("Transaction failed: {:?}", e);
                }
            }
        },
        Err(e) => println!("Failed to send transaction: {:?}", e),
    }

    Ok(())
}