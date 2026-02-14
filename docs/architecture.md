# Architecture

## Overview

Doctainr is a cross-platform Docker management desktop application built with **Rust** and **Dioxus 0.7**. It provides a native user interface for managing Docker containers, images, and volumes with real-time updates and minimal resource overhead.

## Design Goals

- **Performance**: Native Rust performance with minimal memory footprint
- **Simplicity**: Clean, intuitive interface for common Docker operations
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Reactive**: Real-time updates using Dioxus signals
- **Reliable**: Robust error handling and connection management

## Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| **Language** | Rust 2024 Edition | Systems programming, safety, performance |
| **UI Framework** | Dioxus 0.7 | Reactive component-based UI |
| **Docker Client** | Bollard | Docker Engine API client |
| **Async Runtime** | Tokio | Asynchronous task execution |
| **Serialization** | Serde | JSON data handling |
| **Styling** | Tailwind CSS | Utility-first CSS framework |

## System Architecture

````
┌──────────────────────────────────────────────────────┐
│                   Dioxus Desktop                      │
│  ┌────────────────────────────────────────────────┐  │
│  │              User Interface (Views)             │  │
│  │  Dashboard │ Containers │ Images │ Volumes     │  │
│  └───────────────────┬─────────────────────────────┘  │
│                      │                                │
│  ┌───────────────────▼─────────────────────────────┐  │
│  │         Application State (AppState)            │  │
│  │   • Reactive Signals (Dioxus)                   │  │
│  │   • State Management                            │  │
│  └───────────────────┬─────────────────────────────┘  │
│                      │                                │
│  ┌───────────────────▼─────────────────────────────┐  │
│  │         Docker Service Layer                    │  │
│  │   • DockerService (Bollard wrapper)             │  │
│  │   • API abstraction                             │  │
│  └───────────────────┬─────────────────────────────┘  │
└──────────────────────┼──────────────────────────────┘
                       │
                       │ Unix socket / TCP
                       │
┌──────────────────────▼──────────────────────────────┐
│              Docker Engine                          │
│  • Containers  • Images  • Volumes                  │
└────────────────────────────────────────────────────┘
````

## Module Structure

### `src/main.rs`
Application entry point that:
- Defines routing structure using Dioxus Router
- Initializes application state
- Launches the Dioxus desktop runtime

### `src/components/`
Reusable UI components:
- **MetricCard** - Displays statistics (e.g., container counts)
- **SectionHeader** - Page section headings with optional subtitles
- **StatusPill** - Colored status badges (running, stopped, etc.)

### `src/services/`
Business logic and external integrations:
- **DockerService** - Wraps Bollard Docker client
- **ContainerInfo, ImageInfo, VolumeInfo** - Domain models
- **ContainerState** - Enum for container states with UI helpers

### `src/utils/`
Shared utilities and state management:
- **AppState** - Central reactive state using Dioxus signals
  - Manages all Docker resources
  - Handles async operations
  - Provides state update methods

### `src/views/`
Full-page view components:
- **AppShell** - Application layout with sidebar navigation
- **Dashboard** - Overview with metrics and status
- **Containers** - List and manage containers
- **Images** - Browse local images
- **Volumes** - Manage persistent storage
- **Settings** - Configuration options

## State Management

Doctainr uses **Dioxus Signals** for reactive state management:

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

### Signal Flow

1. **User Action** → Component event handler
2. **Event Handler** → Calls AppState method
3. **AppState Method** → Spawns async task with DockerService
4. **Async Task** → Updates Signal with result
5. **Signal Update** → Triggers component re-render

### Context API

AppState is provided to all components using Dioxus Context API:

````rust
// In main.rs
use_context_provider(|| AppState::new());

// In any component
let app_state = use_context::<AppState>();
````

## Routing

Routes are defined using a Rust enum with the `#[derive(Routable)]` macro:

````rust
#[derive(Debug, Clone, Routable, PartialEq)]
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

The `AppShell` layout wraps all routes, providing consistent navigation.

## Docker Integration

### Connection

Doctainr connects to Docker using the **Bollard** library, which supports:
- Unix sockets (`unix:///var/run/docker.sock`) on Linux/macOS
- Named pipes (`npipe:////./pipe/docker_engine`) on Windows
- TCP connections for remote Docker hosts

````rust
pub fn new() -> Result<Self> {
    let docker = Docker::connect_with_local_defaults()?;
    Ok(Self { docker })
}
````

### API Abstraction

The `DockerService` layer abstracts Bollard's API:
- Converts Bollard types to simplified domain models
- Handles common error cases
- Provides async methods for all operations

### Async Operations

All Docker operations are asynchronous using Tokio:

````rust
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
    let containers = self.docker.list_containers(options).await?;
    // Transform and return
}
````

UI operations spawn tasks that update signals:

````rust
spawn(async move {
    match service.list_containers().await {
        Ok(data) => containers.set(data),
        Err(e) => error_message.set(Some(format!("Error: {}", e))),
    }
});
````

## Error Handling

Errors are handled at multiple levels:

1. **Service Layer**: Returns `Result<T, anyhow::Error>`
2. **AppState Layer**: Catches errors and updates `error_message` signal
3. **UI Layer**: Displays error messages to users

````rust
if let Some(error) = error_message {
    div { class: "error-message", "{error}" }
}
````

## Build System

### Dioxus CLI

Doctainr uses `dx` (Dioxus CLI) for development and builds:

````bash
# Development with hot reload
dx serve --platform desktop

# Release build
dx build --release --platform desktop
````

### Cargo Features

````toml
[features]
default = ["desktop"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
````

This allows building for different platforms from the same codebase.

## Styling

The application uses **Tailwind CSS** for styling:

- `assets/styling/main.css` - Custom styles and Tailwind imports
- `tailwind.css` - Generated Tailwind output
- CSS classes applied directly in RSX

## Performance Considerations

### Efficient Re-renders

Dioxus only re-renders components when their dependencies change:
- Components read signals to establish dependencies
- Signal updates trigger minimal re-renders
- `use_memo` caches expensive computations

### Async Task Management

- Docker API calls don't block the UI thread
- Tasks are spawned with `spawn()` for parallelism
- Loading states provide user feedback

### Resource Management

- Docker service is cloned (cheap Arc clone) for async tasks
- Signals use interior mutability for efficient updates
- Components are lightweight function calls

## Security

- **No credential storage** - Uses Docker's socket authentication
- **Local-only by default** - Connects to local Docker daemon
- **Input validation** - Container IDs validated before API calls

## Future Architecture Considerations

Potential enhancements:
- **Multi-daemon support** - Manage multiple Docker hosts
- **Plugin system** - Extensible functionality
- **Advanced filtering** - Complex queries for resources
- **Real-time streaming** - WebSocket updates for container logs
- **Compose integration** - Docker Compose file support

## References

- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7)
- [Bollard Documentation](https://docs.rs/bollard/)
- [Docker Engine API](https://docs.docker.com/engine/api/)
