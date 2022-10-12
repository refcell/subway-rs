//! A transaction hook module

use serde::{Deserialize, Serialize};
use ethers::prelude::*;

/// A transaction filter
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn matches(&self, tx: &Transaction) -> bool {

        false
    }
}