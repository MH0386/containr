# Docker Service Architecture

## Overview

The Docker Service layer provides an abstraction over the Bollard Docker API client, exposing type-safe methods for interacting with Docker. It handles all communication with the Docker daemon and translates Docker API responses into application-specific types.

**Location**: `src/services/docker.rs`

## Architecture

```
┌─────────────────────────────────────────┐
│         AppState / Views                │
│    (High-level application logic)       │
└───────────────┬─────────────────────────┘
                │
                │ DockerService methods
                │
┌───────────────▼─────────────────────────┐
│          DockerService                  │
│   (Type-safe Docker API wrapper)        │
└───────────────┬─────────────────────────┘
                │
                │ Bollard API
                │
┌───────────────▼─────────────────────────┐
│            Bollard Client               │
│   (Async Docker API client)             │
└───────────────┬─────────────────────────┘
                │
                │ HTTP/Unix Socket
                │
┌───────────────▼─────────────────────────┐
│          Docker Daemon                  │
│   (Docker Engine)                       │
└─────────────────────────────────────────┘
```

## DockerService Structure

```rust
use bollard::Docker;

#[derive(Clone)]
pub struct DockerService {
    client: Docker,
}
```

The service wraps a `bollard::Docker` client and implements `Clone` for use in async contexts.

## Connection Management

### Initialization

```rust
impl DockerService {
    pub fn new() -> Result<Self> {
        let client = Docker::connect_with_local_defaults()?;
        Ok(Self { client })
    }
}
```

The service connects to Docker using Bollard's default connection logic:
1. Checks `DOCKER_HOST` environment variable
2. Falls back to platform defaults:
   - Linux/macOS: `unix:///var/run/docker.sock`
   - Windows: Named pipe

### Connection String

The connection string can be customized via environment variable:

```bash
# Unix socket (default on Linux/macOS)
export DOCKER_HOST=unix:///var/run/docker.sock

# TCP connection
export DOCKER_HOST=tcp://127.0.0.1:2375

# TCP with TLS
export DOCKER_HOST=tcp://127.0.0.1:2376
```

## Data Types

### ContainerState

Enum representing container lifecycle states:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    Stopped,
}
```

**Methods**:
- `label()` - Human-readable label ("Running" or "Stopped")
- `css_class()` - CSS class for styling ("running" or "stopped")
- `action_label()` - Action button label ("Stop" or "Start")

### ContainerInfo

Container metadata structure:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerInfo {
    pub id: String,           // Container ID (short format)
    pub name: String,         // Container name
    pub image: String,        // Image name
    pub status: String,       // Status text (e.g., "Up 2 hours")
    pub ports: String,        // Port mappings
    pub state: ContainerState, // Running or Stopped
}
```

### ImageInfo

Docker image metadata:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    pub id: String,           // Image ID (short format)
    pub repository: String,   // Repository name
    pub tag: String,          // Image tag
    pub size: String,         // Human-readable size
}
```

### VolumeInfo

Docker volume metadata:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VolumeInfo {
    pub name: String,         // Volume name
    pub driver: String,       // Volume driver (usually "local")
    pub mountpoint: String,   // Filesystem path
}
```

## Service Methods

### Listing Resources

#### list_containers

Lists all Docker containers (running and stopped):

```rust
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>>
```

**Implementation**:
1. Calls Bollard `list_containers` with options to show all containers
2. Maps Docker API response to `ContainerInfo` structs
3. Extracts and formats container metadata
4. Determines state based on container status

**Usage**:
```rust
let containers = docker_service.list_containers().await?;
```

#### list_images

Lists all local Docker images:

```rust
pub async fn list_images(&self) -> Result<Vec<ImageInfo>>
```

**Implementation**:
1. Calls Bollard `list_images` with options to show all images
2. Maps Docker API response to `ImageInfo` structs
3. Formats image size from bytes to human-readable format
4. Handles untagged images

**Usage**:
```rust
let images = docker_service.list_images().await?;
```

#### list_volumes

Lists all Docker volumes:

```rust
pub async fn list_volumes(&self) -> Result<Vec<VolumeInfo>>
```

**Implementation**:
1. Calls Bollard `list_volumes`
2. Maps Docker API response to `VolumeInfo` structs
3. Extracts volume metadata

**Usage**:
```rust
let volumes = docker_service.list_volumes().await?;
```

### Container Operations

#### start_container

Starts a stopped container:

```rust
pub async fn start_container(&self, id: &str) -> Result<()>
```

**Parameters**:
- `id`: Container ID or name

**Implementation**:
Uses Bollard's `start_container` with default options.

**Usage**:
```rust
docker_service.start_container(&container_id).await?;
```

#### stop_container

Stops a running container:

```rust
pub async fn stop_container(&self, id: &str) -> Result<()>
```

**Parameters**:
- `id`: Container ID or name

**Implementation**:
Uses Bollard's `stop_container` with default timeout.

**Usage**:
```rust
docker_service.stop_container(&container_id).await?;
```

## Error Handling

All service methods return `Result<T>` using `anyhow::Result`:

```rust
use anyhow::Result;

pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
    // Docker API call that may fail
}
```

**Error Sources**:
- Docker daemon not running
- Permission issues (socket access)
- Network errors (TCP connections)
- Invalid container/image IDs
- Docker API errors

