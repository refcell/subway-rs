//! A relayer module

use ethers::prelude::*;
use ethers_flashbots::{BundleRequest, BundleTransaction, SimulatedBundle};

/// Helper function to help catch the various ways errors can be thrown from simulation
/// This helper function is needed as simulation response has many ways where the
/// error can be thrown.... which is not documented
pub fn validate_simulation_response(sim: &SimulatedBundle) -> eyre::Result<()> {
    // Make sure no simulated bundle transactions have errors or reverts
    for tx in &sim.transactions {
        if let Some(e) = &tx.error {
            eyre::bail!("Error in bundled transaction: {:?}", e);
        }
        if let Some(r) = &tx.revert {
            eyre::bail!("Transaction reverts: {:?}", r);
        }
    }
    Ok(())
}

/// Construct a Bundle Request for FlashBots
pub fn construct_bundle<T: Into<BundleTransaction>>(
    signed_transactions: Vec<T>,
    block_number: U64,
) -> eyre::Result<BundleRequest> {
    // Create the ethers-flashbots bundle request
    let mut bundle_request = BundleRequest::new();

    // Sign the transactions and add to the bundle
    for tx in signed_transactions {
        let bundled: BundleTransaction = tx.into();
        bundle_request = bundle_request.push_transaction(bundled);
    }

    // Set other bundle parameters
    bundle_request = bundle_request
        .set_block(block_number + 1)
        .set_simulation_block(block_number)
        .set_simulation_timestamp(0);

    // Return the constructed bundle request
    Ok(bundle_request)
}
