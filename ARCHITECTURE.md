# Architecture

This document describes the high-level architecture of Doctainr, a Docker desktop management application built with Rust and Dioxus.

## Overview

Doctainr is a native desktop application that provides a graphical interface for managing Docker containers, images, and volumes. It uses the Dioxus framework for UI rendering and the Bollard library for Docker API communication.

## Technology Stack

- **Language**: Rust (Edition 2021)
- **UI Framework**: Dioxus 0.7.1
- **Docker Client**: Bollard 0.18
- **Async Runtime**: Tokio 1.0
- **Serialization**: Serde 1.0
- **HTTP Client**: Reqwest 0.13

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        Doctainr App                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌───────────────────────────────────────────────────┐    │
│  │              Views (UI Layer)                      │    │
│  │  - Dashboard    - Containers   - Images            │    │
│  │  - Volumes      - Settings                         │    │
│  └────────────────┬──────────────────────────────────┘    │
│                   │                                         │
│  ┌────────────────▼──────────────────────────────────┐    │
│  │              Components                            │    │
│  │  - MetricCard   - StatusPill   - SectionHeader     │    │
│  └────────────────┬──────────────────────────────────┘    │
│                   │                                         │
│  ┌────────────────▼──────────────────────────────────┐    │
│  │           App State (State Management)             │    │
│  │  - Signals for reactive state                      │    │
│  │  - Context API for global state                    │    │
│  └────────────────┬──────────────────────────────────┘    │
│                   │                                         │
│  ┌────────────────▼──────────────────────────────────┐    │
│  │           Services (Business Logic)                │    │
│  │  - DockerService: Docker API operations            │    │
│  └────────────────┬──────────────────────────────────┘    │
│                   │                                         │
└───────────────────┼─────────────────────────────────────────┘
                    │
         ┌──────────▼──────────┐
         │   Bollard (Docker   │
         │    API Client)      │
         └──────────┬──────────┘
                    │
         ┌──────────▼──────────┐
         │   Docker Daemon     │
         │  (unix socket/tcp)  │
         └─────────────────────┘
