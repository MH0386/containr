# Troubleshooting

Common issues and their solutions when working with Doctainr.

## Connection Issues

### Docker Daemon Not Responding

**Symptoms**:
- Error: "Failed to connect to Docker"
- Application shows "Docker service not available"
- All operations fail immediately

**Solutions**:

1. **Verify Docker is running**:
   ````bash
   docker info
   ````
   
   If this fails, start Docker:
   
   ````bash
   # Linux (systemd)
   sudo systemctl start docker
   sudo systemctl enable docker  # Auto-start on boot
   
   # macOS
   open -a Docker
   
   # Windows
   # Start Docker Desktop from Start menu
   ````

2. **Check Docker socket permissions** (Linux/macOS):
   ````bash
   ls -l /var/run/docker.sock
   # Should show: srw-rw---- 1 root docker
   
   # Add your user to docker group
   sudo usermod -aG docker $USER
   # Log out and back in for changes to take effect
   ````

3. **Verify socket path**:
   ````bash
   echo $DOCKER_HOST
   # Should be empty or show correct path
   
   # If using non-standard path:
   export DOCKER_HOST=unix:///path/to/docker.sock
   dx serve
   ````

### Connection Timeout

**Symptoms**:
- Application hangs during startup
- Operations never complete
- No error message displayed

**Solutions**:

1. **Check Docker daemon status**:
   ````bash
   docker version
   ````

2. **Test connection manually**:
   ````bash
   curl --unix-socket /var/run/docker.sock http://localhost/version
   ````

3. **Restart Docker daemon**:
   ````bash
   sudo systemctl restart docker  # Linux
   # Or restart Docker Desktop
   ````

### Remote Docker Host Issues

**Symptoms**:
- Cannot connect to remote Docker daemon
- TLS authentication errors

**Solutions**:

1. **Set DOCKER_HOST**:
   ````bash
   export DOCKER_HOST=tcp://remote-host:2375
   dx serve
   ````

2. **For TLS connections**:
   ````bash
   export DOCKER_HOST=tcp://remote-host:2376
   export DOCKER_TLS_VERIFY=1
   export DOCKER_CERT_PATH=/path/to/certs
   dx serve
   ````

3. **Test connection**:
   ````bash
   docker -H tcp://remote-host:2375 info
   ````

## Build Issues

### Compilation Errors

**Symptoms**:
- `cargo build` fails
- Dependency resolution errors

**Solutions**:

1. **Update Rust**:
   ````bash
   rustup update stable
   rustc --version  # Should be 1.70+
   ````

2. **Clean build artifacts**:
   ````bash
   cargo clean
   cargo build
   ````

3. **Update dependencies**:
   ````bash
   cargo update
   cargo build
   ````

4. **Check Cargo.lock conflicts**:
   ````bash
   git checkout main -- Cargo.lock
   cargo build
   ````

### Missing Dependencies

**Symptoms**:
- Linker errors
- Missing library errors

**Solutions**:

1. **Install system dependencies** (Linux):
   ````bash
   # Debian/Ubuntu
   sudo apt install libwebkit2gtk-4.0-dev \
       build-essential curl wget libssl-dev \
       libgtk-3-dev libayatana-appindicator3-dev \
       librsvg2-dev
   
   # Fedora
   sudo dnf install webkit2gtk3-devel openssl-devel \
       gtk3-devel libappindicator-gtk3-devel librsvg2-devel
   ````

2. **macOS dependencies**:
   ````bash
   # Usually none needed, but ensure Xcode CLI tools:
   xcode-select --install
   ````

3. **Windows dependencies**:
   - Install Visual Studio Build Tools
   - Ensure Windows SDK is installed

### Dioxus CLI Issues

**Symptoms**:
- `dx` command not found
- Hot reload not working

**Solutions**:

1. **Reinstall dx**:
   ````bash
   cargo install dioxus-cli --force
   dx --version
   ````

2. **Add cargo bin to PATH**:
   ````bash
   export PATH="$HOME/.cargo/bin:$PATH"
   # Add to ~/.bashrc or ~/.zshrc
   ````

3. **Use cargo directly**:
   ````bash
   cargo run
   ````

## Runtime Issues

### Application Won't Start

**Symptoms**:
- Window doesn't appear
- Crashes immediately
- Black screen

**Solutions**:

1. **Check logs**:
   ````bash
   RUST_LOG=debug cargo run 2>&1 | tee app.log
   ````

2. **Try different renderer** (Linux):
   ````bash
   # Force software rendering
   WEBKIT_DISABLE_COMPOSITING_MODE=1 dx serve
   ````

3. **Check graphics drivers**:
   - Update to latest GPU drivers
   - Try running in virtual machine to isolate

### Container Operations Fail

**Symptoms**:
- "Failed to start container"
- "Failed to stop container"
- Permission denied errors

**Solutions**:

1. **Check Docker permissions**:
   ````bash
   docker ps  # Should work without sudo
   ````

2. **Verify container exists**:
   ````bash
   docker ps -a | grep container_name
   ````

