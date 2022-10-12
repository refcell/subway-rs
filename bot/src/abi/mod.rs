//! ABIs
//!
//! Contract ABIs are refactored into their own module to gracefully deal with allowing missing docs on the abigen macro.
#![allow(missing_docs)]

use ethers::{abi::AbiDecode, prelude::*};
use eyre::Result;

abigen!(UniswapV2Pair, "src/abi/IUniswapV2Pair.json");
abigen!(UniswapV2Router02, "src/abi/IUniswapV2Router02.json");

/// Decodes the raw tx data into a UniswapV2Router02 Call
pub fn decode_uniswap_router_calldata(data: &Bytes) -> Result<SwapExactETHForTokensCall> {
    SwapExactETHForTokensCall::decode(data).map_err(|e| eyre::eyre!(e))
}
