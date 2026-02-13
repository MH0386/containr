use dioxus::prelude::*;

use crate::services::{ContainerInfo, ContainerState, DockerService, ImageInfo, VolumeInfo};

#[derive(Clone)]
pub struct AppState {
    pub docker_host: Signal<String>,
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub last_action: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
    docker_service: Option<DockerService>,
}

impl AppState {
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
