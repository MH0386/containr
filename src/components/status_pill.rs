//! Status pill component for displaying container states.

use dioxus::prelude::*;

/// A colored pill badge for displaying status labels.
///
/// Used primarily to show container state (Running, Stopped) with appropriate styling.
///
/// # Props
///
/// * `label` - Text to display in the pill
/// * `class_name` - CSS class for styling (e.g., "running", "stopped")
///
/// # Example
///
/// ```rust
/// rsx! {
///     StatusPill {
///         label: "Running".to_string(),
///         class_name: "running".to_string()
///     }
/// }
/// ```
#[component]
pub fn StatusPill(label: String, class_name: String) -> Element {
    let class_value = format!("pill {class_name}");

    rsx! {
        span { class: "{class_value}", "{label}" }
    }
}
