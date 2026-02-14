# Contributing to Doctainr

Thank you for your interest in contributing to Doctainr! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and considerate in all interactions.

## How to Contribute

### Reporting Bugs

Before creating a bug report:
1. Check if the issue already exists in [GitHub Issues](https://github.com/MH0386/doctainr/issues)
2. Verify you're using the latest version
3. Ensure Docker is running correctly

When creating a bug report, include:
- **Description**: Clear description of the problem
- **Steps to Reproduce**: Numbered steps to trigger the issue
- **Expected Behavior**: What should happen
- **Actual Behavior**: What actually happens
- **Environment**:
  - OS and version
  - Docker version
  - Doctainr version
  - Rust version
- **Logs**: Any error messages or logs
- **Screenshots**: If applicable

### Suggesting Features

Feature suggestions are welcome! Please:
1. Check existing issues and discussions first
2. Provide a clear use case
3. Explain the benefits
4. Consider implementation complexity

Open an issue with the "enhancement" label and describe your idea.

### Contributing Code

#### Getting Started

1. **Fork the repository**
   ````bash
   # Fork on GitHub, then clone your fork
   git clone https://github.com/YOUR_USERNAME/doctainr.git
   cd doctainr
   ````

2. **Set up your environment**
   ````bash
   # Add upstream remote
   git remote add upstream https://github.com/MH0386/doctainr.git
   
   # Install dependencies
   cargo build
   ````

3. **Create a branch**
   ````bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ````

#### Making Changes

**Code Style**:
- Follow Rust naming conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Write clear, self-documenting code
- Add comments for complex logic only

**Documentation**:
- Add rustdoc comments for public APIs
- Update relevant documentation files
- Include code examples in docstrings

**Testing**:
- Add tests for new functionality
- Ensure existing tests pass: `cargo test`
- Test manually with different Docker configurations

#### Code Review Checklist

Before submitting your pull request, ensure:

- [ ] Code follows project style guidelines
- [ ] All tests pass locally
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] Commit messages are clear and descriptive
- [ ] No unnecessary dependencies added
- [ ] Code is properly formatted (`cargo fmt`)
- [ ] No Clippy warnings (`cargo clippy`)
- [ ] Changes work on your target platform

#### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

````
type(scope): short description

Longer description if needed.

Fixes #123
````

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
````
feat(containers): add bulk container operations

- Add select all functionality
- Implement bulk start/stop actions
- Add confirmation dialog for bulk operations

Closes #45
````

````
fix(docker): handle connection timeout gracefully

The app would hang indefinitely if Docker daemon was unreachable.
Now displays a user-friendly error message after 5 second timeout.

Fixes #78
````

#### Submitting a Pull Request

1. **Push your branch**
   ````bash
   git push origin feature/your-feature-name
   ````

2. **Create a pull request** on GitHub

3. **Fill out the PR template** with:
   - Description of changes
   - Related issues
   - Testing performed
   - Screenshots (for UI changes)

4. **Respond to feedback** from reviewers

5. **Update your PR** if requested:
   ````bash
   # Make changes
   git add .
   git commit -m "Address review feedback"
   git push origin feature/your-feature-name
   ````

## Development Guidelines

### Project Structure

````
src/
├── components/      # Reusable UI components
├── services/        # Business logic, external APIs
├── utils/           # Helpers, state management
└── views/           # Full-page components
````

### Architectural Principles

1. **Separation of Concerns**
   - UI components in `components/` and `views/`
   - Business logic in `services/`
   - State management in `utils/`

2. **Reactive State**
   - Use Dioxus signals for state
   - Keep components pure when possible
   - Minimize signal dependencies

3. **Error Handling**
   - Return `Result<T, anyhow::Error>` from services
   - Display user-friendly error messages in UI
   - Log technical details with `eprintln!`

4. **Async Operations**
   - Use `spawn()` for async Docker operations
   - Don't block the UI thread
   - Provide loading states

### Rust Best Practices

- **Ownership**: Prefer owned types for props
- **Error Handling**: Use `Result` and `?` operator
- **Options**: Use `Option` for nullable values
- **Iterators**: Use iterator chains over loops
- **Types**: Use strong types over primitives
- **Clippy**: Address all Clippy suggestions

### Dioxus Patterns

**Components**:
````rust
#[component]
pub fn MyComponent(prop: String) -> Element {
    rsx! {
        div { "{prop}" }
    }
}
````

**State**:
````rust
let mut state = use_signal(|| initial_value);
let value = state();  // Read
state.set(new_value); // Write
````

**Context**:
````rust
// Provide
use_context_provider(|| AppState::new());

// Consume
let app_state = use_context::<AppState>();
````

**Async**:
````rust
spawn(async move {
    let result = async_operation().await;
    signal.set(result);
});
````

### Testing

**Unit Tests**:
````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        assert_eq!(function(), expected);
    }
}
````

**Async Tests**:
````rust
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await.unwrap();
    assert_eq!(result, expected);
}
````

### Documentation

**Rustdoc Format**:
````rust
/// Brief one-line description.
///
/// Longer description with more details.
///
/// # Arguments
///
/// * `arg1` - Description of arg1
/// * `arg2` - Description of arg2
///
/// # Returns
///
/// Description of return value.
///
/// # Errors
///
/// Describes when this function returns an error.
///
/// # Example
///
/// ```no_run
/// use crate::Module;
///
/// let result = function(arg1, arg2)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn function(arg1: Type1, arg2: Type2) -> Result<ReturnType> {
    // Implementation
}
````

## Areas for Contribution

### Good First Issues

Look for issues labeled "good first issue":
- Documentation improvements
- UI polish and styling
- Error message improvements
- Test coverage
- Minor feature additions

### Priority Areas

- **Testing**: Increase test coverage
- **Documentation**: API docs, tutorials, examples
- **Accessibility**: Keyboard navigation, screen reader support
- **Performance**: Optimize rendering, reduce memory usage
- **Features**: Docker Compose support, advanced filtering

### UI/UX Improvements

- Improve visual design
- Add dark mode support
- Enhance error messages
- Improve loading states
- Add keyboard shortcuts

## Getting Help

- **Documentation**: Check [docs/](../)
- **Issues**: Search existing issues or create a new one
- **Discussions**: Use GitHub Discussions for questions
- **Code**: Read the source code and comments

## License

By contributing to Doctainr, you agree that your contributions will be licensed under the same license as the project.

## Recognition

Contributors will be recognized in:
- GitHub contributors list
- Release notes for significant contributions
- Special thanks in README for major features

Thank you for contributing to Doctainr! 🚀
