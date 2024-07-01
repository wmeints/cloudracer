use std::path::PathBuf;
use std::{collections::HashMap, process::Command};

use cloudracer::{
    config::{SolutionConfig, SolutionLocations},
    containers,
    manifest::DeploymentManifest,
};

#[test]
fn test_can_build_containers() {
    let apphost_path = PathBuf::from("playground/Playground.AppHost/Playground.AppHost.csproj");
    let manifest = DeploymentManifest::generate(&apphost_path).unwrap();

    let config = SolutionConfig {
        host: apphost_path.clone(),
        repository: "samplerepo.io".to_string(),
        resources: Some(HashMap::new()),
    };

    let locations = SolutionLocations::new(&config).unwrap();
    let image_version = "1".to_string();

    containers::build(&config, &locations, &manifest, &image_version).unwrap();

    assert!(locate_image("samplerepo.io/frontend:1".to_string()));
}

fn locate_image(image_tag: String) -> bool {
    let status = Command::new("docker")
        .arg("inspect")
        .arg(image_tag)
        .status()
        .unwrap();

    status.success()
}
