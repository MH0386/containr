# Contributing to Doctainr

Thank you for your interest in contributing to Doctainr! This guide will help you get started.

## Code of Conduct

Please be respectful and constructive in all interactions. We're building a welcoming community.

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report, please:
1. **Check existing issues** to avoid duplicates
2. **Verify the bug** with the latest version
3. **Collect information** about your environment

Create a bug report with:
- **Clear title** describing the issue
- **Steps to reproduce** the problem
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Docker version, Doctainr version)
- **Error messages** or screenshots if applicable

### Suggesting Features

Feature suggestions are welcome! Please:
1. **Check discussions** for similar ideas
2. **Describe the use case** clearly
3. **Explain the expected behavior**
4. **Consider alternatives** you've explored

### Contributing Code

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a feature branch** from `main`
4. **Make your changes** following our guidelines
5. **Test thoroughly**
6. **Submit a pull request**

## Development Setup

### Prerequisites

- Docker Desktop (running)
- Rust 1.70+
- Dioxus CLI (`dx`)
- Git

### Setup Steps

````bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/doctainr.git
cd doctainr

# Install dependencies
dx check

# Run in development mode
dx serve --platform desktop
````

See [Development Guide](docs/development.md) for detailed instructions.

## Coding Standards

### Use `dx` Not `cargo`

Always use `dx` CLI for development:

````bash
✅ dx serve --platform desktop
✅ dx build --release
✅ dx fmt
✅ dx check

❌ cargo run
❌ cargo build
````

### Code Style

- **Format code** before committing: `dx fmt`
- **Run linter**: `cargo clippy`
- **Follow Rust conventions**: snake_case, CamelCase as appropriate
- **Add documentation**: rustdoc comments for public APIs
- **Keep functions small**: Single responsibility principle

### Dioxus 0.7 Patterns

Follow modern Dioxus patterns:

````rust
// ✅ Correct (Dioxus 0.7)
#[component]
fn MyComponent(title: String) -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        button {
            onclick: move |_| *count.write() += 1,
            "{title}: {count}"
        }
    }
}

// ❌ Outdated (pre-0.7)
fn MyComponent(cx: Scope, title: &str) -> Element {
    let count = use_state(cx, || 0);
    // ...
}
````

Key points:
- No more `cx` or `Scope` parameters
- Use `use_signal()` instead of `use_state()`
- Props must be owned types (`String`, not `&str`)

### Documentation

All public items must have documentation:

````rust
/// Brief one-line description.
///
/// Longer explanation if needed, with examples.
///
/// # Arguments
///
/// * `arg` - Argument description
///
/// # Returns
///
/// Return value description
///
/// # Errors
///
/// Error conditions
pub fn function(arg: Type) -> Result<Type> {
    // ...
}
````

### Testing

- Add unit tests for new functionality
- Ensure existing tests pass: `cargo test`
- Add integration tests for complex features

## Commit Guidelines

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

````
<type>(<scope>): <subject>

<body>

<footer>
````

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Examples:**

````
feat(containers): add container restart functionality

Adds a restart button to the containers view that stops
and immediately starts a container.

Closes #123
````

````
fix(docker): handle connection timeout gracefully

Previously, connection timeouts would crash the app.
Now shows a user-friendly error message.
````

````
docs(api): add rustdoc comments to DockerService
````

### Commit Best Practices

- **Atomic commits** - One logical change per commit
- **Clear messages** - Explain *what* and *why*, not *how*
- **Reference issues** - Include `Closes #123` or `Fixes #456`

## Pull Request Process

### Before Submitting

1. **Update from main**:
   ````bash
   git checkout main
   git pull upstream main
   git checkout your-branch
   git rebase main
   ````

2. **Format and lint**:
   ````bash
   dx fmt
   cargo clippy
   ````

3. **Run tests**:
   ````bash
   cargo test
   ````

4. **Test manually** in the app

### PR Description

Include:
- **Purpose** - What problem does this solve?
- **Changes** - What was modified?
- **Testing** - How was this tested?
- **Screenshots** - For UI changes
- **Breaking changes** - If any
- **Related issues** - Link to issues

**Template:**

````markdown
## Purpose
Brief description of the problem and solution.

## Changes
- Added X feature
- Modified Y component
- Fixed Z bug

## Testing
- [ ] Unit tests pass
- [ ] Manual testing completed
- [ ] Tested on [OS/platform]

## Screenshots
[If applicable]

Closes #123
````

### Review Process

1. **Automated checks** must pass (CI)
2. **Maintainer review** - May request changes
3. **Address feedback** - Make requested changes
4. **Merge** - Once approved, we'll merge your PR

### After Merge

- Delete your feature branch
- Update your fork's main branch
- Celebrate! 🎉

## Project Structure

Understand the codebase:

````
src/
├── main.rs              # Entry point & routing
├── components/          # Reusable UI components
├── services/            # External integrations (Docker API)
├── utils/               # Shared utilities & state
└── views/               # Route views/pages
````

See [Architecture](docs/architecture.md) for details.

## Areas for Contribution

### Good First Issues

Look for issues labeled `good first issue`:
- Documentation improvements
- UI polish
- Small bug fixes
- Test coverage

### High Priority

- Container logs viewer
- Image pull functionality
- Docker Compose support
- Performance optimizations

### Documentation

- Improve rustdoc comments
- Add more examples
- Create tutorials
- Fix typos and clarity

## Style Guide

### Rust Code

- **Line length**: 100 characters (enforced by rustfmt)
- **Naming**: Follow Rust conventions
  - Types: `PascalCase`
  - Functions/variables: `snake_case`
  - Constants: `SCREAMING_SNAKE_CASE`
- **Error handling**: Use `Result` and `?` operator
- **Async**: Use `async`/`await` for I/O operations

### Dioxus RSX

- **Indentation**: 4 spaces
- **Attributes**: One per line for readability
- **Naming**: Use `class` not `class_name` attribute

````rust
rsx! {
    div {
        class: "container",
        onclick: move |_| handle_click(),
        
        h1 { "Title" }
        p { "Content" }
    }
}
````

### CSS

- **Class naming**: Use kebab-case
- **Organization**: Group related styles
- **Comments**: Explain non-obvious styles

## Getting Help

- **Questions?** Open a [Discussion](https://github.com/MH0386/doctainr/discussions)
- **Stuck?** Comment on the issue or PR
- **Need docs?** Check [Development Guide](docs/development.md)

## Recognition

Contributors will be:
- Listed in release notes
- Credited in the repository
- Thanked publicly

Thank you for contributing to Doctainr! 🚀

---

**Related Documentation:**
- [Development Guide](docs/development.md) - How to build and extend
- [Architecture](docs/architecture.md) - System design
- [Code of Conduct](CODE_OF_CONDUCT.md) - Community guidelines
