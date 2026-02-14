# Doctainr - Docker Desktop UI

A Docker desktop application built with
Rust and Dioxus, providing real-time Docker management.

## ✨ Features

- 📊 **Dashboard** - Overview of containers, images, and volumes
- 🐳 **Container Management** - Start, stop, and monitor Docker containers
- 💿 **Image Browser** - View all local Docker images
- 📦 **Volume Manager** - Browse Docker volumes
- 🔄 **Real-time Updates** - Refresh data on demand
- ⚡ **Fast & Lightweight** - Native Rust performance

## 🚀 Quick Start

### Prerequisites

1. **Docker** must be running on your system
2. **Rust** toolchain (1.70+)

<!-- end list -->

```bash
# Check Docker is running
docker info

# Verify Rust installation
rustc --version

# Verify dx installation
dx --version
```

### Installation

```bash
# Clone the repository
git clone https://github.com/MH0386/doctainr.git
cd doctainr

# Build and run
dx run
```

## 🎯 Usage

### Dashboard

- View total counts of containers, images, and volumes
- See running vs stopped containers
- Check Docker engine status
- Use "Refresh All" to reload all data

### Containers View

- See all containers (running and stopped)
- Click "Start" to start a stopped container
- Click "Stop" to stop a running container
- Use "Refresh" to reload the container list
- View status, ports, and image information

### Images View

- Browse all local Docker images
- See repository, tag, ID, and size
- Use "Refresh" to reload the image list

### Volumes View

- List all Docker volumes
- See driver and mount point information
- Use "Refresh" to reload the volume list

## 🛠️ Development

### Build for Development

```bash
# Run with hot reload (if using dx)
dx serve --platform desktop

# Or standard cargo
cargo run
```

### Build for Release

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Check Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

<!--## 📄 License

[Your License Here]-->

## 📚 Documentation

- **[Development Setup](docs/tutorials/development-setup.md)** - Complete dev environment guide
- **[Architecture Overview](docs/reference/architecture.md)** - System design and module structure
- **[Troubleshooting](docs/how-to/troubleshooting.md)** - Common issues and solutions
- **[State Management](docs/explanation/state-management.md)** - How reactive state works
- **[Docker Integration](docs/explanation/docker-integration.md)** - Bollard implementation details

## 🔧 Troubleshooting

### Docker Connection Issues

If you see "Failed to connect to Docker", ensure:

1. Docker is running: `docker info`
2. You have permissions (Linux): `sudo usermod -aG docker $USER`
3. Socket is accessible: Check `DOCKER_HOST` environment variable

For more solutions, see the [Troubleshooting Guide](docs/how-to/troubleshooting.md).

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Development workflow
- Coding standards
- Testing requirements
- Commit message conventions

## 📧 Support

For issues and questions:

- **Bug reports**: [GitHub Issues](https://github.com/MH0386/doctainr/issues)
- **Questions**: [GitHub Discussions](https://github.com/MH0386/doctainr/discussions)
- **Documentation**: See [docs/](docs/) directory

---

**Built with** 🦀 Rust + 🎨 Dioxus
