# Troubleshooting

> **How-to Guide** - Solutions to common problems and issues with Doctainr.

## Docker Connection Issues

### "Failed to connect to Docker"

**Symptoms:**
- Error message on startup: "Failed to connect to Docker"
- Empty dashboard with no data

**Solutions:**

#### 1. Verify Docker is Running

````bash
# Check Docker daemon status
docker info

# If not running, start Docker Desktop
# macOS: Open Docker Desktop application
# Linux: sudo systemctl start docker
# Windows: Start Docker Desktop application
````

#### 2. Check Docker Socket Permissions (Linux)

````bash
# Check socket exists and permissions
ls -l /var/run/docker.sock

# If permission denied, add user to docker group
sudo usermod -aG docker $USER

# Log out and back in for changes to take effect
# Or use: newgrp docker
````

#### 3. Verify DOCKER_HOST Environment Variable

````bash
# Check if DOCKER_HOST is set
echo $DOCKER_HOST

# If incorrectly set, unset or fix it
unset DOCKER_HOST

# Or set to correct value
export DOCKER_HOST=unix:///var/run/docker.sock
````

#### 4. Test Docker CLI Access

````bash
# If this fails, Doctainr will also fail
docker ps

# Check version
docker version
````

### "Docker service not available"

**Cause:** DockerService failed to initialize, but app started anyway.

**Solutions:**
- Follow steps above to fix Docker connection
- Restart Doctainr after fixing Docker
- Check stderr logs for detailed error messages

## Build Issues

### "dx: command not found"

**Solution:**

````bash
# Install Dioxus CLI
curl -sSL http://dioxus.dev/install.sh | sh

# Add to PATH (usually automatic, but if needed)
export PATH="$HOME/.cargo/bin:$PATH"

# Verify installation
dx --version
````

### "error: failed to load manifest"

**Solutions:**

````bash
# Update Rust
rustup update

# Clean build artifacts
cargo clean

# Try building again
dx build
````

### "linker errors" or "undefined reference"

**Linux Solution:**

````bash
# Install required development packages
# Ubuntu/Debian:
sudo apt-get install build-essential pkg-config libssl-dev

# Fedora:
sudo dnf install gcc openssl-devel

# Arch:
sudo pacman -S base-devel openssl
````

**macOS Solution:**

````bash
# Install Xcode command line tools
xcode-select --install
````

### Slow compilation times

**Solutions:**

1. **Use incremental compilation** (enabled by default)

2. **Enable parallel compilation**:
   ````bash
   # Add to ~/.cargo/config.toml
   [build]
   jobs = 8  # Adjust to your CPU core count
   ````

3. **Use mold linker** (Linux, much faster):
   ````bash
   # Install mold
   # Ubuntu: sudo apt install mold
   # Arch: sudo pacman -S mold
   
   # Use in builds
   RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build
   ````

4. **Use sccache**:
   ````bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ````

## Runtime Issues

### App crashes on startup

**Check logs:**

````bash
# Run with debug logging
RUST_LOG=debug dx serve --platform desktop

# Look for panic messages or error details
````

**Common causes:**
1. Docker not running (see above)
2. Missing system libraries
3. Corrupted configuration

### "Permission denied" errors

**Linux/macOS:**

````bash
# Check file permissions
ls -l /var/run/docker.sock

# Add user to docker group
sudo usermod -aG docker $USER

# Restart Docker service
sudo systemctl restart docker
````

**Windows:**

- Run Docker Desktop as Administrator
- Ensure user is in "docker-users" group

### UI not responding / frozen

**Possible causes:**
- Long-running Docker operation blocking UI
- Large number of containers causing slowdown

**Solutions:**

1. **Check Docker operations:**
   ````bash
   docker ps
   docker stats
   ````

2. **Restart the application**

3. **Report the issue** with:
   - Steps to reproduce
   - Number of containers/images
   - System specs

### Data not refreshing

**Solution:**

1. Click **Refresh** button manually
2. Check Docker daemon is responsive:
   ````bash
   docker ps  # Should return quickly
   ````
3. Restart Doctainr

