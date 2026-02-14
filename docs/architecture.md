# Architecture Reference

## Overview

Doctainr is a Docker desktop management application built with Rust and Dioxus 0.7, providing real-time container, image, and volume management through a native desktop interface.

## Technology Stack

- **UI Framework**: Dioxus 0.7 (Rust-native reactive UI)
- **Docker Integration**: Bollard (async Docker API client)
- **Async Runtime**: Tokio
- **Routing**: Dioxus Router
- **Serialization**: Serde/Serde JSON
- **Platform**: Desktop (native)

## Project Structure

````
doctainr/
├── src/
│   ├── main.rs              # Application entry point, routing
│   ├── components/          # Reusable UI components
│   │   ├── metric_card.rs   # Dashboard metric display
│   │   ├── section_header.rs # Page section headers
│   │   └── status_pill.rs   # Container status indicators
│   ├── services/            # External service integrations
│   │   └── docker.rs        # Docker API wrapper
│   ├── utils/               # Shared utilities
│   │   └── app_state.rs     # Global application state
│   └── views/               # Page-level components
│       ├── dashboard.rs     # Overview page
│       ├── containers.rs    # Container management
│       ├── images.rs        # Image browser
│       ├── volumes.rs       # Volume manager
│       ├── settings.rs      # Application settings
│       └── shell.rs         # Layout wrapper
├── assets/                  # Static assets (CSS, icons)
├── Cargo.toml              # Rust dependencies
└── Dioxus.toml             # Dioxus configuration
````

## Core Components

### Application State (`src/utils/app_state.rs`)

The `AppState` struct manages global application state using Dioxus signals for reactive updates:

- **docker_host**: Docker socket connection string
- **containers**: List of Docker containers
- **images**: List of Docker images
- **volumes**: List of Docker volumes
- **last_action**: User action feedback
- **error_message**: Error notification state
- **is_loading**: Loading indicator state
- **docker_service**: Bollard Docker client instance

State updates automatically trigger UI re-renders through Dioxus signals.

### Docker Service (`src/services/docker.rs`)

Wraps the Bollard Docker API client with simplified async methods:

**Data Types:**
- `ContainerInfo`: Container metadata (id, name, image, status, ports, state)
- `ImageInfo`: Image metadata (id, repository, tag, size)
- `VolumeInfo`: Volume metadata (name, driver, mountpoint)
- `ContainerState`: Enum (Running, Stopped) with display helpers

**Methods:**
- `list_containers()`: Fetch all containers (running and stopped)
- `list_images()`: Fetch local Docker images
- `list_volumes()`: Fetch Docker volumes
- `start_container(id)`: Start a stopped container
- `stop_container(id)`: Stop a running container

### Routing (`src/main.rs`)

The `Route` enum defines application routes using Dioxus Router:

````rust
#[derive(Routable)]
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

All routes are wrapped in the `AppShell` layout for consistent navigation.

## Data Flow

### Initialization
1. `main()` launches the `App` component
2. `App` creates `AppState` with Docker service connection
3. `AppState` is provided via Dioxus context API
4. Initial data load spawns async tasks for containers, images, volumes

### User Actions
1. User clicks button (e.g., "Start" on a container)
2. Event handler calls `AppState` method (e.g., `start_container()`)
3. Method clones signals and Docker service
4. Spawns async task via `spawn()` macro
5. Docker API call executed via Bollard
6. On success/error, signals updated
7. Dioxus automatically re-renders dependent components

### Reactive Updates
- Signals are read in components via `use_context::<AppState>()`
- Reading a signal (e.g., `app_state.containers()`) subscribes the component
- Writing a signal (e.g., `containers.set(data)`) triggers re-render
- No manual state subscription required

## Styling

- **CSS Framework**: Custom CSS in `assets/styling/main.css`
- **Tailwind CSS**: Configured but minimal usage (future enhancement)
- **Responsive Design**: Desktop-first approach
- **Theme**: Light mode (dark mode potential)

## Build Configuration

### Cargo.toml Features
- `default = ["desktop"]`: Desktop platform enabled by default
- `web`: Web platform (browser-based)
- `desktop`: Desktop platform (native application)

### Dioxus.toml
- **Application**: Doctainr
- **Platform**: Desktop
- **Assets**: `assets` directory

## Dependencies

**Core:**
- `dioxus = "0.7.1"` (with `router`, `desktop` features)
- `bollard = "0.18"` (Docker API)
- `tokio = "1.0"` (async runtime, `full` features)

**Serialization:**
- `serde = "1.0"` (with `derive` feature)
- `serde_json = "1.0"`

**Utilities:**
- `anyhow = "1.0"` (error handling)
- `uuid = "1.0"` (with `v4` feature)
- `reqwest = "0.13"` (HTTP client, `json` feature)

## Error Handling

- **Service Layer**: Returns `anyhow::Result<T>` for propagation
- **UI Layer**: Errors displayed in `error_message` signal
- **Connection Failures**: Gracefully handled with user-friendly messages
- **Docker Unavailable**: Application continues with error state

## Performance Considerations

- **Async Operations**: All Docker API calls run asynchronously
- **Minimal Blocking**: UI remains responsive during Docker operations
- **Efficient Re-renders**: Dioxus only re-renders components reading updated signals
- **Connection Pooling**: Bollard manages HTTP/socket connections internally

## Security

- **Local Socket**: Connects via local Docker socket (Unix domain socket or Windows named pipe)
- **No Remote Access**: No network exposure by default
- **No Authentication**: Relies on Docker daemon's permission model
- **User Permissions**: Requires user in `docker` group (Linux/macOS)

## Extension Points

### Adding New Views
1. Create component in `src/views/`
2. Add route variant to `Route` enum
3. Implement route attribute `#[route("/path")]`
4. Add navigation link in `AppShell`

### Adding New Docker Operations
1. Add method to `DockerService` using Bollard API
2. Add state update method to `AppState`
3. Call from UI component event handler

### Custom Components
1. Create in `src/components/`
2. Annotate with `#[component]`
3. Define props as function parameters
4. Return `Element` from `rsx!` macro

## Testing

- **Unit Tests**: Located in `#[cfg(test)]` modules
- **Integration Tests**: Require Docker daemon running
- **Component Tests**: Dioxus testing utilities (future addition)

## Deployment

### Desktop Distribution
- **Platform**: macOS, Linux, Windows
- **Bundle**: Single executable with embedded assets
- **Requirements**: Docker daemon must be installed and running

### Build Commands
````bash
# Development build
dx serve --platform desktop

# Release build
cargo build --release

# Run tests
cargo test
````

## Future Architecture Considerations

1. **State Persistence**: Save window position, preferences
2. **Multi-daemon Support**: Connect to remote Docker hosts
3. **Compose Integration**: Docker Compose stack management
4. **Logs Viewer**: Real-time container log streaming
5. **Statistics**: Container resource usage (CPU, memory)
6. **Plugin System**: Extension architecture for custom integrations
