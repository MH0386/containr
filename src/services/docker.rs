//! Docker service integration for managing containers, images, and volumes.
//!
//! This module provides a high-level interface to the Docker Engine API using the Bollard library.
//! It handles all Docker operations including listing, starting, and stopping containers.

use anyhow::Result;
use bollard::container::{ListContainersOptions, StartContainerOptions, StopContainerOptions};
use bollard::image::ListImagesOptions;
use bollard::volume::ListVolumesOptions;
use bollard::Docker;

/// Represents the runtime state of a Docker container.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    /// Container is currently running and active
    Running,
    /// Container is stopped or paused
    Stopped,
}

impl ContainerState {
    /// Returns a human-readable label for the container state.
    pub fn label(&self) -> &'static str {
        match self {
            ContainerState::Running => "Running",
            ContainerState::Stopped => "Stopped",
        }
    }

    /// Returns the CSS class name for styling this container state.
    pub fn css_class(&self) -> &'static str {
        match self {
            ContainerState::Running => "running",
            ContainerState::Stopped => "stopped",
        }
    }

    /// Returns the action button label for toggling this container state.
    pub fn action_label(&self) -> &'static str {
        match self {
            ContainerState::Running => "Stop",
            ContainerState::Stopped => "Start",
        }
    }
}

/// Information about a Docker container including its configuration and runtime state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerInfo {
    /// Unique container ID (typically first 12 characters of the full ID)
    pub id: String,
    /// Human-readable container name (without leading slash)
    pub name: String,
    /// Docker image name that the container was created from
    pub image: String,
    /// Detailed status description from Docker Engine
    pub status: String,
    /// Port mappings in format "host:container" or empty if none
    pub ports: String,
    /// Current runtime state (Running or Stopped)
    pub state: ContainerState,
}

/// Information about a Docker image stored locally.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    /// Unique image ID (typically first 12 characters)
    pub id: String,
    /// Image repository name (e.g., "nginx", "alpine")
    pub repository: String,
    /// Image tag (e.g., "latest", "1.0.0")
    pub tag: String,
    /// Human-readable size of the image (e.g., "125MB")
    pub size: String,
}

/// Information about a Docker volume for persistent data storage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VolumeInfo {
    /// Unique volume name
    pub name: String,
    /// Storage driver used (typically "local")
    pub driver: String,
    /// Filesystem path where the volume is mounted
    pub mountpoint: String,
    /// Human-readable size of the volume
    pub size: String,
}

/// Service for interacting with the Docker Engine API.
///
/// This service uses the Bollard library to communicate with Docker and provides
/// high-level operations for managing containers, images, and volumes.
#[derive(Clone)]
pub struct DockerService {
    docker: Docker,
}

