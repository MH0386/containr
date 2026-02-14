# Code Style Guide

This guide defines code conventions and best practices for Doctainr.

## General Principles

1. **Clarity over cleverness**: Write code that's easy to understand
2. **Consistency**: Follow existing patterns in the codebase
3. **Simplicity**: Prefer simple solutions over complex ones
4. **Type safety**: Leverage Rust's type system for correctness

## Rust Code Style

### Formatting

Use `rustfmt` for all code formatting:

```bash
# Format all code
cargo fmt

# Check formatting (CI)
cargo fmt --check
```

**Key formatting rules** (enforced by rustfmt):
- 4 spaces for indentation (no tabs)
- 100 character line limit
- Consistent spacing around operators
- Trailing commas in multi-line expressions

### Naming Conventions

#### Types and Traits

```rust
// PascalCase for types
struct ContainerInfo { }
enum ContainerState { }
trait DockerClient { }

// Descriptive names
struct ImageMetadata { }  // Good
struct ImgData { }        // Avoid abbreviations
```

#### Functions and Variables

```rust
// snake_case for functions and variables
fn list_containers() { }
let container_count = 0;

// Descriptive names
fn format_container_status() { }  // Good
fn fmt_status() { }               // Avoid

// Boolean functions use is/has/can prefix
fn is_running() -> bool { }
fn has_volumes() -> bool { }
fn can_start() -> bool { }
```

#### Constants

```rust
// SCREAMING_SNAKE_CASE for constants
const MAX_RETRIES: u32 = 3;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
```

#### Type Parameters

```rust
// Single uppercase letter or PascalCase
fn generic<T>(value: T) { }
fn with_state<State>(state: State) { }
```

### Code Organization

#### Import Groups

Organize imports in three groups, separated by blank lines:

```rust
// 1. Standard library
use std::collections::HashMap;
use std::time::Duration;

// 2. External crates
use anyhow::Result;
use bollard::Docker;
use dioxus::prelude::*;

// 3. Local modules
use crate::services::DockerService;
use crate::utils::AppState;
```

#### Module Structure

```rust
// mod.rs exports public API
mod docker;
pub use docker::{DockerService, ContainerInfo};

// Internal module (private)
mod internal;
use internal::Helper;
```

### Documentation

#### Public Items

Document all public items with doc comments:

```rust
/// Lists all Docker containers on the system.
///
/// This includes both running and stopped containers. Containers are
/// returned with their current state, image information, and port mappings.
///
/// # Errors
///
/// Returns an error if the Docker daemon is not accessible or if the
/// operation fails for any reason.
///
/// # Examples
///
/// ```
/// let service = DockerService::new()?;
/// let containers = service.list_containers().await?;
/// for container in containers {
///     println!("{}: {}", container.name, container.state.label());
/// }
/// ```
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
    // Implementation
}
```

#### Private Items

Document complex private items when helpful:

```rust
// Simple helper function - no doc comment needed
fn format_size(bytes: i64) -> String { }

// Complex logic - document it
/// Parses Docker API container state into our state enum.
/// Handles various status strings and edge cases.
fn parse_container_state(status: &str) -> ContainerState { }
```

### Error Handling

#### Use Result for Fallible Operations

```rust
// Good: Returns Result
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
    let response = self.client.list_containers(None).await?;
    Ok(parse_containers(response))
}

// Avoid: Panics on error
pub async fn list_containers(&self) -> Vec<ContainerInfo> {
    self.client.list_containers(None).await.unwrap()  // Bad!
}
```

#### Provide Context for Errors

```rust
use anyhow::Context;

// Add context to errors
pub async fn start_container(&self, id: &str) -> Result<()> {
    self.client
        .start_container(id, None)
        .await
        .context(format!("Failed to start container {}", id))?;
    Ok(())
}
```

#### Handle Errors Appropriately

```rust
// In UI code: display errors to user
match docker_service.list_containers().await {
    Ok(containers) => {
        *app_state.containers.write() = containers;
        *app_state.error_message.write() = None;
    }
    Err(e) => {
        *app_state.error_message.write() = Some(format!("Failed to list containers: {}", e));
    }
}
```

### Type Design

#### Use Enums for States

```rust
// Good: Type-safe states
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    Stopped,
}

// Avoid: Stringly-typed
let state = "running";  // Error-prone
```

#### Derive Common Traits

```rust
// Always derive Debug
#[derive(Debug)]
struct MyType { }

// Derive Clone for types in signals
#[derive(Clone, Debug)]
struct ContainerInfo { }

// Derive PartialEq for comparison
#[derive(Clone, Debug, PartialEq, Eq)]
enum State { }
```

#### Use NewType Pattern for Clarity

```rust
// Good: Clear intent
struct ContainerId(String);
struct ImageTag(String);

