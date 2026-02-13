# Architecture Overview

This document describes the architecture, design patterns, and technical decisions behind Doctainr.

## System Architecture

````
┌─────────────────────────────────────────────────────────┐
│                    Dioxus Desktop App                    │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Views      │  │  Components  │  │   Assets     │  │
│  │ (Pages/UI)   │  │ (Reusable)   │  │  (CSS/Icons) │  │
│  └──────┬───────┘  └──────┬───────┘  └──────────────┘  │
│         │                  │                              │
│         └──────────┬───────┘                             │
│                    │                                      │
│         ┌──────────▼───────────┐                         │
│         │    AppState (Signals) │                         │
│         │  - Context Provider   │                         │
│         │  - Reactive State     │                         │
│         └──────────┬───────────┘                         │
│                    │                                      │
│         ┌──────────▼───────────┐                         │
│         │   Services Layer      │                         │
│         │  - DockerService      │                         │
│         └──────────┬───────────┘                         │
└────────────────────┼─────────────────────────────────────┘
                     │
            ┌────────▼────────┐
            │   Bollard Crate  │
            │  (Docker Client) │
            └────────┬────────┘
                     │
            ┌────────▼────────┐
            │  Docker Daemon   │
            │  (Unix Socket)   │
            └─────────────────┘
````

## Core Layers

### 1. Views Layer

**Purpose**: Page-level components that correspond to routes.

**Location**: `src/views/`

**Responsibilities**:
- Render complete pages
- Manage page-specific state
- Coordinate multiple components
- Handle user interactions
- Connect to AppState via Context API

**Key Files**:
- `dashboard.rs`: Overview metrics and status
- `containers.rs`: Container management interface
- `images.rs`: Image listing and operations
- `volumes.rs`: Volume management
- `settings.rs`: Configuration interface
- `shell.rs`: Layout wrapper (AppShell)

**Example Pattern**:

````rust
#[component]
pub fn Dashboard() -> Element {
    // 1. Access global state
    let app_state = use_context::<AppState>();
    
    // 2. Extract needed data
    let containers = (app_state.containers)();
    let is_loading = (app_state.is_loading)();
    
    // 3. Derive computed values
    let running_count = containers.iter()
        .filter(|c| c.state == ContainerState::Running)
        .count();
    
    // 4. Render UI
    rsx! {
        div { class: "dashboard",
            SectionHeader { title: "Dashboard" }
            MetricCard { 
                label: "Running", 
                value: running_count 
            }
        }
    }
}
````

### 2. Components Layer

**Purpose**: Reusable, composable UI elements.

**Location**: `src/components/`

**Characteristics**:
- Stateless (when possible)
- Highly reusable
- Single responsibility
- Prop-driven
- Minimal external dependencies

**Current Components**:

#### MetricCard
Displays a metric with label and value.

````rust
#[component]
pub fn MetricCard(label: String, value: usize) -> Element {
    rsx! {
        div { class: "metric-card",
            span { class: "label", "{label}" }
            span { class: "value", "{value}" }
        }
    }
}
````

#### StatusPill
Visual indicator for container states.

````rust
#[component]
pub fn StatusPill(state: ContainerState) -> Element {
    rsx! {
        span { 
            class: "status-pill {state.css_class()}",
            "{state.label()}"
        }
    }
}
````

#### SectionHeader
Consistent section heading with optional actions.

````rust
#[component]
pub fn SectionHeader(title: String) -> Element {
    rsx! {
        h2 { class: "section-header", "{title}" }
    }
}
````

### 3. State Management (AppState)

**Purpose**: Centralized, reactive application state.

**Location**: `src/utils/app_state.rs`

**Architecture**: Uses Dioxus Signals for fine-grained reactivity.

````rust
#[derive(Clone)]
pub struct AppState {
    // Configuration
    pub docker_host: Signal<String>,
    
    // Data
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    
    // UI State
    pub is_loading: Signal<bool>,
    pub error_message: Signal<Option<String>>,
    pub last_action: Signal<Option<String>>,
    
    // Service instance
    docker_service: Option<DockerService>,
}
````

**State Access Pattern**:

````rust
// In main.rs - provide state
fn App() -> Element {
    let app_state = AppState::new();
    use_context_provider(|| app_state);
    // ...
}

// In any view/component - consume state
fn MyView() -> Element {
    let app_state = use_context::<AppState>();
    let containers = (app_state.containers)(); // Read signal
    // ...
}
````

**State Mutation**:

````rust
impl AppState {
    pub fn start_container(&self, id: String) {
        self.is_loading.set(true);
        
        let service = self.docker_service.clone();
        let containers = self.containers;
        
        spawn(async move {
            if let Some(service) = service {
                service.start_container(&id).await.ok();
                // Refresh data
                let new_containers = service.list_containers().await.ok();
                if let Some(list) = new_containers {
                    containers.set(list);
                }
            }
        });
        
        self.is_loading.set(false);
    }
}
````

### 4. Services Layer

**Purpose**: Abstract external integrations and business logic.

**Location**: `src/services/`

**Current Services**:

#### DockerService

Wraps the Bollard Docker client with domain-specific methods.

````rust
pub struct DockerService {
    docker: Docker,
}

impl DockerService {
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker })
    }
    
    pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
        let options = ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        };
        
        let containers = self.docker
            .list_containers(Some(options))
            .await?;
        
        // Transform to domain model
        Ok(containers.into_iter().map(|c| {
            ContainerInfo {
                id: c.id.unwrap_or_default(),
                name: c.names
                    .and_then(|n| n.first().cloned())
                    .unwrap_or_default(),
                // ... other fields
            }
        }).collect())
    }
}
````

