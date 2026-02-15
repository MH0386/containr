# Contributing to Doctainr

Thank you for your interest in contributing to Doctainr! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Issue Guidelines](#issue-guidelines)

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment. We expect all contributors to:

- Be respectful and considerate in communication
- Accept constructive criticism gracefully
- Focus on what's best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

Before you begin, ensure you have:

- Rust toolchain 1.70 or later
- Docker installed and running
- Git for version control
- (Optional) Nix with devenv for development environment

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:

```bash
git clone https://github.com/YOUR_USERNAME/doctainr.git
cd doctainr
```

3. Add the upstream repository:

```bash
git remote add upstream https://github.com/MH0386/doctainr.git
```

## Development Setup

### Option 1: Using Nix and devenv (Recommended)

The project uses Nix and devenv for reproducible development environments:

```bash
# Install Nix (if not already installed)
curl -L https://nixos.org/nix/install | sh

# Install devenv
nix profile install nixpkgs#devenv

# Enter the development environment
devenv shell

# Install git hooks
devenv up
```

### Option 2: Manual Setup

Install dependencies manually:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli

# Build dependencies
dx build
```

### Verify Setup

```bash
# Check versions
rustc --version
cargo --version
dx --version

# Ensure Docker is running
docker info

# Run tests
cargo test

# Run the application
dx run
```

## How to Contribute

### Types of Contributions

We welcome various types of contributions:

- **Bug Fixes**: Fix issues in existing code
- **Features**: Implement new functionality
- **Documentation**: Improve or add documentation
- **Tests**: Add or improve test coverage
- **UI/UX**: Improve the user interface
- **Performance**: Optimize code performance
- **Refactoring**: Improve code structure

### Finding Work

- Check the [Issues](https://github.com/MH0386/doctainr/issues) page
- Look for issues labeled `good first issue` or `help wanted`
- Comment on an issue to express interest
- Ask questions if anything is unclear

### Creating Issues

Before creating a new issue:

1. Search existing issues to avoid duplicates
2. Use a clear, descriptive title
3. Provide detailed information:
   - Steps to reproduce (for bugs)
   - Expected vs actual behavior
   - Your environment (OS, Rust version, Docker version)
   - Relevant logs or screenshots

## Coding Standards

### Rust Style Guide

Follow the official Rust style guidelines:

- Use `rustfmt` for code formatting
- Use `clippy` for linting
- Follow Rust naming conventions:
  - `snake_case` for functions and variables
  - `PascalCase` for types and traits
  - `SCREAMING_SNAKE_CASE` for constants

### Code Organization

```
src/
├── main.rs          # Application entry point
├── components/      # Reusable UI components
├── services/        # Business logic and external APIs
├── utils/           # Utilities and shared state
└── views/           # Page-level components
```

### Dioxus Conventions

Follow Dioxus 0.7 best practices:

- Use `#[component]` macro for all components
- Component names start with capital letter
- Use signals for reactive state
- Prefer `use_signal` over `use_state`
- Use `use_context` for global state

### Documentation

All public items should have documentation comments:

````rust
/// Starts a Docker container by ID.
///
/// # Arguments
///
/// * `id` - The container ID or name
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the operation fails.
///
/// # Example
///
/// ```
/// let service = DockerService::new()?;
/// service.start_container("my_container").await?;
/// ```
pub async fn start_container(&self, id: &str) -> Result<()> {
    // Implementation
}
````

### Error Handling

- Use `Result<T, E>` for fallible operations
- Use `anyhow::Result` for application errors
- Provide meaningful error messages
- Don't unwrap in production code
- Use `?` operator for error propagation

### Async Code

- Use `async/await` for asynchronous operations
- Use `spawn` for fire-and-forget tasks
- Update signals from async contexts
- Handle errors properly in async blocks

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run tests with devenv
devenv test
```

### Writing Tests

Add tests for:

- New features
- Bug fixes
- Edge cases
- Error conditions

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state_labels() {
        assert_eq!(ContainerState::Running.label(), "Running");
        assert_eq!(ContainerState::Stopped.label(), "Stopped");
    }

    #[tokio::test]
    async fn test_docker_service_creation() {
        let result = DockerService::new();
        assert!(result.is_ok());
    }
}
```

### Test Guidelines

- Write clear test names that describe what's being tested
- Test one thing per test
- Use descriptive assertions
- Mock external dependencies when necessary
- Ensure tests are deterministic

## Pull Request Process

### Before Submitting

1. **Update from upstream**:

   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Create a feature branch**:

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**:
   - Follow coding standards
   - Add tests for new functionality
   - Update documentation as needed

4. **Run quality checks**:

   ```bash
   dx fmt
   dx check
   cargo test
   ```

5. **Commit your changes**:

   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

   Follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation
   - `test:` for tests
   - `refactor:` for refactoring
   - `style:` for formatting
   - `chore:` for maintenance

6. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

### Submitting the PR

1. Go to the original repository on GitHub
2. Click "New Pull Request"
3. Select your fork and branch
4. Fill in the PR template:
   - Clear title describing the change
   - Description of what changed and why
   - Link to related issues
   - Screenshots for UI changes
   - Test results

### PR Review Process

- Maintainers will review your PR
- Address any requested changes
- Keep the PR focused and small
- Be responsive to feedback
- Once approved, a maintainer will merge

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] Self-review of code completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] Tests added/updated and passing
- [ ] No new warnings from clippy
- [ ] Commit messages follow conventions
- [ ] PR description is clear and complete

## Issue Guidelines

### Bug Reports

Include:

- Clear, descriptive title
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, Docker version)
- Relevant logs or error messages
- Screenshots if applicable

### Feature Requests

Include:

- Clear description of the feature
- Use case or problem it solves
- Proposed solution or implementation ideas
- Alternative approaches considered
- Willingness to implement

### Questions

For questions:

- Search existing issues first
- Check the documentation
- Provide context about what you're trying to do
- Include relevant code snippets

## Development Workflow

### Typical Development Cycle

1. Pick an issue or create one
2. Create a feature branch
3. Make changes incrementally
4. Test frequently during development
5. Run quality checks
6. Submit PR
7. Address review feedback
8. Get merged!

### Git Workflow

```bash
# Start new feature
git checkout -b feature/my-feature

# Make changes and commit
git add .
git commit -m "feat: implement my feature"

# Keep up to date with main
git fetch upstream
git rebase upstream/main

# Push to your fork
git push origin feature/my-feature

# After PR is merged, clean up
git checkout main
git pull upstream main
git branch -d feature/my-feature
```

## Getting Help

If you need help:

- Check existing documentation
- Search closed issues
- Ask in a new issue
- Be specific about what you need help with

## Recognition

Contributors will be:

- Listed in CHANGELOG.md for their contributions
- Acknowledged in release notes
- Added to the contributors list

Thank you for contributing to Doctainr! 🎉
