# Contributing Guidelines

Thank you for your interest in contributing to Doctainr! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Testing](#testing)
- [Documentation](#documentation)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors.

### Our Standards

- Use welcoming and inclusive language
- Be respectful of differing viewpoints
- Accept constructive criticism gracefully
- Focus on what's best for the community
- Show empathy towards other contributors

## How Can I Contribute?

### Reporting Bugs

Before submitting a bug report:
1. Check the [existing issues](https://github.com/MH0386/doctainr/issues)
2. Try the latest version to see if the issue is already fixed
3. Collect information about the bug

**Good Bug Report includes**:
- Clear title and description
- Steps to reproduce
- Expected vs actual behavior
- Screenshots (if applicable)
- Environment details (OS, Rust version, Docker version)

**Example**:
```markdown
**Title**: Container fails to start with port binding error

**Description**: When clicking "Start" on a stopped container that has port 8080 mapped, the operation fails with "port already in use".

**Steps to Reproduce**:
1. Have container with port 8080:80 mapping
2. Click "Start" button
3. Error appears

**Expected**: Container starts successfully
**Actual**: Error message "port already in use"

**Environment**:
- OS: Ubuntu 22.04
- Docker: 24.0.5
- Doctainr: 0.1.0
```

### Suggesting Features

Feature suggestions are welcome! Please:
1. Check if the feature is already requested
2. Explain the use case
3. Describe how it should work
4. Consider implementation complexity

**Feature Request Template**:
```markdown
**Feature**: Add container logs viewer

**Use Case**: As a user, I want to view container logs without switching to terminal

**Proposed Solution**: Add a "Logs" button in Containers view that opens a modal showing real-time logs

**Alternatives Considered**: 
- Inline logs display
- Separate logs view

**Additional Context**: 
Screenshot of potential UI
```

### Contributing Code

Areas where contributions are welcome:
- Bug fixes
- New features
- Performance improvements
- Documentation
- Tests
- UI/UX improvements

## Development Setup

### Prerequisites

1. **Install dependencies**:
```bash
# See Installation Guide for platform-specific requirements
```

2. **Clone the repository**:
```bash
git clone https://github.com/MH0386/doctainr.git
cd doctainr
```

3. **Install Dioxus CLI** (recommended):
```bash
cargo install dioxus-cli
```

### Build and Run

```bash
# Run in development mode with hot-reload
dx serve --platform desktop

# Or using cargo
cargo run

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings
```

## Making Changes

### Branch Strategy

1. **Create a feature branch**:
```bash
git checkout -b feature/your-feature-name
# Or for bugs
git checkout -b fix/bug-description
```

2. **Make your changes**:
- Keep commits focused and atomic
- Write clear commit messages
- Test your changes

3. **Keep your branch updated**:
```bash
git fetch origin
git rebase origin/main
```

### Commit Messages

Follow conventional commit format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(containers): add container logs viewer

Adds a new "Logs" button to the Containers view that opens a modal
displaying real-time container logs.

Closes #123
```

```
fix(docker): handle permission denied error gracefully

Previously, permission errors would crash the app. Now displays
a user-friendly error message with instructions.

Fixes #456
```

## Submitting Changes

### Pull Request Process

1. **Update documentation**:
   - Update relevant docs in `docs/`
   - Update README if needed
   - Add/update code comments

2. **Ensure tests pass**:
```bash
cargo test
cargo clippy
cargo fmt --check
```

3. **Push your branch**:
```bash
git push origin feature/your-feature-name
```

4. **Create Pull Request**:
   - Go to GitHub and create PR
   - Fill out the PR template
   - Link related issues
   - Request review

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tests added/updated
- [ ] Manual testing completed
- [ ] All tests pass

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings

## Related Issues
Closes #(issue number)

## Screenshots
(if applicable)
```

### Code Review

- Be responsive to feedback
- Make requested changes promptly
- Ask questions if anything is unclear
- Be respectful of reviewers' time

## Code Style

See [Code Style Guide](code-style.md) for detailed conventions.

### Quick Reference

**Rust Code**:
```rust
// Use descriptive names
let container_count = containers.len();

// Add doc comments for public items
/// Lists all Docker containers.
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
    // Implementation
}

// Keep functions focused
fn format_size(bytes: i64) -> String {
    // Single responsibility
}

// Use proper error handling
match docker_service.list_containers().await {
    Ok(containers) => {/* ... */},
    Err(e) => {/* handle error */},
}
```

**Dioxus Components**:
```rust
#[component]
pub fn MyComponent(
    // Use owned types for props
    title: String,
    count: i32,
) -> Element {
    // Keep component logic simple
    rsx! {
        div { class: "my-component",
            h2 { "{title}" }
            p { "Count: {count}" }
        }
    }
}
```

## Testing

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state_labels() {
        assert_eq!(ContainerState::Running.label(), "Running");
    }

    #[tokio::test]
    async fn test_list_containers() {
        // Async test
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

## Documentation

### Code Documentation

```rust
/// Brief description.
///
/// Longer description with details about behavior,
/// edge cases, and usage.
///
/// # Arguments
/// * `id` - Container ID or name
///
/// # Returns
/// Returns `Ok(())` on success or error if operation fails
///
/// # Examples
/// ```
/// let result = docker_service.start_container("my-container").await?;
/// ```
pub async fn start_container(&self, id: &str) -> Result<()> {
    // Implementation
}
```

### Documentation Files

When adding features, update:
- Architecture docs if design changes
- API docs if interfaces change
- User guide if user-facing features added
- README if installation/usage changes

## Getting Help

### Questions?

- Open a discussion on GitHub
- Ask in pull request comments
- Check existing issues and docs

### Resources

- [Development Setup Guide](development.md)
- [Code Style Guide](code-style.md)
- [Architecture Documentation](../architecture/overview.md)
- [API Reference](../api/docker-service.md)

## Recognition

Contributors will be:
- Listed in project contributors
- Mentioned in release notes
- Credited in relevant documentation

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to Doctainr! 🎉
