use dioxus::prelude::*;

use crate::utils::AppState;
use crate::Route;

#[component]
pub fn AppShell() -> Element {
    let app_state = use_context::<AppState>();
    let last_action = (app_state.last_action)();

    rsx! {
        div { class: "app-shell",
            aside { class: "sidebar",
                div { class: "brand", "Doctainr" }
                nav { class: "nav-list",
                    Link { to: Route::Dashboard {}, class: "nav-link", "Dashboard" }
                    Link { to: Route::Containers {}, class: "nav-link", "Containers" }
                    Link { to: Route::Images {}, class: "nav-link", "Images" }
                    Link { to: Route::Volumes {}, class: "nav-link", "Volumes" }
                    Link { to: Route::Settings {}, class: "nav-link", "Settings" }
                }
            }
            section { class: "main",
                header { class: "main-header",
                    div {
                        h1 { class: "app-title", "Doctainr Desktop" }
                        p { class: "app-subtitle", "Local engine workspace" }
                    }
                    if let Some(action) = last_action {
                        div { class: "header-action", "Last action: {action}" }
                    }
                }
                main { class: "page", Outlet::<Route> {} }
            }
        }
    }
}