// Less clear
fn remove(id: String, tag: String) { }  // Which is which?
fn remove(id: ContainerId, tag: ImageTag) { }  // Clear!
```

## Dioxus Code Style

### Component Definition

```rust
/// Component that displays container information.
///
/// Shows container name, status, image, and ports with controls
/// to start/stop the container.
#[component]
pub fn ContainerCard(
    // Use owned types for props
    container_id: String,
    container_name: String,
    // Signals for reactive data
    state: ReadOnlySignal<ContainerState>,
) -> Element {
    rsx! {
        // Component JSX
    }
}
```

### RSX Style

#### Indentation and Formatting

```rust
rsx! {
    div { class: "container",
        h1 { "Title" }
        p { class: "description",
            "Some text"
        }
        // Prefer explicit closing for clarity
        button {
            onclick: move |_| handle_click(),
            "Click me"
        }
    }
}
```

#### Conditional Rendering

```rust
rsx! {
    // if-let for Options
    if let Some(error) = error_message {
        div { class: "error",
            "Error: {error}"
        }
    }
    
    // if-else
    if is_loading {
        div { "Loading..." }
    } else {
        div { "Loaded!" }
    }
}
```

#### Loops and Iteration

```rust
rsx! {
    // Prefer for loops over iterators
    for container in containers {
        div { key: "{container.id}",
            ContainerCard {
                container_id: container.id.clone(),
                container_name: container.name.clone(),
            }
        }
    }
    
    // Use iterators when you need map/filter
    {containers.iter()
        .filter(|c| c.state == ContainerState::Running)
        .map(|c| rsx! {
            div { "{c.name}" }
        })}
}
```

### Event Handlers

```rust
rsx! {
    button {
        // Move required for closures in RSX
        onclick: move |_event| {
            // Handle click
            app_state.refresh_all()
        },
        "Refresh"
    }
    
    input {
        // Events provide typed data
        oninput: move |event| {
            let value = event.value();
            // Use value
        },
    }
}
```

### Props

```rust
// Define props clearly
#[component]
pub fn MyComponent(
    // Required props
    title: String,
    count: i32,
    // Optional props with default
    #[props(default = false)]
    show_details: bool,
    // Optional props
    description: Option<String>,
) -> Element {
    rsx! { /* ... */ }
}

// Use component
rsx! {
    MyComponent {
        title: "Test".to_string(),
        count: 42,
        show_details: true,
        description: Some("Details".to_string()),
    }
}
```

## Project-Specific Conventions

### Signal Usage

```rust
// Create signals with use_signal
let mut count = use_signal(|| 0);

// Read by calling the signal
let value = count();

// Write with write()
*count.write() = 42;

// Mutate with with_mut
count.with_mut(|c| *c += 1);
```

### AppState Access

```rust
#[component]
pub fn MyView() -> Element {
    // Get AppState from context
    let app_state = use_context::<AppState>();
    
    // Read signals by calling them
    let containers = (app_state.containers)();
    
    // Call methods on AppState
    app_state.refresh_all();
    
    rsx! { /* ... */ }
}
```

### Async Operations

```rust
// Spawn async tasks with spawn
spawn(async move {
    match docker_service.list_containers().await {
        Ok(containers) => {
            *signal.write() = containers;
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
});
```

## Testing Style

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected);
    }
    
    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Test Naming

```rust
#[test]
fn test_container_state_label_running() { }  // Good

#[test]
fn test1() { }  // Bad - unclear
```

## Linting

### Clippy

Run Clippy regularly:

```bash
# Check for issues
cargo clippy

# Treat warnings as errors (CI)
cargo clippy -- -D warnings
```

### Custom Lint Configuration

See `clippy.toml` for project-specific lint configuration.

## Comments

### When to Comment

```rust
// Good: Explain WHY, not WHAT
// Retry connection because Docker may not be ready immediately after startup
for _ in 0..MAX_RETRIES {
    // ...
}

// Bad: Comments that repeat the code
// Increment counter
counter += 1;
```

### TODO Comments

```rust
// TODO: Add pagination support for large container lists
// TODO(@username): Consider caching Docker responses
// FIXME: Handle edge case where container has no ports
```

## Code Review Checklist

Before submitting code:

- [ ] Runs `cargo fmt`
- [ ] Runs `cargo clippy -- -D warnings`
- [ ] All tests pass: `cargo test`
- [ ] Public items have documentation
- [ ] Error handling is appropriate
- [ ] No unwrap() or panic() in production code
- [ ] Follows naming conventions
- [ ] Consistent with existing code style

## Anti-Patterns to Avoid

### Don't Use unwrap() in Production Code

```rust
// Bad
let containers = docker.list_containers().await.unwrap();

// Good
let containers = docker.list_containers().await?;
```

### Don't Ignore Errors

```rust
// Bad
let _ = docker.start_container(id).await;

// Good
match docker.start_container(id).await {
    Ok(_) => {/* success */},
    Err(e) => {/* handle error */},
}
```

### Don't Use Magic Numbers

```rust
// Bad
thread::sleep(Duration::from_secs(5));

// Good
const RETRY_DELAY: Duration = Duration::from_secs(5);
thread::sleep(RETRY_DELAY);
```

### Don't Repeat Yourself (DRY)

```rust
// Bad: Repeated logic
fn format_container_a() { /* same logic */ }
fn format_container_b() { /* same logic */ }

// Good: Extract common logic
fn format_container(container: &Container) -> String { }
```

## Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7)
- [Rust Book](https://doc.rust-lang.org/book/)

## Related Documentation

- [Contributing Guidelines](contributing.md)
- [Development Guide](development.md)
- [Architecture Overview](../architecture/overview.md)
