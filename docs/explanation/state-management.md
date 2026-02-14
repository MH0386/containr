# State Management in Dioxus 0.7

Understanding how Doctainr manages application state using Dioxus 0.7's reactive primitives.

## Core Concepts

### Signals: Reactive State Primitive

Dioxus 0.7 introduces `Signal<T>` as the primary state management mechanism. Signals provide:

1. **Automatic tracking** — Components automatically subscribe when reading a signal
2. **Fine-grained reactivity** — Only components using a changed signal re-render
3. **Thread-safe** — Signals work across async boundaries
4. **Copy semantics** — Signals are `Copy`, making them easy to pass around

### Example: Counter

````rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        div {
            "Count: {count}"  // Reading triggers subscription
            button {
                onclick: move |_| *count.write() += 1,  // Writing triggers re-render
                "Increment"
            }
        }
    }
}
````

When `count.write()` is called, any component reading `count()` automatically re-renders.

## AppState: Global State Container

Doctainr centralizes all application state in `AppState`:

````rust
#[derive(Clone)]
pub struct AppState {
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
    docker_service: Option<DockerService>,
}
````

### Why This Design?

1. **Single source of truth** — All state lives in one place
2. **Context API** — Provided once at root, accessible anywhere
3. **Separation of concerns** — State (signals) separate from services (DockerService)
4. **Testability** — State can be mocked without Docker dependency

## Context API: Providing State

At the app root, state is provided to all descendants:

````rust
#[component]
fn App() -> Element {
    let app_state = AppState::new();
    use_context_provider(|| app_state);  // Makes available to all children
    
    rsx! { Router::<Route> {} }
}
````

Any component can access state:

````rust
#[component]
fn Dashboard() -> Element {
    let app_state = use_context::<AppState>();
    let containers = (app_state.containers)();  // Read signal
    // ...
}
````

## Reading Signals

### Method 1: Call the Signal

````rust
let containers = (app_state.containers)();  // Returns Vec<ContainerInfo>
````

This clones the value and subscribes the component to changes.

### Method 2: Read Reference

````rust
let containers_ref = app_state.containers.read();  // Returns ReadOnlySignal
// Use containers_ref as &Vec<ContainerInfo>
````

This borrows without cloning (more efficient for large data).

## Writing Signals

### Method 1: Direct Write

````rust
*app_state.containers.write() = new_containers;
````

Replaces the entire value.

### Method 2: Mutable Reference

````rust
app_state.containers.with_mut(|containers| {
    containers.push(new_container);
});
````

Mutates the value in place.

## Asynchronous State Updates

Doctainr fetches data asynchronously and updates signals:

````rust
impl AppState {
    pub fn refresh_containers(&self) {
        let containers_signal = self.containers;
        let service = self.docker_service.clone();
        
        spawn(async move {
            if let Some(service) = service {
                match service.list_containers().await {
                    Ok(data) => *containers_signal.write() = data,
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        });
    }
}
````

### Why `spawn()`?

- **Non-blocking** — UI remains responsive during API calls
- **Automatic re-render** — Writing to signal triggers component updates
- **Error handling** — Can handle failures gracefully

## Error State Management

Doctainr uses an `Option<String>` signal for errors:

````rust
pub error_message: Signal<Option<String>>,
````

### Setting an Error

````rust
*app_state.error_message.write() = Some("Failed to connect to Docker".to_string());
````

### Displaying Errors

````rust
if let Some(error) = (app_state.error_message)() {
    rsx! {
        div { class: "error-message",
            "⚠️ {error}"
        }
    }
}
````

### Clearing Errors

````rust
*app_state.error_message.write() = None;
````

## Loading State

To show loading indicators:

````rust
pub is_loading: Signal<bool>,
````

````rust
// Before async operation
*app_state.is_loading.write() = true;

spawn(async move {
    let data = fetch_data().await;
    *containers_signal.write() = data;
    *loading_signal.write() = false;
});
````

## Derived State

Computed values that depend on signals:

````rust
let running_count = containers
    .iter()
    .filter(|c| c.state == ContainerState::Running)
    .count();
````

When `containers` signal changes, this component re-runs and recomputes `running_count`.

### Using `use_memo`

For expensive computations:

````rust
let running_count = use_memo(move || {
    (app_state.containers)()
        .iter()
        .filter(|c| c.state == ContainerState::Running)
        .count()
});
````

`use_memo` caches the result and only recomputes when dependencies change.

## State Lifecycle

### Initialization

````rust
impl AppState {
    pub fn new() -> Self {
        let containers = use_signal(Vec::new);
        // Initialize Docker service
        let docker_service = DockerService::new().ok();
        
        let state = Self { containers, docker_service, /* ... */ };
        state.refresh_all();  // Initial data load
        state
    }
}
````

### Cleanup

Signals are automatically cleaned up when the component unmounts. No manual cleanup needed.

## Comparison with Legacy Patterns

### Dioxus 0.6 (Old)
````rust
let cx = use_context::<Scope>();
let state = use_ref(cx, || AppState::new());
let containers = use_state(cx, Vec::new);
````

### Dioxus 0.7 (New)
````rust
let app_state = use_context::<AppState>();
let containers = app_state.containers;  // Signal is Copy
````

**Changes:**
- `Scope` removed — no `cx` parameter
- `use_state` replaced by `use_signal`
- Signals are `Copy` — no `Rc` or cloning needed

## Best Practices

### ✅ Do

- Use signals for all reactive state
- Read signals in render logic to subscribe
- Update signals via `write()` or `with_mut()`
- Keep signals in a centralized `AppState`
- Use `spawn()` for async operations

### ❌ Don't

- Mutate state outside of signals
- Block the main thread with sync I/O
- Read signals in non-reactive contexts (won't subscribe)
- Store heavy data in signals (use references/IDs)

## Performance Considerations

### Fine-Grained Reactivity

Only components reading a changed signal re-render:

````rust
// Component A reads containers
let containers = (app_state.containers)();

// Component B reads images
let images = (app_state.images)();
````

Updating `containers` only re-renders Component A, not B.

### Avoiding Unnecessary Clones

````rust
// Bad: Clones entire vector on every access
let containers = (app_state.containers)();
for container in containers { /* ... */ }

// Good: Borrows without cloning
let containers = app_state.containers.read();
for container in &*containers { /* ... */ }
````

## Debugging State

### Logging State Changes

````rust
*app_state.containers.write() = new_containers;
println!("Containers updated: {} items", new_containers.len());
````

### Inspecting State

Add debug views:

````rust
rsx! {
    div { class: "debug",
        "Containers: {(app_state.containers)().len()}"
        "Images: {(app_state.images)().len()}"
    }
}
````

## Future Enhancements

Planned state management improvements:

- **Persistence** — Save/restore state to localStorage
- **Undo/redo** — State history for user actions
- **Optimistic updates** — Immediate UI feedback before API confirmation
- **State middleware** — Logging, analytics, debugging

---

See also:
- [Architecture](../reference/architecture.md) — Overall system design
- [Docker API Integration](./docker-api.md) — How services populate state
- [Dioxus 0.7 State Guide](https://dioxuslabs.com/learn/0.7/guide/state) — Official documentation
