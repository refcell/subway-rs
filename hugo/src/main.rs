//! The main entrypoint for hugo.

use clap::Parser;
use eyre::Result;
use std::time::Duration;

/// The tui components
pub mod components;

/// The app module
pub mod app;

#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "crossterm")]
use crate::crossterm::run;

mod termion;
use crate::termion::run;

pub mod ui;

pub mod list;


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
    /// Just display the raw logs, no TUI
    #[clap(long, short, default_value = "false")]
    without_tui: bool,
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

    /// Returns if the tui is disabled.
    pub fn get_without_tui(&self) -> bool {
        self.without_tui
    }
}

fn main() -> Result<()> {
    // Parse command args
    let cli = Hugo::parse();

    // Get the tick rate
    let tick_rate: Duration = cli.get_tick_rate();

    // Get the enhanced graphics flag
    let enhanced_graphics: bool = cli.get_enhanced_graphics();

    // Match on if tui is enabled
    match cli.get_without_tui() {
        true => {
            // TODO: Run Hugo without a TUI
        }
        false => {
            // Run the app
            run("Hugo", tick_rate, enhanced_graphics)?;
        }
    }

    // End ok
    Ok(())
}