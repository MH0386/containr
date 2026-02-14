# Development Environment Setup

This guide walks you through setting up a complete development environment for Doctainr.

## Prerequisites

### Required Software

1. **Docker** (20.10+)
   - [Docker Desktop](https://www.docker.com/products/docker-desktop) (macOS/Windows)
   - Docker Engine (Linux)

2. **Rust** (1.70+)
   - Install via [rustup](https://rustup.rs/)

3. **Dioxus CLI** (optional but recommended)
   - Provides hot reload and development server

### Optional but Recommended

- **Git** for version control
- **VS Code** with rust-analyzer extension
- **Docker Compose** for multi-container testing

## Step-by-Step Setup

### 1. Install Rust

````bash
# Install rustup (official Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Configure current shell
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
````

### 2. Install Docker

#### macOS/Windows

Download and install [Docker Desktop](https://www.docker.com/products/docker-desktop)

#### Linux (Ubuntu/Debian)

````bash
# Update package index
sudo apt-get update

# Install Docker
sudo apt-get install docker.io

# Start Docker service
sudo systemctl start docker
sudo systemctl enable docker

# Add user to docker group
sudo usermod -aG docker $USER

# Log out and back in, then verify
docker ps
````

### 3. Install Dioxus CLI

````bash
# Install via official script
curl -sSL http://dioxus.dev/install.sh | sh

# Or via cargo
cargo install dioxus-cli

# Verify installation
dx --version
````

### 4. Clone the Repository

````bash
git clone https://github.com/MH0386/doctainr.git
cd doctainr
````

### 5. Build Dependencies

````bash
# Fetch and build all dependencies
cargo build
````

This may take several minutes on the first run.

## Running the Application

### Development Mode (with hot reload)

````bash
dx serve --platform desktop
````

Changes to `.rs` files will automatically trigger a rebuild and reload.

### Standard Development Run

````bash
cargo run
````

### Release Build

````bash
cargo build --release
# Binary located at: target/release/doctainr
````

## Development Tools

### Code Formatting

````bash
# Format all code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
````

### Linting

````bash
# Run clippy for code quality checks
cargo clippy

# Automatically apply fixes
cargo clippy --fix
````

### Testing

````bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_docker_service
````

## IDE Setup

### VS Code (Recommended)

#### Extensions

Install these extensions for the best experience:

1. **rust-analyzer** - Rust language server
2. **CodeLLDB** - Debugger
3. **crates** - Cargo.toml dependency management
4. **Even Better TOML** - TOML syntax highlighting

#### Settings

Add to `.vscode/settings.json`:

````json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": ["desktop"],
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
}
````

#### Debug Configuration

Add to `.vscode/launch.json`:

````json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Doctainr",
      "cargo": {
        "args": ["build", "--bin=doctainr", "--package=doctainr"]
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
````

### IntelliJ IDEA / CLion

Install the **Rust plugin** and open the project. IntelliJ will automatically detect the Cargo project.

## Project Structure Overview

````
doctainr/
├── src/
│   ├── main.rs              # Application entry point
│   ├── components/          # Reusable UI components
│   ├── services/            # Docker API integration
│   ├── utils/               # Application state and helpers
│   └── views/               # UI views (pages)
├── assets/                  # Static assets (CSS, icons)
├── docs/                    # Documentation
├── Cargo.toml               # Rust dependencies
├── Dioxus.toml              # Dioxus configuration
└── README.md
````

## Common Development Workflows

### Adding a New Feature

1. Create a feature branch: `git checkout -b feature/your-feature`
2. Implement changes with tests
3. Run tests: `cargo test`
4. Format and lint: `cargo fmt && cargo clippy`
5. Commit with conventional commit message
6. Push and create PR

### Testing Docker Integration

````bash
# Ensure Docker is running
docker info

# Create test containers
docker run -d --name test-nginx nginx:alpine
docker run -d --name test-redis redis:alpine

# Run Doctainr to see test containers
cargo run

# Clean up
docker rm -f test-nginx test-redis
````

### Debugging

#### Logging

Enable debug logging:

````bash
RUST_LOG=debug cargo run
````

Log levels: `error`, `warn`, `info`, `debug`, `trace`

#### Breakpoints

Use VS Code debugger with CodeLLDB:

1. Set breakpoints in code
2. Press F5 or run "Debug Doctainr" configuration
3. Application starts with debugger attached

## Troubleshooting Setup Issues

See [Troubleshooting Guide](../how-to/troubleshooting.md) for common issues and solutions.

### Quick Checks

````bash
# Verify all tools are installed
rustc --version    # Should be 1.70+
cargo --version
docker --version
dx --version

# Test Docker connection
docker ps

# Build check
cargo check
````

## Next Steps

- Read [Architecture Overview](../reference/architecture.md) to understand the codebase
- Explore [Contributing Guidelines](../../CONTRIBUTING.md) for contribution workflow
- Check [State Management Explanation](../explanation/state-management.md) for state architecture

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/MH0386/doctainr/issues)
- **Discussions**: [GitHub Discussions](https://github.com/MH0386/doctainr/discussions)
- **Contributing**: See [CONTRIBUTING.md](../../CONTRIBUTING.md)
