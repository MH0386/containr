# Troubleshooting Guide

## Common Issues and Solutions

### Docker Connection Issues

#### Cannot connect to Docker daemon

**Symptoms:**
- Error message: "Failed to connect to Docker"
- Dashboard shows "Docker service not available"
- Empty lists for containers, images, volumes

**Solutions:**

1. **Verify Docker is running:**
   ````bash
   docker info
   docker ps
   ````

2. **Check Docker socket (Linux/macOS):**
   ````bash
   ls -la /var/run/docker.sock
   ````
   
   If missing, start Docker:
   ````bash
   # macOS
   open -a Docker
   
   # Linux (systemd)
   sudo systemctl start docker
   ````

3. **Check permissions (Linux):**
   ````bash
   # Add user to docker group
   sudo usermod -aG docker $USER
   
   # Log out and back in, then verify
   groups
   docker ps
   ````

4. **Windows Docker Desktop:**
   - Open Docker Desktop app
   - Wait for "Docker Desktop is running" status
   - Check settings: Settings > General > "Use the WSL 2 based engine"

5. **Custom Docker host:**
   
   If using remote Docker or custom socket:
   ````bash
   export DOCKER_HOST=tcp://192.168.1.100:2376
   # or
   export DOCKER_HOST=unix:///custom/path/docker.sock
   ````

#### Permission denied on Docker socket

**Error:**
````
Error: Permission denied (os error 13)
````

**Solution:**
````bash
# Linux: Add user to docker group
sudo usermod -aG docker $USER
newgrp docker  # Or log out and back in

# Verify permissions
ls -la /var/run/docker.sock
````

### Build and Compilation Issues

#### Dioxus CLI not found

**Error:**
````bash
dx: command not found
````

**Solution:**
````bash
# Install Dioxus CLI
curl -sSL http://dioxus.dev/install.sh | sh

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"

# Verify installation
dx --version
````

#### Dependency compilation failures

**Symptoms:**
- Long compile times ending in errors
- Missing system dependencies
- OpenSSL errors

**Solutions:**

1. **Install system dependencies:**

   **macOS:**
   ````bash
   brew install openssl pkg-config
   ````
   
   **Ubuntu/Debian:**
   ````bash
   sudo apt-get update
   sudo apt-get install build-essential pkg-config libssl-dev
   ````
   
   **Fedora/RHEL:**
   ````bash
   sudo dnf install gcc pkg-config openssl-devel
   ````

2. **Clean and rebuild:**
   ````bash
   cargo clean
   rm -rf target/
   cargo build
   ````

3. **Update Rust toolchain:**
   ````bash
   rustup update stable
   rustc --version
   ````

#### Linking errors

**Error:**
````
error: linking with `cc` failed
````

**Solution:**

1. **Ensure compiler is installed:**
   ````bash
   # macOS
   xcode-select --install
   
   # Linux
   sudo apt-get install build-essential
   ````

2. **Check Rust installation:**
   ````bash
   rustup show
   rustup default stable
   ````

### Runtime Issues

#### Application crashes on startup

**Solutions:**

1. **Check Docker availability:**
   ````bash
   docker info
   ````
   
   Doctainr continues with errors if Docker is unavailable, but verify it's running.

2. **Run with debug output:**
   ````bash
   RUST_BACKTRACE=1 cargo run
   ````

3. **Check asset files:**
   ````bash
   ls -la assets/
   ````
   
   Ensure `assets/favicon.ico`, `assets/icon.svg`, and `assets/styling/main.css` exist.

#### Container actions fail (start/stop)

**Symptoms:**
- Clicking "Start" or "Stop" shows error
- Container state doesn't change

**Solutions:**

1. **Check container exists:**
   ````bash
   docker ps -a | grep <container-id>
   ````

2. **Verify Docker permissions:**
   ````bash
   docker start <container-id>
   docker stop <container-id>
   ````

3. **Check container status:**
   - Cannot start already running containers
   - Cannot stop already stopped containers
   - Some containers may require force stop

4. **Review error message:**
   - Doctainr displays error messages in red banner
   - Check terminal output for detailed logs

#### UI not updating after actions

**Solutions:**

1. **Click "Refresh" button:**
   - Dashboard: "Refresh All"
   - Containers/Images/Volumes: "Refresh"

2. **Wait for async operations:**
   - Docker API calls are asynchronous
   - UI updates after operation completes

3. **Check for errors:**
   - Look for red error banner in UI
   - Check terminal for error logs

### Development Issues

#### Hot reload not working

**Symptoms:**
- Changes to code don't reflect in running app
- Must restart `dx serve` manually

**Solutions:**

1. **Verify using dx serve:**
   ````bash
   dx serve --platform desktop
   ````
   
   Not `cargo run` (no hot reload with cargo).

2. **Check file watchers:**
   ````bash
   # Linux: Increase inotify limit
   echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf
   sudo sysctl -p
   ````

3. **Restart dx serve:**
   ````bash
   # Ctrl+C to stop
   dx serve --platform desktop
   ````

#### Formatting/linting issues

**Problem:** Code doesn't pass CI checks

**Solutions:**

1. **Format code:**
   ````bash
   dx fmt
   # or
   cargo fmt
   ````

2. **Fix Clippy warnings:**
   ````bash
   dx check
   cargo clippy --fix --allow-dirty
   ````

3. **Run checks before commit:**
   ````bash
   dx fmt --check
   dx check
   cargo test
   ````

#### Tests failing

**Solutions:**

1. **Ensure Docker is running:**
   ````bash
   docker info
   ````

2. **Run specific failing test:**
   ````bash
   cargo test test_name -- --nocapture
   ````

3. **Clean test artifacts:**
   ````bash
   cargo clean
   cargo test
   ````

