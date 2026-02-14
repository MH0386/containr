# Installation Guide

## Prerequisites

Before installing Doctainr, ensure you have the following:

### 1. Docker

Docker must be installed and running on your system.

**Linux**:
```bash
# Install Docker Engine
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add your user to docker group
sudo usermod -aG docker $USER

# Restart your session or run:
newgrp docker

# Verify Docker is running
docker info
```

**macOS**:
```bash
# Install Docker Desktop from:
# https://www.docker.com/products/docker-desktop

# Or using Homebrew:
brew install --cask docker

# Start Docker Desktop and verify
docker info
```

**Windows**:
```powershell
# Install Docker Desktop from:
# https://www.docker.com/products/docker-desktop

# After installation, verify in PowerShell:
docker info
```

### 2. Rust Toolchain

Doctainr requires Rust 1.70 or later.

**Install Rust**:
```bash
# Using rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, then restart your terminal

# Verify installation
rustc --version
cargo --version
```

**Update Rust** (if already installed):
```bash
rustup update stable
```

### 3. Dioxus CLI (Optional but Recommended)

The Dioxus CLI (`dx`) provides hot-reload and better development experience.

```bash
# Install dx
curl -sSL http://dioxus.dev/install.sh | sh

# Or using cargo
cargo install dioxus-cli

# Verify installation
dx --version
```

## Installation Methods

### Method 1: From Source (Recommended)

Build and install Doctainr from source:

```bash
# Clone the repository
git clone https://github.com/MH0386/doctainr.git
cd doctainr

# Build the release version
cargo build --release

# The binary will be at:
# ./target/release/doctainr

# Run it
./target/release/doctainr
```

**Using Dioxus CLI**:
```bash
# Clone the repository
git clone https://github.com/MH0386/doctainr.git
cd doctainr

# Run with dx (includes hot-reload for development)
dx serve --platform desktop

# Or build for release
dx build --release --platform desktop
```

### Method 2: Install Binary

Install directly using cargo:

```bash
# Install from crates.io (when published)
cargo install doctainr

# Run
doctainr
```

### Method 3: Pre-built Binaries

Download pre-built binaries from the releases page (when available):

```bash
# Download for your platform from:
# https://github.com/MH0386/doctainr/releases

# Linux/macOS: Make executable
chmod +x doctainr
./doctainr

# Or move to PATH
sudo mv doctainr /usr/local/bin/
doctainr
```

## Platform-Specific Setup

### Linux

**Dependencies**:

On some Linux distributions, you may need additional dependencies:

```bash
# Ubuntu/Debian
sudo apt-get install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  gtk3-devel \
  libappindicator-gtk3-devel \
  librsvg2-devel

# Arch
sudo pacman -S webkit2gtk \
  base-devel \
  curl \
  wget \
  openssl \
  gtk3 \
  libappindicator-gtk3 \
  librsvg
```

**Docker Socket Permissions**:

Ensure your user has access to the Docker socket:

```bash
# Check if you're in the docker group
groups | grep docker

# If not, add yourself
sudo usermod -aG docker $USER

# Log out and log back in, or run
newgrp docker

# Verify you can access Docker without sudo
docker ps
```

### macOS

**Xcode Command Line Tools**:

```bash
xcode-select --install
```

**Docker Desktop**:

Ensure Docker Desktop is running before starting Doctainr.

### Windows

**Build Tools**:

Install Visual Studio Build Tools:
- Download from: https://visualstudio.microsoft.com/downloads/
- Select "Desktop development with C++"

**Docker Desktop**:

Ensure Docker Desktop is running in Windows containers or WSL2 mode.

## Configuration

### Docker Host

By default, Doctainr connects to Docker using the default socket. You can customize this with the `DOCKER_HOST` environment variable:

```bash
# Unix socket (default on Linux/macOS)
export DOCKER_HOST=unix:///var/run/docker.sock

# TCP connection (for remote Docker)
export DOCKER_HOST=tcp://192.168.1.100:2375

# Windows named pipe
set DOCKER_HOST=npipe:////./pipe/docker_engine
```

### Environment Variables

Create a `.env` file in the project directory (optional):

```bash
# .env
DOCKER_HOST=unix:///var/run/docker.sock
```

## Verifying Installation

After installation, verify Doctainr is working:

```bash
# Run Doctainr
doctainr  # or ./target/release/doctainr if running from source

# You should see the application window open
# The dashboard should display your Docker containers, images, and volumes
```

**Troubleshooting Connection**:

If Doctainr can't connect to Docker:

```bash
# Verify Docker is running
docker info

# Check Docker socket permissions (Linux)
ls -la /var/run/docker.sock

# Test Docker connection with the same host
docker -H unix:///var/run/docker.sock ps
```

## Updating Doctainr

### From Source

```bash
cd doctainr
git pull origin main
cargo build --release
```

### From Cargo

```bash
cargo install doctainr --force
```

## Uninstalling

### Cargo Installation

```bash
cargo uninstall doctainr
```

### From Source

Simply remove the cloned directory:

```bash
rm -rf ~/path/to/doctainr
```

### Binary Installation

Remove the binary from your PATH:

```bash
sudo rm /usr/local/bin/doctainr
```

## Next Steps

- [Quick Start Tutorial](quick-start.md) - Learn how to use Doctainr
- [User Guide](user-guide.md) - Comprehensive usage guide
- [Troubleshooting](troubleshooting.md) - Common issues and solutions

## Common Installation Issues

### Issue: "Docker not running"

**Solution**: Start Docker Desktop or Docker daemon:
```bash
# Linux
sudo systemctl start docker

# macOS/Windows
# Start Docker Desktop application
```

### Issue: "Permission denied" on Docker socket

**Solution**: Add user to docker group (Linux):
```bash
sudo usermod -aG docker $USER
# Log out and back in
```

### Issue: Build fails with "linker 'cc' not found"

**Solution**: Install C compiler:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS
xcode-select --install
```

### Issue: Rust version too old

**Solution**: Update Rust:
```bash
rustup update stable
```

### Issue: WebKit errors on Linux

**Solution**: Install webkit2gtk:
```bash
# Ubuntu/Debian
sudo apt-get install libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel
```

## Getting Help

If you encounter issues not covered here:
- Check the [Troubleshooting Guide](troubleshooting.md)
- Review [FAQ](faq.md)
- Open an issue on [GitHub](https://github.com/MH0386/doctainr/issues)
