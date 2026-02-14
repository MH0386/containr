//! Reusable UI components for the Doctainr application.
//!
//! This module contains small, focused components that are used across multiple views:
//! - `MetricCard`: Displays a metric with title, value, and optional hint
//! - `SectionHeader`: Section title with optional subtitle
//! - `StatusPill`: Colored pill for displaying container status

use dioxus::prelude::*;

/// A card component displaying a metric with title, value, and optional hint text.
///
/// # Props
///
/// * `title` - Main label for the metric (e.g., "Running containers")
/// * `value` - The metric value to display (e.g., "5")
/// * `hint` - Optional secondary text (e.g., "Across all projects")
///
/// # Example
///
/// ```rust
/// rsx! {
///     MetricCard {
///         title: "Running containers".to_string(),
///         value: "5".to_string(),
///         hint: Some("Across all projects".to_string())
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
