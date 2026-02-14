# Component Structure

## Overview

Doctainr uses a component-based architecture powered by Dioxus. Components are the building blocks of the UI, each encapsulating a specific piece of functionality or visual element.

## Component Organization

### Views (Page Components)

Views are top-level components that correspond to routes in the application.

**Location**: `src/views/`

#### AppShell
The layout wrapper for all pages, containing the navigation sidebar and outlet for child routes.

```rust
#[component]
pub fn AppShell() -> Element {
    rsx! {
        div { class: "app-shell",
            // Navigation sidebar
            nav { /* ... */ }
            // Content area where child routes render
            Outlet::<Route> {}
        }
    }
}
```

#### Dashboard
Overview page showing container, image, and volume counts.

**Route**: `/`

**Features**:
- Displays aggregate statistics
- Shows running vs stopped containers
- Docker host information
- Refresh all button

#### Containers
Container management page.

**Route**: `/containers`

**Features**:
- Lists all containers
- Start/stop controls
- Container details (status, ports, image)
- Individual refresh

#### Images
Docker image browser.

**Route**: `/images`

**Features**:
- Lists all local images
- Shows repository, tag, ID, and size
- Image refresh

#### Volumes
Docker volume manager.

**Route**: `/volumes`

**Features**:
- Lists all volumes
- Shows driver and mount point
- Volume refresh

#### Settings
Application configuration.

**Route**: `/settings`

**Features**:
- Docker host configuration
- Application preferences

### Reusable Components

Small, focused components used across multiple views.

**Location**: `src/components/`

#### MetricCard

Displays a metric with title, value, and optional hint.

**Props**:
```rust
#[component]
pub fn MetricCard(
    title: String,
    value: String,
    hint: Option<String>,
) -> Element
```

**Usage**:
```rust
MetricCard {
    title: "Running containers".to_string(),
    value: running.to_string(),
    hint: Some("Across all projects".to_string())
}
```

**Styling**: Uses `.metric-card` CSS class

#### SectionHeader

Page header with title and optional subtitle.

**Props**:
```rust
#[component]
pub fn SectionHeader(
    title: String,
    subtitle: Option<String>,
) -> Element
```

**Usage**:
```rust
SectionHeader {
    title: "Dashboard".to_string(),
    subtitle: Some("Overview of your local Docker engine".to_string())
}
```

**Styling**: Uses `.section-header` CSS class

#### StatusPill

Colored status indicator for container states.

**Props**:
```rust
#[component]
pub fn StatusPill(
    status: String,
    css_class: String,
) -> Element
```

**Usage**:
```rust
StatusPill {
    status: container.state.label().to_string(),
    css_class: container.state.css_class().to_string()
}
```

**Styling**: Uses `.status-pill` with additional state-specific classes

## Component Patterns

### Component Definition

All components follow these conventions:

```rust
#[component]
pub fn MyComponent(
    // Props: owned types (String, not &str)
    title: String,
    // Optional props
    subtitle: Option<String>,
    // Signals for reactive data
    mut count: Signal<i32>,
) -> Element {
    // Component logic
    
    rsx! {
        // RSX markup
    }
}
```

**Key Points**:
- Annotated with `#[component]` macro
- Function names start with capital letter
- Return type is `Element`
- Props must be owned types
- Use `rsx!` macro for UI

### Accessing Application State

Components can access global state via Context API:

```rust
#[component]
pub fn MyView() -> Element {
    // Get AppState from context
    let app_state = use_context::<AppState>();
    
    // Read signal values by calling them
    let containers = (app_state.containers)();
    
    // Use in component
    rsx! {
        div { "Total containers: {containers.len()}" }
    }
}
```

### Event Handling

Event handlers use closures with move semantics:

```rust
rsx! {
    button {
        onclick: move |_| {
            // Handle click
            app_state.refresh_all()
        },
        "Refresh"
    }
}
```

### Conditional Rendering

Use Rust's standard control flow:

```rust
rsx! {
    // if-let for Option types
    if let Some(error) = error_message {
        div { class: "error", "Error: {error}" }
    }
    
    // if-else
    if is_loading {
        div { "Loading..." }
    } else {
        div { "Data loaded" }
    }
}
```

### Lists and Iteration

Prefer `for` loops over iterators:

```rust
rsx! {
    // Using for loop (preferred)
    for container in containers {
        div { key: "{container.id}",
            "{container.name}"
        }
    }
    
    // Using iterator (wrap in braces)
    {containers.iter().map(|c| rsx! {
        div { "{c.name}" }
    })}
}
```

## Component Lifecycle

### Initialization

Components initialize when first rendered:

```rust
#[component]
pub fn MyComponent() -> Element {
    // Initialize local state
    let mut count = use_signal(|| 0);
    
    // Use use_effect for side effects on mount
    use_effect(move || {
        // Runs once on mount
        println!("Component mounted");
    });
    
    rsx! { /* ... */ }
}
```

### Re-rendering

Components re-render when:
1. Props change (determined by `PartialEq`)
2. Signals they depend on are updated

```rust
#[component]
pub fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        // Reading count() makes this component depend on it
        div { "Count: {count()}" }
        
        button {
            // Writing to count triggers re-render
            onclick: move |_| *count.write() += 1,
            "Increment"
        }
    }
}
```

## Styling Components

### CSS Classes

Components use Tailwind CSS classes via the main stylesheet:

```rust
rsx! {
    div { class: "flex flex-col gap-4 p-4",
        // Content
    }
}
```

### Custom Styles

Additional styles in `/assets/styling/main.css`:

```css
.metric-card {
    background: white;
    border-radius: 8px;
    padding: 1rem;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}
```

## Best Practices

### 1. Keep Components Small
Each component should have a single, clear responsibility.

### 2. Use Props for Configuration
Pass data and callbacks via props rather than accessing global state when possible.

### 3. Minimize State
Only create signals when you need reactivity. Use local variables for non-reactive data.

### 4. Extract Reusable Logic
If logic is shared across components, extract it to utility functions.

### 5. Type Safety
Use strong types for props. Prefer enums over strings for states.

```rust
// Good: Type-safe state
#[derive(Clone, Copy, PartialEq)]
pub enum Status {
    Running,
    Stopped,
}

// Less ideal: String-based state
let status = "running".to_string();
```

### 6. Error Handling
Display errors to users, don't silently fail:

```rust
if let Some(error) = error_message {
    div { class: "error-message",
        "⚠️ {error}"
    }
}
```

## Creating New Components

### Step 1: Create the File

```bash
# For reusable component
touch src/components/my_component.rs

# For view component
touch src/views/my_view.rs
```

### Step 2: Define the Component

```rust
use dioxus::prelude::*;

#[component]
pub fn MyComponent(title: String) -> Element {
    rsx! {
        div { class: "my-component",
            h2 { "{title}" }
        }
    }
}
```

### Step 3: Export from Module

In `src/components/mod.rs` or `src/views/mod.rs`:

```rust
mod my_component;
pub use my_component::MyComponent;
```

### Step 4: Use the Component

```rust
use crate::components::MyComponent;

rsx! {
    MyComponent {
        title: "Hello World".to_string()
    }
}
```

## Testing Components

Component testing focuses on behavior:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_card_renders() {
        // Test component logic
        let title = "Test".to_string();
        let value = "42".to_string();
        
        // Assertions on component behavior
        assert_eq!(title, "Test");
    }
}
```

## Related Documentation

- [Architecture Overview](overview.md)
- [State Management](state-management.md)
- [Creating Custom Components Tutorial](../examples/custom-component.md)
- [Code Style Guide](../guides/code-style.md)
