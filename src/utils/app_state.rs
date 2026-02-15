//! Global application state management.
//!
//! This module provides the `AppState` struct which manages all reactive state
//! for the Doctainr application using Dioxus signals.

use dioxus::prelude::*;

use crate::services::{ContainerInfo, ContainerState, DockerService, ImageInfo, VolumeInfo};

/// Global application state for managing Docker resources and UI state.
///
/// This struct contains reactive signals that automatically trigger UI updates
/// when their values change. It also holds a reference to the DockerService
/// for performing operations on the Docker daemon.
#[derive(Clone)]
pub struct AppState {
    /// Docker host connection string
    pub docker_host: Signal<String>,
    /// List of all Docker containers
    pub containers: Signal<Vec<ContainerInfo>>,
    /// List of all Docker images
    pub images: Signal<Vec<ImageInfo>>,
    /// List of all Docker volumes
    pub volumes: Signal<Vec<VolumeInfo>>,
    /// Last action message for user feedback
    pub last_action: Signal<Option<String>>,
    /// Current error message, if any
    pub error_message: Signal<Option<String>>,
    /// Loading state indicator
    pub is_loading: Signal<bool>,
    /// Docker service instance for API operations
    docker_service: Option<DockerService>,
}

impl AppState {
    /// Creates a new AppState instance and initializes the Docker connection.
    ///
    /// This function:
    /// 1. Attempts to connect to the Docker daemon
    /// 2. Initializes all reactive signals
    /// 3. Triggers an initial data load
    ///
    /// # Returns
    /// A new `AppState` instance
    ///
    /// # Example
    /// ```no_run
    /// use dioxus::prelude::*;
    /// use doctainr::utils::AppState;
    ///
    /// #[component]
    /// fn App() -> Element {
    ///     let app_state = AppState::new();
    ///     use_context_provider(|| app_state);
    ///
    ///     rsx! {
    ///         div { "App content" }
    ///     }
    /// }
    /// ```
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

    /// Refreshes all Docker data (containers, images, and volumes).
    ///
    /// This method spawns async tasks to fetch fresh data from Docker
    /// for all resource types.
    pub fn refresh_all(&self) {
        self.refresh_containers();
        self.refresh_images();
        self.refresh_volumes();
    }

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

    pub fn set_container_state(&self, id: &str, next_state: ContainerState) {
        match next_state {
            ContainerState::Running => self.start_container(id.to_string()),
            ContainerState::Stopped => self.stop_container(id.to_string()),
        }
    }

    pub fn record_action(&self, message: impl Into<String>) {
        let mut last_action_signal = self.last_action.clone();
        last_action_signal.set(Some(message.into()));
    }
}
