//! Application state management using Dioxus signals.

use dioxus::prelude::*;

use crate::services::{ContainerInfo, ContainerState, DockerService, ImageInfo, VolumeInfo};

/// Central application state container using Dioxus signals.
///
/// Manages all reactive state for the application including Docker resources,
/// connection status, and user actions. Each field is a `Signal` that automatically
/// triggers re-renders when modified.
#[derive(Clone)]
pub struct AppState {
    /// Docker daemon connection endpoint
    pub docker_host: Signal<String>,
    /// Current list of containers
    pub containers: Signal<Vec<ContainerInfo>>,
    /// Current list of images
    pub images: Signal<Vec<ImageInfo>>,
    /// Current list of volumes
    pub volumes: Signal<Vec<VolumeInfo>>,
    /// Most recent user action message
    pub last_action: Signal<Option<String>>,
    /// Current error message, if any
    pub error_message: Signal<Option<String>>,
    /// Loading state indicator
    pub is_loading: Signal<bool>,
    /// Docker service instance for API calls
    docker_service: Option<DockerService>,
}

impl AppState {
    /// Creates a new application state instance.
    ///
    /// Initializes all signals and attempts to connect to the Docker daemon.
    /// If connection fails, an error is logged but the app continues with
    /// disabled Docker functionality.
    ///
    /// Automatically triggers an initial data load on creation.
    pub fn new() -> Self {
        let docker_service = match DockerService::new() {
            Ok(service) => Some(service),
            Err(e) => {
                eprintln!("Failed to connect to Docker: {}", e);
                None
            }
        };

        let docker_host = use_signal(|| {
            std::env::var("DOCKER_HOST")
                .unwrap_or_else(|_| "unix:///var/run/docker.sock".to_string())
        });
        let containers = use_signal(Vec::new);
        let images = use_signal(Vec::new);
        let volumes = use_signal(Vec::new);
        let last_action = use_signal(|| None);
        let error_message = use_signal(|| None);
        let is_loading = use_signal(|| false);

        let state = Self {
            docker_host,
            containers,
            images,
            volumes,
            last_action,
            error_message,
            is_loading,
            docker_service,
        };

        // Spawn initial data load
        state.refresh_all();

        state
    }

    /// Refreshes all Docker resources (containers, images, and volumes).
    pub fn refresh_all(&self) {
        self.refresh_containers();
        self.refresh_images();
        self.refresh_volumes();
    }

    /// Refreshes the container list from the Docker daemon.
    ///
    /// Spawns an async task to fetch containers. Updates the `containers`
    /// signal on success or `error_message` on failure.
    pub fn refresh_containers(&self) {
        if let Some(service) = &self.docker_service {
            let service = service.clone();
            let mut containers = self.containers.clone();
            let mut error_message = self.error_message.clone();
            let mut is_loading = self.is_loading.clone();

            spawn(async move {
                is_loading.set(true);
                match service.list_containers().await {
                    Ok(data) => {
                        containers.set(data);
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to list containers: {}", e)));
                    }
                }
                is_loading.set(false);
            });
        } else {
            self.error_message
                .clone()
                .set(Some("Docker service not available".to_string()));
        }
    }

    /// Refreshes the image list from the Docker daemon.
    ///
    /// Spawns an async task to fetch images. Updates the `images`
    /// signal on success or `error_message` on failure.
    pub fn refresh_images(&self) {
        if let Some(service) = &self.docker_service {
            let service = service.clone();
            let mut images = self.images.clone();
            let mut error_message = self.error_message.clone();

            spawn(async move {
                match service.list_images().await {
                    Ok(data) => {
                        images.set(data);
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to list images: {}", e)));
                    }
                }
            });
        }
    }

    /// Refreshes the volume list from the Docker daemon.
    ///
    /// Spawns an async task to fetch volumes. Updates the `volumes`
    /// signal on success or `error_message` on failure.
    pub fn refresh_volumes(&self) {
        if let Some(service) = &self.docker_service {
            let service = service.clone();
            let mut volumes = self.volumes.clone();
            let mut error_message = self.error_message.clone();

            spawn(async move {
                match service.list_volumes().await {
                    Ok(data) => {
                        volumes.set(data);
                        error_message.set(None);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to list volumes: {}", e)));
                    }
                }
            });
        }
    }

    /// Starts a stopped container.
    ///
    /// # Arguments
    ///
    /// * `id` - Container ID or name
    ///
    /// Spawns an async task to start the container. Updates `last_action`
    /// on success and refreshes the container list. Sets `error_message` on failure.
    pub fn start_container(&self, id: String) {
        if let Some(service) = &self.docker_service {
            let service = service.clone();
            let mut last_action = self.last_action.clone();
            let mut error_message = self.error_message.clone();
            let id_clone = id.clone();
            let app_state = self.clone();

            spawn(async move {
                match service.start_container(&id_clone).await {
                    Ok(_) => {
                        last_action.set(Some(format!("Started container {}", id_clone)));
                        error_message.set(None);
                        // Refresh containers to get updated state
                        app_state.refresh_containers();
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to start container: {}", e)));
                    }
                }
            });
        }
    }

    /// Stops a running container.
    ///
    /// # Arguments
    ///
    /// * `id` - Container ID or name
    ///
    /// Spawns an async task to stop the container. Updates `last_action`
    /// on success and refreshes the container list. Sets `error_message` on failure.
    pub fn stop_container(&self, id: String) {
        if let Some(service) = &self.docker_service {
            let service = service.clone();
            let mut last_action = self.last_action.clone();
            let mut error_message = self.error_message.clone();
            let id_clone = id.clone();
            let app_state = self.clone();

            spawn(async move {
                match service.stop_container(&id_clone).await {
                    Ok(_) => {
                        last_action.set(Some(format!("Stopped container {}", id_clone)));
                        error_message.set(None);
                        // Refresh containers to get updated state
                        app_state.refresh_containers();
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Failed to stop container: {}", e)));
                    }
                }
            });
        }
    }

    /// Changes a container's state to the specified target state.
    ///
    /// # Arguments
    ///
    /// * `id` - Container ID or name
    /// * `next_state` - Target state (Running or Stopped)
    ///
    /// Delegates to `start_container` or `stop_container` based on the target state.
    pub fn set_container_state(&self, id: &str, next_state: ContainerState) {
        match next_state {
            ContainerState::Running => self.start_container(id.to_string()),
            ContainerState::Stopped => self.stop_container(id.to_string()),
        }
    }

    /// Records a user action message.
    ///
    /// # Arguments
    ///
    /// * `message` - Action description to display
    pub fn record_action(&self, message: impl Into<String>) {
        let mut last_action_signal = self.last_action.clone();
        last_action_signal.set(Some(message.into()));
    }
}
