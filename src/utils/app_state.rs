//! Application state management using Dioxus Signals.
//!
//! This module provides the `AppState` struct which manages all global state for the application
//! including Docker data (containers, images, volumes) and UI state (loading, errors).
//! State is shared across components using Dioxus's context API.

use dioxus::prelude::*;

use crate::services::{ContainerInfo, ContainerState, DockerService, ImageInfo, VolumeInfo};

/// Global application state shared across all components.
///
/// This struct holds all reactive state for the application using Dioxus Signals.
/// Each field is a Signal that automatically triggers re-renders when its value changes.
///
/// # State Management
///
/// - All state is reactive via Dioxus Signals
/// - State is provided at the App level and consumed in child components
/// - Use `use_context::<AppState>()` to access state in components
///
/// # Example
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use crate::AppState;
/// #[component]
/// fn MyComponent() -> Element {
///     let app_state = use_context::<AppState>();
///     let containers = (app_state.containers)();
///     rsx! { "Found {containers.len()} containers" }
/// }
/// ```
/// Global application state shared across all components.
///
/// This struct holds all reactive state for the application using Dioxus Signals.
/// Each field is a Signal that automatically triggers re-renders when its value changes.
///
/// # State Management
///
/// - All state is reactive via Dioxus Signals
/// - State is provided at the App level and consumed in child components
/// - Use `use_context::<AppState>()` to access state in components
///
/// # Example
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use crate::AppState;
/// #[component]
/// fn MyComponent() -> Element {
///     let app_state = use_context::<AppState>();
///     let containers = (app_state.containers)();
///     rsx! { "Found {containers.len()} containers" }
/// }
/// ```
#[derive(Clone)]
pub struct AppState {
    /// Docker daemon connection string (e.g., "unix:///var/run/docker.sock")
    pub docker_host: Signal<String>,
    /// List of all Docker containers (running and stopped)
    pub containers: Signal<Vec<ContainerInfo>>,
    /// List of all locally stored Docker images
    pub images: Signal<Vec<ImageInfo>>,
    /// List of all Docker volumes
    pub volumes: Signal<Vec<VolumeInfo>>,
    /// Most recent action performed (for feedback/logging)
    pub last_action: Signal<Option<String>>,
    /// Current error message to display to user, if any
    pub error_message: Signal<Option<String>>,
    /// Whether a background operation is currently in progress
    pub is_loading: Signal<bool>,
    /// Docker service instance for API calls (not reactive)
    docker_service: Option<DockerService>,
}

impl AppState {
    /// Creates a new AppState instance and initializes Docker connection.
    ///
    /// This function:
    /// 1. Attempts to connect to Docker using default connection settings
    /// 2. Initializes all Signal state to empty/default values
    /// 3. Triggers an initial data refresh to load Docker information
    ///
    /// If Docker connection fails, the service will be None but the app will still initialize.
    /// Errors are logged to stderr.
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
    /// This spawns background tasks for each data type concurrently.
    pub fn refresh_all(&self) {
        self.refresh_containers();
        self.refresh_images();
        self.refresh_volumes();
    }

    /// Refreshes the container list from Docker.
    ///
    /// Spawns a background async task to fetch containers and update state.
    /// Errors are stored in `error_message` for display to the user.
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

    /// Refreshes the image list from Docker.
    ///
    /// Spawns a background async task to fetch images and update state.
    /// Errors are stored in `error_message` for display to the user.
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

    /// Refreshes the volume list from Docker.
    ///
    /// Spawns a background async task to fetch volumes and update state.
    /// Errors are stored in `error_message` for display to the user.
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

    /// Starts a stopped Docker container.
    ///
    /// # Arguments
    ///
    /// * `id` - The container ID or name to start
    ///
    /// After starting, automatically refreshes the container list to show updated state.
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

    /// Stops a running Docker container.
    ///
    /// # Arguments
    ///
    /// * `id` - The container ID or name to stop
    ///
    /// After stopping, automatically refreshes the container list to show updated state.
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

    /// Toggles a container's state (start if stopped, stop if running).
    ///
    /// # Arguments
    ///
    /// * `id` - The container ID or name
    /// * `next_state` - The desired state to transition to
    ///
    /// This is a convenience method used by the UI toggle buttons.
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
