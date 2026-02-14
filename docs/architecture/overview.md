# Architecture Overview

## Introduction

Doctainr is a Docker desktop application built with Rust and Dioxus 0.7. It provides a native, fast, and lightweight interface for managing Docker containers, images, and volumes.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Doctainr Application                    │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌───────────────┐                                           │
│  │   UI Layer    │    Dioxus Components & RSX               │
│  │  (Views)      │    - Dashboard, Containers, Images, etc.  │
│  └───────┬───────┘                                           │
│          │                                                    │
│  ┌───────▼────────┐                                          │
│  │  State Layer   │   AppState (Signals)                    │
│  │  (AppState)    │   - Reactive state management           │
│  └───────┬────────┘                                          │
│          │                                                    │
│  ┌───────▼────────┐                                          │
│  │ Service Layer  │   Docker Service                        │
│  │ (DockerService)│   - Bollard Docker API client           │
│  └───────┬────────┘                                          │
│          │                                                    │
└──────────┼──────────────────────────────────────────────────┘
           │
           ▼
    ┌──────────────┐
    │ Docker Engine│
    │  (via Unix   │
    │   socket)    │
    └──────────────┘
```

## Core Technologies

### Rust
- **Language**: Rust 2021 edition
- **Benefits**: Memory safety, performance, zero-cost abstractions

### Dioxus 0.7
- **UI Framework**: React-like component model for Rust
- **Features Used**:
  - Component system with `#[component]` macro
  - RSX macro for declarative UI
  - Reactive state with Signals
  - Context API for state sharing
  - Router for navigation

### Bollard
- **Docker Client**: Asynchronous Docker API client
- **Protocol**: Communicates with Docker via Unix socket or TCP

### Tokio
- **Async Runtime**: Handles all asynchronous operations
- **Usage**: Docker API calls, background tasks

## Application Structure

### Modules

```
src/
├── main.rs           # Entry point, routing, app setup
├── components/       # Reusable UI components
│   ├── metric_card.rs
│   ├── section_header.rs
│   └── status_pill.rs
├── views/            # Page components (routes)
│   ├── dashboard.rs
│   ├── containers.rs
│   ├── images.rs
│   ├── volumes.rs
│   ├── settings.rs
│   └── shell.rs     # AppShell layout
├── services/         # External service integrations
│   └── docker.rs    # Docker API wrapper
└── utils/            # Shared utilities and state
    └── app_state.rs # Global application state
```

## Design Principles

### 1. Component-Based Architecture
- Small, reusable components
- Clear separation of concerns
- Props-based communication

### 2. Reactive State Management
- Signals for automatic re-rendering
- Context API for shared state
- Minimal state mutations

### 3. Async-First
- Non-blocking Docker operations
- Background data refreshing
- Responsive UI during operations

### 4. Type Safety
- Strong typing throughout
- Compile-time guarantees
- Minimal runtime errors

## Key Patterns

### Routing Pattern
Routes are defined as an enum with the `Routable` derive:

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/containers")]
        Containers {},
        // ... more routes
}
```

### State Management Pattern
Global state uses the Context API:

```rust
// In App component
let app_state = AppState::new();
use_context_provider(|| app_state);

// In child components
let app_state = use_context::<AppState>();
```

### Service Pattern
Services encapsulate external integrations:

```rust
impl DockerService {
    pub fn new() -> Result<Self>
    pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>>
    pub async fn start_container(&self, id: &str) -> Result<()>
    // ... more methods
}
```

## Data Flow

1. **User Action** → User clicks a button in the UI
2. **Event Handler** → Component event handler is triggered
3. **State Update** → AppState signal is modified
4. **Service Call** → DockerService makes API call to Docker
5. **State Update** → Results update AppState signals
6. **Re-render** → Components automatically re-render with new data

## Performance Considerations

### Efficient Rendering
- Dioxus only re-renders components when their dependencies change
- Signals track reads/writes automatically
- Minimal DOM updates

### Background Operations
- Docker API calls are async and non-blocking
- UI remains responsive during operations
- Error handling prevents UI freezes

### Memory Management
- Rust's ownership system prevents memory leaks
- No garbage collection overhead
- Efficient data structures

## Security

### Docker Socket Access
- Requires appropriate permissions
- Unix socket by default: `/var/run/docker.sock`
- Can be configured via `DOCKER_HOST` environment variable

### Input Validation
- Container IDs and names are validated
- Docker API provides additional validation

## Extensibility

The architecture supports easy extension:

### Adding New Views
1. Create component in `src/views/`
2. Add route to `Route` enum
3. Link from navigation in `AppShell`

### Adding New Docker Features
1. Add methods to `DockerService`
2. Add state fields to `AppState` if needed
3. Create or update views to use new functionality

### Adding New Components
1. Create component in `src/components/`
2. Export from `mod.rs`
3. Use in views as needed

## Related Documentation

- [Component Structure](components.md)
- [State Management](state-management.md)
- [Docker Service](docker-service.md)
- [Development Setup](../guides/development.md)
