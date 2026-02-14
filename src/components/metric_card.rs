//! Metric card component for displaying statistics.

use dioxus::prelude::*;

/// A card component for displaying a single metric value.
///
/// # Props
///
/// * `title` - Metric label (e.g., "Total Containers")
/// * `value` - Metric value to display (e.g., "42")
/// * `hint` - Optional additional information text
///
/// # Example
///
/// ```no_run
/// # use dioxus::prelude::*;
/// # use doctainr::components::MetricCard;
/// rsx! {
///     MetricCard {
///         title: "Running".to_string(),
///         value: "5".to_string(),
///         hint: Some("Active containers".to_string())
///     }
/// }
/// # ;
/// ```
#[component]
pub fn MetricCard(title: String, value: String, hint: Option<String>) -> Element {
    rsx! {
        div { class: "card",
            p { class: "card-title", "{title}" }
            p { class: "card-value", "{value}" }
            if let Some(hint) = hint {
                p { class: "card-hint", "{hint}" }
            }
        }
    }
}
