# Development Guide

> **How-to Guide** - Practical instructions for developing, extending, and maintaining Doctainr.

## Prerequisites

Before you begin, ensure you have:

1. **Docker Desktop** installed and running
   ````bash
   docker info  # Verify Docker is accessible
   ````

2. **Rust toolchain** (1.70+)
   ````bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustc --version  # Should be 1.70 or higher
   ````

3. **Dioxus CLI** (`dx`)
   ````bash
   curl -sSL http://dioxus.dev/install.sh | sh
   dx --version
   ````

4. **Git** for version control
   ````bash
   git --version
   ````

## Getting Started

### Clone and Build

````bash
# Clone the repository
git clone https://github.com/MH0386/doctainr.git
cd doctainr

# Build and run in development mode
dx serve --platform desktop

# Or use cargo directly
cargo run
````

### Project Structure

````
doctainr/
├── src/
│   ├── main.rs              # App entry point & routing
│   ├── components/          # Reusable UI components
│   │   ├── mod.rs
│   │   ├── metric_card.rs
│   │   ├── section_header.rs
│   │   └── status_pill.rs
│   ├── services/            # External service integrations
│   │   ├── mod.rs
│   │   └── docker.rs        # Docker API client
│   ├── utils/               # Shared utilities
│   │   ├── mod.rs
│   │   └── app_state.rs     # Global state management
│   └── views/               # Route views/pages
│       ├── mod.rs
│       ├── dashboard.rs
│       ├── containers.rs
│       ├── images.rs
│       └── volumes.rs
├── assets/                  # Static assets
│   ├── favicon.ico
│   ├── icon.svg
│   └── styling/
│       └── main.css         # Main stylesheet
├── docs/                    # Documentation
├── Cargo.toml              # Rust dependencies
├── Dioxus.toml             # Dioxus configuration
└── README.md               # User documentation
````

## Development Workflow

### Use `dx` Instead of `cargo`

**Important:** Always prefer `dx` over `cargo` for Doctainr development:

````bash
# ✅ Correct
dx serve --platform desktop    # Development with hot reload
dx build --release             # Production build
dx fmt                         # Format code
dx check                       # Type checking

# ❌ Avoid
cargo run
cargo build
````

The `dx` CLI provides:
- Hot reload for faster iteration
- Optimized bundling of assets
- Platform-specific configuration
- Better error messages for Dioxus-specific issues

### Hot Reload Development

````bash
dx serve --platform desktop --hot-reload
````

Changes to Rust code will trigger automatic recompilation and reload.

### Code Quality

````bash
# Format code (required before commit)
dx fmt

# Run linter
dx check
cargo clippy

# Run tests
cargo test
````

## Adding New Features

### Creating a New View

1. **Create the view component** in `src/views/`:

````rust
// src/views/logs.rs
use dioxus::prelude::*;
use crate::components::SectionHeader;
use crate::utils::AppState;

/// View for displaying container logs.
#[component]
pub fn Logs() -> Element {
    let app_state = use_context::<AppState>();
    
    rsx! {
        SectionHeader {
            title: "Logs".to_string(),
            subtitle: Some("Container output".to_string())
        }
        // Your UI here
    }
}
````

2. **Export the component** in `src/views/mod.rs`:

````rust
mod logs;
pub use logs::Logs;
````

3. **Add route** in `src/main.rs`:

````rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(AppShell)]
        // ... existing routes
        #[route("/logs")]
        Logs {},
}
````

4. **Update navigation** in `src/views/mod.rs` (AppShell component).

### Creating a Reusable Component

1. **Create component file** in `src/components/`:

````rust
// src/components/button.rs
use dioxus::prelude::*;

