# API Reference

Complete reference for Doctainr's internal APIs and data structures.

## Table of Contents

- [Services](#services)
  - [DockerService](#dockerservice)
- [State Management](#state-management)
  - [AppState](#appstate)
- [Data Models](#data-models)
- [Components](#components)
- [Views](#views)

---

## Services

### DockerService

**Location**: `src/services/docker.rs`

Provides async methods for interacting with the Docker daemon via the Bollard library.

#### Constructor

````rust
pub fn new() -> Result<Self, bollard::errors::Error>
````

**Description**: Creates a new DockerService instance, connecting to the Docker daemon using local defaults (Unix socket or Windows named pipe).

**Returns**: 
- `Ok(DockerService)` - Successfully connected to Docker
- `Err(bollard::errors::Error)` - Connection failed

**Example**:
````rust
let service = DockerService::new()?;
````

---

#### Container Operations

##### `list_containers`

````rust
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>>
````

**Description**: Lists all containers (running and stopped).

**Returns**: Vector of `ContainerInfo` structs

**Example**:
````rust
let containers = service.list_containers().await?;
for container in containers {
    println!("{}: {}", container.name, container.state.label());
}
````

---

##### `start_container`

````rust
pub async fn start_container(&self, id: &str) -> Result<()>
````

**Description**: Starts a stopped container.

**Parameters**:
- `id` - Container ID or name

**Returns**: 
- `Ok(())` - Container started successfully
- `Err(_)` - Failed to start (already running, not found, etc.)

**Example**:
````rust
service.start_container("my-container").await?;
````

---

##### `stop_container`

````rust
pub async fn stop_container(&self, id: &str) -> Result<()>
````

**Description**: Stops a running container.

**Parameters**:
- `id` - Container ID or name

**Returns**: 
- `Ok(())` - Container stopped successfully
- `Err(_)` - Failed to stop

**Example**:
````rust
service.stop_container("my-container").await?;
````

---

##### `toggle_container`

````rust
pub async fn toggle_container(&self, id: &str, state: ContainerState) -> Result<()>
````

**Description**: Toggles container state (start if stopped, stop if running).

**Parameters**:
- `id` - Container ID or name
- `state` - Current state of the container

**Returns**: Result indicating success or failure

**Example**:
````rust
service.toggle_container("my-container", ContainerState::Running).await?;
// Container is now stopped
````

---

#### Image Operations

##### `list_images`

````rust
pub async fn list_images(&self) -> Result<Vec<ImageInfo>>
````

**Description**: Lists all Docker images on the host.

**Returns**: Vector of `ImageInfo` structs

**Example**:
````rust
let images = service.list_images().await?;
for image in images {
    println!("{}:{} - {}", image.repository, image.tag, image.size);
}
````

---

#### Volume Operations

##### `list_volumes`

````rust
pub async fn list_volumes(&self) -> Result<Vec<VolumeInfo>>
````

**Description**: Lists all Docker volumes.

**Returns**: Vector of `VolumeInfo` structs

**Example**:
````rust
let volumes = service.list_volumes().await?;
for volume in volumes {
    println!("{}: {} ({})", volume.name, volume.driver, volume.mountpoint);
}
````

---

## State Management

### AppState

**Location**: `src/utils/app_state.rs`

Global application state using Dioxus signals for reactivity.

#### Structure

````rust
#[derive(Clone)]
pub struct AppState {
    pub docker_host: Signal<String>,
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub last_action: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
}
````

#### Constructor

````rust
pub fn new() -> Self
````

**Description**: Creates a new AppState with default values and initiates background data loading.

**Side Effects**:
- Connects to Docker daemon
- Spawns async task to fetch initial data
- Sets up signal subscriptions

**Example**:
````rust
fn App() -> Element {
    let app_state = AppState::new();
    use_context_provider(|| app_state);
    // ...
}
````

---

#### Methods

##### `refresh_all`

````rust
pub fn refresh_all(&self)
````

**Description**: Refreshes all Docker data (containers, images, volumes) in the background.

**Side Effects**: Updates signals when data is fetched

**Example**:
````rust
let app_state = use_context::<AppState>();
app_state.refresh_all();
````

---

##### `refresh_containers`

````rust
pub fn refresh_containers(&self)
````

**Description**: Refreshes only the containers list.

**Example**:
````rust
app_state.refresh_containers();
````

---

##### `refresh_images`

````rust
pub fn refresh_images(&self)
````

**Description**: Refreshes only the images list.

---

##### `refresh_volumes`

````rust
pub fn refresh_volumes(&self)
````

**Description**: Refreshes only the volumes list.

---

##### `start_container`

````rust
pub fn start_container(&self, id: String)
````

**Description**: Starts a container and refreshes the containers list.

**Parameters**:
- `id` - Container ID or name

**Side Effects**:
- Sets `is_loading` to true
- Starts container
- Refreshes containers list
- Updates `last_action` on success
- Sets `error_message` on failure

**Example**:
````rust
app_state.start_container(container.id.clone());
````

---

##### `stop_container`

````rust
pub fn stop_container(&self, id: String)
````

**Description**: Stops a container and refreshes the containers list.

**Parameters**:
- `id` - Container ID or name

---

##### `toggle_container`

````rust
pub fn toggle_container(&self, id: String, state: ContainerState)
````

**Description**: Toggles container state based on current state.

**Parameters**:
- `id` - Container ID or name
- `state` - Current container state

---

## Data Models

### ContainerInfo

**Location**: `src/services/docker.rs`

````rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
    pub state: ContainerState,
}
````

**Fields**:
- `id` - Container ID (shortened)
- `name` - Container name (without leading `/`)
- `image` - Image name the container is based on
- `status` - Human-readable status text
- `ports` - Port mappings (e.g., "0.0.0.0:8080->80/tcp")
- `state` - Enum representing running/stopped state

---

### ContainerState

````rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    Stopped,
}
````

**Methods**:

##### `label`

````rust
pub fn label(&self) -> &'static str
````

Returns display label: "Running" or "Stopped"

---

##### `css_class`

````rust
pub fn css_class(&self) -> &'static str
````

Returns CSS class name: "running" or "stopped"

---

##### `action_label`

````rust
pub fn action_label(&self) -> &'static str
````

Returns button label for toggle action: "Stop" or "Start"

---

### ImageInfo

````rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
    pub created: String,
}
````

**Fields**:
- `id` - Image ID
- `repository` - Repository name
- `tag` - Image tag (e.g., "latest", "1.0")
- `size` - Human-readable size (e.g., "145 MB")
- `created` - Human-readable creation time

---

### VolumeInfo

````rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VolumeInfo {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
}
````

**Fields**:
- `name` - Volume name
- `driver` - Storage driver (usually "local")
- `mountpoint` - Host filesystem path

---

## Components

### MetricCard

**Location**: `src/components/metric_card.rs`

````rust
#[component]
pub fn MetricCard(label: String, value: usize) -> Element
````

**Description**: Displays a labeled numeric metric.

**Props**:
- `label` - Metric name/description
- `value` - Numeric value to display

**Example**:
````rust
rsx! {
    MetricCard { label: "Running Containers", value: 5 }
}
````

---

### SectionHeader

**Location**: `src/components/section_header.rs`

````rust
#[component]
pub fn SectionHeader(title: String) -> Element
````

**Description**: Consistent heading for page sections.

**Props**:
- `title` - Section title text

**Example**:
````rust
rsx! {
    SectionHeader { title: "Containers" }
}
````

---

### StatusPill

**Location**: `src/components/status_pill.rs`

````rust
#[component]
pub fn StatusPill(state: ContainerState) -> Element
````

**Description**: Visual indicator for container state.

**Props**:
- `state` - Container state enum

**Example**:
````rust
rsx! {
    StatusPill { state: ContainerState::Running }
}
````

**Styling**: Uses CSS classes `.status-pill`, `.running`, `.stopped`

---

## Views

### Dashboard

**Location**: `src/views/dashboard.rs`

**Route**: `/`

**Description**: Overview page showing aggregate metrics and quick status.

**State Dependencies**:
- `containers` - For counting running/stopped
- `images` - For total image count
- `volumes` - For total volume count
- `docker_host` - For connection info
- `error_message` - For error display

---

### Containers

**Location**: `src/views/containers.rs`

**Route**: `/containers`

**Description**: Full container management interface with start/stop actions.

**Features**:
- List all containers
- Start/stop individual containers
- Display container details (ID, name, image, status, ports)
- Real-time state updates

---

### Images

**Location**: `src/views/images.rs`

**Route**: `/images`

**Description**: Browse Docker images.

**Features**:
- List all images
- Display image details (repository, tag, size, created date)

---

### Volumes

**Location**: `src/views/volumes.rs`

**Route**: `/volumes`

**Description**: Manage Docker volumes.

**Features**:
- List all volumes
- Display volume details (name, driver, mountpoint)

---

### Settings

**Location**: `src/views/settings.rs`

**Route**: `/settings`

**Description**: Configuration and preferences.

**Features**:
- Display Docker host connection
- Application settings (placeholder for future features)

---

### AppShell

**Location**: `src/views/shell.rs`

**Description**: Layout component wrapping all routes.

**Features**:
- Navigation bar with links
- Outlet for child routes
- Consistent layout structure

---

## Type Aliases and Constants

### Assets

````rust
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
````

**Description**: Asset references resolved at compile time.

---

## Error Types

Doctainr uses `anyhow::Result<T>` for most error handling, providing flexible error propagation.

**Common Error Sources**:
- `bollard::errors::Error` - Docker API errors
- `std::io::Error` - File system and network I/O
- Connection failures
- Permission issues

**Error Display**: Errors are shown in the UI via `AppState.error_message` signal.

---

## Async Conventions

All async operations use Tokio runtime:

````rust
use tokio::spawn;

spawn(async move {
    // Async work here
});
````

Signals are updated from async tasks to trigger UI re-renders.

---

## Testing Utilities

### Mock Data

For component testing, create mock data:

````rust
let mock_container = ContainerInfo {
    id: "abc123".to_string(),
    name: "test-container".to_string(),
    image: "nginx:latest".to_string(),
    status: "Up 2 hours".to_string(),
    ports: "80/tcp".to_string(),
    state: ContainerState::Running,
};
````

### Test Helpers

````rust
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> DockerService {
        DockerService::new().expect("Docker not available for testing")
    }
}
````

---

**Version**: 0.1.0  
**Last Updated**: February 2026
