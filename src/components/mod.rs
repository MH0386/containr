//! Reusable UI components.
//!
//! This module provides small, composable UI building blocks like
//! cards, pills, and section headers that are used across multiple views.

mod metric_card;
pub use metric_card::MetricCard;

mod section_header;
pub use section_header::SectionHeader;

mod status_pill;
pub use status_pill::StatusPill;