**Error Handling Pattern**:
```rust
match docker_service.list_containers().await {
    Ok(containers) => {
        // Update state with containers
    }
    Err(e) => {
        // Display error to user
        eprintln!("Failed to list containers: {}", e);
    }
}
```

## Async Operations

All Docker operations are asynchronous and non-blocking:

### Why Async?

1. **Responsiveness**: UI remains responsive during Docker operations
2. **Concurrent Operations**: Multiple Docker calls can run in parallel
3. **Resource Efficiency**: Tokio handles many operations efficiently

### Async Runtime

Doctainr uses Tokio as the async runtime:

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

Bollard is built on Tokio and integrates seamlessly.

### Spawning Tasks

Docker operations are spawned as background tasks:

```rust
use dioxus::prelude::*;

spawn(async move {
    match docker_service.list_containers().await {
        Ok(containers) => {
            // Update UI
        }
        Err(e) => {
            // Handle error
        }
    }
});
```

## Data Transformation

### Container Status Parsing

Converting Docker status to application state:

```rust
let state = if status.to_lowercase().starts_with("up") {
    ContainerState::Running
} else {
    ContainerState::Stopped
};
```

### Size Formatting

Converting bytes to human-readable format:

```rust
fn format_size(bytes: i64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let bytes_f = bytes as f64;
    let unit_idx = (bytes_f.log2() / 10.0) as usize;
    let size = bytes_f / (1024_f64.powi(unit_idx as i32));
    
    format!("{:.2} {}", size, UNITS[unit_idx])
}
```

### Port Formatting

Converting port bindings to display format:

```rust
fn format_ports(ports: &[Port]) -> String {
    ports
        .iter()
        .filter_map(|p| {
            p.public_port.map(|public| {
                format!("{}:{}", p.ip.as_deref().unwrap_or(""), public)
            })
        })
        .collect::<Vec<_>>()
        .join(", ")
}
```

## Testing

### Unit Tests

Test data transformation logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state_labels() {
        assert_eq!(ContainerState::Running.label(), "Running");
        assert_eq!(ContainerState::Stopped.label(), "Stopped");
    }

    #[test]
    fn test_container_state_actions() {
        assert_eq!(ContainerState::Running.action_label(), "Stop");
        assert_eq!(ContainerState::Stopped.action_label(), "Start");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1_048_576), "1.00 MB");
    }
}
```

### Integration Tests

Test actual Docker operations (requires Docker running):

```rust
#[tokio::test]
async fn test_list_containers() {
    let service = DockerService::new().expect("Failed to connect to Docker");
    let containers = service.list_containers().await;
    assert!(containers.is_ok());
}
```

## Performance Considerations

### Caching

Currently, no caching is implemented. Each request hits the Docker API. This ensures fresh data but may be slow for large environments.

**Future Enhancement**: Implement caching with TTL for less critical data.

### Pagination

Docker API supports pagination for large result sets. Currently, Doctainr fetches all results at once.

**Future Enhancement**: Implement pagination for environments with many containers/images.

### Parallel Requests

Multiple independent requests can run in parallel:

```rust
use futures::future::join_all;

let (containers, images, volumes) = tokio::join!(
    docker_service.list_containers(),
    docker_service.list_images(),
    docker_service.list_volumes(),
);
```

## Security Considerations

### Socket Access

Docker socket requires appropriate permissions:

```bash
# Add user to docker group (Linux)
sudo usermod -aG docker $USER

# Or run with appropriate permissions
```

### Input Validation

Container IDs and names are validated by Docker API. Additional validation in Doctainr:

```rust
pub async fn start_container(&self, id: &str) -> Result<()> {
    if id.is_empty() {
        return Err(anyhow::anyhow!("Container ID cannot be empty"));
    }
    
    self.client
        .start_container(id, None::<StartContainerOptions<String>>)
        .await?;
    
    Ok(())
}
```

### Error Information

Be careful not to expose sensitive information in error messages:

```rust
// ❌ Avoid: May expose internal details
Err(e) => format!("Docker error: {:?}", e)

// ✅ Preferred: User-friendly message
Err(_) => "Failed to connect to Docker. Is Docker running?".to_string()
```

## Extending the Service

### Adding New Operations

To add a new Docker operation:

1. **Add method to DockerService**:
```rust
pub async fn remove_container(&self, id: &str) -> Result<()> {
    self.client
        .remove_container(id, None::<RemoveContainerOptions>)
        .await?;
    Ok(())
}
```

2. **Add method to AppState**:
```rust
pub fn remove_container(&self, id: String) {
    let docker_service = self.docker_service.clone();
    // ... spawn async task
}
```

3. **Use in UI component**:
```rust
button {
    onclick: move |_| app_state.remove_container(id.clone()),
    "Remove"
}
```

### Adding New Data Types

To support new Docker resources:

1. **Define info struct**:
```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkInfo {
    pub id: String,
    pub name: String,
    pub driver: String,
}
```

2. **Add list method**:
```rust
pub async fn list_networks(&self) -> Result<Vec<NetworkInfo>> {
    // Implementation
}
```

3. **Add to AppState**:
```rust
pub networks: Signal<Vec<NetworkInfo>>,
```

## Related Documentation

- [Architecture Overview](overview.md)
- [State Management](state-management.md)
- [API Reference: Docker Service](../api/docker-service.md)
- [Testing Guide](../guides/testing.md)
- [Extending Docker Integration Tutorial](../examples/docker-integration.md)
