# Doctainr

> A modern Docker container management application built with Dioxus 0.7

[![CI](https://github.com/MH0386/doctainr/actions/workflows/ci.yaml/badge.svg)](https://github.com/MH0386/doctainr/actions/workflows/ci.yaml)

## Overview

Doctainr is a native desktop application for managing Docker containers, images, and volumes. Built with [Dioxus 0.7](https://dioxuslabs.com/learn/0.7), it provides a reactive, cross-platform interface for Docker operations.

## Features

- **Container Management**: View, start, stop, and monitor Docker containers
- **Image Management**: Browse and manage Docker images
- **Volume Management**: Inspect and manage Docker volumes
- **Real-time Status**: Live updates of container states and resource usage
- **Native Desktop UI**: Built with WebKit for a fast, native experience

## Prerequisites

- Docker daemon running and accessible
- Rust toolchain (1.70 or later)
- Platform-specific dependencies:
  - **Linux**: WebKitGTK 4.1, OpenSSL

## Installation

### From Source

````bash
# Clone the repository
git clone https://github.com/MH0386/doctainr.git
cd doctainr

# Install Dioxus CLI
curl -sSL http://dioxus.dev/install.sh | sh

# Build and run
dx serve
````

### Using devenv

This project uses [devenv](https://devenv.sh/) for development environment management:

````bash
# Enter the development environment
devenv shell

# Run the application
devenv up
````

## Usage

### Basic Operations

Launch the application and access:

- **Dashboard** (`/`): Overview of all Docker resources
- **Containers** (`/containers`): Manage running and stopped containers
- **Images** (`/images`): View available Docker images
- **Volumes** (`/volumes`): Inspect Docker volumes
- **Settings** (`/settings`): Configure Docker connection

### Docker Connection

By default, Doctainr connects to Docker via the Unix socket at `/var/run/docker.sock`. To use a different Docker host, set the `DOCKER_HOST` environment variable:

````bash
export DOCKER_HOST=tcp://localhost:2375
dx serve
````

## Architecture

### Project Structure

````
doctainr/
├── src/
│   ├── main.rs              # Application entry point
│   ├── components/          # Reusable UI components
│   │   ├── metric_card.rs   # Metric display component
│   │   ├── section_header.rs # Section header component
│   │   └── status_pill.rs   # Status indicator component
│   ├── services/            # External service integrations
│   │   └── docker.rs        # Docker API wrapper
│   ├── utils/               # Shared utilities
│   │   └── app_state.rs     # Application state management
│   └── views/               # Page components
│       ├── containers.rs    # Container management view
│       ├── dashboard.rs     # Dashboard view
│       ├── images.rs        # Image management view
│       ├── volumes.rs       # Volume management view
│       └── settings.rs      # Settings view
├── assets/                  # Static assets (CSS, icons)
└── Cargo.toml              # Rust dependencies
````

### Key Technologies

- **[Dioxus 0.7](https://dioxuslabs.com/)**: Reactive UI framework
- **[Bollard](https://docs.rs/bollard/)**: Docker API client
- **[Tokio](https://tokio.rs/)**: Async runtime
- **Serde**: Serialization/deserialization

### State Management

Doctainr uses Dioxus signals for reactive state management. The `AppState` struct maintains:

- Docker connection details
- Container, image, and volume lists
- Loading states and error messages
- Last action performed

State is provided via the Context API and accessible throughout the component tree.

## Development

### Building

````bash
# Development build
cargo build

# Release build
cargo build --release

# Using Dioxus CLI (recommended)
dx build --release
````

### Running Tests

````bash
cargo test
````

### Code Quality

````bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings
````

### Development Tasks

This project includes predefined devenv tasks:

````bash
devenv tasks        # List available tasks
dx:build           # Build the project
dx:serve           # Run development server
dx:check           # Run tests and linting
````

## Configuration

### Cargo Features

- `default`: Enables desktop platform
- `desktop`: Desktop application support (via `dioxus/desktop`)
- `web`: Web application support (via `dioxus/web`)

### Docker Connection

Doctainr respects the standard `DOCKER_HOST` environment variable. Common configurations:

````bash
# Unix socket (default)
DOCKER_HOST=unix:///var/run/docker.sock

# TCP connection
DOCKER_HOST=tcp://localhost:2375

# SSH connection
DOCKER_HOST=ssh://user@remote-host
````

## Troubleshooting

### Docker Connection Issues

If Doctainr cannot connect to Docker:

1. Verify Docker daemon is running: `docker ps`
2. Check socket permissions: `ls -la /var/run/docker.sock`
3. Ensure your user is in the `docker` group: `groups`

### Build Issues

**Missing WebKitGTK on Linux:**

````bash
# Ubuntu/Debian
sudo apt-get install libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel

# Arch
sudo pacman -S webkit2gtk-4.1
````

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Ensure all clippy warnings are addressed
- Add tests for new functionality
- Update documentation for API changes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Dioxus](https://dioxuslabs.com/) for the reactive UI framework
- [Bollard](https://github.com/fussybeaver/bollard) for Docker API integration
- The Rust community for excellent tooling and libraries

## Roadmap

- [ ] Container logs viewer
- [ ] Container shell/exec functionality
- [ ] Network management
- [ ] Docker Compose integration
- [ ] Resource usage monitoring (CPU, memory)
- [ ] Multi-host support
- [ ] Container creation wizard

## Support

- **Issues**: [GitHub Issues](https://github.com/MH0386/doctainr/issues)
- **Discussions**: [GitHub Discussions](https://github.com/MH0386/doctainr/discussions)
