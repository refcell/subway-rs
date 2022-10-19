#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

/// A banner
pub mod banner;

/// A Relayer
pub mod relayer;

/// Common Utilities
pub mod utils;

/// ABIs
pub mod abi;

/// Uniswap Utilities
pub mod uniswap;

/// Numeric Operations
pub mod numeric;

/// Telemetry
/// Uses [tracing](https://crates.io/crates/tracing) and [tracing-subscriber](https://crates.io/crates/tracing-subscriber)
pub mod telemetry;

/// Re-export a prelude
pub mod prelude {
    pub use super::{abi::*, banner::*, numeric::*, relayer::*, uniswap::*, utils::*};
}
