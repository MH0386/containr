# Troubleshooting Guide

This guide helps you resolve common issues with Doctainr.

## Connection Issues

### "Failed to connect to Docker"

**Symptoms**:
- Error message on startup
- Empty dashboard
- "Docker service not available" error

**Possible Causes**:
1. Docker is not running
2. Permission issues
3. Incorrect Docker host configuration

**Solutions**:

#### 1. Check if Docker is Running

```bash
# Test Docker
docker info

# If error, start Docker
# Linux
sudo systemctl start docker

# macOS/Windows
# Start Docker Desktop application
```

#### 2. Fix Permission Issues (Linux)

```bash
# Check if you're in docker group
groups | grep docker

# Add yourself to docker group
sudo usermod -aG docker $USER

# Log out and back in, or run
newgrp docker

# Verify access
docker ps
```

#### 3. Check Docker Socket

```bash
# Linux/macOS
ls -la /var/run/docker.sock

# Should show:
# srw-rw---- 1 root docker ... /var/run/docker.sock

# If permissions are wrong
sudo chmod 666 /var/run/docker.sock  # Temporary
# Or properly add user to docker group (recommended)
```

#### 4. Verify Docker Host Configuration

```bash
# Check current DOCKER_HOST
echo $DOCKER_HOST

# Try with explicit host
DOCKER_HOST=unix:///var/run/docker.sock doctainr

# For remote Docker
DOCKER_HOST=tcp://192.168.1.100:2375 doctainr
```

### "Permission denied" on Docker Socket

**Solution**:

```bash
# Add user to docker group (Linux)
sudo usermod -aG docker $USER

# Log out and back in

# Verify
docker ps  # Should work without sudo
```

## Application Issues

### Application Won't Start

**Symptoms**:
- No window appears
- Crashes immediately
- Error messages in terminal

**Solutions**:

#### 1. Check System Dependencies

**Linux**:
```bash
# Ubuntu/Debian
sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel gtk3-devel

# Arch
sudo pacman -S webkit2gtk gtk3
```

#### 2. Rebuild Application

```bash
cd doctainr
cargo clean
cargo build --release
```

#### 3. Check Logs

```bash
# Run with verbose output
RUST_LOG=debug cargo run
```

### Application Freezes or Hangs

**Symptoms**:
- UI becomes unresponsive
- Buttons don't work
- Can't close window

**Solutions**:

#### 1. Force Close and Restart

```bash
# Find process
ps aux | grep doctainr

# Kill it
kill -9 <PID>

# Restart
doctainr
```

#### 2. Check Docker Operations

```bash
# See if Docker is responding
docker ps

# Check Docker daemon status
docker system info
```

### UI Not Updating

**Symptoms**:
- Container status doesn't change
- Click "Refresh" but nothing happens
- Data seems stale

**Solutions**:

#### 1. Manual Refresh

- Click "Refresh" button in the current view
- Click "Refresh All" on Dashboard
- Navigate away and back to the view

#### 2. Verify Docker Operations

```bash
# Start a container via CLI
docker start <container-name>

# Check in Docker CLI
docker ps

# Then refresh in Doctainr
```

#### 3. Check for Errors

- Look for error messages at top of screen
- Check terminal output if running from source

## Data Display Issues

### "0 containers" When Containers Exist

**Symptoms**:
- Dashboard shows 0 containers
- But `docker ps -a` shows containers

**Solutions**:

#### 1. Verify Connection

```bash
# Check Docker responds
docker ps -a

# Check if Doctainr sees same Docker host
echo $DOCKER_HOST
```

#### 2. Check for Errors

- Look for error messages in UI
- Check terminal for connection errors

#### 3. Restart Doctainr

Sometimes a fresh start resolves synchronization issues.

### Incorrect Container States

**Symptoms**:
- Container shows "Running" but is stopped (or vice versa)
- Status doesn't match `docker ps`

**Solutions**:

#### 1. Refresh Data

Click "Refresh" button in Containers view.

