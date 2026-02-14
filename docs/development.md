# Development Guide

## Setting Up Your Development Environment

### Prerequisites

1. **Rust** (1.70 or later)
   ````bash
   # Install Rust using rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Verify installation
   rustc --version
   cargo --version
   ````

2. **Docker** (running on your system)
   ````bash
   # Verify Docker is running
   docker info
   ````

3. **Dioxus CLI** (recommended)
   ````bash
   # Install dx
   curl -sSL http://dioxus.dev/install.sh | sh
   
   # Or via cargo
   cargo install dioxus-cli
   
   # Verify installation
   dx --version
   ````

4. **Node.js and npm** (for Tailwind CSS)
   ````bash
   # Install Tailwind CSS
   npm install -D tailwindcss
   ````

### Clone and Build

````bash
# Clone the repository
git clone https://github.com/MH0386/doctainr.git
cd doctainr

# Build the project
dx build

# Or with cargo
cargo build
````

## Development Workflow

### Running the Application

````bash
# Development mode with hot reload (recommended)
dx serve --platform desktop

# Standard cargo run (no hot reload)
cargo run

# Run with custom Docker host
DOCKER_HOST=tcp://192.168.1.100:2375 dx serve
````

### Project Structure

````
doctainr/
├── src/
│   ├── main.rs              # Entry point, routing setup
│   ├── components/          # Reusable UI components
│   │   ├── mod.rs
│   │   ├── metric_card.rs
│   │   ├── section_header.rs
│   │   └── status_pill.rs
│   ├── services/            # Business logic, Docker API
│   │   ├── mod.rs
│   │   └── docker.rs
│   ├── utils/               # State management, helpers
│   │   ├── mod.rs
│   │   └── app_state.rs
│   └── views/               # Full-page components
│       ├── mod.rs
│       ├── shell.rs
│       ├── dashboard.rs
│       ├── containers.rs
│       ├── images.rs
│       ├── volumes.rs
│       └── settings.rs
├── assets/
│   ├── styling/main.css     # Custom styles
│   ├── favicon.ico
│   └── icon.svg
├── docs/                    # Documentation
├── Cargo.toml               # Rust dependencies
├── Dioxus.toml              # Dioxus configuration
└── tailwind.css             # Generated Tailwind CSS
````

## Common Development Tasks

### Adding a New Component

1. Create a new file in `src/components/`:

````rust
// src/components/my_component.rs
use dioxus::prelude::*;

/// Component description.
#[component]
pub fn MyComponent(prop: String) -> Element {
    rsx! {
        div { class: "my-component",
            "{prop}"
        }
    }
}
````

2. Export it from `src/components/mod.rs`:

````rust
mod my_component;
pub use my_component::MyComponent;
````

3. Use it in your views:

````rust
use crate::components::MyComponent;

rsx! {
    MyComponent { prop: "Hello".to_string() }
}
````

### Adding a New View

1. Create a new file in `src/views/`:

````rust
// src/views/my_view.rs
use dioxus::prelude::*;
use crate::components::SectionHeader;
use crate::utils::AppState;

#[component]
pub fn MyView() -> Element {
    let app_state = use_context::<AppState>();
    
    rsx! {
        SectionHeader {
            title: "My View".to_string(),
            subtitle: Some("Description".to_string())
        }
        div { "Content goes here" }
    }
}
````

2. Export from `src/views/mod.rs`:

````rust
mod my_view;
pub use my_view::MyView;
````

3. Add a route in `src/main.rs`:

````rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(AppShell)]
        // ... existing routes ...
        #[route("/my-view")]
        MyView {},
}
````

4. Add navigation link in `src/views/shell.rs`:

````rust
nav { class: "nav-list",
    // ... existing links ...
    Link { to: Route::MyView {}, class: "nav-link", "My View" }
}
````

### Adding a Docker Service Method

1. Add method to `src/services/docker.rs`:

````rust
impl DockerService {
    /// New Docker operation description.
    pub async fn my_operation(&self) -> Result<Vec<SomeInfo>> {
        // Implementation using self.docker
        Ok(vec![])
    }
}
````

2. Add corresponding AppState method in `src/utils/app_state.rs`:

````rust
impl AppState {
    /// Triggers the new operation.
    pub fn trigger_my_operation(&self) {
        if let Some(service) = &self.docker_service {
            let service = service.clone();
            spawn(async move {
                match service.my_operation().await {
                    Ok(data) => {
                        // Update signals
                    }
                    Err(e) => {
                        // Handle error
                    }
                }
            });
        }
    }
}
````

