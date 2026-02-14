# Architecture

Doctainr is built with a clean, modular architecture following Rust best practices and Dioxus 0.7 patterns.

## System Overview

````
┌─────────────────────────────────────────────────────────┐
│                    Doctainr Desktop                     │
│                   (Dioxus 0.7 App)                      │
├─────────────────────────────────────────────────────────┤
│  Views Layer                                            │
│  ┌──────────┐  ┌───────────┐  ┌────────┐  ┌─────────┐ │
│  │Dashboard │  │Containers │  │ Images │  │ Volumes │ │
│  └────┬─────┘  └─────┬─────┘  └───┬────┘  └────┬────┘ │
│       │              │            │             │       │
├───────┴──────────────┴────────────┴─────────────┴──────┤
│  State Management (AppState + Signals)                  │
│  ┌────────────────────────────────────────────────────┐ │
│  │ Reactive Signals for containers, images, volumes   │ │
│  └────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│  Services Layer                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │          DockerService (Bollard)                │   │
│  └─────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────┤
│  Docker Engine (via Unix socket or TCP)                │
└─────────────────────────────────────────────────────────┘
````

## Module Structure

### `main.rs`
Entry point that:
- Defines the `Route` enum with all application routes
- Sets up the Dioxus runtime with `dioxus::launch()`
- Provides global `AppState` via context
- Configures assets (favicon, CSS)

### `components/`
Reusable UI building blocks:

- **`MetricCard`** — Displays numeric metrics with title and hint
- **`SectionHeader`** — Page headers with title and optional subtitle
- **`StatusPill`** — Styled status indicators (Running/Stopped)

All components follow Dioxus 0.7 conventions:
- Use `#[component]` macro
- Accept props via function parameters
- Return `Element`

### `views/`
Page-level components corresponding to routes:

- **`AppShell`** — Layout wrapper with navigation sidebar
- **`Dashboard`** — Overview metrics and engine info
- **`Containers`** — Container list with start/stop actions
- **`Images`** — Local image browser
- **`Volumes`** — Volume list
- **`Settings`** — Configuration (future)

Each view:
- Accesses `AppState` via `use_context()`
- Reads reactive `Signal` values
- Triggers service methods for data refresh

### `services/`
Business logic and external integrations:

- **`DockerService`** — Wrapper around Bollard Docker client
  - `list_containers()` — Fetch all containers
  - `list_images()` — Fetch all images
  - `list_volumes()` — Fetch all volumes
  - `start_container(id)` — Start a stopped container
  - `stop_container(id)` — Stop a running container

Data structures:
- `ContainerInfo` — Container metadata
- `ImageInfo` — Image metadata
- `VolumeInfo` — Volume metadata
- `ContainerState` — Enum for Running/Stopped

### `utils/`
Shared utilities and state management:

- **`AppState`** — Global application state
  - Reactive `Signal` fields for containers, images, volumes
  - Methods: `refresh_all()`, `refresh_containers()`, etc.
  - Initializes `DockerService` on creation
  - Provides centralized error handling

## State Management

Doctainr uses Dioxus 0.7's reactive `Signal` primitive for state:

````rust
pub struct AppState {
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub error_message: Signal<Option<String>>,
    // ...
}
````

### Reactive Flow

1. User clicks "Refresh" button
2. Event handler calls `app_state.refresh_containers()`
3. Service fetches data asynchronously via `spawn()`
4. Signal is updated: `*containers.write() = new_data`
5. All components reading that signal automatically re-render

### Why Signals?

- **Automatic tracking** — No manual subscriptions
- **Fine-grained reactivity** — Only components using a signal re-render
- **Thread-safe** — Safe to use across async boundaries
- **Composable** — Easy to derive computed values

See [State Management](../explanation/state-management.md) for deeper discussion.

## Routing

Routes are defined in a single enum:

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

The `AppShell` layout wraps all routes, providing:
- Navigation sidebar
- Active route highlighting
- Shared layout structure

## Styling

Doctainr uses a custom CSS file loaded via the `asset!()` macro:

````rust
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
````

Styles are applied via the `class` attribute in RSX:

````rust
rsx! {
    div { class: "card",
        h3 { "Title" }
    }
}
````

Conditional classes use string interpolation:

````rust
class: format!("status {}", state.css_class())
````

## Docker Integration

### Bollard Client

Doctainr uses [Bollard](https://docs.rs/bollard/), a native Rust Docker client:

````rust
let docker = Docker::connect_with_socket_defaults()?;
````

Connection methods:
- **Unix socket** — `/var/run/docker.sock` (Linux/macOS)
- **Named pipe** — `npipe:////./pipe/docker_engine` (Windows)
- **TCP** — Configurable via `DOCKER_HOST` environment variable

### Asynchronous Operations

All Docker operations are async and use Tokio:

````rust
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
    let containers = self.docker.list_containers(None).await?;
    // Process and return
}
````

UI triggers async operations using `spawn()`:

````rust
spawn(async move {
    if let Some(service) = &self.docker_service {
        let containers = service.list_containers().await;
        *self.containers.write() = containers;
    }
});
````

## Build System

### Development
````bash
dx serve --platform desktop
````

Uses Dioxus CLI with hot reloading.

### Production
````bash
dx bundle --platform desktop --release
````

Creates optimized native binary.

### Dependencies

Key dependencies (from `Cargo.toml`):
- **dioxus 0.7.1** — UI framework (desktop + router)
- **bollard 0.18** — Docker API client
- **tokio 1.0** — Async runtime
- **serde + serde_json** — Serialization
- **reqwest** — HTTP client (future API calls)
- **anyhow** — Error handling

## Security Considerations

- **No authentication** — Doctainr runs locally and trusts the Docker socket
- **Socket permissions** — User must have Docker socket access
- **No remote API** — Currently desktop-only, no web server
- **Input sanitization** — Container IDs are validated before operations

## Performance

- **Native performance** — Compiled Rust, no interpreted layers
- **Lazy loading** — Data fetched on-demand, not continuously polled
- **Efficient rendering** — Dioxus only re-renders changed components
- **Small binary** — Release builds are ~5-10 MB

## Future Architecture

Planned enhancements:
- **Background polling** — Optional auto-refresh for dashboard
- **WebSocket events** — Real-time Docker event stream
- **Multi-context support** — Switch between Docker contexts
- **Remote Docker** — Enhanced TCP/TLS connection support

---

See also:
- [Docker API Integration](../explanation/docker-api.md) — Deeper dive into Bollard usage
- [State Management](../explanation/state-management.md) — Reactive patterns in Dioxus
