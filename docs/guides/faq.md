# Frequently Asked Questions (FAQ)

## General Questions

### What is Doctainr?

Doctainr is a Docker desktop application built with Rust and Dioxus. It provides a native, fast, and lightweight interface for managing Docker containers, images, and volumes.

### Why use Doctainr instead of Docker Desktop?

Doctainr is:
- **Lightweight**: Smaller resource footprint
- **Fast**: Native Rust performance
- **Focused**: Simple interface for common tasks
- **Open Source**: Community-driven development

Doctainr complements Docker Desktop rather than replacing it.

### Is Doctainr free?

Yes, Doctainr is open source and free to use.

### What platforms does Doctainr support?

Doctainr runs on:
- Linux (with Docker Engine or Docker Desktop)
- macOS (with Docker Desktop)
- Windows (with Docker Desktop)

## Installation & Setup

### Do I need Docker Desktop?

**Linux**: No, Docker Engine is sufficient.

**macOS/Windows**: Yes, Docker Desktop is recommended as it provides the Docker daemon.

### Can I use Doctainr with remote Docker?

Yes! Set the `DOCKER_HOST` environment variable:

```bash
export DOCKER_HOST=tcp://192.168.1.100:2375
doctainr
```

### How do I update Doctainr?

**From source**:
```bash
cd doctainr
git pull origin main
cargo build --release
```

**From cargo** (when published):
```bash
cargo install doctainr --force
```

### Where is configuration stored?

Currently, Doctainr has minimal configuration:
- Docker host via `DOCKER_HOST` environment variable
- No persistent settings file yet

Future versions will include a settings file.

## Using Doctainr

### How do I create a new container?

Use Docker CLI to create containers:
```bash
docker run -d --name myapp nginx
```

Then use Doctainr to start/stop/monitor it.

### Can I pull Docker images with Doctainr?

Not currently. Use Docker CLI:
```bash
docker pull nginx:latest
```

Then refresh the Images view in Doctainr to see it.

### Can I view container logs?

Not yet. Use Docker CLI:
```bash
docker logs mycontainer

# Or follow logs
docker logs -f mycontainer
```

Container logs viewer is a planned feature.

### Can I execute commands in containers?

Not currently. Use Docker CLI:
```bash
docker exec -it mycontainer bash
```

Interactive terminal access may be added in future.

### Why don't I see all my containers?

Possible reasons:
1. Containers are on different Docker host
2. Permission issues
3. Doctainr not refreshed after CLI changes

**Solutions**:
- Verify `docker ps -a` shows containers
- Click "Refresh" in Doctainr
- Check Docker host configuration

### How do I remove containers/images/volumes?

Use Docker CLI:
```bash
# Remove stopped container
docker rm mycontainer

# Remove image
docker rmi nginx:latest

# Remove volume
docker volume rm myvolume

# Clean up everything unused
docker system prune -a --volumes
```

Then refresh Doctainr to see changes.

## Troubleshooting

### "Failed to connect to Docker"

**Most common causes**:
1. Docker not running
2. Permission issues
3. Wrong Docker host

