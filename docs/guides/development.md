# Development Guide

This guide helps you set up a development environment for contributing to Doctainr.

## Prerequisites

### Required Tools

1. **Rust Toolchain** (1.70+)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

2. **Docker**
```bash
# Verify Docker is installed and running
docker info
```

3. **Git**
```bash
git --version
```

### Recommended Tools

1. **Dioxus CLI** (for hot-reload)
```bash
cargo install dioxus-cli
dx --version
```

2. **Code Editor with Rust support**
   - VS Code with rust-analyzer
   - IntelliJ IDEA with Rust plugin
   - Vim/Neovim with rust-analyzer

3. **cargo-watch** (for automatic rebuilds)
```bash
cargo install cargo-watch
```

### Platform-Specific Dependencies

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

#### Linux (Fedora)
```bash
sudo dnf install \
  webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  gtk3-devel \
  libappindicator-gtk3-devel \
  librsvg2-devel
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### Windows
- Install Visual Studio Build Tools with C++ support
- Install Docker Desktop

## Setting Up the Project

### 1. Clone the Repository

```bash
git clone https://github.com/MH0386/doctainr.git
cd doctainr
```

### 2. Verify Project Structure

```bash
tree -L 2 .
```

Expected structure:
```
.
├── Cargo.toml
├── Cargo.lock
├── Dioxus.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── components/
│   ├── views/
│   ├── services/
│   └── utils/
├── assets/
├── docs/
└── .github/
```

### 3. Install Dependencies

```bash
# Fetch and compile dependencies
cargo build
```

This may take several minutes the first time.

### 4. Verify Setup

```bash
# Run the application
cargo run

# Or with Dioxus CLI (hot-reload)
dx serve --platform desktop
```

If the application opens successfully, your setup is complete!

## Development Workflow

### Running the Application

#### Standard Cargo

```bash
# Development build (faster compilation, slower runtime)
cargo run

# Release build (slower compilation, faster runtime)
cargo run --release
```

#### Dioxus CLI (Recommended)

```bash
# Hot-reload development mode
dx serve --platform desktop

# Build for release
dx build --release --platform desktop
```

**Hot-reload** automatically rebuilds and refreshes when you save changes.

### File Watching

Use `cargo-watch` for automatic rebuilds:

```bash
# Watch and run
cargo watch -x run

# Watch and test
cargo watch -x test

# Watch and check (faster than build)
cargo watch -x check
```

### Project Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build

# Run
cargo run                      # Run debug
cargo run --release            # Run release

# Test
cargo test                     # Run all tests
cargo test test_name           # Run specific test
cargo test -- --nocapture      # Show output

# Check (fast, no binary)
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
cargo clippy -- -D warnings    # Treat warnings as errors

# Clean build artifacts
cargo clean

# Update dependencies
cargo update
```

## Code Organization

### Module Structure

```
src/
├── main.rs              # Entry point, app setup, routing
├── components/          # Reusable UI components
│   ├── mod.rs          # Module exports
│   ├── metric_card.rs  # MetricCard component
│   ├── section_header.rs
│   └── status_pill.rs
├── views/               # Page components (routes)
│   ├── mod.rs
│   ├── dashboard.rs    # Dashboard view
│   ├── containers.rs   # Containers view
│   ├── images.rs       # Images view
│   ├── volumes.rs      # Volumes view
│   ├── settings.rs     # Settings view
│   └── shell.rs        # AppShell layout
├── services/            # External integrations
│   ├── mod.rs
│   └── docker.rs       # Docker API wrapper
└── utils/               # Shared utilities
    ├── mod.rs
    └── app_state.rs    # Application state
```

### Adding New Files

When adding a new module:

1. **Create the file**
```bash
touch src/components/my_component.rs
```

2. **Add to mod.rs**
```rust
// In src/components/mod.rs
mod my_component;
pub use my_component::MyComponent;
```

3. **Use in other modules**
```rust
use crate::components::MyComponent;
```

## Making Changes

### Development Cycle

1. **Create a branch**
```bash
git checkout -b feature/my-feature
```

2. **Make changes**
   - Edit source files
   - Add tests
   - Update documentation

3. **Test your changes**
```bash
cargo test
cargo clippy
cargo fmt
```

4. **Run the application**
```bash
dx serve --platform desktop
```

5. **Commit changes**
```bash
git add .
git commit -m "feat: add my feature"
```

### Common Development Tasks

#### Adding a New Component

See [Creating Custom Components Tutorial](../examples/custom-component.md).

#### Adding a New View

See [Adding a New View Tutorial](../examples/new-view.md).

