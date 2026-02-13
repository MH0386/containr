# Contributing to Doctainr

Thank you for your interest in contributing to Doctainr! This guide will help you get started.

## Code of Conduct

This project adheres to principles of respect, inclusivity, and collaboration. Please be kind and constructive in all interactions.

## Getting Started

### Prerequisites

1. **Rust toolchain** (1.70 or later)
2. **Docker** installed and running
3. **Dioxus CLI**: Install with `curl -sSL http://dioxus.dev/install.sh | sh`
4. **Platform dependencies**:
   - Linux: `libwebkit2gtk-4.1-dev`, `libssl-dev`

### Development Setup

````bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/doctainr.git
cd doctainr

# Create a feature branch
git checkout -b feature/my-new-feature

# Install dependencies and run
dx serve
````

Alternatively, use devenv for a complete development environment:

````bash
# Install devenv: https://devenv.sh/getting-started/
devenv shell
devenv up
````

## Development Workflow

### 1. Code Organization

- **`src/main.rs`**: Application entry point and routing
- **`src/components/`**: Reusable UI components
- **`src/services/`**: External integrations (Docker API)
- **`src/utils/`**: State management and utilities
- **`src/views/`**: Page-level components

### 2. Dioxus 0.7 Conventions

This project uses **Dioxus 0.7**, which has breaking changes from 0.6:

- ✅ Use `use_signal` (not `use_state`)
- ✅ Use `#[component]` macro on all components
- ✅ No `cx: Scope` parameter (removed in 0.7)
- ✅ Use Context API for shared state
- ✅ Prefer `rsx!` macro for JSX-like syntax

**Example Component:**

````rust
#[component]
fn MyComponent(name: String) -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        div {
            "Hello, {name}!"
            button {
                onclick: move |_| *count.write() += 1,
                "Clicked {count} times"
            }
        }
    }
}
````

### 3. State Management

Use signals for reactive state:

````rust
// Local state
let mut local_state = use_signal(|| initial_value);

// Global state (via Context)
let app_state = use_context::<AppState>();
let containers = (app_state.containers)();
````

### 4. Async Operations

Use `use_resource` for async operations:

````rust
let data = use_resource(move || async move {
    fetch_data().await
});

match data() {
    Some(result) => rsx! { DisplayData { result } },
    None => rsx! { "Loading..." },
}
````

## Code Quality

### Formatting

````bash
cargo fmt --all
````

### Linting

````bash
cargo clippy --all-targets --all-features -- -D warnings
````

Project-specific clippy rules are in `clippy.toml`.

### Testing

````bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
````

## Making Changes

### Component Guidelines

1. **Keep components focused**: Each component should have a single responsibility
2. **Use descriptive names**: `ContainerStatusPill` vs `StatusComp`
3. **Document props**: Add comments for non-obvious props
4. **Handle loading/error states**: Always account for async operations

### Service Layer

When adding Docker operations:

1. Add method to `DockerService` (`src/services/docker.rs`)
2. Update `AppState` with corresponding state/action
3. Call from UI component via Context API

**Example:**

````rust
// In docker.rs
pub async fn restart_container(&self, id: &str) -> Result<()> {
    self.docker.restart_container(id, None).await?;
    Ok(())
}

// In app_state.rs
pub fn restart_container(&self, id: String) {
    let service = self.docker_service.clone();
    spawn(async move {
        if let Some(service) = service {
            service.restart_container(&id).await.ok();
        }
    });
}
````

### UI/UX Guidelines

- Use existing components from `src/components/` when possible
- Follow the existing CSS structure in `assets/styling/`
- Ensure responsive layouts (consider different screen sizes)
- Provide user feedback (loading states, success/error messages)

## Commit Guidelines

Follow [Conventional Commits](https://www.conventionalcommits.org/):

````
feat: add container restart functionality
fix: resolve Docker connection timeout
docs: update installation instructions
refactor: simplify container state management
test: add tests for image listing
chore: update dependencies
````

### Commit Message Format

````
<type>(<scope>): <subject>

<body>

<footer>
````

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `refactor`: Code refactoring
- `test`: Adding/updating tests
- `chore`: Tooling/dependencies

## Pull Request Process

### Before Submitting

1. ✅ Run `cargo fmt` and `cargo clippy`
2. ✅ Ensure all tests pass
3. ✅ Update documentation if needed
4. ✅ Add tests for new functionality
5. ✅ Rebase on latest `main`

### PR Description Template

````markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
How were these changes tested?

## Screenshots
(if applicable)

## Checklist
- [ ] Code follows project style
- [ ] Self-reviewed the code
- [ ] Commented complex logic
- [ ] Updated documentation
- [ ] No new warnings
- [ ] Added tests
- [ ] All tests pass
````

### Review Process

1. Maintainers will review within 3-5 business days
2. Address feedback promptly
3. Keep PR scope focused (break large changes into multiple PRs)
4. Squash commits if requested

## Adding New Features

### Example: Adding Container Logs View

1. **Create component**: `src/views/logs.rs`

````rust
#[component]
pub fn ContainerLogs(container_id: String) -> Element {
    let logs = use_resource(move || async move {
        fetch_logs(&container_id).await
    });
    
    rsx! {
        div { class: "logs-container",
            match logs() {
                Some(content) => rsx! { pre { "{content}" } },
                None => rsx! { "Loading logs..." }
            }
        }
    }
}
````

2. **Add route**: Update `Route` enum in `src/main.rs`

````rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(AppShell)]
        // ... existing routes
        #[route("/containers/:id/logs")]
        ContainerLogs { id: String },
}
````

3. **Add service method**: In `src/services/docker.rs`

````rust
pub async fn get_logs(&self, id: &str) -> Result<String> {
    // Implementation
}
````

4. **Update navigation**: Add link in `AppShell` or relevant view

5. **Add tests**: Create test in appropriate module

6. **Update documentation**: Add to README.md features section

## Documentation Standards

### Code Comments

````rust
/// Fetches container information from Docker API.
///
/// # Arguments
/// * `id` - Container ID or name
///
/// # Returns
/// * `Ok(ContainerInfo)` - Container details
/// * `Err(Error)` - Connection or API error
pub async fn get_container(&self, id: &str) -> Result<ContainerInfo> {
    // Implementation
}
````

### README Updates

When adding features, update:
- Features list
- Usage section (if user-facing)
- Configuration (if adding new options)
- Troubleshooting (if common issues expected)

## Testing

### Unit Tests

````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state_label() {
        assert_eq!(ContainerState::Running.label(), "Running");
        assert_eq!(ContainerState::Stopped.label(), "Stopped");
    }
}
````

### Integration Tests

Place in `tests/` directory:

````rust
// tests/docker_integration.rs
#[tokio::test]
async fn test_list_containers() {
    let service = DockerService::new().unwrap();
    let containers = service.list_containers().await;
    assert!(containers.is_ok());
}
````

## Getting Help

- **Questions**: Open a [Discussion](https://github.com/MH0386/doctainr/discussions)
- **Bugs**: File an [Issue](https://github.com/MH0386/doctainr/issues)
- **Security**: Email maintainers privately (see SECURITY.md)

## Recognition

Contributors will be acknowledged in:
- GitHub contributors page
- Release notes
- Project README (for significant contributions)

Thank you for contributing to Doctainr! 🎉
