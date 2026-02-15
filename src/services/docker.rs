//! Docker service integration module.
//!
//! This module provides integration with the Docker daemon using the Bollard library.
//! It defines data structures for Docker resources (containers, images, volumes) and
//! provides methods for common Docker operations.

use anyhow::Result;
use bollard::Docker;
use bollard::container::{ListContainersOptions, StartContainerOptions, StopContainerOptions};
use bollard::image::ListImagesOptions;
use bollard::volume::ListVolumesOptions;

/// Represents the state of a Docker container.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    /// Container is currently running
    Running,
    /// Container is stopped
    Stopped,
}

impl ContainerState {
    /// Returns the display label for the container state.
    ///
    /// # Returns
    /// - "Running" for `ContainerState::Running`
    /// - "Stopped" for `ContainerState::Stopped`
    pub fn label(&self) -> &'static str {
        match self {
            ContainerState::Running => "Running",
            ContainerState::Stopped => "Stopped",
        }
    }

    /// Returns the CSS class name for styling the container state.
    ///
    /// # Returns
    /// - "running" for `ContainerState::Running`
    /// - "stopped" for `ContainerState::Stopped`
    pub fn css_class(&self) -> &'static str {
        match self {
            ContainerState::Running => "running",
            ContainerState::Stopped => "stopped",
        }
    }

    /// Returns the label for the action button based on container state.
    ///
    /// # Returns
    /// - "Stop" for running containers
    /// - "Start" for stopped containers
    pub fn action_label(&self) -> &'static str {
        match self {
            ContainerState::Running => "Stop",
            ContainerState::Stopped => "Start",
        }
    }
}

/// Information about a Docker container.
///
/// Contains essential details about a container including its ID, name, image,
/// status, port mappings, and current state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerInfo {
    /// Short container ID (12 characters)
    pub id: String,
    /// Container name (without leading `/`)
    pub name: String,
    /// Image name used by the container
    pub image: String,
    /// Status string from Docker (e.g., "Up 2 hours")
    pub status: String,
    /// Formatted port mappings (e.g., "8080:80" or "--" if none)
    pub ports: String,
    /// Current state of the container
    pub state: ContainerState,
}

/// Information about a Docker image.
///
/// Contains details about a Docker image including its ID, repository, tag, and size.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    /// Full image ID
    pub id: String,
    /// Repository name
    pub repository: String,
    /// Image tag (version)
    pub tag: String,
    /// Formatted size (e.g., "125.3MB")
    pub size: String,
}

/// Information about a Docker volume.
///
/// Contains details about a Docker volume including its name, driver, and mount point.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VolumeInfo {
    /// Volume name
    pub name: String,
    /// Volume driver (usually "local")
    pub driver: String,
    /// Mount point path on the host
    pub mountpoint: String,
    /// Size information (currently not available from API, shows "--")
    pub size: String,
}

/// Service for interacting with the Docker daemon.
///
/// Provides methods for listing and managing Docker containers, images, and volumes.
/// Uses the Bollard library to communicate with the Docker daemon via Unix socket
/// or TCP connection.
#[derive(Clone)]
pub struct DockerService {
    docker: Docker,
}

impl DockerService {
    /// Creates a new DockerService instance.
    ///
    /// Attempts to connect to the Docker daemon using default connection methods:
    /// 1. `DOCKER_HOST` environment variable
    /// 2. Unix socket at `/var/run/docker.sock`
    /// 3. Named pipe on Windows
    ///
    /// # Returns
    /// - `Ok(DockerService)` if connection succeeds
    /// - `Err` if unable to connect to Docker daemon
    ///
    /// # Example
    /// ```no_run
    /// use doctainr::services::DockerService;
    ///
    /// let service = DockerService::new()?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker })
    }

    /// Lists all Docker containers (both running and stopped).
    ///
    /// # Returns
    /// - `Ok(Vec<ContainerInfo>)` with information about all containers
    /// - `Err` if the Docker API call fails
    ///
    /// # Example
    /// ```no_run
    /// # use doctainr::services::DockerService;
    /// # async fn example() -> anyhow::Result<()> {
    /// let service = DockerService::new()?;
    /// let containers = service.list_containers().await?;
    /// for container in containers {
    ///     println!("{}: {}", container.name, container.state.label());
    /// }
    /// # Ok(())
    /// # }
    /// ```
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

    /// Lists all Docker images on the local system.
    ///
    /// # Returns
    /// - `Ok(Vec<ImageInfo>)` with information about all images
    /// - `Err` if the Docker API call fails
    ///
    /// # Example
    /// ```no_run
    /// # use doctainr::services::DockerService;
    /// # async fn example() -> anyhow::Result<()> {
    /// let service = DockerService::new()?;
    /// let images = service.list_images().await?;
    /// for image in images {
    ///     println!("{}: {} ({})", image.repository, image.tag, image.size);
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
    /// # Returns
    /// - `Ok(Vec<VolumeInfo>)` with information about all volumes
    /// - `Err` if the Docker API call fails
    ///
    /// # Example
    /// ```no_run
    /// # use doctainr::services::DockerService;
    /// # async fn example() -> anyhow::Result<()> {
    /// let service = DockerService::new()?;
    /// let volumes = service.list_volumes().await?;
    /// for volume in volumes {
    ///     println!("{}: {} ({})", volume.name, volume.driver, volume.mountpoint);
    /// }
    /// # Ok(())
    /// # }
    /// ```
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
    /// - `id`: Container ID or name
    ///
    /// # Returns
    /// - `Ok(())` if the container was started successfully
    /// - `Err` if the operation fails
    ///
    /// # Example
    /// ```no_run
    /// # use doctainr::services::DockerService;
    /// # async fn example() -> anyhow::Result<()> {
    /// let service = DockerService::new()?;
    /// service.start_container("my_container").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn start_container(&self, id: &str) -> Result<()> {
        self.docker
            .start_container(id, None::<StartContainerOptions<String>>)
            .await?;
        Ok(())
    }

    /// Stops a running Docker container.
    ///
    /// # Arguments
    /// - `id`: Container ID or name
    ///
    /// # Returns
    /// - `Ok(())` if the container was stopped successfully
    /// - `Err` if the operation fails
    ///
    /// # Example
    /// ```no_run
    /// # use doctainr::services::DockerService;
    /// # async fn example() -> anyhow::Result<()> {
    /// let service = DockerService::new()?;
    /// service.stop_container("my_container").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stop_container(&self, id: &str) -> Result<()> {
        self.docker
            .stop_container(id, None::<StopContainerOptions>)
            .await?;
        Ok(())
    }
}

/// Formats a byte size into a human-readable string.
///
/// # Arguments
/// - `size`: Size in bytes
///
/// # Returns
/// A formatted string with appropriate unit (B, KB, MB, or GB)
///
/// # Example
/// ```
/// # fn format_size(size: i64) -> String {
/// #     const KB: i64 = 1024;
/// #     const MB: i64 = KB * 1024;
/// #     const GB: i64 = MB * 1024;
/// #     if size >= GB {
/// #         format!("{:.1}GB", size as f64 / GB as f64)
/// #     } else if size >= MB {
/// #         format!("{:.1}MB", size as f64 / MB as f64)
/// #     } else if size >= KB {
/// #         format!("{:.1}KB", size as f64 / KB as f64)
/// #     } else {
/// #         format!("{}B", size)
/// #     }
/// # }
/// assert_eq!(format_size(1024), "1.0KB");
/// assert_eq!(format_size(1048576), "1.0MB");
/// assert_eq!(format_size(1073741824), "1.0GB");
/// ```
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