impl DockerService {
    /// Creates a new Docker service by connecting to the local Docker daemon.
    ///
    /// This uses the default connection method which will try:
    /// 1. Unix socket at /var/run/docker.sock (Linux/Mac)
    /// 2. Named pipe (Windows)
    /// 3. DOCKER_HOST environment variable if set
    ///
    /// # Errors
    ///
    /// Returns an error if Docker is not running or connection fails.
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker })
    }

    /// Lists all Docker containers (both running and stopped).
    ///
    /// # Returns
    ///
    /// A vector of `ContainerInfo` with details about each container.
    ///
    /// # Errors
    ///
    /// Returns an error if the Docker API call fails.
    pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
        let options = Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        });

        let containers = self.docker.list_containers(options).await?;

        let container_infos = containers
            .into_iter()
            .map(|container| {
                let id = container
                    .id
                    .as_ref()
                    .map(|s| s.chars().take(12).collect())
                    .unwrap_or_else(|| "unknown".to_string());

                let name = container
                    .names
                    .as_ref()
                    .and_then(|names| names.first())
                    .map(|n| n.trim_start_matches('/').to_string())
                    .unwrap_or_else(|| "unnamed".to_string());

                let image = container.image.unwrap_or_else(|| "unknown".to_string());

                let status = container.status.unwrap_or_else(|| "unknown".to_string());

                let ports = if let Some(ports) = container.ports {
                    if ports.is_empty() {
                        "--".to_string()
                    } else {
                        ports
                            .iter()
                            .filter_map(|p| match (p.public_port, p.private_port) {
                                (Some(pub_port), priv_port) => {
                                    Some(format!("{}:{}", pub_port, priv_port))
                                }
                                (None, priv_port) => Some(format!("{}", priv_port)),
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                } else {
                    "--".to_string()
                };

                let state = if let Some(st) = container.state {
                    if st == "running" {
                        ContainerState::Running
                    } else {
                        ContainerState::Stopped
                    }
                } else {
                    ContainerState::Stopped
                };

                ContainerInfo {
                    id,
                    name,
                    image,
                    status,
                    ports,
                    state,
                }
            })
            .collect();

        Ok(container_infos)
    }

    /// Lists all Docker images stored locally.
    ///
    /// # Returns
    ///
    /// A vector of `ImageInfo` with repository, tag, ID, and size information.
    ///
    /// # Errors
    ///
    /// Returns an error if the Docker API call fails.
    pub async fn list_images(&self) -> Result<Vec<ImageInfo>> {
        let options = Some(ListImagesOptions::<String> {
            all: false,
            ..Default::default()
        });

        let images = self.docker.list_images(options).await?;

        let image_infos = images
            .into_iter()
            .map(|image| {
                let id = image.id;

                // Parse repository and tag from repo_tags (Vec<String>)
                let (repository, tag) = if let Some(first) = image.repo_tags.first() {
                    let parts: Vec<&str> = first.split(':').collect();
                    let repo = parts.first().unwrap_or(&"<none>").to_string();
                    let tag_part = parts.get(1).unwrap_or(&"<none>").to_string();
                    (repo, tag_part)
                } else {
                    ("<none>".to_string(), "<none>".to_string())
                };

                // Format size directly (it's i64, not Option<i64>)
                let size = format_size(image.size);

                ImageInfo {
                    id,
                    repository,
                    tag,
                    size,
                }
            })
            .collect();

        Ok(image_infos)
    }

    /// Lists all Docker volumes for persistent data storage.
    ///
    /// # Returns
    ///
    /// A vector of `VolumeInfo` with name, driver, and mount point information.
    ///
    /// # Errors
    ///
    /// Returns an error if the Docker API call fails.
    pub async fn list_volumes(&self) -> Result<Vec<VolumeInfo>> {
        let options = ListVolumesOptions::<String> {
            ..Default::default()
        };

        let volumes_response = self.docker.list_volumes(Some(options)).await?;

        let volume_infos = volumes_response
            .volumes
            .unwrap_or_default()
            .into_iter()
            .map(|volume| {
                let name = volume.name;
                let driver = volume.driver;
                let mountpoint = volume.mountpoint;
                // Note: Size is not directly available from Docker API without additional inspection
                let size = "--".to_string();

                VolumeInfo {
                    name,
                    driver,
                    mountpoint,
                    size,
                }
            })
            .collect();

        Ok(volume_infos)
    }

    /// Starts a stopped Docker container.
    ///
    /// # Arguments
    ///
    /// * `id` - The container ID or name to start
    ///
    /// # Errors
    ///
    /// Returns an error if the container doesn't exist or cannot be started.
    pub async fn start_container(&self, id: &str) -> Result<()> {
        self.docker
            .start_container(id, None::<StartContainerOptions<String>>)
            .await?;
        Ok(())
    }

    /// Stops a running Docker container.
    ///
    /// # Arguments
    ///
    /// * `id` - The container ID or name to stop
    ///
    /// # Errors
    ///
    /// Returns an error if the container doesn't exist or cannot be stopped.
    pub async fn stop_container(&self, id: &str) -> Result<()> {
        self.docker
            .stop_container(id, None::<StopContainerOptions>)
            .await?;
        Ok(())
    }
}

fn format_size(size: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if size >= GB {
        format!("{:.1}GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.1}MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.1}KB", size as f64 / KB as f64)
    } else {
        format!("{}B", size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_state_labels_match() {
        assert_eq!(ContainerState::Running.label(), "Running");
        assert_eq!(ContainerState::Stopped.label(), "Stopped");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(100), "100B");
        assert_eq!(format_size(1024), "1.0KB");
        assert_eq!(format_size(1048576), "1.0MB");
        assert_eq!(format_size(1073741824), "1.0GB");
    }
}
