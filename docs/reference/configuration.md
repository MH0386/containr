# Configuration

Configuration options and environment variables for Doctainr.

## Environment Variables

### DOCKER_HOST

Specifies the Docker daemon connection endpoint.

**Default:**
- **Linux/macOS:** `unix:///var/run/docker.sock`
- **Windows:** `npipe:////./pipe/docker_engine`

**Usage:**

````bash
# Connect to local Docker socket (default)
dx serve

# Connect to remote Docker over TCP
export DOCKER_HOST=tcp://192.168.1.100:2375
dx serve

# Connect via SSH tunnel
export DOCKER_HOST=ssh://user@remote-host
dx serve
````

**Formats:**
- **Unix socket:** `unix:///path/to/docker.sock`
- **TCP:** `tcp://host:port`
- **TLS:** `tcp://host:port` (requires TLS certificates)
- **SSH:** `ssh://user@host`
- **Windows named pipe:** `npipe:////./pipe/docker_engine`

---

### RUST_LOG

Controls logging verbosity for debugging.

**Default:** Not set (minimal logging)

**Usage:**

````bash
# Enable debug logs
export RUST_LOG=debug
dx serve

# Enable trace logs for specific module
export RUST_LOG=doctainr::services=trace
dx serve

# Multiple modules
export RUST_LOG=doctainr::services=debug,bollard=info
dx serve
````

**Levels:**
- `error` — Errors only
- `warn` — Warnings and errors
- `info` — Informational messages
- `debug` — Detailed debugging
- `trace` — Very verbose (includes API calls)

---

## Cargo Features

Build-time features to customize the application.

### Available Features

- **`desktop`** (default) — Native desktop application
- **`web`** — WebAssembly build for browsers (experimental)

**Usage:**

````bash
# Desktop build (default)
cargo build --features desktop

# Web build
cargo build --features web

# No default features
cargo build --no-default-features --features web
````

---

## Dioxus Configuration

Doctainr uses `Dioxus.toml` for build configuration.

### Dioxus.toml

````toml
[application]
name = "doctainr"
default_platform = "desktop"

[desktop]
title = "Doctainr"
````

**Fields:**
- `name` — Application name (affects binary name)
- `default_platform` — Platform to use with `dx serve`
- `title` — Desktop window title

---

## Asset Configuration

Assets are referenced via the `asset!()` macro and bundled at build time.

### Asset Directory Structure

````
assets/
├── favicon.ico          # Window icon
├── icon.svg             # Application logo
├── styling/
│   └── main.css         # Main stylesheet
└── tailwind.css         # Tailwind styles
````

### Loading Assets

````rust
const FAVICON: Asset = asset!("/assets/icon.svg");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }
}
````

Assets are:
- **Optimized** — Minified at build time
- **Embedded** — Bundled into binary for desktop
- **Hashed** — Cache-busted for web builds

---

## Docker Configuration

Doctainr respects standard Docker configuration files.

### Docker Context

To switch Docker contexts:

````bash
# List contexts
docker context ls

# Switch context
docker context use my-remote-context

# Run Doctainr (uses active context)
dx serve
````

### TLS Certificates

For TLS-secured Docker connections, ensure certificates are in the default location:

````
~/.docker/
├── ca.pem       # Certificate Authority
├── cert.pem     # Client certificate
└── key.pem      # Client private key
````

Set `DOCKER_HOST` with TLS:

````bash
export DOCKER_HOST=tcp://docker.example.com:2376
export DOCKER_TLS_VERIFY=1
export DOCKER_CERT_PATH=~/.docker
dx serve
````

---

## Development Configuration

### Hot Reload

Enable hot reload during development:

````bash
dx serve --hot-reload
````

Changes to source files trigger automatic rebuild and reload.

### Custom Port

Change the development server port:

````bash
dx serve --port 9000
````

### Platform Selection

Target specific platform:

````bash
# Desktop (native window)
dx serve --platform desktop

# Web (browser)
dx serve --platform web
````

---

## Build Configuration

### Release Build

Optimize for production:

````bash
dx bundle --release
````

**Optimizations:**
- Binary size reduction (~5-10 MB)
- Aggressive inlining
- Dead code elimination
- Asset minification

### Custom Build Directory

````bash
cargo build --target-dir /custom/path
````

---

## Runtime Configuration (Future)

Planned configuration options (not yet implemented):

### Settings File

````toml
# ~/.config/doctainr/config.toml

[general]
theme = "dark"
auto_refresh = true
refresh_interval = 30

[docker]
host = "unix:///var/run/docker.sock"
timeout = 10

[ui]
show_stopped_containers = true
compact_mode = false
````

### CLI Arguments

````bash
doctainr --theme dark --auto-refresh
````

---

## Troubleshooting Configuration

### Verify Docker Connection

Test Docker connection independently:

````bash
docker info
````

If this fails, Doctainr will also fail to connect.

### Debug Environment Variables

Print effective configuration:

````bash
env | grep DOCKER
````

### Check File Permissions

Ensure user can access Docker socket:

````bash
ls -la /var/run/docker.sock
# Should show: srw-rw---- 1 root docker

# Add user to docker group if needed
sudo usermod -aG docker $USER
newgrp docker
````

### Validate Dioxus Config

Check syntax:

````bash
dx config
````

---

## Platform-Specific Notes

### Linux

- **Socket:** `/var/run/docker.sock`
- **Permissions:** User must be in `docker` group
- **Dependencies:** Requires WebKitGTK for desktop build

### macOS

- **Socket:** `/var/run/docker.sock` (Docker Desktop)
- **Permissions:** Automatic with Docker Desktop
- **Dependencies:** No extra dependencies

### Windows

- **Pipe:** `npipe:////./pipe/docker_engine`
- **Permissions:** Docker Desktop handles permissions
- **Dependencies:** WebView2 runtime

---

## See Also

- [Getting Started](../tutorials/getting-started.md) — Installation and setup
- [Docker API Integration](../explanation/docker-api.md) — Connection details
- [Architecture](./architecture.md) — System overview
