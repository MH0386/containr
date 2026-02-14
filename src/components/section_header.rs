//! Section header component with title and optional subtitle.

use dioxus::prelude::*;

/// A header component for sections with title and optional subtitle.
///
/// # Props
///
/// * `title` - Main section heading
/// * `subtitle` - Optional descriptive text below the title
///
/// # Example
///
/// ```rust
/// rsx! {
///     SectionHeader {
///         title: "Dashboard".to_string(),
///         subtitle: Some("Overview of your local Docker engine".to_string())
///     }
/// }
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
