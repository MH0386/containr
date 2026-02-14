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

Comprehensive documentation is available in the `docs/` directory:

- **[Architecture Reference](docs/architecture.md)** - System design, data flow, and technical architecture
- **[Development Guide](docs/development.md)** - Setup, workflow, and extending Doctainr
- **[API Reference](docs/api-reference.md)** - Complete API documentation for all modules
- **[Contributing Guide](docs/contributing.md)** - How to contribute code, report bugs, and coding standards
- **[Troubleshooting Guide](docs/troubleshooting.md)** - Common issues and solutions

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](docs/contributing.md) before submitting a Pull Request.

Quick start for contributors:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Follow our [code style guidelines](docs/contributing.md#code-style-guidelines)
4. Test your changes: `cargo test`
5. Submit a pull request

## 📧 Support

- **Issues**: For bugs and feature requests, open an issue on GitHub
- **Documentation**: Check the [docs/](docs/) directory first
- **Questions**: Use GitHub Discussions for questions and community support

---

**Built with** 🦀 Rust + 🎨 Dioxus
