# API Documentation

This document provides detailed information about the public APIs and modules in Doctainr.

## Table of Contents

- [Services](#services)
  - [DockerService](#dockerservice)
- [Data Models](#data-models)
- [State Management](#state-management)
- [Components](#components)
- [Views](#views)

## Services

### DockerService

The `DockerService` provides an interface to interact with the Docker daemon.

#### Module Path

```rust
use crate::services::DockerService;
```

#### Constructor

```rust
pub fn new() -> Result<Self>
```

Creates a new DockerService instance and connects to the Docker daemon.

**Returns:** `Result<DockerService, Error>`

**Example:**

```rust
let service = DockerService::new()?;
```

#### Methods

##### list_containers

```rust
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>>
```

Lists all Docker containers (both running and stopped).

**Returns:** `Result<Vec<ContainerInfo>>`

**Example:**

```rust
let containers = service.list_containers().await?;
for container in containers {
    println!("{}: {}", container.name, container.state.label());
}
```

##### list_images

```rust
pub async fn list_images(&self) -> Result<Vec<ImageInfo>>
```

Lists all Docker images on the local system.

**Returns:** `Result<Vec<ImageInfo>>`

**Example:**

```rust
let images = service.list_images().await?;
for image in images {
    println!("{}: {}", image.repository, image.size);
}
```

##### list_volumes

```rust
pub async fn list_volumes(&self) -> Result<Vec<VolumeInfo>>
```

Lists all Docker volumes.

**Returns:** `Result<Vec<VolumeInfo>>`

**Example:**

```rust
let volumes = service.list_volumes().await?;
for volume in volumes {
    println!("{}: {}", volume.name, volume.driver);
}
```

##### start_container

```rust
pub async fn start_container(&self, id: &str) -> Result<()>
```

Starts a stopped Docker container.

**Parameters:**

- `id` - Container ID or name

**Returns:** `Result<()>`

**Example:**

```rust
service.start_container("my_container").await?;
```

##### stop_container

```rust
pub async fn stop_container(&self, id: &str) -> Result<()>
```

Stops a running Docker container.

**Parameters:**

- `id` - Container ID or name

**Returns:** `Result<()>`

**Example:**

```rust
service.stop_container("my_container").await?;
```

## Data Models

### ContainerInfo

Represents information about a Docker container.

```rust
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
    pub state: ContainerState,
}
```

**Fields:**

- `id`: Short container ID (12 characters)
- `name`: Container name (without leading `/`)
- `image`: Image name used by the container
- `status`: Status string from Docker
- `ports`: Formatted port mappings (e.g., "8080:80")
- `state`: Current state (Running or Stopped)

### ImageInfo

Represents information about a Docker image.

```rust
pub struct ImageInfo {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
}
```

**Fields:**

- `id`: Full image ID
- `repository`: Repository name
- `tag`: Image tag
- `size`: Formatted size (e.g., "125.3MB")

### VolumeInfo

Represents information about a Docker volume.

```rust
pub struct VolumeInfo {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub size: String,
}
```

**Fields:**

- `name`: Volume name
- `driver`: Volume driver (usually "local")
- `mountpoint`: Mount point path
- `size`: Size information (currently "--")

### ContainerState

Enum representing container states.

```rust
pub enum ContainerState {
    Running,
    Stopped,
}
```

**Methods:**

```rust
pub fn label(&self) -> &'static str
```

Returns display label: "Running" or "Stopped"

```rust
pub fn css_class(&self) -> &'static str
```

Returns CSS class name: "running" or "stopped"

```rust
pub fn action_label(&self) -> &'static str
```

Returns action button label: "Stop" or "Start"

## State Management

### AppState

Global application state managed using Dioxus signals.

```rust
pub struct AppState {
    pub docker_host: Signal<String>,
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub last_action: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
}
```

#### Constructor

```rust
pub fn new() -> Self
```

Creates a new AppState and initializes Docker service connection.

**Example:**

```rust
let app_state = AppState::new();
use_context_provider(|| app_state);
```

#### Methods

##### refresh_all

```rust
pub fn refresh_all(&self)
```

Refreshes containers, images, and volumes data from Docker.

**Example:**

```rust
app_state.refresh_all();
```

##### refresh_containers

```rust
pub fn refresh_containers(&self)
```

Refreshes only container data.

##### refresh_images

```rust
pub fn refresh_images(&self)
```

Refreshes only image data.

##### refresh_volumes

```rust
pub fn refresh_volumes(&self)
```

Refreshes only volume data.

##### start_container

```rust
pub fn start_container(&self, id: String)
```

Starts a container and refreshes the container list.

**Parameters:**

- `id`: Container ID

##### stop_container

```rust
pub fn stop_container(&self, id: String)
```

Stops a container and refreshes the container list.

**Parameters:**

- `id`: Container ID

##### set_container_state

```rust
pub fn set_container_state(&self, id: &str, next_state: ContainerState)
```

Sets a container to the specified state (starts or stops it).

**Parameters:**

- `id`: Container ID
- `next_state`: Desired state

##### record_action

```rust
pub fn record_action(&self, message: impl Into<String>)
```

Records an action message in the last_action signal.

## Components

### MetricCard

Displays a metric with title, value, and optional hint.

```rust
#[component]
pub fn MetricCard(
    title: String,
    value: String,
    hint: Option<String>
) -> Element
```

**Props:**

- `title`: Card title
- `value`: Main value to display
- `hint`: Optional hint text

**Example:**

```rust
rsx! {
    MetricCard {
        title: "Running Containers".to_string(),
        value: "5".to_string(),
        hint: Some("Across all projects".to_string())
    }
}
```

### StatusPill

Displays a status badge.

```rust
#[component]
pub fn StatusPill(
    label: String,
    class_name: String
) -> Element
```

**Props:**

- `label`: Status text
- `class_name`: CSS class for styling

**Example:**

```rust
rsx! {
    StatusPill {
        label: "Running".to_string(),
        class_name: "running".to_string()
    }
}
```

### SectionHeader

Displays a page header with title and optional subtitle.

```rust
#[component]
pub fn SectionHeader(
    title: String,
    subtitle: Option<String>
) -> Element
```

**Props:**

- `title`: Main title
- `subtitle`: Optional subtitle

**Example:**

```rust
rsx! {
    SectionHeader {
        title: "Containers".to_string(),
        subtitle: Some("Manage running services".to_string())
    }
}
```

## Views

### Dashboard

Main dashboard view showing overview of Docker resources.

```rust
#[component]
pub fn Dashboard() -> Element
```

**Features:**

- Displays metric cards for containers, images, volumes
- Shows running vs stopped container counts
- "Refresh All" button to reload data

### Containers

Container management view.

```rust
#[component]
pub fn Containers() -> Element
```

**Features:**

- Lists all containers in a table
- Shows name, image, ports, state
- Start/Stop buttons for each container
- Refresh button

### Images

Image browser view.

```rust
#[component]
pub fn Images() -> Element
```

**Features:**

- Lists all images in a table
- Shows repository, tag, ID, size
- Refresh button

### Volumes

Volume browser view.

```rust
#[component]
pub fn Volumes() -> Element
```

**Features:**

- Lists all volumes in a table
- Shows name, driver, mountpoint
- Refresh button

### Settings

Settings view (placeholder).

```rust
#[component]
pub fn Settings() -> Element
```

### AppShell

Layout component providing navigation and header.

```rust
#[component]
pub fn AppShell() -> Element
```

**Features:**

- Sidebar with navigation links
- Header with app title
- Last action display
- Outlet for child routes

## Routing

Application routes are defined in the `Route` enum:

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/containers")]
        Containers {},
        #[route("/images")]
        Images {},
        #[route("/volumes")]
        Volumes {},
        #[route("/settings")]
        Settings {},
}
```

## Error Handling

All service methods return `Result` types:

```rust
type Result<T> = std::result::Result<T, anyhow::Error>;
```

Errors are handled by:

1. Propagating with `?` operator
2. Setting `error_message` signal in AppState
3. Displaying error in UI

## Async Operations

All Docker operations are async:

```rust
// Service operations are async
let containers = service.list_containers().await?;

// State updates spawn async tasks
spawn(async move {
    match service.list_containers().await {
        Ok(data) => containers.set(data),
        Err(e) => error_message.set(Some(e.to_string())),
    }
});
```

## Testing

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state() {
        let state = ContainerState::Running;
        assert_eq!(state.label(), "Running");
        assert_eq!(state.action_label(), "Stop");
    }

    #[tokio::test]
    async fn test_docker_service() {
        let service = DockerService::new();
        assert!(service.is_ok());
    }
}
```

## Best Practices

1. **Always use signals for reactive state**
2. **Handle errors gracefully**
3. **Use context for global state**
4. **Keep components pure and focused**
5. **Document public APIs**
6. **Write tests for critical functionality**
7. **Use async/await for I/O operations**
8. **Validate input before Docker operations**
