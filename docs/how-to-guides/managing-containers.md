# Managing Containers

This guide shows you how to view, start, stop, and monitor Docker containers using Doctainr.

## Viewing All Containers

1. Click **Containers** in the sidebar navigation
2. The container list displays both running and stopped containers

Each container shows:
- **Name** — Container name (auto-generated if not set)
- **Status** — Current state with visual indicator
- **Image** — Base image used to create the container
- **Ports** — Exposed port mappings (e.g., `8080/tcp -> 0.0.0.0:8080`)
- **ID** — Short container ID

## Starting a Stopped Container

To start a stopped container:

1. Navigate to **Containers**
2. Locate the container with status **Stopped**
3. Click the **Start** button
4. The status updates to **Running**

````
Example:
┌──────────────────────────────────────────────────┐
│ nginx-web          [Stopped]                     │
│ Image: nginx:latest                       [Start]│
│ Ports: 80/tcp -> 0.0.0.0:8080                   │
│ ID: a3f2c8d                                      │
└──────────────────────────────────────────────────┘
````

Click **Start** → Status becomes **Running**

## Stopping a Running Container

To stop a running container:

1. Navigate to **Containers**
2. Locate the container with status **Running**
3. Click the **Stop** button
4. The status updates to **Stopped**

````
Example:
┌──────────────────────────────────────────────────┐
│ nginx-web          [Running]                     │
│ Image: nginx:latest                        [Stop]│
│ Ports: 80/tcp -> 0.0.0.0:8080                   │
│ ID: a3f2c8d                                      │
└──────────────────────────────────────────────────┘
````

Click **Stop** → Status becomes **Stopped**

## Refreshing the Container List

To get the latest container information:

1. Click the **Refresh** button at the top of the Containers view
2. Doctainr queries the Docker API and updates the list

Use this after:
- Creating containers via CLI (`docker run`)
- Removing containers via CLI (`docker rm`)
- External changes to container state

## Understanding Container Status

Doctainr displays two primary states:

### Running
- Container is actively executing
- CPU and memory are allocated
- Network ports are bound (if exposed)
- **Action available:** Stop

### Stopped
- Container exists but is not executing
- No CPU/memory usage
- State is preserved (filesystem, volumes)
- **Action available:** Start

## Port Mappings

Port information shows how container ports map to host ports:

- **`80/tcp`** — Container port 80 (TCP protocol)
- **`0.0.0.0:8080`** — Host IP and port (accessible at `localhost:8080`)
- **Empty** — No published ports (container-to-container networking only)

## Common Tasks

### Start a web server container

````bash
# Create container from CLI
docker run -d --name my-nginx -p 8080:80 nginx:latest
````

Then in Doctainr:
1. Navigate to **Containers**
2. See `my-nginx` with status **Running**
3. Ports show `80/tcp -> 0.0.0.0:8080`
4. Visit `http://localhost:8080` in browser

### Stop a database container

To safely stop a database:
1. Find the container (e.g., `postgres-dev`)
2. Click **Stop**
3. Wait for status to update
4. Database connections are gracefully closed

### Restart a crashed container

If a container stopped unexpectedly:
1. Check container status in Doctainr
2. Click **Start** to restart
3. Monitor logs externally (`docker logs <name>`) to diagnose

## Limitations

Current limitations (planned for future releases):

- **No log viewing** — Use `docker logs` in terminal
- **No container removal** — Use `docker rm` in terminal
- **No container creation** — Use `docker run` in terminal
- **No exec/attach** — Use `docker exec` in terminal
- **No resource usage** — Use `docker stats` for CPU/memory metrics

## Troubleshooting

### Container won't start

If clicking **Start** doesn't work:

1. Click **Refresh** to verify current state
2. Check Docker logs: `docker logs <container-name>`
3. Verify Docker daemon is healthy: `docker info`
4. Look for port conflicts if the container exposes ports

### Container list is empty

If no containers appear:

1. Verify Docker is running: `docker ps -a`
2. Check Doctainr is connected (view Dashboard for engine status)
3. Click **Refresh** to reload
4. Review error messages at the top of the view

### Actions don't respond

If buttons don't work:

1. Check for error messages in the UI
2. Verify Docker socket permissions
3. Restart Doctainr
4. Test Docker CLI: `docker start <container-name>`

## Next Steps

- [Working with Images](./working-with-images.md) — Manage Docker images
- [Configuring Docker Connection](./configuring-docker-connection.md) — Connect to remote Docker
- [Architecture Reference](../reference/architecture.md) — Understand the container service

---

**Pro tip:** Use Doctainr for quick visual management and the Docker CLI for advanced operations like logs, exec, and creating containers.
