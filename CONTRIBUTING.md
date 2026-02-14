# Contributing to Doctainr

Thank you for your interest in contributing to Doctainr! This guide will help you get started.

## Code of Conduct

Be respectful, inclusive, and constructive. We welcome contributions from everyone.

## Getting Started

### 1. Fork and Clone

````bash
# Fork on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/doctainr.git
cd doctainr
````

### 2. Set Up Development Environment

Ensure you have:
- **Rust 1.70+**: `rustup update`
- **Dioxus CLI**: `curl -sSL http://dioxus.dev/install.sh | sh`
- **Docker**: Running and accessible

````bash
# Verify setup
rustc --version
dx --version
docker info
````

### 3. Create a Branch

````bash
git checkout -b feature/my-awesome-feature
````

## Development Workflow

### Running the App

````bash
dx serve --platform desktop
````

The app will launch with hot reload enabled.

### Code Style

Run the formatter before committing:

````bash
cargo fmt
````

### Linting

Check for common issues:

````bash
cargo clippy
````

Fix all warnings before submitting a PR.

### Testing

Run tests (when available):

````bash
cargo test
````

## Making Changes

### Project Structure

````
doctainr/
├── src/
│   ├── main.rs           # Entry point, routing
│   ├── components/       # Reusable UI components
│   ├── views/            # Page-level components
│   ├── services/         # Docker API integration
│   └── utils/            # State management, helpers
├── assets/               # Images, CSS
├── docs/                 # Documentation (you are here!)
└── Cargo.toml            # Dependencies
````

### Adding a New View

1. Create component in `src/views/my_view.rs`:

````rust
use dioxus::prelude::*;

#[component]
pub fn MyView() -> Element {
    rsx! {
        div { "My new view!" }
    }
}
````

2. Export in `src/views/mod.rs`:

````rust
mod my_view;
pub use my_view::MyView;
````

3. Add route in `src/main.rs`:

````rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(AppShell)]
        // ... existing routes
        #[route("/myview")]
        MyView {},
}
````

### Adding a Component

1. Create in `src/components/my_component.rs`:

````rust
use dioxus::prelude::*;

#[component]
pub fn MyComponent(
    title: String,
    content: String,
) -> Element {
    rsx! {
        div { class: "my-component",
            h3 { "{title}" }
            p { "{content}" }
        }
    }
}
````

2. Export in `src/components/mod.rs`:

````rust
mod my_component;
pub use my_component::MyComponent;
````

### Adding a Service Method

Extend `DockerService` in `src/services/docker.rs`:

````rust
impl DockerService {
    pub async fn my_new_method(&self) -> Result<Vec<MyData>> {
        // Use self.docker (Bollard client)
        let data = self.docker.list_something(None).await?;
        // Transform and return
        Ok(data)
    }
}
````

## Contribution Guidelines

### Code Quality

- **Type Safety**: Use Rust's type system; avoid `unwrap()` in production code
- **Error Handling**: Propagate errors with `?` or handle gracefully
- **Documentation**: Add doc comments (`///`) for public APIs
- **Comments**: Explain *why*, not *what* (code is self-documenting)

### Commit Messages

Follow conventional commits:

````
feat: Add container logs view
fix: Resolve port parsing error
docs: Update architecture diagram
refactor: Simplify state management
test: Add unit tests for DockerService
````

### Pull Request Process

1. **Update Documentation**: If you add features, update relevant docs
2. **Run Lints**: `cargo fmt && cargo clippy`
3. **Test Locally**: Verify the app works as expected
4. **Write Clear Description**: Explain what and why
5. **Link Issues**: Reference related issues (e.g., "Closes #42")

### PR Template

````markdown
## Description
Brief description of changes.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Refactoring

## Testing
How did you test this?

## Screenshots
(If UI changes)

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-reviewed the code
- [ ] Documentation updated
- [ ] No new warnings
````

## Areas for Contribution

### High Priority

- **Container Logs View**: Implement log streaming and viewing
- **Image Management**: Pull, remove, inspect images
- **Volume Management**: Create, remove, inspect volumes
- **Network View**: List and manage Docker networks
- **Tests**: Unit and integration tests for services
- **Remote Docker**: Enhanced TCP/TLS support

### Medium Priority

- **Auto-Refresh**: Optional polling for real-time updates
- **Search/Filter**: Search containers, images by name/tag
- **Dark Mode**: Theme switching
- **Settings Page**: User preferences UI
- **Docker Compose**: List and manage compose projects

### Low Priority

- **Metrics/Stats**: CPU, memory usage graphs
- **Notifications**: System tray notifications
- **Keyboard Shortcuts**: Vim-like keybindings
- **Export Data**: Export container list to CSV/JSON

## Documentation Contributions

### Style Guide

Follow these conventions:

- **Diátaxis Framework**: Organize by user needs (tutorials, how-tos, reference, explanation)
- **Active Voice**: "Click the button" not "The button should be clicked"
- **Plain English**: Avoid jargon; explain acronyms
- **Code Examples**: Use 4 backticks for markdown with code blocks
- **Progressive Disclosure**: High-level first, details second

### Adding Documentation

1. Identify the type:
   - **Tutorial**: Learning-oriented, step-by-step
   - **How-to Guide**: Problem-oriented, specific tasks
   - **Reference**: Information-oriented, technical specs
   - **Explanation**: Understanding-oriented, concepts

2. Create in appropriate directory:
   - `docs/tutorials/`
   - `docs/how-to-guides/`
   - `docs/reference/`
   - `docs/explanation/`

3. Update `docs/README.md` with a link

4. Use clear headings and cross-references

### Documentation PR

Documentation-only PRs are welcome and encouraged!

## Getting Help

- **GitHub Issues**: Ask questions, report bugs
- **Discussions**: General questions, ideas
- **Code Review**: Request feedback on draft PRs

## Recognition

Contributors will be acknowledged in:
- `CONTRIBUTORS.md` file (coming soon)
- Release notes for significant contributions

---

Thank you for contributing to Doctainr! 🦀🐳
