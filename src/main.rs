use clap::Parser;
use cloudracer::cli;
use cloudracer::config;
use cloudracer::error::{Error, Result};
use std::process::ExitCode;
use tracing::{error, info};

fn main() -> ExitCode {
    match run_tool() {
        Ok(..) => ExitCode::from(0x0),
        Err(err) => {
            error!("FAILURE - {}", err);
            ExitCode::from(0x1)
        }
    }
}

fn run_tool() -> Result<()> {
    setup_tracing();

    let arguments = cli::Cli::parse();
    let config = config::SolutionConfig::load()?;

    match &arguments.command {
        Some(cli::Commands::Build) => {
            info!("Building and pushing containers")
        }
        Some(cli::Commands::Push) => {
            info!("Pushing containers")
        }
        Some(cli::Commands::Provision) => {
            info!("Provisioning apps in the container app environment")
        }
        _ => return Err(Error::InvalidCommand),
    }

    Ok(())
}

fn setup_tracing() {
    tracing_subscriber::fmt::init();
}
