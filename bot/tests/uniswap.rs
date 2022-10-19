#![allow(unused)]

use ethers::prelude::*;
use std::str::FromStr;

use subway_rs::uniswap::*;

#[test]
fn test_get_univ2_router_address() {
    let factory = get_univ2_router_address();
    assert_eq!(
        factory,
        Address::from_str("0x7a250d5630b4cf539739df2c5dacb4c659f2488d").unwrap()
    );
}

#[test]
fn test_get_univ2_factory_address() {
    let factory = get_univ2_factory_address();
    assert_eq!(
        factory,
        Address::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap()
    );
}

#[tokio::test]
async fn test_get_univ2_pair_contract() {
    let pair = get_univ2_pair_contract(
        1,
        &Address::from_str("0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc").unwrap(),
    )
    .unwrap();

    // Validate the pair contract by calling the name field
    let name = pair.name().call().await.unwrap();
    assert_eq!(name, "Uniswap V2");
}

#[tokio::test]
async fn test_get_univ2_factory_contract() {
    let factory = get_univ2_factory_contract().unwrap();

    // Validate contract
    let pairs = factory.all_pairs_length().call().await.unwrap();
    assert!(pairs > U256::zero());
}

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

#[test]
fn test_get_univ2_data_given_in() {
    // Check valid swap
    let a_in = U256::from(100000u64);
    let a_reserves = U256::from(10000000000u64);
    let b_reserves = U256::from(10000000000u64);
    let (b_out, new_a_reserves, new_b_reserves) =
        get_univ2_data_given_in(&a_in, &a_reserves, &b_reserves);
    assert_eq!(new_a_reserves, U256::from(10000100000u64));
    assert_eq!(new_b_reserves, U256::from(9999900301u64));
    assert_eq!(b_out, U256::from(99699u64));

    // Check zero denominator
    let a_in = U256::from(100);
    let a_reserves = U256::from(0);
    let b_reserves = U256::from(1000000);
    let (b_out, new_a_reserves, new_b_reserves) =
        get_univ2_data_given_in(&a_in, &a_reserves, &b_reserves);
    assert_eq!(b_out, U256::from(1000000u64));

    // Check zero numerator
    let a_in = U256::from(100);
    let a_reserves = U256::from(1000000);
    let b_reserves = U256::from(0);
    let (b_out, new_a_reserves, new_b_reserves) =
        get_univ2_data_given_in(&a_in, &a_reserves, &b_reserves);
    assert_eq!(b_out, U256::zero());
}

#[test]
fn test_get_univ2_data_given_out() {
    // Check valid swap
    let b_out = U256::from(100000u64);
    let a_reserves = U256::from(10000000000u64);
    let b_reserves = U256::from(10000000000u64);
    let (a_in, new_a_reserves, new_b_reserves) =
        get_univ2_data_given_out(&b_out, &a_reserves, &b_reserves);
    assert_eq!(new_a_reserves, U256::from(10000100302u64));
    assert_eq!(new_b_reserves, U256::from(9999900000u64));
    assert_eq!(a_in, U256::from(100302u64));

    // Check zero denominator
    let b_out = U256::from(100);
    let a_reserves = U256::from(0);
    let b_reserves = U256::from(1000000);
    let (a_in, new_a_reserves, new_b_reserves) =
        get_univ2_data_given_out(&b_out, &a_reserves, &b_reserves);
    assert_eq!(a_in, U256::from(1u64));

    // Check zero numerator
    let b_out = U256::from(100);
    let a_reserves = U256::from(1000000);
    let b_reserves = U256::from(0);
    let (a_in, new_a_reserves, new_b_reserves) =
        get_univ2_data_given_out(&b_out, &a_reserves, &b_reserves);
    assert_eq!(new_a_reserves, U256::MAX);
    assert_eq!(new_b_reserves, U256::from(0u64));
    assert_eq!(a_in, U256::MAX);
}

#[tokio::test]
async fn test_get_univ2_exact_weth_token_min_recv() {
    let final_min_recv = U256::from(100_000u64);
    let path = vec![
        Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap(),
        Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap(),
    ];

    // Get the exact weth token min recv
    let min_recv = get_univ2_exact_weth_token_min_recv(&final_min_recv, &path)
        .await
        .unwrap();
    assert_eq!(min_recv, U256::from(100_000u64));
}

#[tokio::test]
async fn test_get_univ2_exact_weth_token_min_recv_multiple_tokens() {
    let final_min_recv = U256::from_dec_str("354891658682124759").unwrap();
    let path = vec![
        Address::from_str("0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap(),
        Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap(),
        Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap(),
    ];

    let (a_in, new_a_reserves, new_b_reserves) = get_univ2_data_given_out(
        &final_min_recv,
        &U256::from_dec_str("46835491059752").unwrap(),
        &U256::from_dec_str("35489165868212475948148").unwrap(),
    );
    assert_eq!(
        new_a_reserves,
        U256::from_dec_str("46835960828653").unwrap()
    );
    assert_eq!(
        new_b_reserves,
        U256::from_dec_str("35488810976553793823389").unwrap()
    );
    assert_eq!(a_in, U256::from_dec_str("469768901").unwrap());

    // Get the exact weth token min recv
    let min_recv = get_univ2_exact_weth_token_min_recv(&final_min_recv, &path)
        .await
        .unwrap();
    assert!(min_recv > U256::from_dec_str("100000000").unwrap());
}
