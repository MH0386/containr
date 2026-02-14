# Architecture

> **Technical Reference** - Understanding-oriented documentation explaining the system design and architecture of Doctainr.

## Overview

Doctainr is a native desktop application for managing Docker containers, images, and volumes. It's built using:

- **Rust** - Systems programming language for performance and safety
- **Dioxus 0.7** - Modern React-like UI framework for Rust
- **Bollard** - Rust client for the Docker Engine API
- **Tokio** - Asynchronous runtime for async operations

## Architecture Diagram

````
┌─────────────────────────────────────────────────────────┐
│                      Desktop App                         │
│                    (Dioxus 0.7)                         │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐             │
│  │Dashboard │  │Containers│  │  Images  │ ...         │
│  │  View    │  │   View   │  │   View   │             │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘             │
│       │             │              │                    │
│       └─────────────┼──────────────┘                    │
│                     │                                   │
│            ┌────────▼─────────┐                        │
│            │    AppState      │  (Reactive Signals)    │
│            │  Context API     │                        │
│            └────────┬─────────┘                        │
│                     │                                   │
│            ┌────────▼─────────┐                        │
│            │  DockerService   │                        │
│            │    (Bollard)     │                        │
│            └────────┬─────────┘                        │
└─────────────────────┼───────────────────────────────────┘
                      │ Unix Socket / Named Pipe
                      │
          ┌───────────▼───────────┐
          │   Docker Engine API    │
          └───────────────────────┘
````

## Module Structure

### `/src/main.rs`

Application entry point that:
- Defines the route structure using Dioxus Router
- Initializes global `AppState` and provides it via context
- Configures assets (favicon, CSS)
- Launches the Dioxus desktop runtime

### `/src/views/`

View components corresponding to application routes:
- `mod.rs` - Exports all views and defines `AppShell` layout
- `dashboard.rs` - Overview dashboard with metrics
- `containers.rs` - Container management with start/stop actions
- `images.rs` - Browse Docker images
- `volumes.rs` - List Docker volumes
- `settings.rs` - Application settings (future)
- `shell.rs` - Shell/terminal interface (future)

Each view:
- Consumes `AppState` via `use_context()`
- Renders UI using `rsx!` macro
- Triggers state updates through AppState methods

### `/src/components/`

Reusable UI components:
- `metric_card.rs` - Card displaying a metric with title, value, and hint
- `section_header.rs` - Section title with optional subtitle
- `status_pill.rs` - Colored badge for container status

Components follow Dioxus 0.7 patterns:
- Annotated with `#[component]`
- Props are owned values (`String`, not `&str`)
- Return `Element`

### `/src/services/`

Integration with external services:
- `docker.rs` - Docker Engine API client using Bollard
  - `DockerService` - Manages connection to Docker daemon
  - `ContainerInfo`, `ImageInfo`, `VolumeInfo` - Data structures
  - Async methods for listing, starting, stopping containers

### `/src/utils/`

Shared utilities and state management:
- `app_state.rs` - Global application state using Dioxus Signals
  - Reactive state with automatic re-rendering
  - Background task spawning for async operations
  - Error handling and user feedback

## State Management

Doctainr uses **Dioxus Signals** for reactive state management:

````rust
#[derive(Clone)]
pub struct AppState {
    pub docker_host: Signal<String>,
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub error_message: Signal<Option<String>>,
    // ...
}
````

### Signal Lifecycle

1. **Initialization** - Signals created with `use_signal()` in `AppState::new()`
2. **Provider** - AppState provided at root level via `use_context_provider()`
3. **Consumer** - Views access state with `use_context::<AppState>()`
4. **Read** - Call signal like function: `(app_state.containers)()`
5. **Write** - Use `.set()` or `.write()` to update value
6. **React** - Components automatically re-render when signals change

### Data Flow

````
User Action (button click)
    ↓
AppState method called (e.g., start_container)
    ↓
Async task spawned with Dioxus spawn()
    ↓
DockerService API call
    ↓
Docker Engine response
    ↓
Signal updated (.set())
    ↓
Component re-renders automatically
````

## Routing

Routes are defined using a `#[derive(Routable)]` enum:

````rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/containers")]
        Containers {},
        // ...
}
````

- `#[layout(AppShell)]` provides a shared navigation wrapper
- Each route variant maps to a view component
- `Router::<Route> {}` component handles rendering

## Async Operations

Async operations use Dioxus's `spawn()` function:

````rust
spawn(async move {
    match service.list_containers().await {
        Ok(data) => containers.set(data),
        Err(e) => error_message.set(Some(format!("{}", e))),
    }
});
````

Key points:
- All Docker API calls are async (Tokio runtime)
- Signals are cloned into async closures
- Errors captured and displayed via `error_message` signal
- No blocking of UI thread

## Docker Integration

Connection to Docker daemon:
1. `DockerService::new()` attempts connection via `Docker::connect_with_local_defaults()`
2. Tries Unix socket (`/var/run/docker.sock`) on Linux/Mac
3. Falls back to named pipe on Windows
4. Respects `DOCKER_HOST` environment variable

All Docker operations use Bollard library:
- `list_containers()` - Lists all containers (running + stopped)
- `list_images()` - Lists locally cached images
- `list_volumes()` - Lists Docker volumes
- `start_container(id)` - Starts a container
- `stop_container(id)` - Stops a container

## Error Handling

Three-layer error handling strategy:

1. **Service Layer** - Returns `Result<T, anyhow::Error>` from Docker calls
2. **State Layer** - Catches errors and stores in `error_message` Signal
3. **View Layer** - Displays errors to user in UI

````rust
if let Some(error) = error_message {
    div { class: "error-message", "⚠️ {error}" }
}
````

## Styling

CSS styling approach:
- Main stylesheet at `/assets/styling/main.css`
- Utility-first CSS with semantic class names
- Responsive design principles
- Dark theme support (future)

Classes are applied directly in RSX:
````rust
rsx! {
    div { class: "card",
        p { class: "card-title", "{title}" }
    }
}
````

## Build & Deployment

- Development: `dx serve --platform desktop`
- Production: `dx build --release --platform desktop`
- Output: Native executable for target platform
- Assets bundled into binary via `asset!()` macro

## Performance Considerations

- **Lazy Loading** - Data fetched on-demand per view
- **Async Operations** - Non-blocking Docker API calls
- **Signal Efficiency** - Fine-grained reactivity only re-renders affected components
- **Native Performance** - Rust compilation to native code

## Security

- **No Credential Storage** - Uses system Docker socket authentication
- **Local Only** - No network exposure, desktop app only
- **Docker Permissions** - Respects Docker daemon access controls

## Future Enhancements

- Real-time event streaming from Docker
- Container logs viewer
- Image pull/build functionality
- Compose stack management
- Resource usage metrics (CPU, memory)
- Multi-host Docker support

---

**Related Documentation:**
- [Development Guide](development.md) - How to build and extend Doctainr
- [Contributing](contributing.md) - Guidelines for contributors
