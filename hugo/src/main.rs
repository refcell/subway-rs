//! The main entrypoint for hugo.

use clap::Parser;
use eyre::Result;
use std::time::Duration;

/// The cli module.
pub mod cli;
pub use cli::*;

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

fn main() -> Result<()> {
    // Parse command args
    let cli = Hugo::parse();

    // Get the tick rate
    let tick_rate: Duration = cli.get_tick_rate();

    // Get the enhanced graphics flag
    let enhanced_graphics: bool = cli.get_enhanced_graphics();

    // Run the app
    run("Hugo", tick_rate, enhanced_graphics)?;

    // End ok
    Ok(())
}