4. **Check test isolation:**
   - Integration tests may interfere with each other
   - Run single-threaded: `cargo test -- --test-threads=1`

### Platform-Specific Issues

#### macOS: App doesn't launch

**Solutions:**

1. **Check Gatekeeper:**
   ````bash
   xattr -d com.apple.quarantine target/release/doctainr
   ````

2. **Verify binary architecture:**
   ````bash
   file target/release/doctainr
   # Should match your Mac (x86_64 or arm64)
   ````

3. **Run from terminal first:**
   ````bash
   ./target/release/doctainr
   ````

#### Linux: Missing graphics libraries

**Error:**
````
error while loading shared libraries: libxcb.so
````

**Solution:**
````bash
# Debian/Ubuntu
sudo apt-get install libxcb1 libxcb-render0 libxcb-shape0 libxcb-xfixes0

# Fedora
sudo dnf install libxcb

# Arch
sudo pacman -S libxcb
````

#### Windows: Application won't start

**Solutions:**

1. **Run as administrator** (first time only)

2. **Check Docker Desktop is running:**
   - Look for whale icon in system tray
   - Open Docker Desktop app

3. **Verify Docker mode:**
   - Docker Desktop > Settings > General
   - "Use the WSL 2 based engine" should be checked (Windows 10/11)

4. **Check Windows Defender:**
   - May block unsigned executable
   - Add exception for Doctainr

### Performance Issues

#### Slow application startup

**Solutions:**

1. **Use release build:**
   ````bash
   cargo build --release
   ./target/release/doctainr
   ````

2. **Reduce Docker load:**
   - Stop unnecessary containers
   - Clean up unused images: `docker image prune`

3. **Check system resources:**
   - Docker Desktop may use significant CPU/RAM
   - Close other applications

#### UI feels sluggish

**Solutions:**

1. **Reduce refresh frequency:**
   - Avoid clicking "Refresh" repeatedly
   - Let async operations complete

2. **Limit Docker objects:**
   - Many containers/images slow API calls
   - Clean up unused resources

3. **Profile the app:**
   ````bash
   cargo install flamegraph
   cargo flamegraph
   ````

### Configuration Issues

#### Environment variables not recognized

**Problem:** Custom `DOCKER_HOST` not working

**Solutions:**

1. **Export before running:**
   ````bash
   export DOCKER_HOST=unix:///custom/docker.sock
   dx run
   ````

2. **Make permanent (add to shell config):**
   ````bash
   # ~/.bashrc or ~/.zshrc
   export DOCKER_HOST=unix:///custom/docker.sock
   ````

3. **Verify variable is set:**
   ````bash
   echo $DOCKER_HOST
   env | grep DOCKER
   ````

#### Custom assets not loading

**Problem:** Changed CSS/icons don't appear

**Solutions:**

1. **Verify asset path:**
   ````bash
   ls -la assets/styling/main.css
   ````

2. **Check asset macro usage:**
   ````rust
   const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
   ````
   
   Path must start with `/` and be relative to project root.

3. **Rebuild after asset changes:**
   ````bash
   dx build
   dx serve --platform desktop
   ````

## Diagnostic Commands

### Check system status

````bash
# Docker status
docker info
docker version

# Rust toolchain
rustc --version
cargo --version

# Dioxus CLI
dx --version

# System info
uname -a  # Linux/macOS
systeminfo  # Windows
````

### Collect logs

````bash
# Run with debug output
RUST_LOG=debug cargo run 2>&1 | tee doctainr.log

# Docker logs
docker logs <container-id>

# System logs (Linux)
journalctl -u docker.service
````

### Test Docker connectivity

````bash
# List containers
docker ps -a

# Run test container
docker run --rm hello-world

# Check socket
curl --unix-socket /var/run/docker.sock http://localhost/version
````

## Getting Help

If you're still experiencing issues:

1. **Check documentation:**
   - [Development Guide](development.md)
   - [Architecture Reference](architecture.md)
   - [Contributing Guide](contributing.md)

2. **Search existing issues:**
   - [GitHub Issues](https://github.com/MH0386/doctainr/issues)

3. **Create a new issue:**
   - Include error messages
   - Provide system information
   - Describe reproduction steps
   - Attach logs if relevant

4. **Community support:**
   - GitHub Discussions
   - Project README

## Debugging Tips

### Enable verbose logging

````bash
# Set Rust log level
export RUST_LOG=debug
cargo run

# Or inline
RUST_LOG=trace cargo run
````

### Inspect Docker API calls

````bash
# Enable Bollard debug logging
export RUST_LOG=bollard=trace
cargo run
````

### Profile performance

````bash
# Install profiling tools
cargo install cargo-flamegraph

# Generate flamegraph
cargo flamegraph

# Open flamegraph.svg in browser
````

### Debug with GDB/LLDB

````bash
# Build with debug symbols
cargo build

# Run in debugger
rust-gdb target/debug/doctainr
# or
rust-lldb target/debug/doctainr
````

## Known Limitations

1. **Volume sizes**: Docker API doesn't provide volume sizes without inspection (displayed as "--")
2. **Remote Docker**: Limited testing with remote Docker hosts
3. **Windows paths**: Some path handling may differ on Windows
4. **Container logs**: Not yet implemented (future feature)
5. **Docker Compose**: No Compose integration yet

## Reporting Bugs

When reporting bugs, include:

1. **System information:**
   - OS and version
   - Docker version
   - Rust version
   - Doctainr version

2. **Error details:**
   - Full error message
   - Stack trace if available
   - Steps to reproduce

3. **Logs:**
   - Run with `RUST_BACKTRACE=1`
   - Include relevant output

4. **Context:**
   - What you were trying to do
   - Expected vs actual behavior

See [Contributing Guide](contributing.md) for bug report template.
