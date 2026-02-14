# State Management in Doctainr

This document explains how Doctainr manages application state using Dioxus 0.7's reactive primitives.

## Overview

Doctainr uses **signal-based reactive state management** powered by Dioxus 0.7. This approach eliminates manual update tracking and provides automatic UI updates when state changes.

## Core Concepts

### Signals

**Signals** are reactive containers that hold values and notify subscribers when values change.

````rust
// Create a signal
let mut count = use_signal(|| 0);

// Read the value (creates subscription)
let current_value = count();

// Write to the signal (triggers updates)
*count.write() += 1;

// Or use with_mut for complex updates
count.with_mut(|c| *c += 1);
````

**Key characteristics:**
- Reading a signal in a component makes that component reactive to changes
- Writing to a signal triggers re-renders of all subscribing components
- Signals are `Copy`, making them easy to pass around

### Context API

The **Context API** provides global state sharing across the component tree without prop drilling.

````rust
// Provide state at a parent level
use_context_provider(|| app_state);

// Consume state in any child component
let state = use_context::<AppState>();
````

## Doctainr's State Architecture

### AppState Structure

Located in `src/utils/app_state.rs`:

````rust
#[derive(Clone)]
pub struct AppState {
    // Docker connection endpoint
    pub docker_host: Signal<String>,
    
    // Docker entities
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    
    // UI state
    pub last_action: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
    
    // Service layer (not reactive)
    docker_service: Option<DockerService>,
}
````

### State Initialization

````rust
impl AppState {
    pub fn new() -> Self {
        // Initialize Docker service
        let docker_service = match DockerService::new() {
            Ok(service) => Some(service),
            Err(e) => {
                eprintln!("Failed to connect to Docker: {}", e);
                None
            }
        };
        
        // Initialize signals with default values
        let docker_host = use_signal(|| {
            std::env::var("DOCKER_HOST")
                .unwrap_or_else(|_| "unix:///var/run/docker.sock".to_string())
        });
        let containers = use_signal(Vec::new);
        let images = use_signal(Vec::new);
        // ... more signals
        
        let state = Self { /* ... */ };
        
        // Load initial data
        state.refresh_all();
        state
    }
}
````

### Providing State to the App

In `src/main.rs`:

````rust
#[component]
fn App() -> Element {
    // Create state once at app root
    let app_state = AppState::new();
    
    // Provide to all child components
    use_context_provider(|| app_state);
    
    rsx! {
        Router::<Route> {}
    }
}
````

## State Flow Patterns

### 1. Reading State

Components subscribe to state by reading signals:

````rust
#[component]
fn Dashboard() -> Element {
    // Get state from context
    let state = use_context::<AppState>();
    
    // Read signals (creates reactive subscriptions)
    let containers = state.containers();
    let images = state.images();
    
    // Component re-renders when containers or images change
    rsx! {
        div { "Containers: {containers.len()}" }
        div { "Images: {images.len()}" }
    }
}
````

### 2. Updating State

State updates happen through `AppState` methods:

````rust
impl AppState {
    pub fn refresh_containers(&self) {
        let Some(docker_service) = self.docker_service.clone() else {
            return;
        };
        
        let containers = self.containers;
        
        spawn(async move {
            match docker_service.list_containers().await {
                Ok(list) => {
                    // Update signal triggers UI re-render
                    containers.set(list);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        });
    }
}
````

### 3. User Actions

Views trigger state updates via event handlers:

````rust
#[component]
fn Containers() -> Element {
    let mut state = use_context::<AppState>();
    
    rsx! {
        button {
            onclick: move |_| {
                // Trigger state refresh
                state.refresh_containers();
            },
            "Refresh"
        }
    }
}
````

## Reactive Data Flow

````
User Action
    ↓
Event Handler (onclick, oninput, etc.)
    ↓
AppState Method Call
    ↓
Async Service Call (DockerService)
    ↓
Signal Update (containers.set(...))
    ↓
Automatic UI Re-render
````

## Advanced Patterns

### Loading States

````rust
pub fn refresh_containers(&self) {
    self.is_loading.set(true);  // Show loading indicator
    
    // ... async operation ...
    
    self.is_loading.set(false);  // Hide loading indicator
}
````

### Error Handling

````rust
pub fn start_container(&self, id: String) {
    let error_message = self.error_message;
    
    spawn(async move {
        match docker_service.start_container(&id).await {
            Ok(_) => {
                error_message.set(None);  // Clear error
            }
            Err(e) => {
                error_message.set(Some(format!("Error: {}", e)));
            }
        }
    });
}
````

### Derived State

Use memos for computed values:

````rust
#[component]
fn Dashboard() -> Element {
    let state = use_context::<AppState>();
    let containers = state.containers();
    
    // Automatically recomputes when containers change
    let running_count = use_memo(move || {
        containers.iter()
            .filter(|c| c.state == ContainerState::Running)
            .count()
    });
    
    rsx! {
        div { "Running: {running_count}" }
    }
}
````

## Best Practices

### Do:
✅ Keep signals in a centralized state struct
✅ Use `spawn` for async operations
✅ Read signals in components to create reactive subscriptions
✅ Update signals through dedicated methods
✅ Use `use_memo` for derived/computed state

### Don't:
❌ Pass signals as props (use Context API instead)
❌ Clone signals unnecessarily (they're already `Copy`)
❌ Perform long operations in signal setters
❌ Update multiple related signals without coordination
❌ Read signals unnecessarily (creates subscriptions)

## Performance Considerations

### Granular Updates

Only subscribing components re-render:

````rust
// Only Dashboard re-renders when containers change
let containers = state.containers();

// Volumes view doesn't re-render (not subscribed)
````

### Batched Updates

Multiple signal updates in a single tick are batched:

````rust
// These updates are batched together
self.containers.set(new_containers);
self.images.set(new_images);
self.is_loading.set(false);
// UI updates once after all three complete
````

### Avoiding Unnecessary Reads

````rust
// ❌ Bad: Creates subscription even though value isn't used
let _containers = state.containers();

// ✅ Good: Only read when needed
if should_display {
    let containers = state.containers();
    // ...
}
````

## Comparison with Other Patterns

| Pattern | Dioxus Signals | Redux | React Context | Vue Composition |
|---------|---------------|-------|---------------|-----------------|
| Boilerplate | Minimal | High | Medium | Low |
| Granular Updates | ✅ | ❌ | ❌ | ✅ |
| Type Safety | ✅ | Partial | ✅ | Partial |
| Learning Curve | Low | High | Medium | Medium |

## Testing State

````rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_initialization() {
        let state = AppState::new();
        
        // Test initial values
        assert_eq!(state.containers().len(), 0);
        assert_eq!(state.is_loading(), false);
    }
}
````

## Further Reading

- [Dioxus 0.7 State Management](https://dioxuslabs.com/learn/0.7/state)
- [Architecture Overview](../reference/architecture.md)
- [Docker Integration Details](docker-integration.md)
