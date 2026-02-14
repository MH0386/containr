# API Reference

Complete reference for Doctainr's public APIs, data structures, and interfaces.

## Services

### DockerService

The `DockerService` wraps the Bollard Docker client and provides high-level methods for interacting with Docker.

#### Constructor

````rust
pub fn new() -> Result<Self>
````

Creates a new Docker service connected to the local Docker socket.

**Returns:**
- `Ok(DockerService)` — Successfully connected
- `Err(anyhow::Error)` — Failed to connect (Docker not running, permissions, etc.)

**Example:**
````rust
let service = DockerService::new()?;
````

---

#### list_containers

````rust
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>>
````

Lists all containers (running and stopped).

**Returns:**
- `Ok(Vec<ContainerInfo>)` — List of containers
- `Err(anyhow::Error)` — API call failed

**Example:**
````rust
let containers = service.list_containers().await?;
for container in containers {
    println!("{}: {}", container.name, container.state.label());
}
````

---

#### list_images

````rust
pub async fn list_images(&self) -> Result<Vec<ImageInfo>>
````

Lists all local Docker images.

**Returns:**
- `Ok(Vec<ImageInfo>)` — List of images
- `Err(anyhow::Error)` — API call failed

**Example:**
````rust
let images = service.list_images().await?;
````

---

#### list_volumes

````rust
pub async fn list_volumes(&self) -> Result<Vec<VolumeInfo>>
````

Lists all Docker volumes.

**Returns:**
- `Ok(Vec<VolumeInfo>)` — List of volumes
- `Err(anyhow::Error)` — API call failed

**Example:**
````rust
let volumes = service.list_volumes().await?;
````

---

#### start_container

````rust
pub async fn start_container(&self, id: &str) -> Result<()>
````

Starts a stopped container.

**Parameters:**
- `id: &str` — Container ID or name

**Returns:**
- `Ok(())` — Container started successfully
- `Err(anyhow::Error)` — Failed to start (container not found, already running, etc.)

**Example:**
````rust
service.start_container("my-nginx").await?;
````

---

#### stop_container

````rust
pub async fn stop_container(&self, id: &str) -> Result<()>
````

Stops a running container.

**Parameters:**
- `id: &str` — Container ID or name

**Returns:**
- `Ok(())` — Container stopped successfully
- `Err(anyhow::Error)` — Failed to stop (container not found, already stopped, etc.)

**Example:**
````rust
service.stop_container("my-nginx").await?;
````

---

## Data Structures

### ContainerInfo

Represents a Docker container with metadata.

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

**Fields:**
- `id` — Short container ID (12 characters)
- `name` — Container name (without leading `/`)
- `image` — Image name and tag (e.g., `nginx:latest`)
- `status` — Human-readable status string (e.g., "Up 2 hours")
- `ports` — Port mappings (e.g., "80/tcp -> 0.0.0.0:8080")
- `state` — Parsed state enum (Running or Stopped)

---

### ImageInfo

Represents a Docker image.

````rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
}
````

**Fields:**
- `id` — Short image ID (12 characters, without `sha256:` prefix)
- `repository` — Repository name (e.g., `nginx`)
- `tag` — Tag (e.g., `latest`)
- `size` — Human-readable size (e.g., "142 MB")

---

### VolumeInfo

Represents a Docker volume.

````rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VolumeInfo {
    pub name: String,
    pub driver: String,
    pub mount_point: String,
}
````

**Fields:**
- `name` — Volume name (user-defined or hash)
- `driver` — Volume driver (usually `local`)
- `mount_point` — Host filesystem path where volume is mounted

---

### ContainerState

Enumeration of container states.

````rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    Stopped,
}
````

**Methods:**

##### label

````rust
pub fn label(&self) -> &'static str
````

Returns display label.
- `Running` → `"Running"`
- `Stopped` → `"Stopped"`

##### css_class

````rust
pub fn css_class(&self) -> &'static str
````

Returns CSS class name.
- `Running` → `"running"`
- `Stopped` → `"stopped"`

##### action_label

````rust
pub fn action_label(&self) -> &'static str
````

Returns action button label.
- `Running` → `"Stop"`
- `Stopped` → `"Start"`

---

## Components

### MetricCard

Displays a metric with title, value, and optional hint.

````rust
#[component]
pub fn MetricCard(
    title: String,
    value: String,
    hint: Option<String>,
) -> Element
````

