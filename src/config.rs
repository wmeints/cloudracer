//! Defines the configuration for the cloudracer tool.
//! 
//! The configuration contains settings used by the tool to build and push containers, 
//! and provision apps in your azure container app environment.

use serde::Deserialize;
use std::path::PathBuf;
use crate::error::Result;
use config::{File, Environment, Config, FileFormat};

/// Defines the configuration for the cloudracer tool.
#[derive(Deserialize)]
pub struct SolutionConfig {
    /// Path to the host project.
    pub host: PathBuf,
}

impl SolutionConfig {
    /// Loads the configuration from disk.
    /// 
    /// The configuration provides extra metadata for the cloudracer tool to act upon.
    /// For example, you can configure scaling settings, and secrets in the configuration file.
    /// 
    /// Optionally, use can provide APP_ prefixed environment variables for things that you don't
    /// want to store in the configuration file for the tool.
    pub fn load() -> Result<Self> {
        let config = Config::builder()
        .add_source(File::with_name("cloudracer").format(FileFormat::Yaml))
        .add_source(Environment::with_prefix("APP"))
        .build()?;

        let config_data: SolutionConfig = config.try_deserialize()?;

        Ok(config_data)
    }
}