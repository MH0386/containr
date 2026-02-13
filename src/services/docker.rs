use anyhow::Result;
use bollard::container::{ListContainersOptions, StartContainerOptions, StopContainerOptions};
use bollard::image::ListImagesOptions;
use bollard::volume::ListVolumesOptions;
use bollard::Docker;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    Stopped,
}

impl ContainerState {
    pub fn label(&self) -> &'static str {
        match self {
            ContainerState::Running => "Running",
            ContainerState::Stopped => "Stopped",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            ContainerState::Running => "running",
            ContainerState::Stopped => "stopped",
        }
    }

    pub fn action_label(&self) -> &'static str {
        match self {
            ContainerState::Running => "Stop",
            ContainerState::Stopped => "Start",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
    pub state: ContainerState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VolumeInfo {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub size: String,
}

#[derive(Clone)]
pub struct DockerService {
    docker: Docker,
}

impl DockerService {
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker })
    }

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

    pub async fn start_container(&self, id: &str) -> Result<()> {
        self.docker
            .start_container(id, None::<StartContainerOptions<String>>)
            .await?;
        Ok(())
    }

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
