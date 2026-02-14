# Development Guide

## Prerequisites

### Required Tools

1. **Docker Desktop or Docker Engine**
   - macOS: [Docker Desktop for Mac](https://docs.docker.com/desktop/install/mac-install/)
   - Linux: [Docker Engine](https://docs.docker.com/engine/install/)
   - Windows: [Docker Desktop for Windows](https://docs.docker.com/desktop/install/windows-install/)
   
   Verify installation:
   ````bash
   docker info
   docker ps
   ````

2. **Rust Toolchain** (1.70 or later)
   
   Install via rustup:
   ````bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustc --version
   cargo --version
   ````

3. **Dioxus CLI** (dx)
   
   Install from official script:
   ````bash
   curl -sSL http://dioxus.dev/install.sh | sh
   dx --version
   ````

### Optional Tools

- **devenv**: Nix-based development environment (see `devenv.nix`)
- **Tailwind CSS**: For style modifications (`npm install -g tailwindcss`)

## Getting Started

### Clone and Build

````bash
# Clone repository
git clone https://github.com/MH0386/doctainr.git
cd doctainr

# Build project
dx build

# Run application
dx serve --platform desktop
````

### Project Setup with devenv (Optional)

If using Nix and devenv:

````bash
# Enter development shell
devenv shell

# All tools (Rust, dx, Docker) pre-configured
dx serve --platform desktop
````

## Development Workflow

### Running the Application

**Development mode with hot reload:**
````bash
dx serve --platform desktop
````

**Standard Cargo run:**
````bash
cargo run --features desktop
````

**Release build:**
````bash
cargo build --release --features desktop
````

The compiled binary will be in `target/release/doctainr`.

### Code Quality

#### Formatting

Use `dx fmt` (preferred) or `cargo fmt`:

````bash
# Format all code
dx fmt

# Check formatting without changes
dx fmt --check
````

#### Linting

Use Clippy for lint checks:

````bash
# Run Clippy
dx check

# Or with cargo
cargo clippy --all-targets --all-features
````

Custom Clippy rules are in `clippy.toml`:
- `too_many_arguments`: Allow up to 10 function arguments
- `type_complexity`: Allow complex types

#### Testing

````bash
# Run all tests
cargo test

# Run specific test
cargo test test_format_size

# Run with output
cargo test -- --nocapture
````

Tests require Docker daemon running for integration tests.

## Project Structure

### Source Code Organization

````
src/
├── main.rs              # Entry point, routing, App component
├── components/          # Reusable UI components
│   ├── mod.rs
│   ├── metric_card.rs   # Dashboard metric cards
│   ├── section_header.rs # Page headers
│   └── status_pill.rs   # Status indicators
├── services/            # External integrations
│   ├── mod.rs
│   └── docker.rs        # Docker API wrapper (Bollard)
├── utils/               # Shared utilities
│   ├── mod.rs
│   └── app_state.rs     # Global state management
└── views/               # Page components
    ├── mod.rs
    ├── dashboard.rs     # Dashboard view
    ├── containers.rs    # Container management
    ├── images.rs        # Image browser
    ├── volumes.rs       # Volume manager
    ├── settings.rs      # Settings page
    └── shell.rs         # Layout/navigation
````

### Configuration Files

- **Cargo.toml**: Rust dependencies and features
- **Dioxus.toml**: Dioxus app configuration
- **clippy.toml**: Clippy linting rules
- **devenv.nix**: Nix development environment
- **.deepsource.toml**: Static analysis configuration

## Adding Features

### Creating a New View

1. **Create component file** in `src/views/`:

````rust
// src/views/my_view.rs
use dioxus::prelude::*;
use crate::utils::AppState;

#[component]
pub fn MyView() -> Element {
    let app_state = use_context::<AppState>();
    
    rsx! {
        div { class: "container",
            h1 { "My New View" }
        }
    }
}
````

2. **Export in `src/views/mod.rs`**:

````rust
pub use my_view::MyView;
mod my_view;
````

3. **Add route in `src/main.rs`**:

````rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(AppShell)]
        // ... existing routes
        #[route("/my-view")]
        MyView {},
}
````

4. **Add navigation link** in `src/views/shell.rs`:

````rust
a {
    href: "/my-view",
    class: "nav-link",
    "My View"
}
````

### Adding a Docker Operation

1. **Add method to `DockerService`** in `src/services/docker.rs`:

````rust
pub async fn remove_container(&self, id: &str) -> Result<()> {
    use bollard::container::RemoveContainerOptions;
    
    let options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });
    
    self.docker.remove_container(id, options).await?;
    Ok(())
}
````

2. **Add state method** in `src/utils/app_state.rs`:

