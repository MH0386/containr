//! Metric card component for displaying key statistics.

use dioxus::prelude::*;

/// A card component for displaying a metric with a title, value, and optional hint.
///
/// # Props
/// - `title`: The metric title (e.g., "Running Containers")
/// - `value`: The metric value to display (e.g., "5")
/// - `hint`: Optional hint text for additional context
///
/// # Example
/// ```no_run
/// use dioxus::prelude::*;
/// use doctainr::components::MetricCard;
///
/// fn App() -> Element {
///     rsx! {
///         MetricCard {
///             title: "Total Images".to_string(),
///             value: "42".to_string(),
///             hint: Some("Across all repositories".to_string())
///         }
///     }
/// }
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
