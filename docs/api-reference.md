# API Reference

## Core Modules

### services::docker

Docker API integration using Bollard.

#### DockerService

Main service for Docker API operations.

````rust
pub struct DockerService {
    docker: Docker,
}
````

**Methods:**

##### `new() -> Result<Self>`

Creates a new Docker service instance connecting to the local Docker daemon.

**Returns:**
- `Ok(DockerService)` on successful connection
- `Err` if Docker daemon is unreachable

**Example:**
````rust
let service = DockerService::new()?;
````

##### `list_containers() -> Result<Vec<ContainerInfo>>`

Lists all Docker containers (running and stopped).

**Returns:**
- `Ok(Vec<ContainerInfo>)` with container information
- `Err` on API failure

**Example:**
````rust
let containers = service.list_containers().await?;
for container in containers {
    println!("{}: {}", container.name, container.state.label());
}
````

##### `list_images() -> Result<Vec<ImageInfo>>`

Lists all local Docker images.

**Returns:**
- `Ok(Vec<ImageInfo>)` with image information
- `Err` on API failure

**Example:**
````rust
let images = service.list_images().await?;
for image in images {
    println!("{}:{} - {}", image.repository, image.tag, image.size);
}
````

##### `list_volumes() -> Result<Vec<VolumeInfo>>`

Lists all Docker volumes.

**Returns:**
- `Ok(Vec<VolumeInfo>)` with volume information
- `Err` on API failure

**Example:**
````rust
let volumes = service.list_volumes().await?;
for volume in volumes {
    println!("{}: {} ({})", volume.name, volume.driver, volume.mountpoint);
}
````

##### `start_container(id: &str) -> Result<()>`

Starts a stopped Docker container.

**Arguments:**
- `id` - Container ID or name

**Returns:**
- `Ok(())` on success
- `Err` if container cannot be started

**Example:**
````rust
service.start_container("my_container").await?;
````

##### `stop_container(id: &str) -> Result<()>`

Stops a running Docker container.

**Arguments:**
- `id` - Container ID or name

**Returns:**
- `Ok(())` on success
- `Err` if container cannot be stopped

**Example:**
````rust
service.stop_container("my_container").await?;
````

#### Data Types

##### ContainerInfo

Container metadata.

````rust
pub struct ContainerInfo {
    pub id: String,          // Shortened container ID (12 chars)
    pub name: String,        // Container name (without leading /)
    pub image: String,       // Image name
    pub status: String,      // Status text from Docker
    pub ports: String,       // Formatted port mappings
    pub state: ContainerState, // Running or Stopped
}
````

##### ImageInfo

Docker image metadata.

````rust
pub struct ImageInfo {
    pub id: String,          // Full image ID
    pub repository: String,  // Repository name (e.g., "nginx")
    pub tag: String,         // Image tag (e.g., "latest")
    pub size: String,        // Formatted size (e.g., "142.5MB")
}
````

##### VolumeInfo

Docker volume metadata.

````rust
pub struct VolumeInfo {
    pub name: String,        // Volume name
    pub driver: String,      // Volume driver (usually "local")
    pub mountpoint: String,  // Host filesystem path
    pub size: String,        // Size (currently "--" as not available from API)
}
````

##### ContainerState

Container runtime state.

````rust
pub enum ContainerState {
    Running,
    Stopped,
}
````

**Methods:**

- `label(&self) -> &'static str` - Returns "Running" or "Stopped"
- `css_class(&self) -> &'static str` - Returns CSS class name
- `action_label(&self) -> &'static str` - Returns "Start" or "Stop"

**Example:**
````rust
match container.state {
    ContainerState::Running => println!("Container is running"),
    ContainerState::Stopped => println!("Container is stopped"),
}

let label = container.state.label(); // "Running" or "Stopped"
let action = container.state.action_label(); // "Stop" or "Start"
````

### utils::app_state

Global application state management using Dioxus signals.

#### AppState

Application-wide state container.

````rust
pub struct AppState {
    pub docker_host: Signal<String>,
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub last_action: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
    docker_service: Option<DockerService>,
}
````

**Methods:**

##### `new() -> Self`

Creates a new AppState instance with Docker service connection.

Automatically loads initial data for containers, images, and volumes.

**Example:**
````rust
let app_state = AppState::new();
use_context_provider(|| app_state);
````

