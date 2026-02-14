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
# Run with hot reload
dx serve --platform desktop

# Or run directly
dx run
```

### Build for Release

```bash
dx build --release
```

### Run Tests

```bash
cargo test
```

### Check Code Quality

```bash
# Format code
dx fmt

# Run linter and checks
dx check
```

## 📚 Documentation

Comprehensive documentation is available in the [`docs/`](docs/) directory:

- **[User Guide](docs/USER_GUIDE.md)** - Complete guide for using Doctainr
- **[Development Guide](docs/DEVELOPMENT.md)** - Setup and development workflow
- **[API Documentation](docs/API.md)** - Technical API reference
- **[Architecture](ARCHITECTURE.md)** - System design and architecture
- **[Deployment Guide](docs/DEPLOYMENT.md)** - Building and distributing
- **[Troubleshooting](docs/TROUBLESHOOTING.md)** - Common issues and solutions

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on:

- Development setup
- Code standards
- Pull request process
- Issue guidelines

## 📧 Support

For issues and questions:
- **Bug Reports**: [Open an issue](https://github.com/MH0386/doctainr/issues)
- **Feature Requests**: [Open an issue](https://github.com/MH0386/doctainr/issues)
- **Questions**: Check the [User Guide](docs/USER_GUIDE.md) or [Troubleshooting](docs/TROUBLESHOOTING.md) guide

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Built with** 🦀 Rust + 🎨 Dioxus
