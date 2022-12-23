//! A command line interface for kwhue
//!

pub mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all lights
    List {},

    /// Toggle light with id
    Toggle {
        /// Light ID
        id: u32,
    },
}
