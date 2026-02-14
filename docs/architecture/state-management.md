# State Management

## Overview

Doctainr uses Dioxus 0.7's reactive state management system based on Signals. The application state is centralized in the `AppState` struct and shared throughout the component tree using the Context API.

## Core Concepts

### Signals

Signals are reactive containers that automatically track reads and writes:

```rust
// Create a signal
let mut count = use_signal(|| 0);

// Read the signal (triggers reactivity)
let value = count();

// Write to the signal (triggers re-renders)
*count.write() = 42;

// Mutate in place
count.with_mut(|c| *c += 1);
```

### Context API

The Context API allows sharing state down the component tree:

```rust
// Provider (in App component)
let app_state = AppState::new();
use_context_provider(|| app_state);

// Consumer (in any child component)
let app_state = use_context::<AppState>();
```

## Application State Structure

The `AppState` struct contains all global application state:

**Location**: `src/utils/app_state.rs`

```rust
#[derive(Clone)]
pub struct AppState {
    // Docker configuration
    pub docker_host: Signal<String>,
    
    // Docker resources
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    
    // UI state
    pub last_action: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
    
    // Service instance (not reactive)
    docker_service: Option<DockerService>,
}
```

### State Fields

#### docker_host
- **Type**: `Signal<String>`
- **Purpose**: Docker daemon connection string
- **Default**: `unix:///var/run/docker.sock` or `$DOCKER_HOST`
- **Usage**: Display in settings, configure Docker connection

#### containers
- **Type**: `Signal<Vec<ContainerInfo>>`
- **Purpose**: List of all Docker containers
- **Updated by**: `refresh_containers()`, `refresh_all()`
- **Read by**: Dashboard, Containers view

#### images
- **Type**: `Signal<Vec<ImageInfo>>`
- **Purpose**: List of all Docker images
- **Updated by**: `refresh_images()`, `refresh_all()`
- **Read by**: Dashboard, Images view

#### volumes
- **Type**: `Signal<Vec<VolumeInfo>>`
- **Purpose**: List of all Docker volumes
- **Updated by**: `refresh_volumes()`, `refresh_all()`
- **Read by**: Dashboard, Volumes view

#### last_action
- **Type**: `Signal<Option<String>>`
- **Purpose**: Last action performed (for user feedback)
- **Usage**: Display success messages

#### error_message
- **Type**: `Signal<Option<String>>`
- **Purpose**: Current error message (if any)
- **Usage**: Display error alerts

#### is_loading
- **Type**: `Signal<bool>`
- **Purpose**: Loading state indicator
- **Usage**: Show loading spinners, disable buttons

### State Methods

#### Initialization

```rust
impl AppState {
    pub fn new() -> Self {
        let docker_service = match DockerService::new() {
            Ok(service) => Some(service),
            Err(e) => {
                eprintln!("Failed to connect to Docker: {}", e);
                None
            }
        };
        
        // Initialize signals
        let docker_host = use_signal(|| /* ... */);
        let containers = use_signal(Vec::new);
        // ... more signals
        
        let state = Self {
            docker_host,
            containers,
            // ...
            docker_service,
        };
        
        // Load initial data
        state.refresh_all();
        
        state
    }
}
```

#### Data Refresh Methods

```rust
// Refresh all data
pub fn refresh_all(&self)

// Refresh specific resources
pub fn refresh_containers(&self)
pub fn refresh_images(&self)
pub fn refresh_volumes(&self)
```

**Pattern**: All refresh methods:
1. Set `is_loading` to `true`
2. Spawn async task with DockerService
3. Update respective Signal with results
4. Clear error or set error message
5. Set `is_loading` to `false`

#### Container Actions

```rust
pub fn start_container(&self, id: String)
pub fn stop_container(&self, id: String)
```

**Pattern**: Action methods:
1. Call DockerService method
2. Update `last_action` with success message
3. Refresh affected data
4. Handle errors by updating `error_message`

## State Flow Patterns

### Read-only Access

Components that only display data:

```rust
#[component]
pub fn DisplayData() -> Element {
    let app_state = use_context::<AppState>();
    
    // Read signal by calling it
    let containers = (app_state.containers)();
    
    rsx! {
        div { "Total: {containers.len()}" }
    }
}
```

### Write Access

Components that modify state:

```rust
#[component]
pub fn ModifyData() -> Element {
    let app_state = use_context::<AppState>();
    
    rsx! {
        button {
            onclick: move |_| {
                // Call method on AppState
                app_state.refresh_all()
            },
            "Refresh"
        }
    }
}
```

### Derived State

Computing values from signals:

