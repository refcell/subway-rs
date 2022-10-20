//! A relayer module

use ethers::prelude::*;
use ethers_flashbots::BundleRequest;

/// Helper function to help catch the various ways errors can be thrown from simulation
/// This helper function is needed as simulation response has many ways where the
/// error can be thrown.... which is not documented
// pub fn check_simulated_response(sim: &SimulatedBundle) -> eyre::Result<()> {
//     // TODO::::
//     Ok(())
// }

/// Construct a Bundle Request for FlashBots
pub fn construct_bundle(
    signed_transactions: &Vec<Bytes>,
    block_number: U64,
) -> eyre::Result<BundleRequest> {
    // Create the ethers-flashbots bundle request
    let mut bundle_request = BundleRequest::new();

    // Sign the transactions and add to the bundle
    for tx in signed_transactions {
        bundle_request = bundle_request.push_transaction(tx.clone());
    }

    // Set other bundle parameters
    bundle_request = bundle_request
        .set_block(block_number + 1)
        .set_simulation_block(block_number)
        .set_simulation_timestamp(0);

    // Return the constructed bundle request
    Ok(bundle_request)
}

// export const sanityCheckSimulationResponse = (sim) => {
//   // Contains first revert
//   if (sim.firstRevert) {
//     throw new Error(sim.firstRevert.revert);
//   }

//   // Contains first revert
//   if (sim.firstRevert) {
//     throw new Error(sim.firstRevert.revert);
//   }

//   // Simulation error type
//   const simE = sim;
//   if (simE.error) {
//     throw new Error(simE.error.message);
//   }

//   // Another type of silent error
//   // This has to be checked last
//   const errors = sim.results
//     .filter((x) => x.error !== undefined)
//     .map((x) => x.error + " " + (x.revert || ""));
//   if (errors.length > 0) {
//     throw new Error(errors.join(", "));
//   }

//   return sim;
// };
