//! Shared UI building blocks for the app shell and pages.
//!
//! This module contains reusable UI components that are used across different views
//! in the Doctainr application. These components follow Dioxus 0.7 patterns and
//! provide consistent styling and behavior.

mod metric_card;
pub use metric_card::MetricCard;

mod section_header;
pub use section_header::SectionHeader;

mod status_pill;
pub use status_pill::StatusPill;
