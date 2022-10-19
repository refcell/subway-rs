//! Uniswap Utilities

use ethers::prelude::*;
use eyre::Result;
use hex::FromHex;
use std::str::FromStr;

use crate::utils::*;
use crate::{abi::UniswapV2Pair, prelude::UniswapV2Factory};

/// Returns the Uniswap V2 Pair Contract Address
///
/// Although this function unwraps the address conversion, it is safe as the string is checked.
pub fn get_univ2_router_address() -> Address {
    Address::from_str("0x7a250d5630b4cf539739df2c5dacb4c659f2488d").unwrap()
}

/// Returns the Uniswap V2 Factory Address
///
/// Although this function unwraps the address conversion, it is safe as the string is checked.
pub fn get_univ2_factory_address() -> Address {
    Address::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap()
}

/// Construct the Uniswap V2 Pair Contract
pub fn get_univ2_pair_contract(
    chain_id: u64,
    address: &Address,
) -> Result<UniswapV2Pair<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    // Create a client
    let provider = get_http_provider()?;
    let client = create_http_client(provider, chain_id)?;

    // Return the contract
    Ok(UniswapV2Pair::new(*address, client))
}

/// Construct the Uniswap V2 Factory Contract
pub fn get_univ2_factory_contract(
) -> Result<UniswapV2Factory<SignerMiddleware<Provider<Http>, LocalWallet>>> {
    // Create a client
    let provider = get_http_provider()?;
    let client = create_http_client(provider, 1)?;

    // Get the factory address
    let factory_address = get_univ2_factory_address();

    // Return the contract
    Ok(UniswapV2Factory::new(factory_address, client))
}

/// Gets the Uniswap V2 Pair Contract Address given two token addresses
pub fn calculate_uniswap_v2_pair_address(a: &Address, b: &Address) -> Result<Address> {
    // Sort the tokens
    let mut tokens = vec![a, b];
    tokens.sort();

    // Copy the token addresses into a byte array
    let mut data = [0u8; 40];
    data[0..20].copy_from_slice(tokens[0].as_bytes());
    data[20..].copy_from_slice(tokens[1].as_bytes());

    // Hash the concatenated token address bytes
    let salt = ethers::utils::keccak256(data);

    // Get the init code
    let init_code =
        Vec::from_hex("96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f")
            .map_err(|_| eyre::eyre!("Invalid init code hex"))?;

    // Compute the address with create2
    Ok(ethers::utils::get_create2_address(
        get_univ2_factory_address(),
        salt,
        Bytes::from(init_code),
    ))
}

/// Gets the Uniswap V2 Pair Contract Address given two token addresses
pub async fn get_uniswap_v2_pair_address(a: &Address, b: &Address) -> Result<Address> {
    // Get the uniswap v2 factory contract
    let factory = get_univ2_factory_contract()?;

    // Get the pair address
    factory
        .get_pair(*a, *b)
        .call()
        .await
        .map_err(|e| eyre::eyre!(e))
}

/// Get the Uniswap V2 Reserves for a give token pair
pub async fn get_uniswap_v2_reserves(pair: &Address) -> Result<(U256, U256)> {
    let contract = get_univ2_pair_contract(1, pair)?;
    let (token_a_reserves, token_b_reserves, _last_time_updated) =
        contract.get_reserves().call().await?;
    Ok((U256::from(token_a_reserves), U256::from(token_b_reserves)))
}

/// Returns how much output if we supply in
/// Follows: Uniswap v2; x * y = k formula
/// Accounts for a 0.3% fee
pub fn get_univ2_data_given_in(
    a_in: &U256,
    a_reserves: &U256,
    b_reserves: &U256,
) -> (U256, U256, U256) {
    // Calculate the output
    let a_in_with_fee: U256 = a_in * 997;
    let numerator: U256 = a_in_with_fee * b_reserves;
    let denominator: U256 = a_reserves * 1000 + a_in_with_fee;
    let b_out: U256 = numerator.checked_div(denominator).unwrap_or(U256::zero());

    // Calculate the new b reserves, accounting for underflow
    let new_b_reserves = b_reserves.checked_sub(b_out).unwrap_or(U256::one());

    // Calculate the new a reserves, accounting for overflow
    let new_a_reserves = a_reserves.checked_add(*a_in).unwrap_or(U256::MAX);

    // Return
    (b_out, new_a_reserves, new_b_reserves)
}

/// Returns how much output if we supply out
/// Follows: Uniswap v2; x * y = k formula
/// Accounts for a 0.3% fee
pub fn get_univ2_data_given_out(
    b_out: &U256,
    a_reserves: &U256,
    b_reserves: &U256,
) -> (U256, U256, U256) {
    // Calculate the new b reserves, accounting for underflow
    let new_b_reserves = b_reserves.checked_sub(*b_out).unwrap_or(U256::zero());

    // Calculate the amount in
    let numerator: U256 = a_reserves * b_out * 1000;
    let denominator: U256 = new_b_reserves * 997;
    let a_in = numerator.checked_div(denominator).unwrap_or(U256::MAX - 1) + 1;

    // Calculate the new a reserves, accounting for overflow
    let new_a_reserves = a_reserves.checked_add(a_in).unwrap_or(U256::MAX);

    // Return
    (a_in, new_a_reserves, new_b_reserves)
}

/// Compute how much the user is willing to accept as a minimum output
pub async fn get_univ2_exact_weth_token_min_recv(
    final_min_recv: &U256,
    path: &Vec<Address>,
) -> Result<U256> {
    let mut user_min_recv = *final_min_recv;

    // Computes the lowest amount of tokens after weth
    let mut i = path.len() - 1;
    while i > 1 {
        // Get the token pair address
        let from_token = path[i - 1];
        let to_token = path[i];
        // TODO: Use the calculate function to get the correct pair address
        let pair = get_uniswap_v2_pair_address(&from_token, &to_token).await?;

        // Get the token pair reserves
        let (from_reserves, to_reserves) = get_uniswap_v2_reserves(&pair).await?;

        // Get the new reserve data
        (user_min_recv, _, _) =
            get_univ2_data_given_out(&user_min_recv, &from_reserves, &to_reserves);

        // Decrement and iterate
        i -= 1;
    }

    // Return the final amount
    Ok(user_min_recv)
}
