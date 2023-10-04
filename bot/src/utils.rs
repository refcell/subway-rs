//! A module containing common utilities

use std::{str::FromStr, sync::Arc};
use dotenv::dotenv; // read .env files
use ethers::{prelude::*, types::transaction::eip2718::TypedTransaction};
use eyre::Result;
use rand::Rng;

/// Sorts two tokens
pub fn sort_tokens(a: &mut Address, b: &mut Address) {
    if a > b {
        std::mem::swap(a, b);
    }
}

/// Returns the WETH Contract Address
///
/// Although this function unwraps the address conversion, it is safe as the string is checked.
/// Edit this address to match the one on the chain you're operating on
pub fn get_weth_address() -> Address {
    Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap()
}

/// Returns the usdc Contract Address
///
/// Although this function unwraps the address conversion, it is safe as the string is checked.
/// Edit this address to match the one on the chain you're operating on
pub fn get_usdc_address() -> Address {
    Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap()
}

/// Get Raw Transaction
pub fn get_raw_transaction(tx: &Transaction) -> TypedTransaction {
    let typed_transaction: TypedTransaction = tx.into();
    typed_transaction
}

/// Calculate the next block base fee
pub fn calculate_next_block_base_fee(block: Block<TxHash>) -> eyre::Result<U256> {
    // Get the block base fee per gas
    let base_fee = block
        .base_fee_per_gas
        .ok_or(eyre::eyre!("Block missing base fee per gas"))?;

    // Get the mount of gas used in the block
    let gas_used = block.gas_used;

    // Get the target gas used
    let mut target_gas_used = block.gas_limit / 2;
    target_gas_used = if target_gas_used == U256::zero() {
        U256::one()
    } else {
        target_gas_used
    };

    // Calculate the new base fee
    let new_base_fee = {
        if gas_used > target_gas_used {
            base_fee
                + ((base_fee * (gas_used - target_gas_used)) / target_gas_used) / U256::from(8u64)
        } else {
            base_fee
                - ((base_fee * (target_gas_used - gas_used)) / target_gas_used) / U256::from(8u64)
        }
    };

    // Add a random seed so it hashes differently
    let seed = rand::thread_rng().gen_range(0..9);
    Ok(new_base_fee + seed)
}

/// Read environment variables
pub fn read_env_vars() -> Result<Vec<(String, String)>> {
    dotenv().ok(); // read .env file
    let mut env_vars = Vec::new();
    let keys = vec![
        "RPC_URL",
        "RPC_URL_WSS",
        "PRIVATE_KEY", // remove the 0x from the key in the .env file
        "FLASHBOTS_AUTH_KEY", // remove the 0x from the key in the .env file
        "SANDWICH_CONTRACT",
    ];
    for key in keys {
        // Read environment variable
        let value = std::env::var(key)
            .map_err(|_| eyre::eyre!("Required environment variable \"{}\" not set", key))?;
        env_vars.push((key.to_string(), value));
    }
    Ok(env_vars)
}

/// Returns the configured Sandwich Contract Address
pub fn get_sandwich_contract_address() -> Result<Address> {
    dotenv().ok(); // read .env
    let addr = std::env::var("SANDWICH_CONTRACT")
        .map_err(|_| eyre::eyre!("Required environment variable \"SANDWICH_CONTRACT\" not set"))?;
    Address::from_str(&addr).map_err(|_| eyre::eyre!("Invalid address \"{}\"", addr))
}

/// Return a Provider for the given URL
pub fn get_http_provider() -> Result<Provider<Http>> {
    dotenv().ok(); // read .env
    let url = std::env::var("RPC_URL")
        .map_err(|_| eyre::eyre!("Required environment variable \"RPC_URL\" not set"))?;
    Provider::<Http>::try_from(url).map_err(|_| eyre::eyre!("Invalid RPC URL"))
}

/// Return a Provider for the given Websocket URL
pub async fn get_ws_provider() -> Result<Provider<Ws>> {
    dotenv().ok(); // read .env
    let url = std::env::var("RPC_URL_WSS")
        .map_err(|_| eyre::eyre!("Required environment variable \"RPC_URL_WSS\" not set"))?;
    Provider::<Ws>::connect(&url)
        .await
        .map_err(|e| eyre::eyre!("RPC Connection Error: {:?}", e))
}

/// Create Websocket Client
pub async fn create_websocket_client() -> Result<Arc<Provider<Ws>>> {
    let client = get_ws_provider().await?;
    Ok(Arc::new(client))
}

/// Construct the searcher wallet
pub fn get_searcher_wallet() -> Result<LocalWallet> {
    dotenv().ok(); // read .env
    let private_key = std::env::var("PRIVATE_KEY")
        .map_err(|_| eyre::eyre!("Required environment variable \"PRIVATE_KEY\" not set"))?;
    private_key
        .parse::<LocalWallet>()
        .map_err(|e| eyre::eyre!("Failed to parse private key: {:?}", e))
}

/// Construct the bundle signer
/// This is your flashbots searcher identity
pub fn get_bundle_signer() -> Result<LocalWallet> {
    dotenv().ok(); // read .env
    let private_key = std::env::var("FLASHBOTS_AUTH_KEY")
        .map_err(|_| eyre::eyre!("Required environment variable \"FLASHBOTS_AUTH_KEY\" not set"))?;
    private_key
        .parse::<LocalWallet>()
        .map_err(|e| eyre::eyre!("Failed to parse flashbots signer: {:?}", e))
}

/// Creates a client from a provider
pub fn create_http_client(
    p: Provider<Http>,
    chain_id: u64,
) -> Result<Arc<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    let wallet = get_searcher_wallet()?;
    let client = SignerMiddleware::new(p, wallet.with_chain_id(chain_id));
    Ok(Arc::new(client))
}
