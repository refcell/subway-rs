//! Uniswap Utilities

use ethers::prelude::*;
use eyre::Result;
use hex::FromHex;

use crate::utils::get_univ2_factory_address;

/// Gets the Uniswap V2 Pair Contract Address given two token addresses
pub fn get_uniswap_v2_pair_address(a: &Address, b: &Address) -> Result<Address> {
    // Sort the tokens
    let mut tokens = vec![a, b];
    tokens.sort();

    // Copy the token addresses into a byte array
    let mut data = [0u8; 32];
    data[12..].copy_from_slice(a.as_bytes());
    data[24..].copy_from_slice(b.as_bytes());

    // Hash the concatenated token address bytes
    let salt = ethers::utils::keccak256(data);

    // Get the init code
    let init_code =
        Vec::from_hex("96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f")
            .map_err(|_| eyre::eyre!("Invalid init code hex"))?;

    // Compute the address with create2
    Ok(ethers::utils::get_create2_address(
        get_univ2_factory_address()?,
        salt,
        Bytes::from(init_code),
    ))
}
