use ethers::prelude::*;
use subway_rs::numeric;

#[test]
fn test_calculate_sandwich_optimal_in_min_recv() {
    // Set up our search parameters
    let user_amount_in = U256::from_dec_str("100").unwrap();
    let user_min_recv_token = U256::from_dec_str("100").unwrap();
    let weth_reserves = U256::from_dec_str("100000").unwrap();
    let token_reserves = U256::from_dec_str("100000").unwrap();

    // Get the amount in
    let amount_in = numeric::calculate_sandwich_optimal_in(
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
    let amount_in = numeric::calculate_sandwich_optimal_in(
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
    let amount_in = numeric::calculate_sandwich_optimal_in(
        &user_amount_in,
        &user_min_recv_token,
        &weth_reserves,
        &token_reserves,
    );

    // The amount in should be
    assert_eq!(U256::from(75), amount_in);
}

#[test]
fn test_fail_calculate_sandwich_context() {
    let optimal_weth_in = U256::from(10);
    let user_amount_in = U256::from(10);
    let user_min_recv = U256::from(10);
    let weth_reserves = U256::from(10);
    let token_reserves = U256::from(10);
    if numeric::calculate_sandwich_context(
        &optimal_weth_in,
        &user_amount_in,
        &user_min_recv,
        &weth_reserves,
        &token_reserves,
    )
    .is_ok()
    {
        panic!("Expecting sandwich context calculation to error");
    }
}

#[test]
fn test_calculate_sandwich_context() {
    // Setup params
    let optimal_weth_in = U256::from(400);
    let user_amount_in = U256::from(20);
    let user_min_recv = U256::from(10);
    let weth_reserves = U256::from(1000);
    let token_reserves = U256::from(1000);

    // Construct the Sandwich Context
    let ctx = numeric::calculate_sandwich_context(
        &optimal_weth_in,
        &user_amount_in,
        &user_min_recv,
        &weth_reserves,
        &token_reserves,
    )
    .unwrap();

    assert_eq!(
        ctx,
        numeric::SandwichContext {
            revenue: U256::from(7),
            optimal_weth_in,
            user_amount_in,
            user_min_recv,
            reserve_state: numeric::ReserveState {
                a_reserves: weth_reserves,
                b_reserves: token_reserves,
            },
            frontrun_state: numeric::PoolState {
                variable: U256::from(285),
                new_a_reserves: U256::from(1400),
                new_b_reserves: U256::from(715),
            },
            target_state: numeric::PoolState {
                variable: U256::from(10),
                new_a_reserves: U256::from(1420),
                new_b_reserves: U256::from(705),
            },
            backrun_state: numeric::PoolState {
                variable: U256::from(407),
                new_a_reserves: U256::from(990),
                new_b_reserves: U256::from(1013),
            },
        }
    );
}

#[test]
fn test_calculate_sandwich_context_optimal() {
    // Setup params
    let user_amount_in = U256::from(20);
    let user_min_recv = U256::from(10);
    let weth_reserves = U256::from(1000);
    let token_reserves = U256::from(1000);

    // Get optimal amount in
    let optimal_weth_in = numeric::calculate_sandwich_optimal_in(
        &user_amount_in,
        &user_min_recv,
        &weth_reserves,
        &token_reserves,
    );
    assert_eq!(optimal_weth_in, U256::from(402));

    // Construct the Sandwich Context
    let ctx = numeric::calculate_sandwich_context(
        &optimal_weth_in,
        &user_amount_in,
        &user_min_recv,
        &weth_reserves,
        &token_reserves,
    )
    .unwrap();

    assert_eq!(
        ctx,
        numeric::SandwichContext {
            revenue: U256::from(7),
            optimal_weth_in,
            user_amount_in,
            user_min_recv,
            reserve_state: numeric::ReserveState {
                a_reserves: weth_reserves,
                b_reserves: token_reserves,
            },
            frontrun_state: numeric::PoolState {
                variable: U256::from(286),
                new_a_reserves: U256::from(1402),
                new_b_reserves: U256::from(714),
            },
            target_state: numeric::PoolState {
                variable: U256::from(10),
                new_a_reserves: U256::from(1422),
                new_b_reserves: U256::from(704),
            },
            backrun_state: numeric::PoolState {
                variable: U256::from(409),
                new_a_reserves: U256::from(990),
                new_b_reserves: U256::from(1013),
            },
        }
    );
}
