//! Service layer for external integrations.
//!
//! This module contains services that interact with external systems,
//! primarily the Docker Engine API through the Bollard client.

/// Docker API integration module.
mod docker;

pub use docker::{ContainerInfo, ContainerState, DockerService, ImageInfo, VolumeInfo};
