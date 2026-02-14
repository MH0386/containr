//! Docker API integration service using Bollard.
//!
//! This module provides a high-level wrapper around the Bollard Docker client,
//! exposing operations for managing containers, images, and volumes.

use anyhow::Result;
use bollard::container::{ListContainersOptions, StartContainerOptions, StopContainerOptions};
use bollard::image::ListImagesOptions;
use bollard::volume::ListVolumesOptions;
use bollard::Docker;

/// Represents the runtime state of a Docker container.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    /// Container is currently running
    Running,
    /// Container is stopped
    Stopped,
}

impl ContainerState {
    /// Returns a human-readable label for the state.
    pub fn label(&self) -> &'static str {
        match self {
            ContainerState::Running => "Running",
            ContainerState::Stopped => "Stopped",
        }
    }

    /// Returns the CSS class name for styling this state.
    pub fn css_class(&self) -> &'static str {
        match self {
            ContainerState::Running => "running",
            ContainerState::Stopped => "stopped",
        }
    }

    /// Returns the label for the action button (opposite of current state).
    pub fn action_label(&self) -> &'static str {
        match self {
            ContainerState::Running => "Stop",
            ContainerState::Stopped => "Start",
        }
    }
}

/// Information about a Docker container.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerInfo {
    /// Short container ID (first 12 characters)
    pub id: String,
    /// Container name (without leading `/`)
    pub name: String,
    /// Image name the container is based on
    pub image: String,
    /// Human-readable status string
    pub status: String,
    /// Formatted port mappings (e.g., "8080:80, 443:443")
    pub ports: String,
    /// Current runtime state
    pub state: ContainerState,
}

/// Information about a Docker image.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    /// Image ID (SHA256 hash)
    pub id: String,
    /// Repository name (e.g., "nginx", "myorg/myapp")
    pub repository: String,
    /// Image tag (e.g., "latest", "1.0.0")
    pub tag: String,
    /// Human-readable size (e.g., "150MB")
    pub size: String,
}

/// Information about a Docker volume.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VolumeInfo {
    /// Volume name
    pub name: String,
    /// Volume driver (usually "local")
    pub driver: String,
    /// Host filesystem path where volume is mounted
    pub mountpoint: String,
    /// Volume size (not available from Docker API, shows "--")
    pub size: String,
}

/// Docker API client wrapper providing high-level operations.
///
/// Uses Bollard to communicate with the Docker daemon via Unix socket
/// (Linux/macOS) or named pipe (Windows).
#[derive(Clone)]
pub struct DockerService {
    docker: Docker,
}

impl DockerService {
    /// Creates a new Docker service by connecting to the local Docker daemon.
    ///
    /// Connects using platform defaults unless `DOCKER_HOST` environment variable is set.
    ///
    /// # Errors
    ///
    /// Returns an error if the Docker daemon is not accessible or if there are
    /// permission issues with the Docker socket.
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker })
    }

    /// Lists all Docker containers (both running and stopped).
    ///
    /// # Errors
    ///
    /// Returns an error if the Docker API request fails.
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

    /// Lists all tagged Docker images on the local system.
    ///
    /// # Errors
    ///
    /// Returns an error if the Docker API request fails.
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

    /// Lists all Docker volumes.
    ///
    /// # Errors
    ///
    /// Returns an error if the Docker API request fails.
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

    /// Starts a stopped container by ID or name.
    ///
    /// # Errors
    ///
    /// Returns an error if the container doesn't exist or is already running.
    pub async fn start_container(&self, id: &str) -> Result<()> {
        self.docker
            .start_container(id, None::<StartContainerOptions<String>>)
            .await?;
        Ok(())
    }

    /// Stops a running container by ID or name.
    ///
    /// # Errors
    ///
    /// Returns an error if the container doesn't exist or is already stopped.
    pub async fn stop_container(&self, id: &str) -> Result<()> {
        self.docker
            .stop_container(id, None::<StopContainerOptions>)
            .await?;
        Ok(())
    }
}

/// Formats a byte size into a human-readable string (B, KB, MB, GB).
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
