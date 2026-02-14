# Getting Started with Doctainr

This tutorial will guide you through installing, running, and exploring Doctainr—a native Docker desktop UI built with Rust and Dioxus.

## Prerequisites

Before starting, ensure you have:

1. **Docker** running on your system
2. **Rust** toolchain (version 1.70 or later)
3. Basic familiarity with Docker concepts (containers, images, volumes)

## Step 1: Verify Prerequisites

First, verify Docker is running:

````bash
docker info
````

You should see Docker engine information. If not, start Docker Desktop or the Docker daemon.

Next, verify Rust is installed:

````bash
rustc --version
cargo --version
````

If Rust is not installed, get it from [rustup.rs](https://rustup.rs/).

## Step 2: Install Dioxus CLI

Doctainr uses the Dioxus CLI (`dx`) for development:

````bash
curl -sSL http://dioxus.dev/install.sh | sh
````

Verify installation:

````bash
dx --version
````

## Step 3: Clone and Build

Clone the repository:

````bash
git clone https://github.com/MH0386/doctainr.git
cd doctainr
````

Build and run in development mode:

````bash
dx serve --platform desktop
````

The application will compile and launch. The first build may take a few minutes as dependencies are downloaded and compiled.

## Step 4: Explore the Interface

### Dashboard View

When Doctainr launches, you'll see the **Dashboard** with:

- **Running containers** — Count of active containers
- **Stopped containers** — Containers ready to restart
- **Images** — Total local images cached
- **Volumes** — Persistent data volumes
- **Engine info** — Docker host and context

Click **Refresh All** to reload all metrics.

### Containers View

Navigate to **Containers** to:

- View all containers (running and stopped)
- See status, ports, and image information
- **Start** stopped containers
- **Stop** running containers
- **Refresh** the container list

### Images View

Browse your local Docker images:

- Repository name and tag
- Image ID (short format)
- Size in MB
- Refresh to reload

### Volumes View

List persistent Docker volumes:

- Volume name
- Driver type
- Mount point
- Refresh to update

## Step 5: Perform Your First Action

Let's start a container:

1. Navigate to **Containers**
2. Find a stopped container in the list
3. Click **Start** next to it
4. The status will update to **Running**

Click **Refresh** to see the updated state.

## Next Steps

Now that you're familiar with the basics:

- Try [Building Your First View](./building-first-view.md) to customize the UI
- Learn more about [Managing Containers](../how-to-guides/managing-containers.md)
- Explore the [Architecture](../reference/architecture.md) to understand the codebase

## Troubleshooting

### Docker connection fails

If you see "Failed to connect to Docker":

- Verify Docker is running: `docker info`
- Check `DOCKER_HOST` environment variable
- See [Configuring Docker Connection](../how-to-guides/configuring-docker-connection.md)

### Build errors

If the build fails:

- Update Rust: `rustup update`
- Clean and rebuild: `cargo clean && dx serve`
- Check [Common Build Issues](../reference/troubleshooting.md#build-errors)

---

**Congratulations!** You've successfully set up and used Doctainr. Continue to the next tutorial or explore the how-to guides.
