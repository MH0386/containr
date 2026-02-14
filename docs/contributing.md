# Contributing to Doctainr

Thank you for considering contributing to Doctainr! This guide will help you get started with contributing to the project.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and professional in all interactions.

## Ways to Contribute

### Reporting Bugs

Before creating a bug report:
1. Check existing issues to avoid duplicates
2. Verify you're using the latest version
3. Ensure Docker is running and accessible

**Bug Report Template:**

````markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce:
1. Go to '...'
2. Click on '...'
3. See error

**Expected behavior**
What you expected to happen.

**Environment:**
- OS: [e.g., macOS 13.2, Ubuntu 22.04]
- Doctainr version: [e.g., 0.0.1]
- Docker version: [e.g., 24.0.6]
- Rust version: [e.g., 1.75.0]

**Logs/Screenshots**
Any relevant logs or screenshots.
````

### Suggesting Enhancements

Enhancement suggestions are welcome! Please:
1. Check if the feature already exists or is planned
2. Explain the use case and benefits
3. Provide mockups or examples if applicable

**Feature Request Template:**

````markdown
**Feature description**
Clear description of the feature.

**Use case**
Why this feature would be valuable.

**Proposed solution**
How you envision this working.

**Alternatives considered**
Other approaches you've thought about.
````

### Contributing Code

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/my-feature`
3. **Make your changes** following our guidelines
4. **Test thoroughly**
5. **Commit with clear messages**
6. **Push and create a pull request**

## Development Setup

### Prerequisites

- Docker Desktop or Engine (running)
- Rust 1.70+ (`rustup` recommended)
- Dioxus CLI (`dx`)

See [Development Guide](development.md) for detailed setup instructions.

### Quick Start

````bash
git clone https://github.com/MH0386/doctainr.git
cd doctainr
dx build
dx serve --platform desktop
````

## Code Style Guidelines

### Rust Conventions

Follow standard Rust conventions:

1. **Naming**:
   - `snake_case` for functions, variables, modules
   - `PascalCase` for types, traits, enums
   - `SCREAMING_SNAKE_CASE` for constants

2. **Formatting**: Use `dx fmt` or `cargo fmt`
   ````bash
   dx fmt
   ````

3. **Linting**: Pass Clippy checks
   ````bash
   dx check
   cargo clippy --all-targets --all-features
   ````

4. **Documentation**: Add rustdoc comments to public APIs
   ````rust
   /// Starts a Docker container by ID.
   ///
   /// # Arguments
   ///
   /// * `id` - The container ID or name
   ///
   /// # Returns
   ///
   /// Returns `Ok(())` on success, or an error if the container cannot be started.
   ///
   /// # Example
   ///
   /// ```no_run
   /// let service = DockerService::new()?;
   /// service.start_container("my_container").await?;
   /// ```
   pub async fn start_container(&self, id: &str) -> Result<()> {
       // implementation
   }
   ````

### Dioxus 0.7 Patterns

1. **Use signals for reactive state**:
   ````rust
   let mut count = use_signal(|| 0);
   count.set(5); // Updates UI
   ````

2. **Context for global state**:
   ````rust
   // Provide
   use_context_provider(|| AppState::new());
   
   // Consume
   let app_state = use_context::<AppState>();
   ````

3. **Component annotations**:
   ````rust
   #[component]
   fn MyComponent(name: String, age: i32) -> Element {
       rsx! { div { "{name} is {age} years old" } }
   }
   ````

4. **Async operations with spawn**:
   ````rust
   spawn(async move {
       let result = async_operation().await;
       signal.set(result);
   });
   ````

### Project Structure Conventions

- **Components**: Reusable UI in `src/components/`
- **Views**: Page-level components in `src/views/`
- **Services**: External integrations in `src/services/`
- **Utils**: Shared helpers in `src/utils/`

Each module has a `mod.rs` that re-exports public items:
````rust
// src/components/mod.rs
pub use metric_card::MetricCard;
pub use section_header::SectionHeader;
pub use status_pill::StatusPill;

mod metric_card;
mod section_header;
mod status_pill;
````

## Testing Requirements

### Running Tests

Before submitting a pull request:

````bash
# Format code
dx fmt

# Lint code
dx check

# Run tests
cargo test

# Build release
cargo build --release
````

### Writing Tests

1. **Unit tests**: In `#[cfg(test)]` modules
   ````rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_format_size() {
           assert_eq!(format_size(1024), "1.0KB");
       }
   }
   ````

2. **Integration tests**: Require Docker running
   ````rust
   #[tokio::test]
   async fn test_list_containers() {
       let service = DockerService::new().unwrap();
       let containers = service.list_containers().await;
       assert!(containers.is_ok());
   }
   ````