## Platform-Specific Issues

### macOS

#### "cannot be opened because the developer cannot be verified"

**Solution:**

````bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /path/to/doctainr

# Or right-click > Open > Open anyway
````

#### M1/M2 (Apple Silicon) issues

**Solution:**

````bash
# Ensure Rust is using native toolchain
rustup default stable-aarch64-apple-darwin

# Rebuild
cargo clean
dx build
````

### Linux

#### Wayland display issues

**Solution:**

````bash
# Force X11 backend
WINIT_UNIX_BACKEND=x11 dx serve --platform desktop
````

#### Missing system libraries

**Ubuntu/Debian:**
````bash
sudo apt-get install libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
````

**Fedora:**
````bash
sudo dnf install webkit2gtk4.1-devel \
    gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel
````

**Arch:**
````bash
sudo pacman -S webkit2gtk \
    gtk3 \
    libappindicator-gtk3 \
    librsvg
````

### Windows

#### "vcruntime140.dll missing"

**Solution:**
- Install [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)

#### Docker Desktop not starting

**Solution:**
1. Enable WSL2 integration in Docker Desktop settings
2. Update WSL2: `wsl --update`
3. Restart Docker Desktop

## Performance Issues

### High memory usage

**Normal behavior:**
- Desktop apps use more memory than web apps
- Dioxus desktop uses WebView, which includes Chromium

**If excessive:**
1. Check for memory leaks in code
2. Limit number of containers displayed
3. Report issue with system specs

### Slow container operations

**Causes:**
- Docker daemon overloaded
- Network latency (remote Docker)
- Large image sizes

**Solutions:**

````bash
# Check Docker performance
docker stats

# Clean up unused resources
docker system prune

# Check disk space
df -h
````

## Development Issues

### Hot reload not working

**Solutions:**

1. **Use correct command:**
   ````bash
   dx serve --platform desktop --hot-reload
   ````

2. **Check file watchers limit** (Linux):
   ````bash
   # Increase limit
   echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf
   sudo sysctl -p
   ````

3. **Restart dx serve**

### Type errors with Signals

**Problem:**
````rust
// Error: expected Signal<T>, found T
let value = app_state.containers;
````

**Solution:**
````rust
// Call the signal to get the value
let value = (app_state.containers)();
````

### "moved value" errors in async closures

**Problem:**
````rust
spawn(async move {
    // Error: value used after move
    app_state.refresh();
});
````

**Solution:**
````rust
// Clone before moving into closure
let app_state = app_state.clone();
spawn(async move {
    app_state.refresh();
});
````

## Logging and Diagnostics

### Enable debug logging

````bash
# All debug logs
RUST_LOG=debug dx serve --platform desktop

# Specific module
RUST_LOG=doctainr=debug dx serve --platform desktop

# Multiple modules
RUST_LOG=doctainr=debug,bollard=info dx serve --platform desktop
````

### Collecting diagnostic information

For bug reports, include:

````bash
# System info
uname -a

# Rust version
rustc --version

# Dioxus version
dx --version

# Docker version
docker --version
docker info

# Docker containers
docker ps -a

# Doctainr logs
RUST_LOG=debug dx serve --platform desktop 2>&1 | tee doctainr.log
````

## Still Having Issues?

### Search Existing Issues

Check [GitHub Issues](https://github.com/MH0386/doctainr/issues) for similar problems.

### Ask for Help

1. **GitHub Discussions** - For questions and support
2. **GitHub Issues** - For confirmed bugs
3. **Include diagnostic info** - Logs, versions, steps to reproduce

### Provide Details

When reporting issues:
- **Operating System** and version
- **Docker version** (`docker --version`)
- **Rust version** (`rustc --version`)
- **Doctainr version** or commit hash
- **Steps to reproduce**
- **Expected vs actual behavior**
- **Error messages** (full text)
- **Logs** (with `RUST_LOG=debug`)

---

**Related Documentation:**
- [Development Guide](development.md) - Development setup and workflow
- [Architecture](architecture.md) - System design
- [Contributing](contributing.md) - How to contribute fixes
