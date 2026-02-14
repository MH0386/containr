# Architecture Overview

Doctainr is a desktop Docker management application built with Rust and Dioxus 0.7, following a modular architecture with clear separation of concerns.

## System Architecture

````
┌─────────────────────────────────────────────────────────────┐
│                         Doctainr                             │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │              Dioxus UI Layer (Views)                │    │
│  │  ┌─────────┐ ┌────────┐ ┌────────┐ ┌─────────┐   │    │
│  │  │Dashboard│ │Container│ │ Images │ │ Volumes │   │    │
│  │  └────┬────┘ └───┬────┘ └───┬────┘ └────┬────┘   │    │
│  └───────┼──────────┼──────────┼───────────┼────────┘    │
│          │          │          │           │              │
│  ┌───────▼──────────▼──────────▼───────────▼────────┐    │
│  │              Application State                      │    │
│  │         (Dioxus Signals + Context API)             │    │
│  └───────────────────┬─────────────────────────────────┘    │
│                      │                                      │
│  ┌───────────────────▼─────────────────────────────────┐    │
│  │              Services Layer                          │    │
│  │           (Business Logic + Docker API)             │    │
│  └───────────────────┬─────────────────────────────────┘    │
│                      │                                      │
└──────────────────────┼──────────────────────────────────────┘
                       │
                 ┌─────▼─────┐
                 │  Bollard   │
                 │ (Docker    │
                 │  Client)   │
                 └─────┬─────┘
                       │
                 ┌─────▼─────┐
                 │  Docker    │
                 │  Engine    │
                 └───────────┘
````

## Module Structure

### `/src/main.rs`
**Entry point** defining the application router and initializing global state.

- Defines `Route` enum with all application routes
- Sets up `AppShell` layout wrapper
- Configures assets (favicon, CSS)
- Launches the Dioxus runtime

### `/src/views/`
**UI components** representing complete screens or layouts.

- `shell.rs` - Main application shell with navigation sidebar
- `dashboard.rs` - Overview displaying container/image/volume counts
- `containers.rs` - Container management view (list, start, stop)
- `images.rs` - Docker images browser
- `volumes.rs` - Docker volumes browser
- `settings.rs` - Application settings

### `/src/components/`
**Reusable UI components** used across views.

- `metric_card.rs` - Displays numeric metrics (e.g., container count)
- `section_header.rs` - Consistent section headings
- `status_pill.rs` - Visual status indicators (running/stopped)

### `/src/services/`
**Business logic and external integrations**.

- `docker.rs` - Docker API client wrapper using Bollard
  - Container operations (list, start, stop)
  - Image listing
  - Volume listing
  - Data type definitions (`ContainerInfo`, `ImageInfo`, `VolumeInfo`)

### `/src/utils/`
**Shared utilities and application state**.

- `app_state.rs` - Global application state management
  - Reactive signals for containers, images, volumes
  - Error and loading state management
  - Centralized refresh operations

## State Management

Doctainr uses **Dioxus 0.7's signal-based reactivity** for state management.

### Key Concepts

1. **Signals**: Reactive values that trigger UI updates when modified
2. **Context API**: Global state sharing via `use_context_provider` and `use_context`
3. **Derived State**: Computed values that update automatically

### State Flow

````rust
AppState (Context Provider)
    ├─ docker_host: Signal<String>
    ├─ containers: Signal<Vec<ContainerInfo>>
    ├─ images: Signal<Vec<ImageInfo>>
    ├─ volumes: Signal<Vec<VolumeInfo>>
    ├─ error_message: Signal<Option<String>>
    └─ is_loading: Signal<bool>
````

**Views** consume state via `use_context::<AppState>()` and trigger updates by calling methods on `AppState`.

## Data Flow

1. **User Action** → View event handler (e.g., button click)
2. **State Update** → `AppState` method called (e.g., `start_container()`)
3. **Service Call** → `DockerService` interacts with Docker API via Bollard
4. **Signal Update** → State signals modified with new data
5. **Reactive Update** → UI components re-render automatically

## Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| UI Framework | Dioxus 0.7 | Reactive UI with RSX syntax |
| Runtime | Tokio | Async runtime for Docker API calls |
| Docker Client | Bollard | Docker Engine API client |
| Serialization | Serde | JSON data handling |
| Error Handling | Anyhow | Ergonomic error propagation |
| Platform | Desktop | Native desktop app via Dioxus desktop feature |

## Design Principles

### 1. Separation of Concerns
- **Views**: Pure presentation logic
- **Services**: External API interactions
- **Utils**: Shared state and helpers
- **Components**: Reusable UI building blocks

### 2. Reactive State Management
- Centralized state in `AppState`
- Signal-based reactivity eliminates manual update tracking
- Context API for global state access

### 3. Type Safety
- Strong typing for Docker entities
- `Result<T, E>` for error handling
- Enum-based state representation

### 4. Async-First
- All Docker operations are async
- Non-blocking UI updates
- Tokio runtime for concurrent operations

## Future Architecture Considerations

- **Plugin System**: Extensible Docker operations
- **Configuration Management**: Persistent user preferences
- **Multi-engine Support**: Podman, containerd integration
- **Real-time Updates**: WebSocket-based live Docker events
- **Testing Strategy**: Unit tests for services, integration tests for views

## Related Documentation

- [State Management Explanation](../explanation/state-management.md)
- [Docker Integration Details](../explanation/docker-integration.md)
- [Component Reference](components.md)
