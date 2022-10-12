#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

/// A Relayer
pub mod relayer;

/// A Hook to be called on new transactions
pub mod hook;

/// Common Utilities
pub mod utils;

/// Re-export a prelude
pub mod prelude {
    pub use super::{hook::*, relayer::*, utils::*};
}
