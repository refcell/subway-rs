use ethers::prelude::*;
use std::str::FromStr;
use subway_rs::utils::*;

#[test]
fn test_sort_tokens() {
    let mut a = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();
    let mut b = Address::from_str("0x0000000000000000000000000000000000000002").unwrap();
    sort_tokens(&mut a, &mut b);
    assert_eq!(
        a,
        Address::from_str("0x0000000000000000000000000000000000000001").unwrap()
    );
    assert_eq!(
        b,
        Address::from_str("0x0000000000000000000000000000000000000002").unwrap()
    );
    sort_tokens(&mut b, &mut a);
    assert_eq!(
        a,
        Address::from_str("0x0000000000000000000000000000000000000002").unwrap()
    );
    assert_eq!(
        b,
        Address::from_str("0x0000000000000000000000000000000000000001").unwrap()
    );
}

#[test]
fn test_get_weth_address() {
    let factory = get_weth_address();
    assert_eq!(
        factory,
        Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap()
    );
}

#[test]
fn test_get_usdc_address() {
    let factory = get_usdc_address();
    assert_eq!(
        factory,
        Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap()
    );
}

#[test]
fn test_calculate_next_block_base_fee() {
    let block = Block {
        base_fee_per_gas: Some(U256::from(100)),
        gas_used: U256::from(100),
        gas_limit: U256::from(100),
        ..Default::default()
    };
    let new_base_fee = calculate_next_block_base_fee(block).unwrap();
    // The fee should be 112 + random number between 0 and 9
    assert!(new_base_fee >= U256::from(112));
    assert!(new_base_fee < U256::from(122));
}

#[test]
fn test_calculate_next_block_base_fee_zero_gas_used() {
    let block = Block {
        base_fee_per_gas: Some(U256::from(100)),
        gas_used: U256::zero(),
        gas_limit: U256::from(100),
        ..Default::default()
    };
    let new_base_fee = calculate_next_block_base_fee(block).unwrap();
    // The fee should be 88 + random number between 0 and 9
    assert!(new_base_fee >= U256::from(88));
    assert!(new_base_fee < U256::from(98));
}

#[test]
fn test_calculate_next_block_base_fee_zero_gas_limit() {
    let block = Block {
        base_fee_per_gas: Some(U256::from(100)),
        gas_used: U256::from(100),
        gas_limit: U256::zero(),
        ..Default::default()
    };
    let new_base_fee = calculate_next_block_base_fee(block).unwrap();
    // The fee should be 1336 + random number between 0 and 9
    assert!(new_base_fee >= U256::from(1336));
    assert!(new_base_fee < U256::from(1346));
}
