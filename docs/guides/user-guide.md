# User Guide

Complete guide to using Doctainr for Docker container management.

## Table of Contents

- [Getting Started](#getting-started)
- [Dashboard](#dashboard)
- [Container Management](#container-management)
- [Image Browser](#image-browser)
- [Volume Manager](#volume-manager)
- [Settings](#settings)
- [Tips and Best Practices](#tips-and-best-practices)

## Getting Started

### Prerequisites

- Docker installed and running
- Doctainr installed (see [Installation Guide](installation.md))
- Basic familiarity with Docker concepts

### Launching Doctainr

```bash
# If installed via cargo
doctainr

# If running from source
cargo run
# or
dx serve --platform desktop
```

The application window opens showing the Dashboard.

## Dashboard

The Dashboard provides an at-a-glance overview of your Docker environment.

### Features

#### Metrics Display

Four main metric cards show:

1. **Running containers**
   - Count of currently running containers
   - Updates automatically when container states change

2. **Stopped containers**
   - Count of stopped containers
   - Helps identify idle resources

3. **Total images**
   - Number of locally stored Docker images
   - Indicates disk space usage

4. **Total volumes**
   - Number of Docker volumes
   - Shows persistent data stores

#### Docker Information

- **Docker host**: Shows connection endpoint
- Helpful for verifying you're connected to the correct Docker instance

#### Actions

- **Refresh All**: Updates all data (containers, images, volumes)
- Use when you've made changes via Docker CLI
- Also refreshes automatically on view changes

### Use Cases

**Quick Health Check**:
```
1. Open Doctainr
2. Check Dashboard
3. Verify expected number of running containers
```

**Before Development**:
```
1. View stopped containers
2. Start needed services
3. Verify all dependencies are running
```

**Resource Audit**:
```
1. Check total images
2. Check total volumes
3. Plan cleanup if numbers are high
```

## Container Management

The Containers view provides full container lifecycle management.

### Container List

Each container shows:
- **Name**: Container name (assigned or auto-generated)
- **ID**: Short container ID (first 12 characters)
- **Status**: Current state with visual indicator
  - 🟢 Green = Running
  - 🔴 Red = Stopped
- **Image**: Docker image used to create container
- **Ports**: Port mappings (e.g., "0.0.0.0:8080→80")

### Container Actions

#### Starting Containers

**To start a stopped container**:
1. Find container in list
2. Locate "Start" button (appears for stopped containers)
3. Click "Start"
4. Status changes to "Running"

**What happens**:
- Container starts with previous configuration
- Ports bind to host system
- Application inside container begins running
- Status updates in UI

**Common use cases**:
- Starting development databases
- Launching web services
- Resuming batch processing

#### Stopping Containers

**To stop a running container**:
1. Find container in list
2. Locate "Stop" button (appears for running containers)
3. Click "Stop"
4. Status changes to "Stopped"

**What happens**:
- Container receives stop signal
- Application has time to shut down gracefully
- Container state is preserved
- Ports are released

**When to stop containers**:
- End of development session
- Before system maintenance
- To free up resources
- Before removing containers

#### Refreshing Container List

**Click "Refresh" to**:
- Update container list
- Sync state with Docker daemon
- Reflect CLI changes

**Auto-refresh triggers**:
- After start/stop operations
- When navigating to Containers view

### Container States

**Running**:
- Container is actively executing
- Consumes CPU and memory
- Network ports are active
- Can be accessed/connected to

**Stopped**:
- Container exists but isn't running
- State and data preserved
- No resource consumption (except disk)
- Can be started again

**Not shown (use Docker CLI)**:
- Paused: Use `docker pause/unpause`
- Dead: Use `docker rm` to remove

### Port Information

Port mappings show how to access container services:

Format: `[host_ip:]host_port→container_port[/protocol]`

Examples:
- `8080→80` - Container port 80 accessible at localhost:8080
- `0.0.0.0:3000→3000` - Container port 3000 accessible on all interfaces
- `5432→5432/tcp` - PostgreSQL database port

**Using mapped ports**:
```bash
# If container shows 8080→80
curl http://localhost:8080

# If container shows 5432→5432
psql -h localhost -p 5432 -U postgres
```

### Working with Container Groups

**Starting Multiple Containers**:
1. Start each container individually
2. Monitor status changes
3. Verify all services are running

**Best Practice**: Use Docker Compose for orchestration, then manage via Doctainr.

## Image Browser

The Images view lists all locally available Docker images.

### Image Information

Each image shows:
- **Repository**: Image name (e.g., "nginx", "postgres")
- **Tag**: Version/variant (e.g., "latest", "15.2")
- **ID**: Short image ID
- **Size**: Disk space used

### Image Actions

#### Viewing Images

Images are listed automatically when you enter the Images view.

#### Refreshing Image List

Click "Refresh" to update the list after:
- Pulling new images via CLI
- Building images locally
- Removing images

### Understanding Image Tags

**Common tags**:
- `latest`: Most recent stable version
- `stable`: Stable release
- `alpine`: Minimal Alpine Linux-based variant
- Version numbers: `15.2`, `3.11`, etc.

**Multiple tags**: Same image can have multiple tags:
```
nginx:latest  (same ID)
nginx:1.25
```

### Image Size

Sizes shown include all layers:
- Larger images take more disk space
- Smaller (Alpine) variants available for many images
- Shared layers reduce actual disk usage

### Image Management

**Operations requiring Docker CLI**:
- Pulling images: `docker pull nginx:latest`
- Building images: `docker build -t myapp .`
- Removing images: `docker rmi <image-id>`
- Tagging images: `docker tag <source> <target>`

**Then use Doctainr to**:
- View available images
- Check image sizes
- Verify tags

## Volume Manager

The Volumes view displays Docker volumes for persistent data.

### Volume Information

Each volume shows:
- **Name**: Volume name (user-defined or auto-generated)
- **Driver**: Storage driver (usually "local")
- **Mountpoint**: Filesystem path where data is stored

### Understanding Volumes

**What are volumes?**:
- Persistent data storage
- Survive container removal
- Can be shared between containers
- Managed by Docker

**Common use cases**:
- Database data
- Application uploads
- Configuration files
- Log files

### Volume Actions

#### Viewing Volumes

Volumes list automatically when entering the Volumes view.

#### Refreshing Volume List

Click "Refresh" to update after:
- Creating volumes via CLI
- Removing unused volumes
- Starting containers with volumes

### Volume Management

**Operations requiring Docker CLI**:
- Create volume: `docker volume create mydata`
- Remove volume: `docker volume rm mydata`
- Inspect volume: `docker volume inspect mydata`
- Cleanup unused: `docker volume prune`

**Use Doctainr to**:
- See all volumes
- Check volume names
- Monitor volume count
- Verify volume existence

### Data Persistence

**Volumes persist**:
- After container stops
- After container removal
- Until explicitly deleted

**Important**: Always back up important volume data before removing volumes.

## Settings

The Settings view shows Docker configuration.

### Current Configuration

Displays:
- **Docker Host**: Connection endpoint
- Current connection status

### Configuration Sources

Docker host is configured via:
1. `DOCKER_HOST` environment variable
2. Platform defaults (Unix socket, named pipe)

**To change Docker host**:
```bash
# Unix socket (Linux/macOS)
export DOCKER_HOST=unix:///var/run/docker.sock

# TCP connection
export DOCKER_HOST=tcp://192.168.1.100:2375

# Then restart Doctainr
doctainr
```

### Future Settings

Settings view will be expanded to include:
- Application preferences
- UI customization
- Update settings
- Advanced options

## Tips and Best Practices

### Workflow Tips

**Daily Development**:
1. Start Doctainr at beginning of day
2. Start needed containers
3. Work on projects
4. Stop containers when done
5. Check Dashboard before closing

**Resource Management**:
1. Regularly check image count
2. Remove unused images: `docker image prune`
3. Clean stopped containers: `docker container prune`
4. Manage volumes: `docker volume prune`

**Monitoring**:
1. Keep Doctainr open during development
2. Refresh after CLI operations
3. Check container status before troubleshooting
4. Verify ports before accessing services

### Performance Tips

**Faster Startup**:
- Keep number of containers reasonable
- Clean up unused resources
- Ensure Docker daemon is healthy

**Smooth Operation**:
- Refresh only when needed
- Close application when not in use
- Monitor system resources

### Integration with Docker CLI

Doctainr complements Docker CLI, not replaces it:

**Use Doctainr for**:
- Viewing container status
- Starting/stopping containers
- Quick overview of resources
- Monitoring during development

**Use Docker CLI for**:
- Building images
- Creating containers
- Complex networking
- Advanced configuration
- Scripting and automation

**Best practice**: Make changes via CLI, verify via Doctainr.

### Error Handling

**If operations fail**:
1. Check error message in UI
2. Verify Docker is running: `docker info`
3. Try operation via CLI: `docker start <container>`
4. Refresh Doctainr
5. Check [Troubleshooting Guide](troubleshooting.md)

### Security Considerations

**Docker Socket Access**:
- Requires appropriate permissions
- Don't expose Docker socket unnecessarily
- Use Docker group on Linux

**Container Security**:
- Review container configurations
- Use trusted images
- Keep images updated
- Monitor running containers

## Keyboard Shortcuts

Currently, Doctainr uses mouse-only interaction.

**Navigation**:
- Click sidebar items to switch views
- Click buttons to perform actions

**Future enhancement**: Keyboard shortcuts may be added.

## Accessibility

**Current features**:
- Clear visual indicators for container states
- Descriptive button labels
- Readable font sizes

**Future improvements**:
- Screen reader support
- Keyboard navigation
- High contrast themes

## Getting Help

**Need assistance?**:
- 📖 Check [FAQ](faq.md)
- 🐛 Review [Troubleshooting](troubleshooting.md)
- 💬 Open [GitHub issue](https://github.com/MH0386/doctainr/issues)

## Related Documentation

- [Quick Start Tutorial](quick-start.md) - Get started quickly
- [Installation Guide](installation.md) - Installation instructions
- [Troubleshooting](troubleshooting.md) - Solve common issues
- [FAQ](faq.md) - Frequently asked questions

---

**Happy Docker managing with Doctainr!** 🐳
