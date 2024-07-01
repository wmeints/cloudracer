use std::backtrace::Backtrace;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid command provided. Please use the --help flag to list all possible commands.")]
    InvalidCommand,

    #[error("failed to parse solution configuration")]
    ConfigFileParseFailure(serde_yaml::Error),

    #[error("failed to open solution configuration file")]
    ConfigFileReadFailure(std::io::Error),

    #[error("container build failed: {details}")]
    ContainerImageBuildFailed { details: String },

    #[error("failed to read configuration file")]
    ReadConfigFileFailure(std::io::Error),

    #[error("failed to determine solution locations")]
    DetermineSolutionLocationsFailure,

    #[error("failed to generate deployment manifest")]
    DeploymentManifestGenerationFailure(std::io::Error),

    #[error("failed to read deployment manifest")]
    ReadDeploymentManifestFailure(std::io::Error),

    #[error("failed to parse deployment manifest")]
    ParseDeploymentManifestFailure(serde_json::Error),
}
