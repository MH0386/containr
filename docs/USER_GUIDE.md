# User Guide

Welcome to Doctainr! This guide will help you get started and make the most of the application.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Getting Started](#getting-started)
- [Interface Overview](#interface-overview)
- [Features](#features)
- [Workflows](#workflows)
- [Tips and Tricks](#tips-and-tricks)
- [Keyboard Shortcuts](#keyboard-shortcuts)
- [FAQ](#faq)

## Introduction

Doctainr is a desktop application for managing Docker containers, images, and volumes. It provides a user-friendly interface for common Docker operations without needing to use the command line.

### What You Can Do

- View all containers, images, and volumes
- Start and stop containers
- Monitor resource usage
- Browse image details
- Inspect volumes

### What You Need

- Docker installed and running
- Doctainr application installed
- Basic understanding of Docker concepts

## Installation

### Prerequisites

1. **Docker**: Ensure Docker is installed and running

   ```bash
   docker --version
   docker info
   ```

2. **System Requirements**:
   - Linux: GTK 3+ and WebKit2GTK
   - macOS: macOS 10.13+
   - Windows: Windows 10+

### Installing Doctainr

#### From Release

1. Download the latest release for your platform from [GitHub Releases](https://github.com/MH0386/doctainr/releases)
2. Extract the archive
3. Run the executable

#### From Source

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli

# Clone and build
git clone https://github.com/MH0386/doctainr.git
cd doctainr
dx run
```

### First Launch

1. Launch Doctainr
2. The application will attempt to connect to Docker
3. If successful, you'll see the Dashboard

### Docker Connection

Doctainr connects to Docker via:

- Unix socket: `/var/run/docker.sock` (Linux/macOS)
- Named pipe: `//./pipe/docker_engine` (Windows)
- TCP: If `DOCKER_HOST` environment variable is set

## Getting Started

### Quick Start

1. **Launch Doctainr**: Open the application
2. **View Dashboard**: See overview of your Docker resources
3. **Explore Containers**: Click "Containers" in the sidebar
4. **Manage Containers**: Start or stop containers as needed

### First Steps

#### Step 1: Check the Dashboard

The Dashboard shows:

- Total containers (running and stopped)
- Total images
- Total volumes

Click "Refresh All" to update the data.

#### Step 2: View Containers

1. Click "Containers" in the sidebar
2. See all containers with their status
3. Container information includes:
   - Name
   - Image
   - Ports
   - State (Running/Stopped)

#### Step 3: Manage a Container

To start a container:

1. Find a stopped container
2. Click the "Start" button
3. Wait for the operation to complete
4. Status updates automatically

To stop a container:

1. Find a running container
2. Click the "Stop" button
3. Wait for the operation to complete

## Interface Overview

### Main Window

```
┌─────────────────────────────────────────────────────┐
│  Sidebar          │   Main Content Area             │
│  ┌──────────┐    │   ┌───────────────────────┐    │
│  │Dashboard │    │   │                       │    │
│  │Containers│◄───┼───┤  Active View          │    │
│  │Images    │    │   │  (Dashboard, etc.)    │    │
│  │Volumes   │    │   │                       │    │
│  │Settings  │    │   └───────────────────────┘    │
│  └──────────┘    │                                 │
└─────────────────────────────────────────────────────┘
```

### Sidebar Navigation

- **Dashboard**: Overview of all resources
- **Containers**: Manage containers
- **Images**: Browse images
- **Volumes**: View volumes
- **Settings**: Application settings (planned)

### Header

Shows:

- Application name: "Doctainr Desktop"
- Environment: "Local engine workspace"
- Last action: Most recent operation performed

### Main Content

Displays the currently selected view with:

- Section header (title and description)
- Action buttons (Refresh, etc.)
- Data tables or metric cards

## Features

### Dashboard

**Purpose**: Quick overview of your Docker environment

**What You See**:

- Running containers count
- Stopped containers count
- Total images count
- Total volumes count

**Actions**:

- **Refresh All**: Update all data from Docker

**Use Case**: Check the state of your Docker environment at a glance

### Containers View

**Purpose**: List and manage Docker containers

**Table Columns**:

- **Name**: Container name
- **Image**: Image used by the container
- **Ports**: Port mappings (e.g., 8080:80)
- **State**: Running or Stopped
- **Action**: Start or Stop button

**Actions**:

- **Refresh**: Update container list
- **Start**: Start a stopped container
- **Stop**: Stop a running container

**Information Displayed**:

- Container status text (e.g., "Up 2 hours")
- Short container ID (12 characters)

**Example Workflow**:

1. Find your web app container
2. Check if it's running
3. If stopped, click "Start"
4. Verify it's running by checking the state

### Images View

**Purpose**: Browse Docker images on your system

**Table Columns**:

- **Repository**: Image repository name
- **Tag**: Image tag (version)
- **ID**: Image ID
- **Size**: Image size (formatted)

**Actions**:

- **Refresh**: Update image list

**Information Displayed**:

- Full image ID
- Size in MB/GB
- Repository and tag (e.g., "nginx:latest")

**Example Workflow**:

1. View all available images
2. Check image sizes
3. Identify unused images

### Volumes View

**Purpose**: View Docker volumes

**Table Columns**:

- **Name**: Volume name
- **Driver**: Volume driver (usually "local")
- **Mountpoint**: Mount point path

**Actions**:

- **Refresh**: Update volume list

**Information Displayed**:

- Volume names
- Mount point locations
- Driver type

**Example Workflow**:

1. List all volumes
2. Identify persistent data locations
3. Check volume drivers

## Workflows

### Starting a Development Environment

**Scenario**: You have multiple containers for a project and want to start them all.

**Steps**:

1. Go to Containers view
2. Find containers for your project
3. Start each container in order:
   - Database first
   - Backend service
   - Frontend application
4. Verify all are running
5. Check Dashboard for confirmation

### Checking Resource Usage

**Scenario**: You want to see what Docker resources exist on your system.

**Steps**:

1. View Dashboard for overview
2. Check container count
3. View Images to see disk usage
4. Review Volumes for persistent data

### Stopping Containers After Work

**Scenario**: End of the day, want to stop development containers.

**Steps**:

1. Go to Containers view
2. Find running development containers
3. Stop each container
4. Verify they're stopped
5. Check Dashboard shows fewer running containers

### Investigating Issues

**Scenario**: A container isn't working properly.

**Steps**:

1. Go to Containers view
2. Find the problematic container
3. Check its status text
4. Note the container ID
5. Stop and restart the container
6. If still problematic, use Docker CLI for logs:
   ```bash
   docker logs <container-id>
   ```

## Tips and Tricks

### Refresh Data

- Use "Refresh" buttons frequently to see latest state
- "Refresh All" on Dashboard updates everything at once
- Data doesn't auto-refresh (manual refresh required)

### Container Naming

- Use descriptive container names for easy identification
- Container names in Doctainr match Docker CLI names

### Port Mappings

- Port format: `host:container` (e.g., "8080:80")
- Multiple ports shown as comma-separated
- "--" indicates no published ports

### Quick Navigation

- Use sidebar to switch between views
- Each view is focused on one type of resource
- Dashboard provides cross-view overview

### Understanding States

**Running**:

- Container is actively running
- Can be stopped
- Shows green status pill

**Stopped**:

- Container is not running
- Can be started
- Shows gray status pill

### Performance

- Doctainr is fast and lightweight
- Operations typically complete in under a second
- UI remains responsive during Docker operations

### Error Handling

If an error occurs:

- Error message displays in the view
- Operation may fail (container doesn't start/stop)
- Check Docker daemon is running
- Verify Docker socket permissions

## Keyboard Shortcuts

Currently, Doctainr uses mouse/pointer interaction. Keyboard shortcuts are planned for a future release.

## FAQ

### General Questions

**Q: Does Doctainr modify my Docker containers?**
A: Yes, but only when you explicitly click Start or Stop buttons. Doctainr never modifies containers automatically.

**Q: Can I use Doctainr with remote Docker hosts?**
A: Currently, Doctainr only supports local Docker daemon connections via Unix socket or named pipe. Remote Docker support is planned.

**Q: Is my data safe?**
A: Doctainr only reads data and performs start/stop operations. It doesn't remove containers, images, or volumes. Your data remains safe.

**Q: Why isn't my Docker data showing?**
A: Ensure Docker daemon is running and you have permission to access the Docker socket. On Linux, you may need to add your user to the `docker` group.

### Feature Questions

**Q: Can I remove containers or images?**
A: Not yet. This feature is planned for a future release. Use Docker CLI for now.

**Q: Can I view container logs?**
A: Not yet. This feature is planned. Use `docker logs <container>` for now.

**Q: Can I pull or push images?**
A: Not yet. This feature is planned. Use Docker CLI for now.

**Q: Does it support Docker Compose?**
A: Not yet. Docker Compose integration is planned for a future release.

**Q: Can I create new containers?**
A: Not yet. Use Docker CLI or Docker Desktop to create containers.

### Technical Questions

**Q: What Docker API version is supported?**
A: Doctainr uses the Bollard library which supports Docker API v1.40+. Most modern Docker installations are compatible.

**Q: Does it work with Podman?**
A: Not tested. Podman has Docker-compatible APIs, so it might work with the docker socket compatibility layer.

**Q: What operating systems are supported?**
A: Linux, macOS, and Windows are all supported.

**Q: How much memory does Doctainr use?**
A: Very little. As a native Rust application, Doctainr is highly efficient and typically uses less than 50MB of RAM.

### Troubleshooting Questions

**Q: The application won't start. What should I do?**
A: Ensure Docker is installed and running. Check that you have permission to access the Docker socket. See the Troubleshooting guide for more details.

**Q: Container operations fail. Why?**
A: Verify Docker daemon is running and responsive. Try the same operation with Docker CLI to see if it's a Docker issue.

**Q: Data isn't updating. What's wrong?**
A: Click the "Refresh" button to manually update data. Doctainr doesn't auto-refresh to reduce Docker API calls.

**Q: Can I run multiple instances of Doctainr?**
A: Yes, but there's no benefit. Multiple instances will show the same data from Docker.

## Getting Help

### Support Resources

- **Documentation**: Check docs/ directory
- **GitHub Issues**: Report bugs or request features
- **Architecture Guide**: Understand how it works

### Reporting Issues

When reporting issues, include:

- Doctainr version
- Operating system
- Docker version
- Error messages
- Steps to reproduce

### Contributing

Interested in contributing? See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## What's Next?

Explore advanced topics:

- [Development Guide](DEVELOPMENT.md) - Build from source
- [API Documentation](API.md) - Technical details
- [Deployment Guide](DEPLOYMENT.md) - Distribution
- [Troubleshooting](TROUBLESHOOTING.md) - Common issues

---

Thank you for using Doctainr! We hope it makes your Docker workflow more efficient.
