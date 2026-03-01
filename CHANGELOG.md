# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial release of Doctainr
- Dashboard view for Docker overview
- Container management (start/stop)
- Image browser
- Volume manager
- Real-time data refresh capabilities
- Desktop application with native Rust performance

### Features

- 📊 Dashboard with container, image, and volume statistics
- 🐳 Container lifecycle management
- 💿 Local Docker image browsing
- 📦 Docker volume inspection
- 🔄 Real-time data updates
- ⚡ Fast and lightweight UI built with Dioxus

## [0.1.0] - 2024

### Added

- Initial project setup
- Core Docker integration using Bollard
- Dioxus-based desktop UI
- Routing system with multiple views
- State management with signals
- Basic container operations
- Image listing functionality
- Volume browsing

### Technical

- Rust 2021 edition
- Dioxus 0.7.1 framework
- Bollard 0.18 for Docker API
- Tokio async runtime
- Desktop-first architecture

---

## Release Notes

### Version 0.1.0

This is the initial release of Doctainr, a Docker desktop management application built with Rust and Dioxus.

**Key Features:**

- Native desktop application for Docker management
- Real-time monitoring of containers, images, and volumes
- Intuitive UI with multiple specialized views
- Fast performance with native Rust compilation

**Requirements:**

- Docker daemon running locally
- Rust toolchain 1.70+
- Linux/macOS/Windows support

**Known Limitations:**

- Limited to local Docker daemon connections
- Basic container operations (start/stop only)
- No image management operations yet
- No network or compose file support

**Future Plans:**

- Advanced container operations (logs, exec, inspect)
- Image management (pull, push, remove)
- Docker Compose integration
- Network management
- Container logs viewer
- Resource usage monitoring