3. **Test coverage**: Aim for meaningful coverage of new features

## Commit Message Guidelines

Use clear, descriptive commit messages:

### Format

````
<type>(<scope>): <subject>

<body>

<footer>
````

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting, no code change
- `refactor`: Code restructuring
- `test`: Adding/updating tests
- `chore`: Maintenance tasks

### Examples

````
feat(containers): Add container removal functionality

Adds DockerService::remove_container() method and UI button
to remove stopped containers.

Closes #42
````

````
fix(dashboard): Correct running container count

The running container count was including stopped containers.
Fixed by filtering on ContainerState::Running.

Fixes #38
````

````
docs: Add architecture documentation

Created docs/architecture.md with system overview, data flow,
and extension points.
````

## Pull Request Process

### Before Submitting

1. **Sync with main**: Rebase on latest main branch
   ````bash
   git fetch origin
   git rebase origin/main
   ````

2. **Run all checks**:
   ````bash
   dx fmt --check
   dx check
   cargo test
   ````

3. **Update documentation** if needed

4. **Add/update tests** for new functionality

### PR Template

````markdown
## Description

Brief description of changes.

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

Describe testing performed:
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing performed

## Checklist

- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings
- [ ] Tests added/updated
- [ ] All tests pass
````

### Review Process

1. **Automated checks**: CI runs tests and lints
2. **Code review**: Maintainers review changes
3. **Feedback**: Address review comments
4. **Approval**: At least one maintainer approval required
5. **Merge**: Squash and merge to main

## Component Development Guidelines

### Creating Components

1. **Keep components focused**: One responsibility per component
2. **Use props for configuration**: Avoid hardcoded values
3. **Document props**: Explain purpose and expected values
4. **Handle edge cases**: Empty lists, missing data, errors

Example:

````rust
/// Displays a metric card with title, value, and optional hint.
///
/// # Props
///
/// * `title` - The metric label (e.g., "Running containers")
/// * `value` - The metric value to display (e.g., "5")
/// * `hint` - Optional explanatory text below the value
#[component]
pub fn MetricCard(title: String, value: String, hint: Option<String>) -> Element {
    rsx! {
        div { class: "card",
            p { class: "card-title", "{title}" }
            p { class: "card-value", "{value}" }
            if let Some(hint) = hint {
                p { class: "card-hint", "{hint}" }
            }
        }
    }
}
````

### Component Testing

Test components in isolation when possible:

````rust
#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn metric_card_renders_title_and_value() {
        let vdom = VirtualDom::new_with_props(
            MetricCard,
            MetricCardProps {
                title: "Test".to_string(),
                value: "42".to_string(),
                hint: None,
            },
        );
        
        let rendered = dioxus_ssr::render(&vdom);
        assert!(rendered.contains("Test"));
        assert!(rendered.contains("42"));
    }
}
````

## Docker Integration Guidelines

### Using Bollard API

1. **Error handling**: Always return `Result<T>`
2. **Async operations**: Use `async fn` with `await`
3. **Clone Docker client**: Cheap to clone, Arc internally
4. **Options structs**: Use Bollard's option types

Example:

````rust
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
    let options = Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    });
    
    let containers = self.docker.list_containers(options).await?;
    
    // Transform to app-specific types
    Ok(transform_containers(containers))
}
````

### Adding Docker Features

When adding new Docker operations:

1. Add method to `DockerService`
2. Add state method to `AppState`
3. Update relevant view component
4. Add tests (unit and integration)
5. Update documentation

## Documentation Guidelines

### Rustdoc Comments

Use `///` for documentation comments:

````rust
/// Brief one-line summary.
///
/// More detailed explanation with examples if needed.
///
/// # Arguments
///
/// * `param1` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Description of error conditions
///
/// # Example
///
/// ```
/// let result = function(arg);
/// ```
pub fn function(param1: Type) -> Result<ReturnType> {
    // implementation
}
````

### Markdown Documentation

- Use clear headings and structure
- Include code examples with syntax highlighting
- Provide context and motivation
- Link to related documentation
- Keep examples up-to-date

## Release Process

Maintainers handle releases:

1. **Version bump**: Update `Cargo.toml`
2. **Changelog**: Update CHANGELOG.md
3. **Tag**: Create version tag (e.g., `v0.1.0`)
4. **Build**: Create release binaries
5. **Publish**: GitHub release with notes

## Questions and Support

- **Issues**: Use GitHub Issues for bugs and features
- **Discussions**: Use GitHub Discussions for questions
- **Documentation**: Check [docs/](.) first

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

## Recognition

Contributors are recognized in:
- GitHub contributors list
- Release notes
- Special recognition for significant contributions

Thank you for contributing to Doctainr! 🦀🐳