##### `refresh_all(&self)`

Refreshes all Docker data (containers, images, volumes) concurrently.

**Example:**
````rust
let app_state = use_context::<AppState>();
app_state.refresh_all();
````

##### `refresh_containers(&self)`

Refreshes only container list.

**Example:**
````rust
app_state.refresh_containers();
````

##### `refresh_images(&self)`

Refreshes only image list.

**Example:**
````rust
app_state.refresh_images();
````

##### `refresh_volumes(&self)`

Refreshes only volume list.

**Example:**
````rust
app_state.refresh_volumes();
````

##### `start_container(&self, id: String)`

Starts a container and refreshes container list on success.

**Arguments:**
- `id` - Container ID

**Example:**
````rust
app_state.start_container(container.id.clone());
````

##### `stop_container(&self, id: String)`

Stops a container and refreshes container list on success.

**Arguments:**
- `id` - Container ID

**Example:**
````rust
app_state.stop_container(container.id.clone());
````

##### `set_container_state(&self, id: &str, next_state: ContainerState)`

Toggles container state (start if stopped, stop if running).

**Arguments:**
- `id` - Container ID
- `next_state` - Desired state

**Example:**
````rust
app_state.set_container_state(&container.id, ContainerState::Running);
````

##### `record_action(&self, message: impl Into<String>)`

Records a user action message for display.

**Arguments:**
- `message` - Action description

**Example:**
````rust
app_state.record_action("Container started successfully");
````

## UI Components

### components::MetricCard

Displays a metric with title, value, and optional hint.

````rust
#[component]
pub fn MetricCard(title: String, value: String, hint: Option<String>) -> Element
````

**Props:**
- `title` - Metric label (e.g., "Running containers")
- `value` - Metric value (e.g., "5")
- `hint` - Optional explanatory text

**Example:**
````rust
rsx! {
    MetricCard {
        title: "Running containers".to_string(),
        value: running.to_string(),
        hint: Some("Across all projects".to_string())
    }
}
````

### components::SectionHeader

Page section header with title and optional subtitle.

````rust
#[component]
pub fn SectionHeader(title: String, subtitle: Option<String>) -> Element
````

**Props:**
- `title` - Section heading
- `subtitle` - Optional descriptive text

**Example:**
````rust
rsx! {
    SectionHeader {
        title: "Dashboard".to_string(),
        subtitle: Some("Overview of your Docker engine".to_string())
    }
}
````

### components::StatusPill

Container status indicator pill.

````rust
#[component]
pub fn StatusPill(state: ContainerState) -> Element
````

**Props:**
- `state` - Container state (Running or Stopped)

**Example:**
````rust
rsx! {
    StatusPill { state: container.state }
}
````

## Views

### views::Dashboard

Main dashboard showing Docker resource overview.

````rust
#[component]
pub fn Dashboard() -> Element
````

**Features:**
- Running/stopped container counts
- Total images count
- Total volumes count
- Docker engine information
- "Refresh All" button

### views::Containers

Container management view.

````rust
#[component]
pub fn Containers() -> Element
````

**Features:**
- List all containers (running and stopped)
- Start/stop buttons for each container
- Container details (ID, name, image, status, ports)
- "Refresh" button

### views::Images

Docker image browser.

````rust
#[component]
pub fn Images() -> Element
````

**Features:**
- List all local images
- Image details (repository, tag, ID, size)
- "Refresh" button

### views::Volumes

Docker volume manager.

````rust
#[component]
pub fn Volumes() -> Element
````

**Features:**
- List all volumes
- Volume details (name, driver, mountpoint)
- "Refresh" button

### views::Settings

Application settings (placeholder).

````rust
#[component]
pub fn Settings() -> Element
````

**Features:**
- Docker host display
- Future: User preferences

### views::AppShell

Main application layout with navigation.

````rust
#[component]
pub fn AppShell() -> Element
````

**Features:**
- Top navigation bar with links
- Outlet for child routes
- Consistent layout across views

## Routing

### Route

Application route definitions.

````rust
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
````

**Routes:**
- `/` - Dashboard view
- `/containers` - Container management
- `/images` - Image browser
- `/volumes` - Volume manager
- `/settings` - Application settings