#### 2. Verify with Docker CLI

```bash
docker ps -a
```

If Doctainr still shows wrong state after refresh, this may be a bug. Please report it.

### Missing Images or Volumes

**Symptoms**:
- Images/volumes don't appear in list
- Count doesn't match `docker images` or `docker volume ls`

**Solutions**:

#### 1. Refresh the View

Click "Refresh" button.

#### 2. Verify with Docker CLI

```bash
# Check images
docker images

# Check volumes
docker volume ls
```

## Performance Issues

### Slow Startup

**Symptoms**:
- Application takes long to open
- Dashboard loads slowly

**Possible Causes**:
- Many containers/images/volumes
- Slow Docker daemon
- System resource constraints

**Solutions**:

#### 1. Check Docker Performance

```bash
# Check Docker daemon responsiveness
time docker ps -a
```

If Docker CLI is slow, the issue is with Docker, not Doctainr.

#### 2. Optimize Docker

```bash
# Clean up unused resources
docker system prune

# Remove unused volumes
docker volume prune

# Remove unused images
docker image prune -a
```

### High Resource Usage

**Symptoms**:
- High CPU or memory usage
- System becomes slow

**Solutions**:

#### 1. Monitor Resources

```bash
# Check Doctainr resource usage
top | grep doctainr
```

#### 2. Check for Loops

If CPU usage is consistently high:
1. Close Doctainr
2. Check Docker is responsive: `docker info`
3. Restart Doctainr
4. If issue persists, report a bug

## Build Issues

### Build Fails with Linker Error

**Error**: "linker 'cc' not found"

**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS
xcode-select --install

# Fedora
sudo dnf install gcc
```

### WebKit Errors on Linux

**Error**: "webkit2gtk-4.1 not found"

**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get install libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel

# Arch
sudo pacman -S webkit2gtk
```

### Cargo Build Fails

**Error**: Various cargo errors

**Solutions**:

```bash
# Update Rust
rustup update stable

# Clean build
cargo clean
cargo build

# Update dependencies
cargo update
```

## Platform-Specific Issues

### macOS

#### "Unable to open application"

**Solution**:
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /path/to/doctainr
```

#### Gatekeeper Issues

If macOS blocks the application:
1. Open System Preferences → Security & Privacy
2. Click "Open Anyway" for Doctainr

### Windows

#### Named Pipe Connection Issues

**Solution**:
```bash
# Set Docker host to named pipe
set DOCKER_HOST=npipe:////./pipe/docker_engine

# Run Doctainr
doctainr
```

#### WSL2 Issues

If using Docker in WSL2:
```bash
# In WSL, set Docker host
export DOCKER_HOST=unix:///var/run/docker.sock

# Run Doctainr from WSL
```

### Linux

#### SELinux Issues

If SELinux blocks Docker access:
```bash
# Check SELinux status
sestatus

# Temporarily disable (not recommended for production)
sudo setenforce 0

# Or configure SELinux policy properly
# (consult SELinux documentation)
```

## Getting More Help

### Check Logs

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Or for installed binary
RUST_LOG=debug doctainr
```

### System Information

When reporting issues, include:

```bash
# System info
uname -a

# Docker version
docker --version
docker info

# Rust version
rustc --version

# Doctainr version
doctainr --version  # If implemented

# Environment
echo $DOCKER_HOST
```

### Reporting Bugs

If none of these solutions work:

1. Check [existing issues](https://github.com/MH0386/doctainr/issues)
2. Gather information (versions, error messages, logs)
3. Create a [new issue](https://github.com/MH0386/doctainr/issues/new)

Include:
- Detailed description
- Steps to reproduce
- Expected vs actual behavior
- System information
- Error messages and logs
- Screenshots (if applicable)

## FAQ

For frequently asked questions, see the [FAQ](faq.md).

## Related Documentation

- [Installation Guide](installation.md)
- [User Guide](user-guide.md)
- [FAQ](faq.md)