**Design Principles**:
- Single Responsibility: Each service handles one external system
- Error Handling: Convert external errors to domain errors
- Testability: Easy to mock for testing
- Async by Default: All I/O operations are async

## Routing

**Router**: Dioxus 0.7 Router

**Configuration**: `src/main.rs`

````rust
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
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

**Key Features**:
- Type-safe routing
- Layout support (AppShell wraps all routes)
- Automatic URL synchronization
- Navigation via `Link` component

## Asset Management

**Location**: `assets/`

**Structure**:
````
assets/
├── favicon.ico
├── header.svg
├── styling/
│   ├── main.css        # Global styles
│   ├── navbar.css      # Navigation styles
│   ├── echo.css        # Component-specific
│   └── blog.css        # View-specific
└── tailwind.css        # Tailwind entry point
````

**Loading Assets**:

````rust
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }
}
````

The `asset!` macro:
- Resolves paths relative to crate root
- Bundles assets for desktop builds
- Optimizes/minifies CSS and JS

## Data Models

**Location**: `src/services/docker.rs`

Domain models represent Docker entities:

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    Stopped,
}
````

**Design Decisions**:
- `Clone`: Cheap to pass around in UI
- `PartialEq`: Required for Signal change detection
- Enums for states: Type-safe, exhaustive matching

## Async Handling

**Runtime**: Tokio

**Pattern**: Use `spawn` for fire-and-forget operations:

````rust
pub fn refresh_all(&self) {
    let service = self.docker_service.clone();
    let containers = self.containers;
    let images = self.images;
    let volumes = self.volumes;
    
    spawn(async move {
        if let Some(service) = service {
            if let Ok(list) = service.list_containers().await {
                containers.set(list);
            }
            if let Ok(list) = service.list_images().await {
                images.set(list);
            }
            if let Ok(list) = service.list_volumes().await {
                volumes.set(list);
            }
        }
    });
}
````

## Error Handling

**Strategy**: Graceful degradation with user feedback.

````rust
pub fn start_container(&self, id: String) {
    let service = self.docker_service.clone();
    let error_message = self.error_message;
    
    spawn(async move {
        match service.unwrap().start_container(&id).await {
            Ok(_) => {
                error_message.set(None);
                // Success - refresh data
            }
            Err(e) => {
                error_message.set(Some(format!("Failed to start: {}", e)));
            }
        }
    });
}
````

**Display Errors**:

````rust
rsx! {
    if let Some(err) = error_message {
        div { class: "error-banner", "{err}" }
    }
}
````

## Performance Considerations

### Signal Granularity

Use fine-grained signals to minimize re-renders:

````rust
// ✅ Good: Individual signals
pub struct AppState {
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
}

// ❌ Bad: Single signal for everything
pub struct AppState {
    pub data: Signal<AllData>,
}
````

### Memoization

Use `use_memo` for expensive computations:

````rust
let running_count = use_memo(move || {
    containers().iter()
        .filter(|c| c.state == ContainerState::Running)
        .count()
});
````

### Async Loading

Show loading states to maintain responsiveness:

````rust
if is_loading {
    rsx! { div { "Loading..." } }
} else {
    rsx! { ContainerList { containers } }
}
````

## Testing Strategy

### Unit Tests

Test individual functions and methods:

````rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_container_state_label() {
        assert_eq!(ContainerState::Running.label(), "Running");
    }
}
````

### Integration Tests

Test service layer with real Docker:

````rust
#[tokio::test]
async fn test_list_containers() {
    let service = DockerService::new().unwrap();
    let containers = service.list_containers().await;
    assert!(containers.is_ok());
}
````

### Component Tests

Test components with mock data:

````rust
#[test]
fn test_metric_card_render() {
    // Render component
    let vdom = rsx! {
        MetricCard { label: "Test", value: 42 }
    };
    // Assert structure
}
````

## Build System

### Development

````bash
dx serve              # Hot-reload dev server
dx serve --hot-reload # With hot module replacement
````

### Production

````bash
dx build --release    # Optimized build
````

### Features

- `desktop`: Desktop app (default)
- `web`: Web app

## Future Architecture Considerations

### Planned Enhancements

1. **Plugin System**: Allow extensions for custom Docker operations
2. **Multi-host Support**: Manage multiple Docker hosts simultaneously
3. **Offline Mode**: Cache data for offline viewing
4. **WebSocket Updates**: Real-time container status updates
5. **Theme System**: User-customizable themes

### Scalability

Current architecture supports:
- Hundreds of containers without performance degradation
- Multiple Docker hosts (with state refactoring)
- Additional service integrations (Kubernetes, Podman)

## Security Considerations

1. **Docker Socket Access**: Requires privileged access - ensure proper permissions
2. **Error Messages**: Avoid exposing sensitive Docker internals
3. **Input Validation**: Sanitize container names and IDs before API calls
4. **Dependency Audits**: Regular `cargo audit` for known vulnerabilities

## Development Environment

### Tools

- **devenv**: Reproducible development environment
- **Dioxus CLI**: Hot reloading and bundling
- **Cargo**: Build and dependency management

### Editor Integration

Recommended VS Code extensions:
- rust-analyzer
- Dioxus VSCode Extension

---

**Last Updated**: February 2026  
**Version**: 0.1.0
