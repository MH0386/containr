# Docker Integration with Bollard

This document explains how Doctainr integrates with Docker using the Bollard library.

## Overview

Doctainr uses [Bollard](https://github.com/fussybeaver/bollard), a fully asynchronous Docker API client for Rust. Bollard provides type-safe access to the Docker Engine API.

## Why Bollard?

- **Async-first**: Built on `tokio`, perfect for non-blocking UI
- **Type-safe**: Strong typing reduces runtime errors
- **Complete API coverage**: Supports all Docker API operations
- **Well-maintained**: Active development and community support

## Architecture

````
┌─────────────────────────────────┐
│     Doctainr (UI Layer)         │
│    Uses: AppState & Views       │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│    DockerService Wrapper        │
│  Location: src/services/docker  │
│  - list_containers()            │
│  - start_container()            │
│  - stop_container()             │
│  - list_images()                │
│  - list_volumes()               │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│      Bollard Library            │
│   Docker API Client (Rust)      │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│      Docker Engine API          │
│   (REST API over Unix socket)   │
└─────────────────────────────────┘
````

## DockerService Implementation

### Connection Setup

````rust
use bollard::Docker;

pub struct DockerService {
    client: Docker,
}

impl DockerService {
    pub fn new() -> Result<Self> {
        // Connects to Docker daemon via socket
        // Default: unix:///var/run/docker.sock (Linux/macOS)
        // Windows: npipe:////./pipe/docker_engine
        let client = Docker::connect_with_local_defaults()?;
        Ok(Self { client })
    }
}
````

**Connection resolution:**
1. Checks `DOCKER_HOST` environment variable
2. Falls back to platform defaults:
   - Unix/macOS: `/var/run/docker.sock`
   - Windows: Named pipe `//./pipe/docker_engine`

### Listing Containers

````rust
use bollard::container::ListContainersOptions;

pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
    let options = Some(ListContainersOptions::<String> {
        all: true,  // Include stopped containers
        ..Default::default()
    });
    
    let containers = self.client.list_containers(options).await?;
    
    // Transform Docker API response to our domain model
    let result = containers.iter().map(|c| {
        ContainerInfo {
            id: c.id.clone().unwrap_or_default(),
            name: c.names.as_ref()
                .and_then(|n| n.first())
                .map(|s| s.trim_start_matches('/').to_string())
                .unwrap_or_default(),
            image: c.image.clone().unwrap_or_default(),
            status: c.status.clone().unwrap_or_default(),
            ports: format_ports(&c.ports),
            state: parse_state(&c.state),
        }
    }).collect();
    
    Ok(result)
}
````

### Starting a Container

````rust
use bollard::container::StartContainerOptions;

pub async fn start_container(&self, id: &str) -> Result<()> {
    self.client
        .start_container(id, None::<StartContainerOptions<String>>)
        .await?;
    Ok(())
}
````

### Stopping a Container

````rust
use bollard::container::StopContainerOptions;

pub async fn stop_container(&self, id: &str) -> Result<()> {
    let options = StopContainerOptions {
        t: 10,  // Timeout in seconds before force kill
    };
    
    self.client.stop_container(id, Some(options)).await?;
    Ok(())
}
````

### Listing Images

````rust
use bollard::image::ListImagesOptions;

pub async fn list_images(&self) -> Result<Vec<ImageInfo>> {
    let options = Some(ListImagesOptions::<String> {
        all: false,  // Only show tagged images
        ..Default::default()
    });
    
    let images = self.client.list_images(options).await?;
    
    let result = images.iter().map(|img| {
        let (repository, tag) = parse_repo_tags(&img.repo_tags);
        
        ImageInfo {
            id: img.id.clone(),
            repository,
            tag,
            size: format_size(img.size),
        }
    }).collect();
    
    Ok(result)
}
````

### Listing Volumes

````rust
use bollard::volume::ListVolumesOptions;

pub async fn list_volumes(&self) -> Result<Vec<VolumeInfo>> {
    let volumes = self.client
        .list_volumes(None::<ListVolumesOptions<String>>)
        .await?;
    
    let result = volumes.volumes
        .unwrap_or_default()
        .iter()
        .map(|v| VolumeInfo {
            name: v.name.clone(),
            driver: v.driver.clone(),
            mountpoint: v.mountpoint.clone(),
        })
        .collect();
    
    Ok(result)
}
````

## Data Type Mapping

### Docker API → Doctainr Domain Models

#### ContainerSummary → ContainerInfo

