//! Status pill component for displaying badges.

use dioxus::prelude::*;

/// A pill-shaped badge for displaying status labels.
///
/// # Props
///
/// * `label` - Text to display in the pill
/// * `class_name` - CSS class for styling (e.g., "running", "stopped")
///
/// # Example
///
/// ```no_run
/// # use dioxus::prelude::*;
/// # use doctainr::components::StatusPill;
/// rsx! {
///     StatusPill {
///         label: "Running".to_string(),
///         class_name: "running".to_string()
///     }
/// }
/// # ;
/// ```
#[component]
pub fn StatusPill(label: String, class_name: String) -> Element {
    let class_value = format!("pill {class_name}");

    rsx! {
        span { class: "{class_value}", "{label}" }
    }
}
