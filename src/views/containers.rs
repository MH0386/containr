use dioxus::prelude::*;

use crate::components::{SectionHeader, StatusPill};
use crate::services::ContainerState;
use crate::utils::AppState;

#[component]
pub fn Containers() -> Element {
    let app_state = use_context::<AppState>();
    // AppState fields are Signals, call them to get the inner value
    let containers = (app_state.containers)();

    rsx! {
        SectionHeader {
            title: "Containers".to_string(),
            subtitle: Some("Manage running services".to_string())
        }

        div { class: "action-bar",
            button {
                class: "button primary",
                onclick: move |_| app_state.refresh_containers(),
                "Refresh"
            }
        }

        div { class: "table",
            div { class: "row header",
                span { "Name" }
                span { "Image" }
                span { "Ports" }
                span { "State" }
                span { "Action" }
            }

            // Use an iterator expression in braces. Each closure builds an rsx! element.
            // Clone only the data we need so closures are 'static' friendly.
            {containers.iter().map(|container| {
                let id = container.id.clone();
                let name = container.name.clone();
                let status = container.status.clone();
                let image = container.image.clone();
                let ports = container.ports.clone();
                let app_state_for_btn = app_state.clone();

                let next_state = if container.state == ContainerState::Running {
                    ContainerState::Stopped
                } else {
                    ContainerState::Running
                };
                let button_label = container.state.action_label();
                let pill_label = container.state.label();
                let pill_class = container.state.css_class();

                rsx! {
                    div { class: "row item",
                        div {
                            p { class: "row-title", "{name}" }
                            p { class: "row-subtitle", "{status}" }
                        }
                        span { "{image}" }
                        span { "{ports}" }
                        StatusPill { label: pill_label.to_string(), class_name: pill_class.to_string() }
                        button {
                            class: "button secondary",
                            // closure captures owned clones above so it's safe to be 'static'
                            onclick: move |_| app_state_for_btn.set_container_state(&id, next_state),
                            "{button_label}"
                        }
                    }
                }
            })}

        }
    }
}