````rust
pub fn remove_container(&self, id: String) {
    if let Some(service) = &self.docker_service {
        let service = service.clone();
        let mut last_action = self.last_action.clone();
        let mut error_message = self.error_message.clone();
        let app_state = self.clone();
        
        spawn(async move {
            match service.remove_container(&id).await {
                Ok(_) => {
                    last_action.set(Some(format!("Removed container {}", id)));
                    error_message.set(None);
                    app_state.refresh_containers();
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to remove: {}", e)));
                }
            }
        });
    }
}
````

3. **Call from UI** in relevant view:

````rust
button {
    onclick: move |_| app_state.remove_container(container.id.clone()),
    "Remove"
}
````

### Creating a Custom Component

1. **Create component file** in `src/components/`:

````rust
// src/components/button.rs
use dioxus::prelude::*;

#[component]
pub fn Button(
    label: String,
    onclick: EventHandler<MouseEvent>,
    variant: Option<String>,
) -> Element {
    let class = format!("button {}", variant.unwrap_or_default());
    
    rsx! {
        button {
            class: "{class}",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}
````

2. **Export in `src/components/mod.rs`**:

````rust
pub use button::Button;
mod button;
````

3. **Use in views**:

````rust
use crate::components::Button;

rsx! {
    Button {
        label: "Click Me".to_string(),
        onclick: |_| println!("Clicked!"),
        variant: Some("primary".to_string()),
    }
}
````

## Styling

### CSS Structure

Main stylesheet: `assets/styling/main.css`

- **Variables**: CSS custom properties for colors, spacing
- **Layout**: Flex/grid layouts for responsiveness
- **Components**: Component-specific styles
- **Utilities**: Helper classes

### Modifying Styles

1. Edit `assets/styling/main.css`
2. Use class names in `rsx!`:

````rust
rsx! {
    div { class: "container",
        button { class: "button primary", "Click" }
    }
}
````

3. Changes are hot-reloaded with `dx serve`

### Tailwind CSS (Future)

Tailwind is configured but minimally used. To enable:

1. Install Tailwind CLI globally
2. Run build watcher: `tailwindcss -i tailwind.css -o assets/tailwind.css --watch`
3. Import in components: `document::Link { href: asset!("/assets/tailwind.css") }`

## Debugging

### Rust Debugging

**Print debugging:**
````rust
println!("Debug: {:?}", value);
eprintln!("Error: {}", error);
````

**Using debugger:**
````bash
# With rust-lldb (macOS/Linux)
rust-lldb target/debug/doctainr

# With rust-gdb (Linux)
rust-gdb target/debug/doctainr
````

### Docker Connection Issues

Check Docker socket:
````bash
# Unix socket (Linux/macOS)
ls -la /var/run/docker.sock

# Environment variable
echo $DOCKER_HOST

# Test connection
docker ps
````

If Docker is not accessible, Doctainr will display error messages in the UI.

## Testing

### Running Tests

````bash
# All tests
cargo test

# Specific module
cargo test services::docker

# With output
cargo test -- --nocapture --test-threads=1
````

### Writing Tests

**Unit test in module:**
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

**Async test with Tokio:**
````rust
#[tokio::test]
async fn test_docker_service() {
    let service = DockerService::new().unwrap();
    let containers = service.list_containers().await.unwrap();
    assert!(containers.len() >= 0);
}
````

## Common Issues

### "Cannot connect to Docker daemon"

**Solution:**
- Ensure Docker Desktop/Engine is running
- Check Docker socket permissions: `ls -la /var/run/docker.sock`
- Add user to `docker` group (Linux): `sudo usermod -aG docker $USER`

### Dioxus CLI not found

**Solution:**
````bash
curl -sSL http://dioxus.dev/install.sh | sh
export PATH="$HOME/.cargo/bin:$PATH"
````

### Hot reload not working

**Solution:**
- Ensure using `dx serve` (not `cargo run`)
- Check file permissions in project directory
- Restart `dx serve`

### Compilation errors after dependency update

**Solution:**
````bash
cargo clean
cargo update
cargo build
````

## Performance Tips

1. **Use release builds** for performance testing:
   ````bash
   cargo build --release
   ````

2. **Profile with `cargo flamegraph`**:
   ````bash
   cargo install flamegraph
   cargo flamegraph
   ````

3. **Reduce async spawns**: Batch operations when possible

4. **Optimize Docker API calls**: Cache results, minimize polling

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on:
- Code style conventions
- Pull request process
- Issue reporting
- Communication channels

## Resources

- **Dioxus 0.7 Documentation**: https://dioxuslabs.com/learn/0.7
- **Bollard API Docs**: https://docs.rs/bollard/
- **Docker API Reference**: https://docs.docker.com/engine/api/
- **Rust Book**: https://doc.rust-lang.org/book/
- **Async Rust**: https://rust-lang.github.io/async-book/
