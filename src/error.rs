use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseConfigFailed(#[from]config::ConfigError),

    #[error("Invalid command provided. Please use the --help flag to list all possible commands.")]
    InvalidCommand,
}