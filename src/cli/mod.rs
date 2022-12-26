//! A command line interface for `kwhue`.
//!
//! Commands
//! --------
//! - user add
//! - light list
//! - light on [id]
//! - light off [id]
//! - light toggle [id]

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
    /// Admin commands
    #[command(subcommand)]
    Admin(AdminSubcommands),

    /// Light commands
    #[command(subcommand)]
    Light(LightSubcommands),
}

#[derive(Subcommand)]
pub enum AdminSubcommands {
    /// Initialise a Hue bridge
    Init {},

    /// Check Hue bridge status
    Check {},
}

#[derive(Subcommand)]
pub enum LightSubcommands {
    /// List all lights
    List {},

    /// Toggle light with id
    Toggle {
        /// The id of the light to toggle
        id: u32,
    },

    /// Light on with id
    On {
        /// The id of the light to toggle
        id: u32,
    },

    /// Light off with id
    Off {
        /// The id of the light to toggle
        id: u32,
    },
}