### Working with Signals

Signals are Dioxus's reactive state mechanism:

````rust
// Create a signal
let mut count = use_signal(|| 0);

// Read the signal (establishes dependency)
let value = count();

// Write to the signal (triggers re-render)
*count.write() = 42;
count.set(42);  // Alternative

// Mutate the signal
count.with_mut(|val| *val += 1);
````

## Code Quality

### Formatting

````bash
# Format all code
cargo fmt

# Or with dx
dx fmt

# Check formatting without modifying
cargo fmt -- --check
````

### Linting

````bash
# Run Clippy
cargo clippy

# Run with all warnings as errors
cargo clippy -- -D warnings
````

Configuration is in `clippy.toml`:
````toml
too-many-arguments-threshold = 8
type-complexity-threshold = 300
single-char-binding-names-threshold = 5
````

### Testing

````bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
````

### Building Documentation

````bash
# Generate and open API documentation
cargo doc --open

# Build docs without opening
cargo doc --no-deps
````

## Debugging

### Logging

Add debug prints:

````rust
eprintln!("Debug: {:?}", value);
````

For structured logging, consider adding the `tracing` crate.

### Docker Connection Issues

````bash
# Test Docker connection
docker info

# Check socket permissions (Linux/macOS)
ls -l /var/run/docker.sock

# Check Docker service (Linux)
systemctl status docker
````

### Dioxus Debugging

Enable Dioxus developer tools:

````rust
// In main.rs
#[cfg(debug_assertions)]
dioxus::logger::init(dioxus::logger::Level::Debug).ok();
````

## Building for Production

### Release Build

````bash
# Build optimized binary
dx build --release --platform desktop

# Or with cargo
cargo build --release

# Binary location
ls -lh target/release/doctainr
````

### Optimizing Binary Size

Add to `Cargo.toml`:

````toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
panic = "abort"     # Smaller binary
strip = true        # Strip symbols
````

### Cross-Platform Builds

````bash
# For Linux
cargo build --release --target x86_64-unknown-linux-gnu

# For macOS
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# For Windows
cargo build --release --target x86_64-pc-windows-msvc
````

## Development Tools

### Recommended VS Code Extensions

- **rust-analyzer** - Rust language server
- **Even Better TOML** - TOML file support
- **Tailwind CSS IntelliSense** - CSS class suggestions
- **Error Lens** - Inline error display

### Editor Configuration

`.vscode/settings.json`:
````json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "rust-lang.rust-analyzer"
}
````

## Performance Profiling

### Compilation Time

````bash
# Measure build time
cargo clean
time cargo build

# Analyze dependencies
cargo tree
cargo tree --duplicates
````

### Runtime Performance

Use `cargo flamegraph` for profiling:

````bash
# Install flamegraph
cargo install flamegraph

# Profile the application
cargo flamegraph --bin doctainr
````

## Troubleshooting

### Common Issues

**Problem**: "Failed to connect to Docker"

**Solution**: Ensure Docker daemon is running and accessible.

````bash
docker info
sudo systemctl start docker  # Linux
````

**Problem**: Hot reload not working with `dx serve`

**Solution**: Check file permissions and try clearing the cache:

````bash
dx clean
dx serve --platform desktop
````

**Problem**: Compilation errors after pulling changes

**Solution**: Clean and rebuild:

````bash
cargo clean
cargo build
````

**Problem**: Missing styles after build

**Solution**: Regenerate Tailwind CSS:

````bash
npm run build:css  # If configured
# Or manually rebuild Tailwind
````

## Contributing Workflow

1. **Fork** the repository
2. **Create a branch** for your feature: `git checkout -b feature/my-feature`
3. **Make changes** following the code style
4. **Test** your changes: `cargo test`
5. **Format** code: `cargo fmt`
6. **Lint** code: `cargo clippy`
7. **Commit** with clear message: `git commit -m "Add feature X"`
8. **Push** to your fork: `git push origin feature/my-feature`
9. **Open a pull request** with description of changes

See [CONTRIBUTING.md](contributing.md) for detailed guidelines.

## Resources

- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Bollard API Docs](https://docs.rs/bollard/)
- [Docker Engine API](https://docs.docker.com/engine/api/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