**Props:**
- `title` — Metric label (e.g., "Running containers")
- `value` — Metric value (e.g., "5")
- `hint` — Optional subtitle (e.g., "Across all projects")

**Example:**
````rust
rsx! {
    MetricCard {
        title: "Images".to_string(),
        value: "42".to_string(),
        hint: Some("Local cache".to_string())
    }
}
````

---

### SectionHeader

Page header with title and optional subtitle.

````rust
#[component]
pub fn SectionHeader(
    title: String,
    subtitle: Option<String>,
) -> Element
````

**Props:**
- `title` — Page title (e.g., "Dashboard")
- `subtitle` — Optional description (e.g., "Overview of your Docker engine")

**Example:**
````rust
rsx! {
    SectionHeader {
        title: "Containers".to_string(),
        subtitle: Some("Manage your Docker containers".to_string())
    }
}
````

---

### StatusPill

Displays a status indicator with color and label.

````rust
#[component]
pub fn StatusPill(
    state: ContainerState,
) -> Element
````

**Props:**
- `state` — Container state (Running or Stopped)

**Rendering:**
- Running: Green pill with "Running" text
- Stopped: Gray pill with "Stopped" text

**Example:**
````rust
rsx! {
    StatusPill { state: ContainerState::Running }
}
````

---

## State Management

### AppState

Global application state accessible via context.

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
    docker_service: Option<DockerService>,
}
````

**Fields:**
- `docker_host` — Docker host connection string
- `containers` — List of containers
- `images` — List of images
- `volumes` — List of volumes
- `last_action` — Last user action (for feedback)
- `error_message` — Current error message (if any)
- `is_loading` — Global loading indicator
- `docker_service` — Docker service instance (private)

**Methods:**

##### new

````rust
pub fn new() -> Self
````

Creates and initializes application state. Automatically triggers initial data load.

##### refresh_all

````rust
pub fn refresh_all(&self)
````

Refreshes all data (containers, images, volumes) asynchronously.

##### refresh_containers

````rust
pub fn refresh_containers(&self)
````

Refreshes only container data.

##### refresh_images

````rust
pub fn refresh_images(&self)
````

Refreshes only image data.

##### refresh_volumes

````rust
pub fn refresh_volumes(&self)
````

Refreshes only volume data.

##### start_container

````rust
pub fn start_container(&self, id: String)
````

Starts a container and refreshes the container list on success.

##### stop_container

````rust
pub fn stop_container(&self, id: String)
````

Stops a container and refreshes the container list on success.

**Example Usage:**
````rust
#[component]
fn MyView() -> Element {
    let app_state = use_context::<AppState>();
    let containers = (app_state.containers)();
    
    rsx! {
        button {
            onclick: move |_| app_state.refresh_containers(),
            "Refresh"
        }
        for container in containers {
            div { "{container.name}" }
        }
    }
}
````

---

## Routes

Application routes defined in `Route` enum.

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
- `/` — Dashboard (default view)
- `/containers` — Container management
- `/images` — Image browser
- `/volumes` — Volume list
- `/settings` — Settings (future)

All routes share the `AppShell` layout.

---

## Environment Variables

### DOCKER_HOST

Overrides the default Docker connection.

**Default:**
- Linux/macOS: `unix:///var/run/docker.sock`
- Windows: `npipe:////./pipe/docker_engine`

**Example:**
````bash
export DOCKER_HOST=tcp://192.168.1.100:2375
dx serve
````

---

## Build Features

Doctainr uses Cargo features to select build targets.

### Available Features

- `desktop` (default) — Desktop application with native window
- `web` — WebAssembly build for browsers (experimental)

**Example:**
````bash
# Desktop build (default)
dx serve --platform desktop

# Web build
dx serve --platform web
````

---

## Error Handling

All service methods return `Result<T, anyhow::Error>`.

**Common Errors:**
- Docker daemon not running
- Permission denied (socket access)
- Container/image/volume not found
- Network timeout

**Handling Errors:**
````rust
match service.list_containers().await {
    Ok(containers) => {
        *app_state.containers.write() = containers;
        *app_state.error_message.write() = None;
    }
    Err(e) => {
        *app_state.error_message.write() = Some(format!("Failed: {}", e));
    }
}
````

---

See also:
- [Architecture](./architecture.md) — System design overview
- [State Management](../explanation/state-management.md) — Reactive patterns
- [Managing Containers](../how-to-guides/managing-containers.md) — Usage examples
