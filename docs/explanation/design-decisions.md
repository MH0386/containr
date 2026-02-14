# Design Decisions

This document explains key architectural and implementation choices in Doctainr, along with the rationale and trade-offs.

## Why Dioxus 0.7?

### Decision
Build the UI with Dioxus 0.7, a modern Rust UI framework.

### Rationale

1. **Type Safety** — Rust's compiler catches UI bugs at compile time
2. **Performance** — Native compilation, no runtime overhead
3. **Single Language** — Rust for both backend (Docker API) and frontend
4. **Cross-Platform** — Same codebase for desktop, web, and mobile
5. **Modern Patterns** — React-like component model with hooks

### Alternatives Considered

- **Electron/Tauri + Web** — Rejected due to JavaScript dependency and bundle size
- **egui** — Rejected due to immediate-mode complexity for complex UIs
- **GTK/Qt** — Rejected due to C++ FFI complexity and platform dependencies

### Trade-offs

**Pros:**
- Fast, native performance
- Small binary size (~5-10 MB release)
- Strong type safety
- Active ecosystem

**Cons:**
- Smaller community than React/Vue
- Fewer third-party component libraries
- Dioxus 0.7 is relatively new (breaking changes from 0.6)

---

## Why Bollard for Docker API?

### Decision
Use the Bollard crate as the Docker API client.

### Rationale

1. **Pure Rust** — No external dependencies or C bindings
2. **Async-First** — Built on Tokio for non-blocking I/O
3. **Complete API** — Covers all Docker Engine APIs
4. **Type-Safe** — Strong typing for requests and responses
5. **Well-Maintained** — Active development and updates

### Alternatives Considered

- **Docker CLI Wrapper** — Rejected due to parsing overhead and fragility
- **shiplift** — Rejected due to less active maintenance
- **Direct REST API** — Rejected due to boilerplate and error-proneness

### Trade-offs

**Pros:**
- Native performance
- Type-safe API
- Async integration

**Cons:**
- Some API gaps (newer Docker features may lag)
- Limited documentation vs Docker CLI

---

## Why No Container Logs?

### Decision
Omit log viewing in the initial release.

### Rationale

1. **Scope Management** — Focus on core container management first
2. **Complexity** — Log streaming requires WebSocket-like connection and infinite scroll
3. **CLI Sufficiency** — `docker logs` is well-established and feature-rich
4. **Performance** — Large logs could freeze the UI without proper virtualization

### Future Plans

Planned for a future release with:
- Virtualized scrolling for large logs
- Search/filter capabilities
- Follow mode (tail -f)
- Color-coded log levels

---

## Why Signal-Based State?

### Decision
Use Dioxus 0.7's `Signal<T>` for all reactive state.

### Rationale

1. **Fine-Grained Reactivity** — Only affected components re-render
2. **Automatic Tracking** — No manual subscriptions or observers
3. **Thread-Safe** — Signals work across async boundaries
4. **Ergonomic** — Copy semantics, no `Rc`/`Arc` noise

### Alternatives Considered

- **Global Mutex** — Rejected due to lock contention and non-reactive
- **Message Passing (Channels)** — Rejected due to boilerplate and manual updates
- **Redux-Style Store** — Rejected due to complexity and verbosity

### Trade-offs

**Pros:**
- Minimal boilerplate
- Efficient re-rendering
- Natural async integration

**Cons:**
- Less explicit than message passing
- Debugging state changes can be harder
- Learning curve for developers new to signals

---

## Why No Daemon/Background Service?

### Decision
Run as a desktop application, not a background daemon.

### Rationale

1. **Simplicity** — User launches when needed, no persistent process
2. **Resource Efficiency** — No CPU/memory usage when closed
3. **User Control** — Explicit start/stop by user
4. **Security** — No always-on attack surface

### Alternatives Considered

- **System Tray App** — Rejected due to platform differences and complexity
- **Web Server** — Rejected due to authentication/security concerns

### Future Plans

May add optional tray icon with:
- Quick container start/stop
- Notification on container state changes
- System startup option

---

## Why Local-Only (No Remote Docker)?

### Decision
Initial release focuses on local Docker socket connections.

### Rationale

1. **Security** — Local socket has implicit trust (no TLS/auth needed)
2. **Simplicity** — No certificate management or credential storage
3. **Use Case** — Majority of users manage local Docker
4. **Permissions** — Unix socket permissions handle access control