See detailed solutions in [Troubleshooting Guide](troubleshooting.md#connection-issues).

### Container won't start

**In Doctainr**:
1. Click "Start" button
2. Check for error message

**If it fails**:
1. Try via CLI: `docker start <container>`
2. Check Docker logs: `docker logs <container>`
3. Verify ports aren't in use

### Changes via CLI don't show up

Click "Refresh" button in the relevant view, or navigate away and back.

### Application is slow

**Possible causes**:
- Many containers/images/volumes
- Slow Docker daemon
- System resource constraints

**Solutions**:
- Clean up unused resources
- Optimize Docker daemon
- Check system resources

### Doctainr crashes on startup

**Common causes**:
1. Missing system dependencies (WebKit on Linux)
2. Docker connection issues
3. Corrupted build

**Solutions**:
1. Install dependencies (see [Installation Guide](installation.md))
2. Verify Docker works: `docker info`
3. Rebuild: `cargo clean && cargo build --release`

## Features

### What features are planned?

**Near term**:
- Container logs viewer
- Container stats (CPU, memory)
- Network management
- Container creation UI

**Future**:
- Image building
- Docker Compose support
- Multi-host management
- Advanced filtering and search

### Can I request a feature?

Yes! Please:
1. Check existing [issues](https://github.com/MH0386/doctainr/issues)
2. Open a new issue describing the feature
3. Explain the use case

### How can I contribute?

See [Contributing Guidelines](contributing.md) for details.

## Technical Questions

### What technologies does Doctainr use?

- **Language**: Rust 2021
- **UI Framework**: Dioxus 0.7
- **Docker Client**: Bollard
- **Async Runtime**: Tokio

### Why Rust and Dioxus?

**Rust** provides:
- Memory safety
- High performance
- Excellent tooling

**Dioxus** offers:
- React-like component model
- Cross-platform support
- Native rendering

### Is Doctainr secure?

Doctainr follows security best practices:
- No credential storage
- Uses Docker socket permissions
- Input validation
- Error handling

Always ensure Docker socket has appropriate permissions.

### Can Doctainr damage my containers?

Doctainr only performs:
- Read operations (list)
- Start/stop operations

It cannot:
- Delete containers
- Modify configurations
- Remove images/volumes

All destructive operations require Docker CLI.

### Does Doctainr work offline?

Doctainr requires Docker daemon connection but doesn't need internet access for basic functionality.

### What's the performance overhead?

Minimal. Doctainr:
- Uses async operations
- Caches no data (fetches fresh)
- Has small memory footprint
- Native compiled binary

## Comparison with Other Tools

### Doctainr vs Docker Desktop

**Docker Desktop**:
- Full-featured GUI
- Built-in Docker daemon (macOS/Windows)
- Resource intensive
- Commercial licensing for companies

**Doctainr**:
- Focused on container management
- Lightweight and fast
- Requires separate Docker daemon
- Free and open source

### Doctainr vs Portainer

**Portainer**:
- Web-based
- Multi-host management
- User/team management
- Commercial edition available

**Doctainr**:
- Native desktop app
- Single-host focus
- Simpler, faster interface
- Completely free

### Doctainr vs Lazydocker

**Lazydocker**:
- Terminal UI
- Keyboard-driven
- Rich feature set

**Doctainr**:
- GUI application
- Mouse-driven
- More accessible to beginners

## Development

### How do I build from source?

```bash
git clone https://github.com/MH0386/doctainr.git
cd doctainr
cargo build --release
```

See [Development Guide](development.md) for details.

### How do I run tests?

```bash
cargo test
```

### Can I use Doctainr as a library?

The Docker service layer could be extracted, but Doctainr is primarily designed as an application.

### How is the codebase structured?

See [Architecture Overview](../architecture/overview.md) for details.

## Community

### Where can I get help?

- 📖 Read the [documentation](../README.md)
- 🐛 Check [Troubleshooting Guide](troubleshooting.md)
- 💬 Open a [GitHub issue](https://github.com/MH0386/doctainr/issues)

### How do I report bugs?

1. Check [existing issues](https://github.com/MH0386/doctainr/issues)
2. Gather system information
3. Create detailed bug report

See [Contributing Guidelines](contributing.md) for bug report template.

### Is there a community forum?

Use [GitHub Discussions](https://github.com/MH0386/doctainr/discussions) for:
- Questions
- Ideas
- General discussion

### How often is Doctainr updated?

Updates depend on:
- Bug reports
- Feature requests
- Community contributions
- Maintainer availability

## Licensing

### What license does Doctainr use?

Check the LICENSE file in the repository (to be added).

### Can I use Doctainr commercially?

Check the project license for commercial use terms.

### Can I modify Doctainr?

Yes, as long as you comply with the license terms.

## Future Plans

### Will Doctainr support Docker Swarm?

Docker Swarm support is not currently planned but could be added based on demand.

### Will there be a web version?

Dioxus supports web compilation, so a web version is technically possible but not currently planned.

### Will Doctainr support Podman?

Podman support could be added as Podman implements the Docker API. Community contributions welcome!

### When will version 1.0 be released?

Version 1.0 will be released when:
- Core features are stable
- Major bugs are resolved
- Documentation is complete
- User feedback is incorporated

## Still Have Questions?

- 📖 Check other [documentation](../README.md)
- 🐛 Review [Troubleshooting](troubleshooting.md)
- 💬 Ask on [GitHub](https://github.com/MH0386/doctainr/issues)

---

**Don't see your question?** Open an issue and we'll add it!
