# Troubleshooting Guide

This guide helps resolve common issues when using or developing Doctainr.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Docker Connection Issues](#docker-connection-issues)
- [Application Startup Issues](#application-startup-issues)
- [Container Operation Issues](#container-operation-issues)
- [Build and Development Issues](#build-and-development-issues)
- [Platform-Specific Issues](#platform-specific-issues)
- [Performance Issues](#performance-issues)
- [Getting Help](#getting-help)

## Installation Issues

### Cannot Install Rust

**Problem**: Rust installation fails

**Solutions**:

1. Check system compatibility:
   ```bash
   # Ensure you have curl
   which curl
   
   # Check internet connection
   ping -c 3 sh.rustup.rs
   ```

2. Manual installation:
   ```bash
   # Download rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh
   
   # Make executable
   chmod +x rustup-init.sh
   
   # Run installer
   ./rustup-init.sh
   ```

3. Check PATH:
   ```bash
   # Add to ~/.bashrc or ~/.zshrc
   export PATH="$HOME/.cargo/bin:$PATH"
   source ~/.bashrc
   ```

### Dioxus CLI Installation Fails

**Problem**: `cargo install dioxus-cli` fails

**Solutions**:

1. Update Rust:
   ```bash
   rustup update stable
   ```

2. Clear cargo cache:
   ```bash
   rm -rf ~/.cargo/registry
   cargo install dioxus-cli
   ```

3. Install from specific version:
   ```bash
   cargo install dioxus-cli --version 0.7.1
   ```

### Missing System Dependencies

**Problem**: Build fails due to missing dependencies

**Linux Solutions**:

```bash
# Debian/Ubuntu
sudo apt-get update
sudo apt-get install -y build-essential libgtk-3-dev libwebkit2gtk-4.1-dev \
  libssl-dev pkg-config

# Fedora
sudo dnf install gtk3-devel webkit2gtk4.1-devel openssl-devel

# Arch
sudo pacman -S gtk3 webkit2gtk base-devel
```

**macOS Solutions**:

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

## Docker Connection Issues

### "Failed to connect to Docker"

**Problem**: Doctainr cannot connect to Docker daemon

**Diagnosis**:

```bash
# Check if Docker is running
docker info

# Check Docker socket
ls -l /var/run/docker.sock
```

**Solutions**:

1. **Start Docker**:
   ```bash
   # Linux (systemd)
   sudo systemctl start docker
   
   # macOS/Windows
   # Start Docker Desktop application
   ```

2. **Check Docker socket permissions**:
   ```bash
   # Add user to docker group (Linux)
   sudo usermod -aG docker $USER
   
   # Log out and back in, or run:
   newgrp docker
   
   # Verify
   docker ps
   ```

3. **Set DOCKER_HOST**:
   ```bash
   # If using non-default socket
   export DOCKER_HOST=unix:///var/run/docker.sock
   
   # For TCP connection
   export DOCKER_HOST=tcp://localhost:2375
   ```

4. **Check Docker socket exists**:
   ```bash
   # Should exist
   ls /var/run/docker.sock
   
   # If missing, Docker isn't running
   ```

### Permission Denied

**Problem**: "Permission denied connecting to Docker socket"

**Solutions**:

1. **Linux**: Add user to docker group
   ```bash
   sudo usermod -aG docker $USER
   newgrp docker
   ```

2. **Temporary fix** (not recommended):
   ```bash
   sudo chmod 666 /var/run/docker.sock
   ```

3. **Verify group membership**:
   ```bash
   groups
   # Should include 'docker'
   ```

### Docker Not Found

**Problem**: Docker command not found

**Solutions**:

1. **Install Docker**:
   - Linux: https://docs.docker.com/engine/install/
   - macOS: Install Docker Desktop
   - Windows: Install Docker Desktop

2. **Check PATH**:
   ```bash
   which docker
   echo $PATH
   ```

## Application Startup Issues

### Window Doesn't Open

**Problem**: Doctainr starts but no window appears

**Solutions**:

1. **Check logs**:
   ```bash
   # Run with logging
   RUST_LOG=debug dx run
   ```

2. **GTK/WebKit issues (Linux)**:
   ```bash
   # Reinstall GTK
   sudo apt-get install --reinstall libgtk-3-0 libwebkit2gtk-4.1-0
   ```

3. **Display issues**:
   ```bash
   # Check DISPLAY variable
   echo $DISPLAY
   
   # Should be something like :0 or :1
   # If empty, set it:
   export DISPLAY=:0
   ```

### Application Crashes on Startup

**Problem**: Doctainr crashes immediately

**Diagnosis**:

```bash
# Run with backtrace
RUST_BACKTRACE=1 dx run

# Run with debug logging
RUST_LOG=debug dx run
```

**Common Causes**:

1. **Docker not running**:
   ```bash
   # Start Docker first
   sudo systemctl start docker
   ```

2. **Missing libraries**:
   ```bash
   # Check for missing libraries
   ldd target/release/doctainr
   ```

3. **Configuration corruption**:
   ```bash
   # Remove config
   rm -rf ~/.config/doctainr/
   ```

### Slow Startup

**Problem**: Application takes a long time to start

**Solutions**:

1. **Docker daemon slow to respond**:
   ```bash
   # Check Docker performance
   time docker ps
   ```

2. **Many containers**:
   - Doctainr loads all data on startup
   - This is normal with many containers

3. **Build in release mode**:
   ```bash
   dx build --release
   dx run --release
   ```

## Container Operation Issues

### "Failed to start container"

**Problem**: Start operation fails

**Diagnosis**:

```bash
# Try with Docker CLI
docker start <container-id>

# Check container status
docker inspect <container-id>
```

**Solutions**:

1. **Port conflicts**:
   - Another container using the same port
   - Check with: `docker ps -a`

2. **Resource limits**:
   - Insufficient memory/CPU
   - Check: `docker stats`

3. **Container errors**:
   ```bash
   # View container logs
   docker logs <container-id>
   ```

### "Failed to stop container"

**Problem**: Stop operation fails

**Solutions**:

1. **Container not responsive**:
   ```bash
   # Force stop
   docker kill <container-id>
   ```

2. **Timeout**:
   - Stop operation has timeout
   - Container may be frozen

3. **Check Docker daemon**:
   ```bash
   # Restart Docker
   sudo systemctl restart docker
   ```

### Data Not Refreshing

**Problem**: Container list doesn't update

**Solutions**:

1. **Click Refresh button**: Data doesn't auto-refresh

2. **Docker operations pending**:
   - Wait for operations to complete
   - Check Docker daemon: `docker ps`

3. **Application state issue**:
   - Restart Doctainr

## Build and Development Issues

### Build Fails

**Problem**: `dx build` fails

**Solutions**:

1. **Update toolchain**:
   ```bash
   rustup update stable
   ```

2. **Clean and rebuild**:
   ```bash
   dx clean
   dx build
   ```

3. **Check dependency versions**:
   ```bash
   # Update dependencies
   cargo update
   ```

4. **Network issues**:
   ```bash
   # Use a different crates.io mirror
   # Edit ~/.cargo/config.toml
   ```

### Linting Errors

**Problem**: `dx check` reports errors

**Solutions**:

1. **Review and fix issues**:
   ```bash
   dx check
   ```

2. **Allow specific lints** (if false positive):
   ```rust
   #[allow(clippy::lint_name)]
   ```

3. **Update clippy**:
   ```bash
   rustup update
   ```

### Tests Fail

**Problem**: `cargo test` fails

**Diagnosis**:

```bash
# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name -- --nocapture
```

**Solutions**:

1. **Docker not running**:
   - Many tests require Docker
   - Start Docker before running tests

2. **Test isolation**:
   ```bash
   # Run tests serially
   cargo test -- --test-threads=1
   ```

3. **Clean test state**:
   ```bash
   # Remove test artifacts
   cargo clean
   ```

### Hot Reload Not Working

**Problem**: Changes don't trigger rebuild

**Solutions**:

1. **Restart dx serve**:
   ```bash
   # Stop with Ctrl+C
   # Start again
   dx serve --platform desktop
   ```

2. **Macro changes**:
   - Macro changes require full rebuild
   - Stop and restart dx serve

3. **Clear build cache**:
   ```bash
   dx clean
   dx serve --platform desktop
   ```

## Platform-Specific Issues

### Linux

#### Wayland Issues

**Problem**: Application doesn't run on Wayland

**Solution**:

```bash
# Force X11
export GDK_BACKEND=x11
dx run
```

#### SELinux Issues

**Problem**: SELinux blocks Docker socket access

**Solution**:

```bash
# Check SELinux status
sestatus

# Allow Docker socket access
sudo semanage fcontext -a -t svirt_sandbox_file_t /var/run/docker.sock
sudo restorecon -v /var/run/docker.sock
```

### macOS

#### Code Signing Issues

**Problem**: "App is damaged and can't be opened"

**Solution**:

```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine Doctainr.app

# Or
xattr -cr Doctainr.app
```

#### Gatekeeper Issues

**Problem**: macOS prevents opening unsigned app

**Solution**:

1. Right-click app and select "Open"
2. Click "Open" in dialog

Or disable Gatekeeper (not recommended):

```bash
sudo spctl --master-disable
```

### Windows

#### SmartScreen Warning

**Problem**: Windows SmartScreen blocks app

**Solution**:

1. Click "More info"
2. Click "Run anyway"

Or sign the binary with a code signing certificate.

#### Docker Desktop Not Starting

**Problem**: Docker Desktop fails to start

**Solution**:

1. Enable WSL2:
   ```powershell
   wsl --install
   ```

2. Enable Hyper-V:
   - Open PowerShell as Administrator
   - Run: `Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All`

3. Restart Windows

## Performance Issues

### High CPU Usage

**Problem**: Doctainr uses excessive CPU

**Diagnosis**:

```bash
# Check with top/htop
top -p $(pgrep doctainr)
```

**Solutions**:

1. **Reduce refresh frequency**: Don't click refresh too often

2. **Many containers**: Large container lists take more CPU

3. **Build in release mode**:
   ```bash
   dx build --release
   ```

### High Memory Usage

**Problem**: Doctainr uses too much memory

**Solutions**:

1. **Expected behavior**: Memory usage scales with data size

2. **Memory leak**: Report as bug with reproduction steps

3. **Restart application**: Temporary workaround

### Slow UI Response

**Problem**: UI feels sluggish

**Solutions**:

1. **Docker daemon slow**:
   ```bash
   # Check Docker performance
   time docker ps
   ```

2. **Too many containers**: Performance degrades with many resources

3. **System resources**: Check overall system load

## Getting Help

### Gather Information

Before asking for help, collect:

1. **Version information**:
   ```bash
   dx --version
   cargo --version
   docker --version
   ```

2. **System information**:
   ```bash
   uname -a  # Linux/macOS
   systeminfo  # Windows
   ```

3. **Error messages**:
   - Copy full error output
   - Include stack traces

4. **Logs**:
   ```bash
   RUST_LOG=debug dx run 2>&1 | tee doctainr.log
   ```

### Where to Get Help

1. **Documentation**: Check all docs in `docs/` directory

2. **GitHub Issues**: https://github.com/MH0386/doctainr/issues
   - Search existing issues
   - Create new issue with template

3. **Community**:
   - Dioxus Discord (for Dioxus questions)
   - Rust forums (for Rust questions)

### Reporting Bugs

Include in bug reports:

- **Description**: Clear description of the problem
- **Steps to reproduce**: Exact steps to reproduce
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Environment**: OS, versions, Docker info
- **Logs**: Relevant log output
- **Screenshots**: If UI issue

### Feature Requests

For feature requests:

- Check existing issues first
- Describe the use case
- Explain why it's useful
- Suggest implementation if possible

## Common Error Messages

### "No such container"

**Meaning**: Container ID is invalid or container was removed

**Solution**: Refresh the container list

### "Container is already running"

**Meaning**: Trying to start an already running container

**Solution**: Refresh to see current state

### "Container is already stopped"

**Meaning**: Trying to stop an already stopped container

**Solution**: Refresh to see current state

### "Connection refused"

**Meaning**: Cannot connect to Docker daemon

**Solution**: Start Docker daemon

### "Permission denied"

**Meaning**: No permission to access Docker socket

**Solution**: Add user to docker group

## Advanced Troubleshooting

### Debug Logging

Enable detailed logging:

```bash
# Maximum verbosity
RUST_LOG=trace dx run

# Specific modules
RUST_LOG=doctainr=debug dx run
```

### Network Debugging

Check Docker socket communication:

```bash
# Test socket directly
curl --unix-socket /var/run/docker.sock http://localhost/containers/json

# Monitor socket access
sudo strace -e trace=connect doctainr
```

### Memory Debugging

Check for memory leaks:

```bash
# Using valgrind (Linux)
valgrind --leak-check=full --show-leak-kinds=all dx run

# Using heaptrack (Linux)
heaptrack dx run
```

### Performance Profiling

Profile the application:

```bash
# Using perf (Linux)
perf record -g dx run
perf report

# Using cargo-flamegraph
cargo install flamegraph
cargo flamegraph
```

## Still Having Issues?

If none of these solutions work:

1. Check for similar issues on GitHub
2. Create a detailed bug report
3. Include all diagnostic information
4. Be patient and responsive to questions

---

For more information:
- [Development Guide](DEVELOPMENT.md)
- [User Guide](USER_GUIDE.md)
- [Contributing Guide](../CONTRIBUTING.md)