### Future Plans

Enhanced remote Docker support with:
- TCP connection with TLS
- SSH tunneling
- Docker context switching
- Certificate management UI

---

## Why No Container Creation UI?

### Decision
Omit container creation in the initial release.

### Rationale

1. **Complexity** — Full creation UI requires:
   - Image selection
   - Port mapping builder
   - Volume mount UI
   - Environment variable editor
   - Network configuration
2. **CLI Sufficiency** — `docker run` and docker-compose are well-established
3. **Scope** — Focus on monitoring/management, not orchestration

### Future Plans

Planned for future releases:
- Quick launch from images (simple `docker run`)
- Form-based creation wizard
- docker-compose integration

---

## Why Desktop-First (Not Web)?

### Decision
Build as a desktop app first, with web as a future target.

### Rationale

1. **Docker Socket** — Local socket access requires native capabilities
2. **Performance** — Desktop avoids browser overhead and limitations
3. **User Expectations** — Docker Desktop is the norm, not web-based
4. **Distribution** — Single binary, no server deployment needed

### Future Plans

Potential web builds:
- Read-only dashboard for remote monitoring
- API server mode with authentication
- Browser-based management for headless servers

---

## Why No Database/Persistence?

### Decision
No local storage; fetch fresh data on each launch.

### Rationale

1. **Simplicity** — No database dependency or migration logic
2. **Accuracy** — Always reflect current Docker state (no stale data)
3. **Docker as Source** — Docker daemon is the source of truth
4. **Performance** — Docker API is fast enough for fresh queries

### Future Plans

Optional persistence for:
- User preferences (theme, layout)
- Custom container labels/notes
- Favorite containers
- Historical metrics

---

## Why No Real-Time Updates?

### Decision
Manual refresh via buttons, not automatic polling or event streaming.

### Rationale

1. **Battery Efficiency** — No background CPU usage
2. **Simplicity** — No WebSocket connection management
3. **User Control** — User decides when to refresh
4. **Docker API Stability** — Avoid overwhelming Docker daemon with requests

### Future Plans

Planned enhancements:
- Optional auto-refresh with configurable interval
- Docker event stream integration (real-time container state changes)
- Background polling with smart throttling

---

## Why Tailwind CSS + Custom Styles?

### Decision
Use a hybrid approach: Tailwind utility classes + custom CSS.

### Rationale

1. **Rapid Prototyping** — Tailwind enables fast UI iteration
2. **Custom Branding** — Custom CSS for unique look/feel
3. **Learning Curve** — Team familiar with Tailwind patterns
4. **File Size** — Minimal CSS footprint with purging

### Alternatives Considered

- **Pure Tailwind** — Rejected due to lack of component-level encapsulation
- **Styled Components** — Rejected due to Rust ecosystem immaturity
- **Pure CSS** — Rejected due to maintenance burden for large stylesheets

### Trade-offs

**Pros:**
- Fast development
- Consistent design tokens
- Small bundle size

**Cons:**
- Tailwind adds build step
- Mixing approaches can be inconsistent
- Less semantic class names

---

## Why No Tests (Yet)?

### Decision
Ship initial version without comprehensive tests.

### Rationale

1. **MVP Focus** — Prioritize core functionality
2. **Evolving API** — Dioxus 0.7 patterns still stabilizing
3. **Manual Testing** — Small codebase allows thorough manual testing
4. **Resource Allocation** — Small team, limited bandwidth

### Future Plans

Testing strategy:
- Unit tests for service layer (Docker API mocking)
- Integration tests with Docker testcontainers
- UI snapshot tests with Dioxus testing tools
- End-to-end tests with real Docker daemon

---

## Summary of Principles

Doctainr's design follows these guiding principles:

1. **Simplicity First** — Start with core use cases, expand incrementally
2. **Rust All the Way** — Leverage Rust's strengths (safety, performance, ecosystem)
3. **User Control** — Explicit actions, no hidden background behaviors
4. **Docker-Native** — Align with Docker's conventions and terminology
5. **Future-Proof** — Architecture supports planned enhancements

---

See also:
- [Architecture](../reference/architecture.md) — System implementation
- [State Management](./state-management.md) — Reactive patterns
- [Docker API Integration](./docker-api.md) — Bollard usage
