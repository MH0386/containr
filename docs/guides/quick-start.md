# Quick Start Tutorial

Welcome to Doctainr! This tutorial will get you up and running in just a few minutes.

## Step 1: Prerequisites

Ensure you have:
- ✅ Docker installed and running
- ✅ Rust toolchain (1.70+)
- ✅ Doctainr installed (see [Installation Guide](installation.md))

Verify Docker is running:
```bash
docker info
```

## Step 2: Start Doctainr

### From Source
```bash
cd doctainr
dx serve --platform desktop
# Or
cargo run
```

### From Binary
```bash
doctainr
```

The application window should open automatically.

## Step 3: Explore the Dashboard

The **Dashboard** is your home screen, showing:

- 📊 **Running containers** - Number of active containers
- ⏹️ **Stopped containers** - Number of inactive containers
- 💿 **Total images** - Images available locally
- 📦 **Total volumes** - Docker volumes on your system
- 🐳 **Docker host** - Connection endpoint

**Try it**: Click the **"Refresh All"** button to reload all data.

## Step 4: Manage Containers

Navigate to **Containers** view (click "Containers" in the sidebar).

### View All Containers

You'll see a list of all your containers with:
- Container name and ID
- Current status (Running/Stopped)
- Image used
- Port mappings

### Start a Container

1. Find a stopped container in the list
2. Click the **"Start"** button
3. Watch the status change to "Running"

### Stop a Container

1. Find a running container
2. Click the **"Stop"** button
3. The container will be stopped

**Try it**: Start and stop a container to see the real-time updates.

## Step 5: Browse Images

Navigate to **Images** view.

### View Your Images

See all locally available Docker images:
- Repository name
- Tag (version)
- Image ID
- Size

**Example**: You might see images like `nginx:latest`, `postgres:15`, etc.

**Try it**: Click **"Refresh"** to reload the image list.

## Step 6: Explore Volumes

Navigate to **Volumes** view.

### View Your Volumes

See all Docker volumes:
- Volume name
- Driver (usually "local")
- Mount point (filesystem path)

Volumes persist data even when containers are removed.

## Step 7: Configure Settings

Navigate to **Settings** view.

### View Docker Configuration

See your Docker connection details:
- Docker host endpoint
- Connection status

**Note**: Currently, settings are read-only. Configuration is done via environment variables.

## Common Tasks

### Task 1: Check Container Status

Want to know which containers are running?

1. Go to **Dashboard**
2. Look at "Running containers" metric
3. Or go to **Containers** and scan the status column

### Task 2: Quick Refresh

Data not updating?

1. Use the **"Refresh"** button in any view
2. Or use **"Refresh All"** on the Dashboard
3. Data is fetched from Docker in real-time

### Task 3: Start All Your Services

Have a set of containers that work together?

1. Go to **Containers**
2. Start each container by clicking **"Start"**
3. Watch them all come online

### Task 4: Clean Up Resources

Check what's using disk space:

1. Go to **Images** to see image sizes
2. Go to **Volumes** to see what's persisted
3. Use Docker CLI for cleanup: `docker system prune`

## Example Workflow

Let's work through a complete example:

### Running a Web Application

1. **Check if you have the image**:
   - Go to **Images**
   - Look for `nginx` or any web server
   - If not, pull one: `docker pull nginx`

2. **Start a container**:
   - Run in terminal: `docker run -d -p 8080:80 nginx`
   - Go to **Containers** in Doctainr
   - Click **"Refresh"** to see the new container

3. **Verify it's running**:
   - Check the status shows "Running"
   - Note the port mapping (8080:80)
   - Open http://localhost:8080 in your browser

4. **Stop when done**:
   - Click **"Stop"** button
   - Verify status changes to "Stopped"

5. **Clean up**:
   - Use Docker CLI: `docker rm <container-name>`
   - Click **"Refresh"** in Doctainr

## Keyboard Navigation

- **Click** sidebar items to navigate between views
- **Click** action buttons to perform operations
- **Close** window to exit Doctainr

## Tips for Success

### 🎯 Tip 1: Keep Docker Running
Always ensure Docker Desktop (or Docker daemon) is running before starting Doctainr.

### 🔄 Tip 2: Refresh Regularly
If you make changes via Docker CLI, click refresh in Doctainr to see updates.

### 📊 Tip 3: Use Dashboard for Overview
Start on the Dashboard to get a quick overview of your Docker environment.

### ⚡ Tip 4: Check for Errors
If operations fail, look for error messages at the top of the screen.

### 🛠️ Tip 5: Complement with CLI
Doctainr shows and controls containers, but for advanced operations (building images, creating volumes), use Docker CLI.

## Next Steps

Now that you know the basics:

- 📖 Read the [User Guide](user-guide.md) for detailed feature documentation
- 🏗️ Learn about [Architecture](../architecture/overview.md) if you want to contribute
- 🐛 Check [Troubleshooting](troubleshooting.md) if you encounter issues
- ❓ Review [FAQ](faq.md) for common questions

## Quick Reference

| View | Purpose | Key Actions |
|------|---------|-------------|
| Dashboard | Overview of all resources | Refresh All |
| Containers | Manage containers | Start, Stop, Refresh |
| Images | Browse images | Refresh |
| Volumes | View volumes | Refresh |
| Settings | View configuration | (Read-only) |

## Getting Help

Need assistance?
- 📝 Check the [FAQ](faq.md)
- 🐛 Review [Troubleshooting](troubleshooting.md)
- 💬 Open an issue on [GitHub](https://github.com/MH0386/doctainr/issues)

---

**Congratulations!** 🎉 You now know how to use Doctainr. Happy Docker managing!
