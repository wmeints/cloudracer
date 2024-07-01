//! Defines the deployment manifest that's used by the cloudracer tool.
//!
//! The manifest format is derived from https://learn.microsoft.com/en-us/dotnet/aspire/deployment/manifest-format.
//! There isn't an official scheme for the manifest, so this code may stop to work when Microsoft publishes new
//! versions of the Aspire tooling.

use crate::error::{Error, Result};
use serde::Deserialize;
use std::{collections::HashMap, fs::File, path::PathBuf, process::Command};

/// Defines the deployment manifest.
#[derive(Deserialize)]
pub struct DeploymentManifest {
    /// A map containing all resources in the deployment manifest.
    pub resources: HashMap<String, Resource>,
}

impl DeploymentManifest {
    /// Generates a deployment manifest from an Aspire host project.
    ///
    /// This function runs the .NET Aspire host project with the manifest publisher to write
    /// a deployment manifest to disk. The deployment manifest is loaded afterwards for further processing.
    pub fn generate(project_file: &PathBuf) -> Result<DeploymentManifest> {
        Command::new("dotnet")
            .arg("run")
            .arg("--project")
            .arg(project_file)
            .arg("--publisher")
            .arg("manifest")
            .arg("--output-path")
            .arg("deployment-manifest.json")
            .status()
            .map_err(|e| Error::DeploymentManifestGenerationFailure(e))?;

        let manifest_path = project_file
            .parent()
            .map(|p| p.join("deployment-manifest.json"))
            .unwrap();

        DeploymentManifest::load(manifest_path)
    }

    /// Loads an existing deployment manifest from disk.
    fn load(filename: PathBuf) -> Result<DeploymentManifest> {
        let manifest_file =
            File::open(filename).map_err(|e| Error::ReadDeploymentManifestFailure(e))?;

        let manifest: DeploymentManifest = serde_json::from_reader(manifest_file)
            .map_err(|e| Error::ParseDeploymentManifestFailure(e))?;

        Ok(manifest)
    }
}

/// Defines a component in the deployment manifest.
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Resource {
    /// A component that's published as a dockerfile
    #[serde(rename = "dockerfile.v0")]
    Dockerfile {
        /// The env map contains environment variables. This may contain placeholder strings.
        env: HashMap<String, String>,

        /// The service bindings for the container
        bindings: Option<HashMap<String, Binding>>,

        /// The path to the docker file relative to the context.
        path: String,

        /// The path to use as the context when building the container image.
        context: String,
    },

    /// Components that aren't supported by the tool.
    #[serde(other)]
    Uknown,
}

/// Defines a service binding for a component.
#[derive(Deserialize)]
pub struct Binding {
    /// The scheme (tcp/udp/http/https)
    pub scheme: String,

    /// The protocol (tcp/udp)
    pub protocol: String,

    /// The transport type (http/http2)
    pub transport: String,

    /// The target port on the container
    #[serde(rename = "containerPort")]
    pub container_port: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize_valid_dockerfile_resources() {
        let content = r#"
        {
            "resources": {
                "test1": {
                    "type": "Dockerfile.v0",
                    "context": "../apps/test",
                    "path": "Dockerfile",
                    "env": {},
                    "bindings": {
                        "http": {
                            "scheme": "http",
                            "protocol": "tcp",
                            "transport": "http",
                            "containerPort": 5000
                        }
                    }
                }
            }
        }
        "#;

        let _manifest: DeploymentManifest = serde_json::from_str(content).unwrap();
    }

    #[test]
    fn can_deserialize_unknown_types() {
        let content = r#"
        {
            "resources": {
                "test": {
                    "type": "value.v0"
                }
            }
        }"#;

        let _manifest: DeploymentManifest = serde_json::from_str(content).unwrap();
    }
}
