//! A transaction hook module

use ethers::prelude::*;
use serde::{Deserialize, Serialize};

/// A transaction filter
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    /// Event logs to watch
    Event(H256),
    /// Addresses to watch to
    To(H160),
    /// Addresses to watch from
    From(H160),
    /// Regex Data Filters
    /// Example: `%1337%` will match any transaction with `1337` in the data
    Data(String),
}

impl Filter {
    /// Check if the filter matches
    pub fn matches(&self, _tx: &Transaction) -> bool {
        false
    }
}
