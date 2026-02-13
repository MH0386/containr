use dioxus::prelude::*;

use crate::components::SectionHeader;
use crate::utils::AppState;

#[component]
pub fn Volumes() -> Element {
    let app_state = use_context::<AppState>();
    let volumes = (app_state.volumes)();

    rsx! {
        SectionHeader {
            title: "Volumes".to_string(),
            subtitle: Some("Persistent storage".to_string())
        }

        div { class: "action-bar",
            button {
                class: "button primary",
                onclick: move |_| app_state.refresh_volumes(),
                "Refresh"
            }
        }

        div { class: "table",
            div { class: "row header",
                span { "Name" }
                span { "Driver" }
                span { "Mountpoint" }
                span { "Size" }
            }
            for volume in volumes {
                div { class: "row item volumes-row",
                    span { "{volume.name}" }
                    span { "{volume.driver}" }
                    span { "{volume.mountpoint}" }
                    span { "{volume.size}" }
                }
            }
        }
    }
}
