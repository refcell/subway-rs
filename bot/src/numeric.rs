//! Numerical Module for Calculations

use ethers::{prelude::*, utils::parse_ether};
use serde::{Deserialize, Serialize};

use crate::prelude::get_univ2_data_given_in;

/// Calculate the max sandwich amount
pub fn calculate_sandwich_optimal_in(
    user_amount_in: &U256,
    user_min_recv_token: &U256,
    weth_reserves: &U256,
    token_reserves: &U256,
) -> U256 {
    // Set our search parameters
    let lower_bound = U256::zero();
    let upper_bound = parse_ether("100").unwrap();

    let calculation = move |amount_in: U256| -> U256 {
        let (_, new_a_reserves, new_b_reserves) =
            get_univ2_data_given_in(&amount_in, weth_reserves, token_reserves);
        let (amount_out, _, _) =
            get_univ2_data_given_in(user_amount_in, &new_a_reserves, &new_b_reserves);
        amount_out
    };

    let conditional = move |amount_out: U256| -> bool { amount_out >= *user_min_recv_token };

    // Binary Search for the optimal swap amount
    binary_search(lower_bound, upper_bound, calculation, conditional, None)
}

/// A Reserve State
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReserveState {
    /// The amount of token a reserves
    pub a_reserves: U256,
    /// The amount of token b reserves
    pub b_reserves: U256,
}

/// The Frontrun State
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PoolState {
    /// Variable
    pub variable: U256,
    /// New token a reserves
    pub new_a_reserves: U256,
    /// New token b reserves
    pub new_b_reserves: U256,
}

impl From<(U256, U256, U256)> for PoolState {
    fn from((variable, new_a_reserves, new_b_reserves): (U256, U256, U256)) -> Self {
        Self {
            variable,
            new_a_reserves,
            new_b_reserves,
        }
    }
}

/// A Sandwich Context
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SandwichContext {
    /// The Revenue
    pub revenue: U256,
    /// The optimal sandwich weth input
    pub optimal_weth_in: U256,
    /// The user input amount
    pub user_amount_in: U256,
    /// The minimum amount the user wants to receive
    pub user_min_recv: U256,
    /// The reserve state
    pub reserve_state: ReserveState,
    /// The frontrun state
    pub frontrun_state: PoolState,
    /// The target state
    pub target_state: PoolState,
    /// The backrun state
    pub backrun_state: PoolState,
}

/// Calculates the sandwich states to achieve a sandwich
pub fn calculate_sandwich_context(
    optimal_weth_in: &U256,
    user_amount_in: &U256,
    user_min_recv: &U256,
    weth_reserves: &U256,
    token_reserves: &U256,
) -> eyre::Result<SandwichContext> {
    // Calculate the frontrun state
    let frontrun_state: PoolState =
        get_univ2_data_given_in(optimal_weth_in, weth_reserves, token_reserves).into();

    // Calculate the target state
    let target_state: PoolState = get_univ2_data_given_in(
        user_amount_in,
        &frontrun_state.new_a_reserves,
        &frontrun_state.new_b_reserves,
    )
    .into();

    // Calculate the backrun state
    let backrun_state: PoolState = get_univ2_data_given_in(
        &frontrun_state.variable,
        &target_state.new_a_reserves,
        &target_state.new_b_reserves,
    )
    .into();

    // Sanity check the minimum acceptable output
    if target_state.variable < *user_min_recv {
        return Err(eyre::eyre!(
            "The minimum acceptable output is less than the user minimum acceptable output"
        ));
    }

    // Return a constructed SandwichContext
    Ok(SandwichContext {
        revenue: backrun_state
            .variable
            .checked_sub(*optimal_weth_in)
            .unwrap_or(U256::zero()),
        optimal_weth_in: *optimal_weth_in,
        user_amount_in: *user_amount_in,
        user_min_recv: *user_min_recv,
        reserve_state: ReserveState {
            a_reserves: *weth_reserves,
            b_reserves: *token_reserves,
        },
        frontrun_state,
        target_state,
        backrun_state,
    })
}

/// Binary Search to find the optimal sandwichable amount
/// Using binary search here as the profit function isn't normally distributed
/// NOTE: Can likely speed this up by using quadratic search. (possibly even furth by using boundries as heuristics)
pub fn binary_search(
    lower_bound: U256,
    upper_bound: U256,
    calculation: impl FnOnce(U256) -> U256 + Copy,
    conditional: impl FnOnce(U256) -> bool + Copy,
    tolerance: Option<U256>,
) -> U256 {
    // Unwrap the tolerance
    // Tolerance should be provided in bips
    let base = U256::from(10_000u64);
    let tolerance = tolerance
        .map(|v| if v > base { base } else { v })
        .unwrap_or_else(|| U256::from(100u64));

    // The delta cannot be 0 or we will stack overflow
    let spread = upper_bound - lower_bound;
    let delta = (tolerance * ((upper_bound + lower_bound) / 2)) / base;
    let delta = if delta.is_zero() { U256::one() } else { delta };

    // Step
    if spread > delta {
        let mid = (upper_bound + lower_bound) / 2;
        let out = calculation(mid);

        // Number go up
        if conditional(out) {
            return binary_search(mid, upper_bound, calculation, conditional, Some(tolerance));
        }

        // Number go down
        return binary_search(lower_bound, mid, calculation, conditional, Some(tolerance));
    }

    // Return the mid
    (upper_bound + lower_bound) / 2
}
