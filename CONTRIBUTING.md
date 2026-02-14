# Contributing to Doctainr

Thank you for your interest in contributing to Doctainr! This guide will help you get started.

## Code of Conduct

Be respectful, inclusive, and constructive. We welcome contributions from everyone.

## How to Contribute

### Reporting Issues

- Check existing issues before creating a new one
- Provide clear reproduction steps
- Include system information (OS, Docker version, Rust version)
- Share relevant error messages or screenshots

### Submitting Pull Requests

1. **Fork and clone** the repository
2. **Create a feature branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes** following our coding standards
4. **Test your changes** thoroughly
5. **Commit with clear messages**: Follow [Conventional Commits](https://www.conventionalcommits.org/)
6. **Push and create a PR** with a detailed description

### Development Setup

See [docs/tutorials/development-setup.md](docs/tutorials/development-setup.md) for detailed setup instructions.

#### Quick Start

````bash
# Ensure prerequisites
docker info
rustc --version
dx --version

# Clone and run
git clone https://github.com/MH0386/doctainr.git
cd doctainr
dx serve --platform desktop
````

## Coding Standards

### Rust Style

- **Format code**: Run `cargo fmt` before committing
- **Lint code**: Ensure `cargo clippy` passes without warnings
- **Document public APIs**: Add doc comments (`///`) to all public items
- **Module documentation**: Include module-level docs (`//!`) explaining purpose

### Documentation

- **Inline docs**: Document all public types, functions, and modules
- **Examples**: Provide usage examples in doc comments where helpful
- **Keep README updated**: Reflect feature additions in the main README

### Commit Messages

Follow Conventional Commits:

- `feat: add container logs view`
- `fix: resolve Docker connection timeout`
- `docs: update API documentation`
- `refactor: simplify state management`
- `test: add unit tests for docker service`

## Testing

````bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
````

## Architecture Guidelines

- **Separation of concerns**: Keep services, views, components, and utils distinct
- **State management**: Use Dioxus signals for reactive state
- **Error handling**: Use `Result<T, anyhow::Error>` for recoverable errors
- **Async operations**: Use `tokio` for async Docker API calls

## Documentation Structure

We follow the [Diátaxis](https://diataxis.fr/) documentation framework:

- **Tutorials**: Learning-oriented guides in `docs/tutorials/`
- **How-to guides**: Problem-solving guides in `docs/how-to/`
- **Reference**: Technical specifications in `docs/reference/`
- **Explanation**: Understanding-oriented content in `docs/explanation/`

## Questions?

Open an issue or start a discussion on GitHub. We're here to help!

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.