| Bollard Field | Doctainr Field | Transformation |
|--------------|----------------|----------------|
| `id` | `id` | Direct clone |
| `names[0]` | `name` | Trim leading `/` |
| `image` | `image` | Direct clone |
| `status` | `status` | Direct clone |
| `state` | `state` | Parse to enum |
| `ports` | `ports` | Format as string |

#### ImageSummary → ImageInfo

| Bollard Field | Doctainr Field | Transformation |
|--------------|----------------|----------------|
| `id` | `id` | Direct clone |
| `repo_tags[0]` | `repository` | Split on `:` |
| `repo_tags[0]` | `tag` | Extract after `:` |
| `size` | `size` | Format bytes to human-readable |

#### Volume → VolumeInfo

| Bollard Field | Doctainr Field | Transformation |
|--------------|----------------|----------------|
| `name` | `name` | Direct clone |
| `driver` | `driver` | Direct clone |
| `mountpoint` | `mountpoint` | Direct clone |

## Error Handling

Bollard operations return `Result<T, bollard::errors::Error>`. We convert these to `anyhow::Error` for easier propagation:

````rust
use anyhow::{Context, Result};

pub async fn start_container(&self, id: &str) -> Result<()> {
    self.client
        .start_container(id, None::<StartContainerOptions<String>>)
        .await
        .context(format!("Failed to start container {}", id))?;
    Ok(())
}
````

**Common error scenarios:**
- **Connection refused**: Docker daemon not running
- **Permission denied**: User lacks Docker socket access
- **Not found**: Container/image/volume doesn't exist
- **Conflict**: Operation not valid for current state

## Async Integration with UI

### Spawning Background Tasks

````rust
use dioxus::prelude::*;

pub fn refresh_containers(&self) {
    let docker_service = self.docker_service.clone();
    let containers = self.containers;
    
    spawn(async move {
        match docker_service.list_containers().await {
            Ok(list) => containers.set(list),
            Err(e) => eprintln!("Failed to list containers: {}", e),
        }
    });
}
````

**Key points:**
- `spawn` runs async code without blocking the UI
- Cloned signals allow updates from async context
- Errors are handled gracefully

## Performance Considerations

### Connection Pooling

Bollard maintains a persistent connection to the Docker daemon, avoiding reconnection overhead.

### Parallel Requests

Multiple Docker operations can run concurrently:

````rust
pub fn refresh_all(&self) {
    self.refresh_containers();
    self.refresh_images();
    self.refresh_volumes();
    // All three requests run in parallel
}
````

### Caching Strategy

Doctainr doesn't implement persistent caching:
- Data is fetched on demand via "Refresh" buttons
- State resets on app restart
- Future: Add background polling for live updates

## Limitations and Future Work

### Current Limitations

- **No streaming**: Container logs and stats not implemented
- **No creation/deletion**: Only list/start/stop operations
- **No Docker Compose support**: Single containers only
- **No image pull/build**: Pre-existing images only

### Planned Enhancements

1. **Real-time updates**: Subscribe to Docker events via WebSocket
2. **Container logs**: Stream and display logs
3. **Resource stats**: CPU/memory/network usage monitoring
4. **Advanced operations**: Create, remove, inspect containers
5. **Image management**: Pull, build, remove images
6. **Volume management**: Create, remove volumes
7. **Network management**: List and configure Docker networks

## Testing Docker Integration

### Unit Tests (Mocked)

````rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_list_containers() {
        // Mock Docker service
        let service = MockDockerService::new();
        let containers = service.list_containers().await.unwrap();
        assert!(containers.is_empty());
    }
}
````

### Integration Tests (Requires Docker)

````rust
#[tokio::test]
#[ignore]  // Run with: cargo test -- --ignored
async fn test_real_docker_connection() {
    let service = DockerService::new().unwrap();
    let containers = service.list_containers().await.unwrap();
    // Assertions based on actual Docker state
}
````

## Security Considerations

- **Socket permissions**: Access to Docker socket grants root-equivalent privileges
- **Input validation**: Container IDs are validated before API calls
- **Error exposure**: Error messages don't leak sensitive information to UI

## Alternative Docker Clients

| Library | Pros | Cons |
|---------|------|------|
| **Bollard** | Async, type-safe, complete | Larger dependency tree |
| shiplift | Simpler API | Less active maintenance |
| dockworker | Lightweight | Limited features |

**Why Bollard**: Best balance of features, type safety, and async support.

## Related Documentation

- [Bollard Documentation](https://docs.rs/bollard/)
- [Docker Engine API](https://docs.docker.com/engine/api/)
- [State Management](state-management.md)
- [Architecture Overview](../reference/architecture.md)