#### Modifying Docker Service

1. Edit `src/services/docker.rs`
2. Add new methods to `DockerService`
3. Update `AppState` if needed
4. Add tests
5. Use in views

#### Updating UI

1. Edit component in `src/views/` or `src/components/`
2. Modify RSX markup
3. Save and see hot-reload (if using dx)
4. Test interactions

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_container_state

# Show output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored

# Test documentation examples
cargo test --doc
```

### Writing Tests

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

#### Async Tests

```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

#### Integration Tests

Create files in `tests/` directory:

```rust
// tests/integration_test.rs
use doctainr::services::DockerService;

#[tokio::test]
async fn test_docker_connection() {
    let service = DockerService::new().expect("Docker not available");
    let containers = service.list_containers().await;
    assert!(containers.is_ok());
}
```

## Code Quality

### Formatting

```bash
# Check formatting
cargo fmt --check

# Format code
cargo fmt
```

### Linting

```bash
# Run Clippy
cargo clippy

# Treat warnings as errors
cargo clippy -- -D warnings

# Fix automatically (when possible)
cargo clippy --fix
```

### Code Coverage (Optional)

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Debugging

### Debug Logging

Add debug output:

```rust
// In code
println!("Debug: {:?}", value);
eprintln!("Error: {}", error);

// Run with debug logging
RUST_LOG=debug cargo run
```

### Using Debugger

#### VS Code

Create `.vscode/launch.json`:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            "cargo": {
                "args": [
                    "build",
                    "--bin=doctainr",
                    "--package=doctainr"
                ]
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

#### Command Line (GDB/LLDB)

```bash
# Build with debug symbols
cargo build

# Run with debugger
rust-gdb ./target/debug/doctainr
# or
rust-lldb ./target/debug/doctainr
```

### Performance Profiling

```bash
# Build with release optimizations and debug symbols
cargo build --release

# Profile with perf (Linux)
perf record ./target/release/doctainr
perf report

# Profile with Instruments (macOS)
# Use Xcode Instruments to profile
```

## Dependencies

### Adding Dependencies

```bash
# Add to Cargo.toml
cargo add <crate-name>

# Or manually edit Cargo.toml
[dependencies]
new-crate = "1.0"

# Then update
cargo build
```

### Updating Dependencies

```bash
# Update all dependencies
cargo update

# Update specific dependency
cargo update <crate-name>

# Check for outdated dependencies
cargo install cargo-outdated
cargo outdated
```

## Documentation

### Code Documentation

Write doc comments:

```rust
/// Brief description of the function.
///
/// More detailed explanation of what it does,
/// how it works, and any important notes.
///
/// # Arguments
/// * `param` - Description of parameter
///
/// # Returns
/// Description of return value
///
/// # Examples
/// ```
/// let result = my_function(42);
/// ```
///
/// # Errors
/// Describes when function returns errors
pub fn my_function(param: i32) -> Result<i32> {
    // Implementation
}
```

### Generating Documentation

```bash
# Build documentation
cargo doc

# Build and open in browser
cargo doc --open

# Include private items
cargo doc --document-private-items
```

## Continuous Integration

The project uses GitHub Actions for CI. Workflows are in `.github/workflows/`.

### Local CI Simulation

```bash
# Run same checks as CI
cargo test
cargo clippy -- -D warnings
cargo fmt --check
cargo build --release
```

## Troubleshooting Development Issues

### "Linker 'cc' not found"

Install C compiler:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential
```

### WebKit errors (Linux)

Install WebKit:
```bash
sudo apt-get install libwebkit2gtk-4.1-dev
```

### Slow Compilation

Use incremental compilation (enabled by default) and:
```bash
# Use cargo check instead of build for faster iteration
cargo check
```

### Dependency Conflicts

```bash
# Clean and rebuild
cargo clean
cargo build
```

## Best Practices

1. **Write tests** for new functionality
2. **Run clippy** before committing
3. **Format code** with cargo fmt
4. **Update documentation** when changing APIs
5. **Keep commits focused** and atomic
6. **Write clear commit messages**
7. **Test on target platforms** before PR

## Related Documentation

- [Contributing Guidelines](contributing.md)
- [Code Style Guide](code-style.md)
- [Architecture Overview](../architecture/overview.md)
- [Testing Guide](testing.md)

## Getting Help

- 📖 Read the [documentation](../README.md)
- 💬 Open [GitHub Discussions](https://github.com/MH0386/doctainr/discussions)
- 🐛 Report issues on [GitHub](https://github.com/MH0386/doctainr/issues)
