use ethers::prelude::*;
use subway_rs::prelude::calculate_sandwich_optimal_in;

#[test]
fn test_calculate_sandwich_optimal_in_min_recv() {
    // Set up our search parameters
    let user_amount_in = U256::from_dec_str("100").unwrap();
    let user_min_recv_token = U256::from_dec_str("100").unwrap();
    let weth_reserves = U256::from_dec_str("100000").unwrap();
    let token_reserves = U256::from_dec_str("100000").unwrap();

    // Get the amount in
    let amount_in = calculate_sandwich_optimal_in(
        &user_amount_in,
        &user_min_recv_token,
        &weth_reserves,
        &token_reserves,
    );

    // The amount in should be 0
    assert_eq!(U256::zero(), amount_in);
}

#[test]
fn test_calculate_sandwich_optimal_in_unchanged() {
    // Set up our search parameters
    let user_amount_in = U256::from_dec_str("101").unwrap();
    let user_min_recv_token = U256::from_dec_str("100").unwrap();
    let weth_reserves = U256::from_dec_str("100000").unwrap();
    let token_reserves = U256::from_dec_str("100000").unwrap();

    // Get the amount in
    let amount_in = calculate_sandwich_optimal_in(
        &user_amount_in,
        &user_min_recv_token,
        &weth_reserves,
        &token_reserves,
    );

    // The amount in should be 298
    assert_eq!(U256::from(298), amount_in);
}

#[test]
fn test_calculate_sandwich_optimal_in_low_liquidity() {
    // Set up our search parameters
    let user_amount_in = U256::from_dec_str("130").unwrap();
    let user_min_recv_token = U256::from_dec_str("100").unwrap();
    let weth_reserves = U256::from_dec_str("1000").unwrap();
    let token_reserves = U256::from_dec_str("1000").unwrap();

    // Get the amount in
    let amount_in = calculate_sandwich_optimal_in(
        &user_amount_in,
        &user_min_recv_token,
        &weth_reserves,
        &token_reserves,
    );

    // The amount in should be
    assert_eq!(U256::from(75), amount_in);
}