```rust
#[component]
pub fn DerivedData() -> Element {
    let app_state = use_context::<AppState>();
    let containers = (app_state.containers)();
    
    // Compute derived values
    let running = containers
        .iter()
        .filter(|c| c.state == ContainerState::Running)
        .count();
    
    rsx! {
        div { "Running: {running}" }
    }
}
```

For expensive computations, use `use_memo`:

```rust
let running = use_memo(move || {
    (app_state.containers)()
        .iter()
        .filter(|c| c.state == ContainerState::Running)
        .count()
});
```

## Async State Updates

### Background Tasks

State updates from async operations:

```rust
pub fn refresh_containers(&self) {
    let containers = self.containers;
    let error_message = self.error_message;
    let is_loading = self.is_loading;
    
    // Clone service for async task
    let docker_service = self.docker_service.clone();
    
    spawn(async move {
        *is_loading.write() = true;
        
        match docker_service {
            Some(service) => {
                match service.list_containers().await {
                    Ok(list) => {
                        *containers.write() = list;
                        *error_message.write() = None;
                    }
                    Err(e) => {
                        *error_message.write() = Some(format!("Failed: {}", e));
                    }
                }
            }
            None => {
                *error_message.write() = Some("Docker service not available".to_string());
            }
        }
        
        *is_loading.write() = false;
    });
}
```

**Key Points**:
1. Clone signals before moving into async block
2. Clone service for thread safety
3. Always update `is_loading`
4. Handle both success and error cases
5. Use `spawn` from Dioxus for async tasks

## Local Component State

For state that doesn't need to be shared:

```rust
#[component]
pub fn LocalState() -> Element {
    // Local signal
    let mut count = use_signal(|| 0);
    
    rsx! {
        div { "Local count: {count()}" }
        button {
            onclick: move |_| *count.write() += 1,
            "Increment"
        }
    }
}
```

**Use local state when**:
- Data is only used in one component
- No need to persist across navigation
- UI-specific state (open/closed, selected tab, etc.)

## State Persistence

Currently, state is not persisted between app restarts. All data is fetched fresh on startup.

**Future Enhancement**: Add state persistence with:
- Local storage for settings
- Cache for Docker data
- User preferences

## Performance Considerations

### Minimizing Re-renders

1. **Read signals selectively**: Only read signals you need
2. **Use memos for expensive computations**: Cache derived values
3. **Split large components**: Smaller components re-render less

### Example: Optimized Component

```rust
#[component]
pub fn Optimized() -> Element {
    let app_state = use_context::<AppState>();
    
    // Only read what we need
    let containers = (app_state.containers)();
    
    // Memo for expensive computation
    let running_count = use_memo(move || {
        containers
            .iter()
            .filter(|c| c.state == ContainerState::Running)
            .count()
    });
    
    rsx! {
        // Only re-renders when running_count changes
        div { "Running: {running_count}" }
    }
}
```

### Signal Access Patterns

```rust
// ❌ Avoid: Creates unnecessary dependency
let app_state = use_context::<AppState>();
let _ = app_state.containers();  // Read but not used

// ✅ Good: Only read when needed
let app_state = use_context::<AppState>();
if should_display {
    let containers = app_state.containers();
    // Use containers
}
```

## Error Handling

### Error State Pattern

```rust
// Set error
*app_state.error_message.write() = Some("Error occurred".to_string());

// Clear error
*app_state.error_message.write() = None;

// Display error
let error = (app_state.error_message)();
if let Some(msg) = error {
    rsx! {
        div { class: "error", "{msg}" }
    }
}
```

### Error Recovery

```rust
pub fn retry_operation(&self) {
    // Clear error before retry
    *self.error_message.write() = None;
    
    // Retry the operation
    self.refresh_all();
}
```

## Testing State Management

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_state_initialization() {
        let state = AppState::new();
        
        // Test initial values
        assert_eq!(state.containers().len(), 0);
        assert!(state.error_message().is_none());
        assert!(!state.is_loading());
    }
}
```

### Integration Tests

Test state updates through UI interactions in integration tests.

## Best Practices

### 1. Centralize Shared State
Keep all shared state in `AppState`, use local state for component-specific needs.

### 2. Use Methods for Mutations
Don't write directly to signals from components; use AppState methods:

```rust
// ❌ Avoid
*app_state.containers.write() = new_containers;

// ✅ Preferred
app_state.refresh_containers();
```

### 3. Handle Loading States
Always show loading indicators for async operations.

### 4. Clear Error Messages
Clear errors when starting new operations.

### 5. Clone Signals for Async
Always clone signals before moving into async blocks.

## Related Documentation

- [Architecture Overview](overview.md)
- [Docker Service](docker-service.md)
- [Component Structure](components.md)
- [API Reference: AppState](../api/app-state.md)
