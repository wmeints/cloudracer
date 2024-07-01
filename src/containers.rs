use std::path::PathBuf;
use std::process::Command;

use crate::config::{SolutionConfig, SolutionLocations};
use crate::error::{Error, Result};
use crate::manifest::{DeploymentManifest, Resource};

/// Builds the deployable units for all projects configured in the application host project.
pub fn build(
    config: &SolutionConfig,
    locations: &SolutionLocations,
    manifest: &DeploymentManifest,
    image_version: &String,
) -> Result<()> {
    manifest
        .resources
        .keys()
        .into_iter()
        .map(|resource_name| {
            let resource = manifest.resources.get(resource_name).unwrap();

            match resource {
                Resource::Dockerfile { context, path, .. } => {
                    // Resolve the absolute path to the context directory relative to the host project.
                    // We need to do this from the host directory as all the locations mentioned in the manifest are relative
                    // to where the manifest is located. The manifest is located in the app host project directory.
                    let context_path = std::path::absolute(locations.host.join(context)).unwrap();

                    build_container(
                        config,
                        locations.host.clone(),
                        path.clone(),
                        context_path,
                        resource_name.clone(),
                        image_version.clone(),
                    )
                }
                _ => Ok(()),
            }
        })
        .collect()
}

fn build_container(
    config: &SolutionConfig,
    root_dir: PathBuf,
    docker_file: String,
    context_path: PathBuf,
    app_name: String,
    image_version: String,
) -> Result<()> {
    let image_tag = format!(
        "{}/{}:{}",
        config.repository.to_string(),
        app_name,
        image_version
    );

    let status = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(image_tag)
        .arg("-f")
        .arg(docker_file)
        .arg(context_path)
        .current_dir(root_dir)
        .status()
        .map_err(|e| Error::ContainerImageBuildFailed {
            details: "Can't run docker executable".to_string(),
        })?;

    if !status.success() {
        return Err(Error::ContainerImageBuildFailed {
            details: "docker build command failed".to_string(),
        });
    }

    Ok(())
}
