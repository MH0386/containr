//! Section header component with title and optional subtitle.

use dioxus::prelude::*;

/// A header component for page sections.
///
/// # Props
///
/// * `title` - Main heading text
/// * `subtitle` - Optional descriptive text below the title
///
/// # Example
///
/// ```no_run
/// # use dioxus::prelude::*;
/// # use doctainr::components::SectionHeader;
/// rsx! {
///     SectionHeader {
///         title: "Containers".to_string(),
///         subtitle: Some("Manage running services".to_string())
///     }
/// }
/// # ;
/// ```
#[component]
pub fn SectionHeader(title: String, subtitle: Option<String>) -> Element {
    rsx! {
        div { class: "section-header",
            div {
                h2 { "{title}" }
                if let Some(subtitle) = subtitle {
                    p { class: "section-subtitle", "{subtitle}" }
                }
            }
        }
    }
}