```

## Module Structure

### 1. Main Application (`main.rs`)

Entry point of the application. Sets up:
- Application routes using Dioxus Router
- Global app state with Context API
- Asset loading (CSS, icons)
- Launch configuration

**Key Components:**
- `Route` enum: Defines all application routes
- `App` component: Root component that provides context and routing

### 2. Views (`src/views/`)

Page-level components that correspond to routes:

- **`dashboard.rs`**: Overview of Docker resources with metric cards
- **`containers.rs`**: List and manage Docker containers
- **`images.rs`**: Browse Docker images
- **`volumes.rs`**: View Docker volumes
- **`settings.rs`**: Application settings (placeholder)
- **`shell.rs`**: AppShell layout with sidebar navigation

### 3. Components (`src/components/`)

Reusable UI building blocks:

- **`metric_card.rs`**: Display key metrics (counts, statistics)
- **`status_pill.rs`**: Show status badges (running/stopped)
- **`section_header.rs`**: Page headers with titles and subtitles

### 4. Services (`src/services/`)

Business logic and external integrations:

- **`docker.rs`**: Docker API integration
  - `DockerService`: Main service struct
  - `ContainerInfo`, `ImageInfo`, `VolumeInfo`: Data models
  - `ContainerState`: Enum for container states
  - Methods: `list_containers()`, `list_images()`, `list_volumes()`, `start_container()`, `stop_container()`

### 5. Utils (`src/utils/`)

Utilities and shared state:

- **`app_state.rs`**: Global application state
  - Manages reactive signals for containers, images, volumes
  - Provides methods for refreshing data
  - Handles container operations

## Design Patterns

### State Management

**Pattern**: Signal-based reactive state with Context API

```rust
// Global state provided via context
pub struct AppState {
    pub docker_host: Signal<String>,
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    // ... more signals
}
```

**Benefits:**
- Automatic UI updates when state changes
- Type-safe state access
- No prop drilling required

### Component Architecture

**Pattern**: Functional components with props

```rust
#[component]
fn MetricCard(title: String, value: String, hint: Option<String>) -> Element {
    // Component implementation
}
```

**Benefits:**
- Pure functions, easy to test
- Props enforce type safety
- Composable and reusable

### Async Operations

**Pattern**: Tokio spawn with signal updates

```rust
spawn(async move {
    match service.list_containers().await {
        Ok(data) => containers.set(data),
        Err(e) => error_message.set(Some(e.to_string())),
    }
});
```

**Benefits:**
- Non-blocking UI operations
- Proper error handling
- Automatic state updates trigger re-renders

## Data Flow

1. **User Interaction**: User clicks a button or navigates to a view
2. **Event Handler**: Component event handler is triggered
3. **State Update**: Handler updates AppState or calls service method
4. **Async Operation**: Service makes Docker API call via Bollard
5. **State Refresh**: Response updates reactive signals
6. **UI Re-render**: Dioxus automatically re-renders affected components

## Docker Integration

### Connection

Doctainr connects to the Docker daemon using Bollard:

```rust
Docker::connect_with_local_defaults()
```

This attempts to connect in order:
1. `DOCKER_HOST` environment variable
2. Unix socket: `/var/run/docker.sock`
3. Named pipe on Windows

### API Operations

All Docker operations are async and use the Bollard library:

- **List Containers**: `docker.list_containers(options)`
- **Start Container**: `docker.start_container(id, options)`
- **Stop Container**: `docker.stop_container(id, options)`
- **List Images**: `docker.list_images(options)`
- **List Volumes**: `docker.list_volumes(options)`

### Error Handling

Errors are captured and stored in state:

```rust
match service.operation().await {
    Ok(data) => {
        // Update success state
        data_signal.set(data);
        error_signal.set(None);
    }
    Err(e) => {
        // Update error state
        error_signal.set(Some(format!("Operation failed: {}", e)));
    }
}
```

## UI Rendering

### Dioxus RSX

UI is defined using RSX (similar to JSX):

```rust
rsx! {
    div { class: "container",
        h1 { "Title" }
        p { "Content" }
    }
}
```

### Styling

- CSS is loaded from `assets/styling/main.css`
- Tailwind CSS utility classes
- Component-specific styling via class names

### Routing

Routes are defined as an enum:

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/containers")]
        Containers {},
        // ... more routes
}
```

## Performance Considerations

### Efficient Updates

- Only components that read changed signals re-render
- Signals provide fine-grained reactivity
- Memoization via `use_memo` for expensive computations

### Async Operations

- Docker API calls don't block the UI
- Loading states provide feedback during operations
- Tokio handles concurrent operations efficiently

### Memory Management

- Rust's ownership system prevents memory leaks
- Signal values are reference-counted
- Components are lightweight function calls

## Security

### Docker Socket Access

- Requires permission to access Docker socket
- Uses local connections only (no remote Docker)
- Inherits Docker daemon's security model

### Input Validation

- Container IDs validated before operations
- Error handling prevents crashes from bad data

## Future Architecture Considerations

### Planned Enhancements

1. **Container Logs**: Stream logs from containers
2. **Exec Terminal**: Execute commands in containers
3. **Image Management**: Pull, push, and remove images
4. **Network Management**: View and configure networks
5. **Compose Support**: Manage Docker Compose projects
6. **Metrics Dashboard**: Real-time resource usage

### Scalability

- Current architecture supports additional views easily
- Service layer can be extended with new operations
- State management scales with additional signals

### Testing

- Components are pure functions (easy to unit test)
- Services can be mocked for testing
- Integration tests can use test containers

## Build and Deployment

### Build Process

```bash
dx build         # Development build
dx bundle        # Production build for desktop app
```

### Platform Support

- **Linux**: Primary development platform
- **macOS**: Supported with Dioxus desktop
- **Windows**: Supported with Dioxus desktop

### Dependencies

Key runtime dependencies:
- Docker daemon must be running
- GTK libraries (Linux)
- WebKit (macOS/Windows)

## References

- [Dioxus Documentation](https://dioxuslabs.com)
- [Bollard Documentation](https://docs.rs/bollard)
- [Docker API Reference](https://docs.docker.com/engine/api)
