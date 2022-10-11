#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

/// A Relayer
pub mod relayer;

/// Re-export a prelude
pub mod prelude {
    pub use super::{relayer::*};
}
