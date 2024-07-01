//! Defines the configuration for the cloudracer tool.
//!
//! The configuration contains settings used by the tool to build and push containers,
//! and provision apps in your azure container app environment.

use crate::error::{Error, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::path::PathBuf;

/// Defines the well-known locations for the solution.
pub struct SolutionLocations {
    pub root: PathBuf,
    pub host: PathBuf,
}

impl SolutionLocations {
    /// Create a new set of well-known locations for the solution configuration.
    pub fn new(config: &SolutionConfig) -> Result<SolutionLocations> {
        let root_dir = current_dir().map_err(|_| Error::DetermineSolutionLocationsFailure)?;

        // Perform a fallback to the root of the solution if we can't resolve the host dir.
        // This should never happen, but you never know.
        let host_dir: PathBuf = config
            .host
            .parent()
            .map(|p| p.into())
            .unwrap_or(root_dir.clone());

        Ok(SolutionLocations {
            root: root_dir,
            host: host_dir,
        })
    }
}

/// Defines the configuration for the cloudracer tool.
#[derive(Deserialize, Debug)]
pub struct SolutionConfig {
    /// Path to the host project.
    pub host: PathBuf,

    /// The name of the container repository
    pub repository: String,

    /// Configuration for scaling components
    pub resources: Option<HashMap<String, ResourceConfig>>,
}

#[derive(Deserialize, Debug)]
pub struct ResourceConfig {
    /// Amount of CPU to assign to the container app
    pub cpu: String,

    /// Amount of memory to assign to the container app
    pub memory: String,
}

impl SolutionConfig {
    /// Loads the configuration from disk.
    ///
    /// The configuration provides extra metadata for the cloudracer tool to act upon.
    /// For example, you can configure resource settings.
    pub fn load(filename: String) -> Result<Self> {
        let config_file = File::open(filename).map_err(|e| e.into())?;

        let config_data: SolutionConfig =
            serde_yaml::from_reader(config_file).map_err(|e| e.into())?;

        Ok(config_data)
    }
}

impl Into<Error> for serde_yaml::Error {
    fn into(self) -> Error {
        Error::ConfigFileParseFailure(self)
    }
}

impl Into<Error> for std::io::Error {
    fn into(self) -> Error {
        Error::ConfigFileReadFailure(self)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use tempfile::NamedTempFile;
    #[test]
    fn can_parse_config() {
        let mut config_file = NamedTempFile::new().unwrap();
        let file_path = config_file.path().to_string_lossy().into_owned();

        let content = r#"
        host: ./host/test/test.csproj
        repository: acrsmartassist.azurecr.io
        resources:
            test:
                cpu: "1"
                memory: "1Gi"
        "#;

        config_file.write_all(content.as_bytes()).unwrap();

        SolutionConfig::load(file_path).unwrap();
    }
}