/// A styled button component with variants.
///
/// # Props
///
/// * `label` - Button text
/// * `variant` - Style variant ("primary", "secondary", "danger")
/// * `onclick` - Click event handler
#[component]
pub fn Button(
    label: String,
    variant: String,
    onclick: EventHandler<MouseEvent>
) -> Element {
    rsx! {
        button {
            class: "button {variant}",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}
````

2. **Export in `src/components/mod.rs`**:

````rust
mod button;
pub use button::Button;
````

3. **Use in views**:

````rust
use crate::components::Button;

rsx! {
    Button {
        label: "Click me".to_string(),
        variant: "primary".to_string(),
        onclick: move |_| println!("Clicked!")
    }
}
````

### Working with AppState

#### Reading State

````rust
let app_state = use_context::<AppState>();
let containers = (app_state.containers)();  // Get current value
let error = (app_state.error_message)();    // Read error
````

#### Triggering Actions

````rust
// Refresh data
app_state.refresh_containers();
app_state.refresh_all();

// Container operations
app_state.start_container("container_id".to_string());
app_state.stop_container("container_id".to_string());
````

#### Adding New State Fields

1. **Add Signal field to AppState** (`src/utils/app_state.rs`):

````rust
pub struct AppState {
    // existing fields...
    pub new_field: Signal<YourType>,
}
````

2. **Initialize in `new()`**:

````rust
let new_field = use_signal(|| default_value);
````

3. **Add to struct construction**:

````rust
Self {
    // existing fields...
    new_field,
}
````

### Adding Docker Operations

1. **Add method to DockerService** (`src/services/docker.rs`):

````rust
impl DockerService {
    /// Pulls a Docker image from registry.
    pub async fn pull_image(&self, image: &str) -> Result<()> {
        // Implementation using bollard
        Ok(())
    }
}
````

2. **Add corresponding AppState method** (`src/utils/app_state.rs`):

````rust
impl AppState {
    /// Pulls a Docker image.
    pub fn pull_image(&self, image: String) {
        if let Some(service) = &self.docker_service {
            let service = service.clone();
            let mut last_action = self.last_action.clone();
            
            spawn(async move {
                match service.pull_image(&image).await {
                    Ok(_) => last_action.set(Some(format!("Pulled {}", image))),
                    Err(e) => eprintln!("Error: {}", e),
                }
            });
        }
    }
}
````

## Styling Guide

### CSS Class Conventions

- **Layout**: `.container`, `.card`, `.row`, `.column`
- **Typography**: `.title`, `.subtitle`, `.text`, `.hint`
- **Components**: `.button`, `.pill`, `.badge`, `.metric-card`
- **State**: `.primary`, `.secondary`, `.running`, `.stopped`
- **Utility**: `.error-message`, `.action-bar`

### Adding Styles

Edit `/assets/styling/main.css`:

````css
.my-component {
    /* Your styles */
}
````

Styles are automatically bundled and hot-reloaded during development.

## Testing

### Unit Tests

Add tests to source files:

````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state() {
        assert_eq!(ContainerState::Running.label(), "Running");
    }
}
````

Run tests:

````bash
cargo test
````

### Integration Tests

Create test files in `tests/`:

````rust
// tests/integration_test.rs
#[tokio::test]
async fn test_docker_service() {
    // Test code
}
````

## Debugging

### Enable Debug Logging

````bash
RUST_LOG=debug dx serve --platform desktop
````

### Common Issues

#### Docker Connection Failed

````bash
# Check Docker is running
docker info

# Check DOCKER_HOST environment
echo $DOCKER_HOST

# On Linux, verify socket permissions
ls -l /var/run/docker.sock
````

#### Build Errors

````bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Reinstall dx
curl -sSL http://dioxus.dev/install.sh | sh
````

## Building for Release

### Desktop Application

````bash
# Build optimized binary
dx build --release --platform desktop

# Output location varies by platform:
# - Linux: target/release/doctainr
# - macOS: target/release/doctainr
# - Windows: target/release/doctainr.exe
````

### Size Optimization

The release build includes optimizations:
- LTO (Link Time Optimization)
- Code stripping
- Asset minification

## Documentation

### Code Documentation

Follow rustdoc conventions:

````rust
/// Brief one-line description.
///
/// More detailed explanation if needed.
///
/// # Arguments
///
/// * `arg` - Description of argument
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// When this function errors
///
/// # Example
///
/// ```rust
/// let result = function(arg);
/// ```
pub fn function(arg: Type) -> Result<()> {
    // Implementation
}
````

Generate documentation:

````bash
cargo doc --open
````

### User Documentation

User-facing docs go in:
- `README.md` - Getting started guide
- `docs/` - Technical documentation

## Performance Tips

1. **Minimize Signal Clones** - Only clone signals you need in async closures
2. **Use Memos** - For expensive computations: `use_memo()`
3. **Batch Updates** - Update multiple signals together when possible
4. **Lazy Load Data** - Fetch data only when views are accessed

## Security Best Practices

1. **Never Store Credentials** - Use system Docker authentication
2. **Validate Input** - Sanitize container IDs and names
3. **Error Messages** - Don't expose sensitive system information
4. **Dependencies** - Keep Cargo dependencies updated

## Git Workflow

````bash
# Create feature branch
git checkout -b feature/my-feature

# Make changes, commit frequently
git add .
git commit -m "feat: add new feature"

# Format and lint before pushing
dx fmt
cargo clippy

# Push and create PR
git push origin feature/my-feature
````

## Getting Help

- **Issues**: Report bugs on [GitHub Issues](https://github.com/MH0386/doctainr/issues)
- **Discussions**: Ask questions in [GitHub Discussions](https://github.com/MH0386/doctainr/discussions)
- **Dioxus**: Check [Dioxus documentation](https://dioxuslabs.com/learn/0.7)

---

**Related Documentation:**
- [Architecture](architecture.md) - System design and structure
- [Contributing](contributing.md) - Contribution guidelines
- [Troubleshooting](troubleshooting.md) - Common problems and solutions
