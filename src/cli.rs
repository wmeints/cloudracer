//! Defines the CLI for the cloudracer tool.

use clap::{Parser, Subcommand};

/// Defines the CLI for the application.
#[derive(Parser)]
pub struct Cli {
    /// The path to the solution config file.
    #[arg(long)]
    pub config: String,

    /// The command that is to be executed.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Lists all possible commands in the application.
#[derive(Subcommand)]
pub enum Commands {
    /// Builds and pushes the containers in the Aspire solution.
    Build {
        /// The image version to build
        version: String,
    },

    /// Pushes the built containers in the Aspire solution.
    Push,

    /// Provisions the apps from the Aspire solution in the container app environment.
    Provision,
}
