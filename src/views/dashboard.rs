use dioxus::prelude::*;

use crate::components::{MetricCard, SectionHeader};
use crate::services::ContainerState;
use crate::utils::AppState;

#[component]
pub fn Dashboard() -> Element {
    let app_state = use_context::<AppState>();
    // Access Signal fields by calling the Signal (they are fields of AppState that are Signals)
    let containers = (app_state.containers)();
    let images = (app_state.images)();
    let volumes = (app_state.volumes)();
    let docker_host = (app_state.docker_host)();
    let error_message = (app_state.error_message)();

    let running = containers
        .iter()
        .filter(|container| container.state == ContainerState::Running)
        .count();
    let stopped = containers.len().saturating_sub(running);

    rsx! {
        SectionHeader {
            title: "Dashboard".to_string(),
            subtitle: Some("Overview of your local Docker engine".to_string())
        }

        if let Some(error) = error_message {
            div { class: "error-message",
                "⚠️ {error}"
            }
        }

        div { class: "action-bar",
            button {
                class: "button primary",
                onclick: move |_| app_state.refresh_all(),
                "Refresh All"
            }
        }

        div { class: "cards",
            MetricCard {
                title: "Running containers".to_string(),
                value: running.to_string(),
                hint: Some("Across all projects".to_string())
            }
            MetricCard {
                title: "Stopped containers".to_string(),
                value: stopped.to_string(),
                hint: Some("Ready to restart".to_string())
            }
            MetricCard {
                title: "Images".to_string(),
                value: images.len().to_string(),
                hint: Some("Local cache".to_string())
            }
            MetricCard {
                title: "Volumes".to_string(),
                value: volumes.len().to_string(),
                hint: Some("Persistent data".to_string())
            }
        }

        div { class: "card",
            h3 { "Engine" }
            p { class: "engine-row", "Host: {docker_host}" }
            p { class: "engine-row", "Context: local" }
            p { class: "engine-row", "Compose: ready" }
        }
    }
}