3. **Check Docker logs**:
   ````bash
   journalctl -u docker.service  # Linux
   ````

### Images Not Loading

**Symptoms**:
- Empty image list
- "Failed to list images" error

**Solutions**:

1. **Verify images exist**:
   ````bash
   docker images
   ````

2. **Check Docker storage**:
   ````bash
   docker system df
   docker system prune  # Free up space
   ````

3. **Pull a test image**:
   ````bash
   docker pull hello-world
   # Refresh in Doctainr
   ````

### Memory or Performance Issues

**Symptoms**:
- High memory usage
- Slow response times
- UI freezes

**Solutions**:

1. **Limit Docker resources** (Docker Desktop):
   - Open Docker Desktop settings
   - Reduce memory/CPU limits
   - Restart Docker

2. **Close unused containers**:
   ````bash
   docker container prune
   ````

3. **Monitor resources**:
   ````bash
   # Monitor Docker
   docker stats
   
   # Monitor application
   ps aux | grep doctainr
   ````

4. **Build release version**:
   ````bash
   dx build --release
   ./target/release/doctainr
   ````

## UI Issues

### Styling Problems

**Symptoms**:
- Missing styles
- Layout broken
- Colors wrong

**Solutions**:

1. **Rebuild Tailwind CSS**:
   ````bash
   npm install -D tailwindcss
   npx tailwindcss build -o tailwind.css
   ````

2. **Clear browser cache** (if web target)

3. **Check CSS file exists**:
   ````bash
   ls -l assets/styling/main.css
   ls -l tailwind.css
   ````

### Window Size Issues

**Symptoms**:
- Window too small
- Content cut off
- Can't resize window

**Solutions**:

1. **Check Dioxus.toml**:
   ````toml
   [application.desktop]
   default_window_size = [1200, 800]
   ````

2. **Reset window state** (delete config file):
   ````bash
   # macOS
   rm ~/Library/Application Support/com.doctainr/window.json
   
   # Linux
   rm ~/.config/doctainr/window.json
   
   # Windows
   del %APPDATA%\doctainr\window.json
   ````

### Navigation Not Working

**Symptoms**:
- Links don't navigate
- Stuck on one page

**Solutions**:

1. **Check browser console** (for web):
   - Open DevTools (F12)
   - Look for JavaScript errors

2. **Verify routes defined**:
   ````rust
   // In main.rs
   #[derive(Routable)]
   enum Route {
       // All routes listed
   }
   ````

## Platform-Specific Issues

### Linux

**Wayland Issues**:
````bash
# Force X11
GDK_BACKEND=x11 dx serve
````

**AppArmor/SELinux**:
````bash
# Check for denials
sudo ausearch -m avc -ts recent
# or
sudo journalctl | grep audit
````

### macOS

**Permission Errors**:
- Grant Full Disk Access in System Preferences → Security & Privacy

**Gatekeeper Warnings**:
````bash
# Remove quarantine flag
xattr -d com.apple.quarantine /path/to/doctainr
````

### Windows

**Named Pipe Issues**:
````bash
# Verify Docker Desktop using named pipes
docker context ls
# Should show: npipe:////./pipe/docker_engine
````

**Firewall Blocking**:
- Add exception in Windows Defender Firewall

## Debugging Tips

### Enable Debug Logging

````rust
// In main.rs
#[cfg(debug_assertions)]
dioxus::logger::init(dioxus::logger::Level::Debug).ok();
````

Run with verbose output:
````bash
RUST_LOG=debug cargo run 2>&1 | tee debug.log
````

### Inspect Docker API

Use `curl` to test Docker API:
````bash
# List containers
curl --unix-socket /var/run/docker.sock \
    http://localhost/v1.41/containers/json?all=1

# Container details
curl --unix-socket /var/run/docker.sock \
    http://localhost/v1.41/containers/CONTAINER_ID/json
````

### Check Network Connectivity

````bash
# Test connection
nc -zv localhost 2375  # TCP
# or
curl --unix-socket /var/run/docker.sock http://localhost/version
````

### Bisect Issues

If regression after update:
````bash
git bisect start
git bisect bad main
git bisect good v0.1.0
# Test at each step
````

## Getting More Help

If issues persist:

1. **Search existing issues**: [GitHub Issues](https://github.com/MH0386/doctainr/issues)
2. **Create new issue** with:
   - Operating system and version
   - Docker version (`docker version`)
   - Rust version (`rustc --version`)
   - Steps to reproduce
   - Full error messages
   - Relevant logs
3. **Join discussions**: [GitHub Discussions](https://github.com/MH0386/doctainr/discussions)

## Common Error Messages

### "Failed to connect to Docker daemon"
→ See [Docker Daemon Not Responding](#docker-daemon-not-responding)

### "Permission denied"
→ Add user to `docker` group or run Docker as your user

### "Address already in use"
→ Another instance is running or port is occupied

### "Container not found"
→ Refresh container list or check container exists with `docker ps -a`

### "No such image"
→ Pull the image first: `docker pull image:tag`

### "Cannot start container"
→ Check container logs: `docker logs container_id`
