
use std::time::Duration;

use clap::Parser;

/// Hugo is a fast, modular and extensible TUI for [subway_rs](https://crates.io/crates/subway_rs).
#[derive(Debug, Parser)]
#[clap(name = "hugo", version = "v0.1.0")]
pub struct Hugo {
    /// Time in ms between two ticks.
    #[clap(long, short, default_value = "250")]
    tick_rate: u64,
    /// Whether unicode symbols are used to improve the overall look of the app
    #[clap(long, short, default_value = "true")]
    enhanced_graphics: bool,
}

impl Hugo {
    /// Returns the tick rate as a `Duration`.
    pub fn get_tick_rate(&self) -> Duration {
        Duration::from_millis(self.tick_rate)
    }

    /// Returns the enhanced graphics flag.
    pub fn get_enhanced_graphics(&self) -> bool {
        self.enhanced_graphics
    }
}