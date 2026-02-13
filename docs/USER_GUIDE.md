# User Guide

Complete guide to using Doctainr for Docker container management.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Dashboard Overview](#dashboard-overview)
3. [Managing Containers](#managing-containers)
4. [Working with Images](#working-with-images)
5. [Managing Volumes](#managing-volumes)
6. [Configuration](#configuration)
7. [Keyboard Shortcuts](#keyboard-shortcuts)
8. [Tips & Tricks](#tips--tricks)
9. [Troubleshooting](#troubleshooting)

---

## Getting Started

### First Launch

When you first launch Doctainr:

1. The application connects to your local Docker daemon
2. Initial data is loaded (containers, images, volumes)
3. The Dashboard view displays an overview of your Docker environment

**Connection Requirements**:
- Docker daemon must be running
- User must have Docker access permissions
- Default connection: `unix:///var/run/docker.sock` (Linux/macOS)

### Interface Layout

````
┌─────────────────────────────────────────────────┐
│  [Dashboard] [Containers] [Images] [Volumes]    │
│                                        [Settings]│
├─────────────────────────────────────────────────┤
│                                                   │
│              Main Content Area                   │
│                                                   │
│                                                   │
└─────────────────────────────────────────────────┘
````

**Navigation Bar**: Top of window with links to all sections  
**Content Area**: Current view/page content  
**Status Indicators**: Real-time updates for container states

---

## Dashboard Overview

The Dashboard provides at-a-glance status of your Docker environment.

### Metrics Displayed

**Running Containers**: Count of currently running containers  
**Stopped Containers**: Count of stopped containers  
**Total Images**: Number of Docker images available  
**Total Volumes**: Number of Docker volumes

### Docker Host Information

Displays the current Docker connection endpoint (e.g., `unix:///var/run/docker.sock`).

### Quick Actions

From the Dashboard, you can:
- Navigate to detailed views
- See connection status
- Identify issues at a glance

---

## Managing Containers

Navigate to **Containers** from the top navigation bar.

### Container List

Each container displays:
- **Name**: Container name (without leading `/`)
- **Image**: Base image (e.g., `nginx:latest`)
- **Status**: Human-readable status (e.g., "Up 2 hours")
- **Ports**: Port mappings (e.g., `0.0.0.0:8080->80/tcp`)
- **State**: Visual indicator (Running/Stopped)
- **Actions**: Start/Stop button

### Starting a Container

1. Locate the stopped container in the list
2. Click the **Start** button
3. Container state updates automatically
4. Status changes to "Running"

**Result**: Container starts and becomes accessible on its configured ports.

### Stopping a Container

1. Locate the running container
2. Click the **Stop** button
3. Container gracefully shuts down
4. Status changes to "Stopped"

**Note**: Stopping does not remove the container - data is preserved.

### Understanding Container States

**Running** (Green indicator):
- Container is actively running
- Processes are executing
- Ports are accessible
- Can be stopped

**Stopped** (Red indicator):
- Container is not running
- No processes active
- Ports unavailable
- Can be started

### Container Details

Each container row shows:

````
┌─────────────────────────────────────────────────────┐
│ my-container                              [Stop]    │
│ Image: nginx:alpine                                  │
│ Status: Up 2 hours                                   │
│ Ports: 0.0.0.0:8080->80/tcp                         │
│ State: ● Running                                     │
└─────────────────────────────────────────────────────┘
````

### Bulk Operations

Currently, actions are performed one container at a time. Bulk operations (start/stop multiple) are planned for future releases.

### Refreshing Container Data

Container data refreshes automatically when:
- Starting a container
- Stopping a container
- Navigating to the Containers view

**Manual Refresh**: Planned for future releases (via refresh button or keyboard shortcut).

---

## Working with Images

Navigate to **Images** from the top navigation bar.

### Image List

Each image displays:
- **Repository**: Image repository name
- **Tag**: Version tag (e.g., `latest`, `1.20-alpine`)
- **ID**: Shortened image ID
- **Size**: Disk space used
- **Created**: When the image was created

### Understanding Images

**Base Images**: Official images from Docker Hub (e.g., `ubuntu`, `postgres`)  
**Custom Images**: Built from Dockerfiles  
**Intermediate Images**: Created during builds (may show as `<none>`)

### Image Details

````
┌─────────────────────────────────────────────────────┐
│ nginx:alpine                                         │
│ ID: abc123def456                                     │
│ Size: 23.4 MB                                        │
│ Created: 2 weeks ago                                 │
└─────────────────────────────────────────────────────┘
````

### Future Features

Planned image operations:
- Pull new images
- Remove unused images
- Build from Dockerfile
- Inspect image layers

---

## Managing Volumes

Navigate to **Volumes** from the top navigation bar.

### Volume List

Each volume displays:
- **Name**: Volume identifier
- **Driver**: Storage driver (usually `local`)
- **Mountpoint**: Host filesystem path

### Understanding Volumes

**Named Volumes**: Created with explicit names (e.g., `postgres-data`)  
**Anonymous Volumes**: Auto-generated names (long hashes)  
**Bind Mounts**: Host directory mounts (not listed here)

### Volume Purpose

Volumes provide persistent storage for containers:
- Database data
- Configuration files
- User-uploaded content
- Logs and state

### Volume Details

````
┌─────────────────────────────────────────────────────┐
│ postgres-data                                        │
│ Driver: local                                        │
│ Mountpoint: /var/lib/docker/volumes/postgres-data   │
└─────────────────────────────────────────────────────┘
````

### Caution

⚠️ **Deleting volumes permanently removes data**. Volume deletion is not yet implemented in Doctainr to prevent accidental data loss.

### Future Features

Planned volume operations:
- Create new volumes
- Backup/export volumes
- Inspect volume usage
- Safe volume deletion with warnings

---

## Configuration

Navigate to **Settings** from the top navigation bar.

### Docker Connection

**Current Host**: Displays the active Docker endpoint

**Changing Connection** (via environment variable):

````bash
# Connect to remote Docker host
export DOCKER_HOST=tcp://192.168.1.100:2375
doctainr

# Connect via SSH
export DOCKER_HOST=ssh://user@remote-host
doctainr
````

⚠️ **Security**: TCP connections without TLS are insecure. Use SSH or configure TLS.

### Application Settings

Currently displays:
- Docker host connection
- Connection status

**Future Settings**:
- Auto-refresh interval
- Theme selection (light/dark)
- Notification preferences
- Default views

---

## Keyboard Shortcuts

*Keyboard shortcuts are planned for future releases.*

**Proposed Shortcuts**:
- `r` - Refresh current view
- `d` - Navigate to Dashboard
- `c` - Navigate to Containers
- `i` - Navigate to Images
- `v` - Navigate to Volumes
- `s` - Navigate to Settings
- `/` - Search/filter
- `?` - Help/shortcuts reference

---

## Tips & Tricks

### Performance

**Large Container Lists**: Doctainr handles hundreds of containers efficiently thanks to reactive updates. Only changed data triggers re-renders.

**Network Latency**: Remote Docker connections may have slower response times. Use SSH for secure remote management.

### Workflow Optimization

1. **Use Named Containers**: Name your containers for easy identification
   ````bash
   docker run --name my-web-server nginx
   ````

2. **Tag Images Meaningfully**: Use semantic versioning for custom images
   ````bash
   docker build -t myapp:1.2.3 .
   ````

3. **Named Volumes**: Create named volumes for important data
   ````bash
   docker volume create app-data
   ````

### Multi-Host Management

To manage multiple Docker hosts, launch separate Doctainr instances with different `DOCKER_HOST` values:

````bash
# Terminal 1: Local Docker
DOCKER_HOST=unix:///var/run/docker.sock doctainr

# Terminal 2: Remote Docker
DOCKER_HOST=ssh://server1 doctainr
````

*Multi-host support within a single instance is planned for future releases.*

---

## Troubleshooting

### Connection Issues

**Problem**: "Failed to connect to Docker"

**Solutions**:
1. Verify Docker is running: `docker ps`
2. Check socket permissions: `ls -la /var/run/docker.sock`
3. Add user to docker group: `sudo usermod -aG docker $USER` (logout/login required)
4. Verify `DOCKER_HOST` environment variable is correct

### Empty Lists

**Problem**: No containers/images/volumes displayed

**Possible Causes**:
- Docker daemon has no resources (clean installation)
- Connection to wrong Docker host
- Permission issues preventing listing

**Solution**: Verify with Docker CLI:
````bash
docker ps -a      # Should show containers
docker images     # Should show images
docker volume ls  # Should show volumes
````

### Container Start/Stop Fails

**Problem**: Action button doesn't work or shows error

**Possible Causes**:
- Container in transitional state
- Permission issues
- Container configuration problems

**Solution**:
1. Check Docker daemon logs
2. Try operation via Docker CLI to see detailed error
3. Restart Docker daemon if necessary

### Performance Issues

**Problem**: Slow response or UI lag

**Possible Causes**:
- Very large number of containers (1000+)
- Slow network connection to remote Docker host
- Resource constraints on host system

**Solutions**:
1. Filter/search functionality (planned feature)
2. Use local Docker connection when possible
3. Increase system resources

### Build Issues

**Problem**: Can't build Doctainr from source

**Solutions**:

**Missing Dependencies** (Linux):
````bash
# Ubuntu/Debian
sudo apt-get install libwebkit2gtk-4.1-dev libssl-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel

# Arch
sudo pacman -S webkit2gtk-4.1 openssl
````

**Rust Version**:
````bash
rustup update stable
cargo --version  # Should be 1.70+
````

---

## Getting Help

### Documentation

- **README.md**: Installation and quick start
- **CONTRIBUTING.md**: Developer guide
- **docs/ARCHITECTURE.md**: Technical architecture
- **docs/API.md**: API reference

### Community Support

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and community help

### Reporting Bugs

When reporting bugs, include:
1. Doctainr version
2. Operating system
3. Docker version: `docker --version`
4. Steps to reproduce
5. Expected vs actual behavior
6. Relevant logs or error messages

### Feature Requests

Feature requests are welcome! Use GitHub Issues with the "enhancement" label.

---

## What's Next?

Explore planned features and roadmap in the main [README.md](../README.md#roadmap).

**Coming Soon**:
- Container logs viewer
- Shell/exec into containers
- Network management
- Docker Compose support
- Resource monitoring (CPU, memory)

---

**Version**: 0.1.0  
**Last Updated**: February 2026
