# Troubleshooting Common Issues

This guide helps you resolve common problems when using Doctainr.

## Docker Connection Issues

### Problem: "Failed to connect to Docker"

**Symptoms:**
- Error message on startup: "Failed to connect to Docker"
- Empty dashboard with no data

**Solutions:**

#### 1. Verify Docker is Running

````bash
docker info
````

If this fails, start Docker Desktop or the Docker daemon:

- **macOS/Windows**: Launch Docker Desktop
- **Linux**: `sudo systemctl start docker`

#### 2. Check Docker Socket Permissions (Linux)

````bash
# Check if Docker socket exists
ls -l /var/run/docker.sock

# Add your user to docker group
sudo usermod -aG docker $USER

# Log out and back in for changes to take effect
````

#### 3. Custom Docker Host

If using a non-standard Docker host, set the `DOCKER_HOST` environment variable:

````bash
# Unix socket (default)
export DOCKER_HOST=unix:///var/run/docker.sock

# TCP connection
export DOCKER_HOST=tcp://localhost:2375

# SSH connection
export DOCKER_HOST=ssh://user@host

# Then run Doctainr
dx serve --platform desktop
````

### Problem: "Permission Denied" when accessing Docker

**Solution:**

On Linux, ensure your user has Docker permissions:

````bash
sudo usermod -aG docker $USER
newgrp docker  # Apply group without logout
````

## Build and Run Issues

### Problem: "dx: command not found"

**Solution:**

Install Dioxus CLI:

````bash
curl -sSL http://dioxus.dev/install.sh | sh

# Or with cargo
cargo install dioxus-cli
````

### Problem: Cargo build fails with dependency errors

**Solutions:**

#### Update Rust toolchain:

````bash
rustup update stable
````

#### Clean build cache:

````bash
cargo clean
cargo build
````

#### Check Rust version:

````bash
rustc --version  # Should be 1.70+
````

### Problem: Application crashes on startup

**Diagnostic steps:**

1. **Check logs** for error messages
2. **Verify Docker** is accessible: `docker ps`
3. **Run with verbose output**: 
   ````bash
   RUST_LOG=debug cargo run
   ````

## UI Issues

### Problem: UI appears blank or frozen

**Solutions:**

1. **Refresh data**: Click the "Refresh" button
2. **Check browser console** (if web build): Open DevTools and check for errors
3. **Restart application**: Close and reopen Doctainr

### Problem: Container actions (start/stop) don't work

**Possible causes:**

1. **Insufficient permissions**: See Docker permission issues above
2. **Container state conflict**: Container might already be in the requested state
3. **Docker daemon issue**: Restart Docker daemon

**Diagnostic:**

````bash
# Test Docker commands directly
docker start <container-id>
docker stop <container-id>
````

## Performance Issues

### Problem: Slow initial load

**Causes:**
- Large number of containers/images/volumes
- Slow Docker daemon response

**Solutions:**
- Reduce number of unused containers: `docker container prune`
- Remove unused images: `docker image prune -a`
- Check Docker daemon performance

### Problem: High CPU/memory usage

**Solutions:**
- Close unnecessary containers
- Restart Doctainr
- Check for Docker daemon resource limits

## Development Issues

### Problem: Hot reload not working with `dx serve`

**Solutions:**

1. **Ensure dx is up to date**:
   ````bash
   cargo install dioxus-cli --force
   ````

2. **Use correct platform**:
   ````bash
   dx serve --platform desktop
   ````

3. **Manual rebuild**:
   ````bash
   cargo build
   cargo run
   ````

### Problem: Clippy warnings or errors

**Solution:**

Fix warnings systematically:

````bash
cargo clippy --fix --allow-dirty
cargo fmt
````

## Platform-Specific Issues

### macOS

**Problem: Application won't open (security warning)**

Right-click the app and select "Open" to bypass Gatekeeper.

### Windows

**Problem: Docker Desktop WSL 2 backend issues**

Ensure WSL 2 is properly installed:

````powershell
wsl --install
wsl --set-default-version 2
````

### Linux

**Problem: Wayland compatibility issues**

Try X11 mode:

````bash
env WAYLAND_DISPLAY= cargo run
````

## Still Having Issues?

1. **Check existing issues**: [GitHub Issues](https://github.com/MH0386/doctainr/issues)
2. **Create a new issue**: Include:
   - OS and version
   - Docker version: `docker --version`
   - Rust version: `rustc --version`
   - Full error message
   - Steps to reproduce
3. **Join discussions**: [GitHub Discussions](https://github.com/MH0386/doctainr/discussions)

## Useful Diagnostic Commands

````bash
# System information
uname -a

# Docker version and info
docker --version
docker info

# Rust toolchain
rustc --version
cargo --version

# Dioxus CLI
dx --version

# Check Docker connectivity
curl --unix-socket /var/run/docker.sock http://localhost/version
````