**Navigation Example:**
````rust
rsx! {
    Link { to: Route::Dashboard {}, "Dashboard" }
    Link { to: Route::Containers {}, "Containers" }
}
````

## Context API

### Accessing AppState

````rust
#[component]
fn MyComponent() -> Element {
    let app_state = use_context::<AppState>();
    
    let containers = (app_state.containers)();
    let error = (app_state.error_message)();
    
    rsx! {
        div { "Container count: {containers.len()}" }
    }
}
````

### Providing AppState

````rust
#[component]
fn App() -> Element {
    let app_state = AppState::new();
    use_context_provider(|| app_state);
    
    rsx! { Router::<Route> {} }
}
````

## Asset Management

### Loading Assets

````rust
const FAVICON: Asset = asset!("/assets/icon.svg");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
````

**Usage:**
````rust
rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }
}
````

Assets are bundled into the application at build time.

## Error Handling

### Service Layer

All service methods return `Result<T, anyhow::Error>`:

````rust
match service.list_containers().await {
    Ok(containers) => {
        // Handle success
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
````

### UI Layer

Errors are displayed via `error_message` signal:

````rust
let error_message = (app_state.error_message)();

if let Some(error) = error_message {
    rsx! {
        div { class: "error-message",
            "⚠️ {error}"
        }
    }
}
````

## Async Operations

### Spawning Tasks

````rust
spawn(async move {
    let result = async_operation().await;
    signal.set(result);
});
````

### Example: Refreshing Data

````rust
pub fn refresh_containers(&self) {
    if let Some(service) = &self.docker_service {
        let service = service.clone();
        let mut containers = self.containers.clone();
        let mut error_message = self.error_message.clone();
        
        spawn(async move {
            match service.list_containers().await {
                Ok(data) => {
                    containers.set(data);
                    error_message.set(None);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed: {}", e)));
                }
            }
        });
    }
}
````

## Testing

### Unit Tests

````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state() {
        assert_eq!(ContainerState::Running.label(), "Running");
        assert_eq!(ContainerState::Stopped.label(), "Stopped");
    }
}
````

### Async Tests

````rust
#[tokio::test]
async fn test_list_containers() {
    let service = DockerService::new().unwrap();
    let result = service.list_containers().await;
    assert!(result.is_ok());
}
````

## Type Conversions

### Size Formatting

````rust
fn format_size(size: i64) -> String
````

Converts byte size to human-readable format:
- `1024` → `"1.0KB"`
- `1048576` → `"1.0MB"`
- `1073741824` → `"1.0GB"`

**Example:**
````rust
let size_str = format_size(image.size);
// "142.5MB"
````

## Signal Patterns

### Reading Signals

````rust
// Clone the value
let containers = app_state.containers();

// Get read reference
let containers = app_state.containers.read();
````

### Writing Signals

````rust
// Set new value
app_state.containers.set(new_containers);

// Write with closure
app_state.containers.write().push(new_container);

// Mutate with closure
app_state.containers.with_mut(|containers| {
    containers.push(new_container);
});
````

## Build Configuration

### Cargo Features

- `default = ["desktop"]` - Desktop platform
- `web` - Web platform (browser)
- `desktop` - Native desktop application

**Using features:**
````bash
# Desktop (default)
cargo build

# Web
cargo build --features web --no-default-features
````

### Dioxus Configuration

`Dioxus.toml`:
````toml
[application]
name = "doctainr"
default_platform = "desktop"

[web.app]
title = "Doctainr"

[bundle]
identifier = "com.doctainr.app"
````

## Environment Variables

### DOCKER_HOST

Specifies Docker daemon connection:

````bash
# Unix socket (default on Linux/macOS)
export DOCKER_HOST=unix:///var/run/docker.sock

# TCP connection
export DOCKER_HOST=tcp://192.168.1.100:2376

# Custom socket
export DOCKER_HOST=unix:///custom/docker.sock
````

### RUST_LOG

Controls logging verbosity:

````bash
# Error level
export RUST_LOG=error

# Debug level
export RUST_LOG=debug

# Module-specific
export RUST_LOG=doctainr=debug,bollard=trace
````

## Version Information

**Current Version:** 0.0.1

**Rust Edition:** 2024

**Dioxus Version:** 0.7.1

**Bollard Version:** 0.18

**Tokio Version:** 1.0
