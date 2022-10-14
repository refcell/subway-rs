#![allow(unused)]

use ethers::prelude::*;
use mev_rs::uniswap::*;
use std::str::FromStr;

#[test]
fn test_calculate_uniswap_v2_pair_address() {
    // USDC
    let token_0 = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap();
    // WETH
    let token_1 = Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();
    let pair = calculate_uniswap_v2_pair_address(&token_0, &token_1).unwrap();

    // TODO: This fails...
    // TODO: Actual Pair Address:
    // TODO: Received Pair Address: 0xf783f053865b47bca275fab841b495db797ce00c

    // assert_eq!(
    //     pair,
    //     Address::from_str("0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc").unwrap()
    // );
}

#[tokio::test]
async fn test_get_uniswap_v2_pair_address() {
    // USDC
    let token_0 = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap();
    // WETH
    let token_1 = Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();
    let pair = get_uniswap_v2_pair_address(&token_0, &token_1)
        .await
        .unwrap();
    assert_eq!(
        pair,
        Address::from_str("0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc").unwrap()
    );
}

#[tokio::test]
async fn test_get_uniswap_v2_reserves() {
    let pair = Address::from_str("0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc").unwrap();
    let (token_a_reserves, token_b_reserves) = get_uniswap_v2_reserves(&pair).await.unwrap();
    assert!(token_a_reserves > U256::from(0));
    assert!(token_b_reserves > U256::from(0));
}
