# Doctainr Documentation

Welcome to the Doctainr documentation! This directory contains comprehensive guides and references for using, developing, and contributing to Doctainr.

## Documentation Structure

Following the [Diátaxis](https://diataxis.fr/) framework, our documentation is organized into four categories:

### 📖 Learning-Oriented (Tutorials)

*Coming soon: Step-by-step tutorials for getting started with Doctainr*

### 🛠️ Problem-Oriented (How-To Guides)

- **[Development Guide](development.md)** - Setting up your development environment, running the app, and common development tasks
- **[Troubleshooting Guide](troubleshooting.md)** - Solutions to common problems and error messages

### 📚 Information-Oriented (Reference)

- **[Architecture Reference](architecture.md)** - System architecture, design patterns, and technical decisions
- **[API Reference](api-reference.md)** - Complete API documentation for all modules, components, and functions

### 💡 Understanding-Oriented (Explanation)

- **[Contributing Guide](contributing.md)** - Understanding our development process, coding standards, and how to contribute effectively

## Quick Navigation

### Getting Started

New to Doctainr? Start here:

1. **[Development Guide](development.md)** - Prerequisites and setup
2. **[Architecture Reference](architecture.md)** - Understand how Doctainr works
3. **[Troubleshooting Guide](troubleshooting.md)** - Solutions to common setup issues

### For Developers

Extending or contributing to Doctainr:

1. **[Development Guide](development.md)** - Development workflow and adding features
2. **[API Reference](api-reference.md)** - Module and component documentation
3. **[Contributing Guide](contributing.md)** - Code style, testing, and PR process

### For Contributors

Want to contribute? Read these:

1. **[Contributing Guide](contributing.md)** - Contribution guidelines and process
2. **[Development Guide](development.md)** - Development environment setup
3. **[API Reference](api-reference.md)** - Understanding the codebase

### Troubleshooting

Having issues? Check:

1. **[Troubleshooting Guide](troubleshooting.md)** - Common problems and solutions
2. **[Development Guide](development.md)** - Development-specific issues
3. **GitHub Issues** - Search or create an issue

## Documentation Goals

Our documentation aims to be:

- **Clear**: Plain English, concise explanations
- **Complete**: Cover all functionality and use cases
- **Accurate**: Stay up-to-date with code changes
- **Accessible**: Easy to navigate and search
- **Inclusive**: Welcome developers of all experience levels

## Contributing to Documentation

Documentation is code! We welcome improvements to docs:

- Fix typos or unclear explanations
- Add missing information
- Create tutorials or examples
- Improve structure or navigation

See [Contributing Guide](contributing.md) for how to submit documentation changes.

## Documentation Style

We follow these style guides:

- **[Google Developer Documentation Style Guide](https://developers.google.com/style)**
- **[Microsoft Writing Style Guide](https://docs.microsoft.com/en-us/style-guide/welcome/)**
- **[Diátaxis Framework](https://diataxis.fr/)**

Key principles:

- Active voice ("Click the button" not "The button should be clicked")
- Present tense ("The function returns" not "The function will return")
- Second person ("You can" not "Users can")
- Inclusive language (avoid master/slave, blacklist/whitelist, etc.)
- Code examples with clear context

## Document Formats

- **Markdown (.md)**: Primary format for all documentation
- **Code blocks**: Use syntax highlighting (````rust, ````bash, etc.)
- **Links**: Relative links to other docs, absolute links to external resources
- **Images**: Store in `docs/images/` (future addition)

## Keeping Docs Current

Documentation should be updated when:

- Adding new features or APIs
- Changing existing behavior
- Fixing bugs that affect usage
- Updating dependencies with breaking changes
- Improving error messages

Documentation gaps are treated like failing tests - they should be fixed before merging.

## Feedback

Found an issue with the documentation?

- **Typos/errors**: Submit a PR with the fix
- **Unclear sections**: Open an issue describing the confusion
- **Missing content**: Open an issue or discussion

## Document Versions

Documentation is version-controlled alongside code:

- **main branch**: Current stable documentation
- **Feature branches**: In-progress documentation updates
- **Releases**: Tagged documentation matching release versions

## Search

To search the documentation:

````bash
# Search all markdown files
grep -r "search term" docs/

# Case-insensitive search
grep -ri "search term" docs/

# Search for function names
grep -rn "fn function_name" docs/
````

## External Resources

### Dioxus

- [Dioxus 0.7 Documentation](https://dioxuslabs.com/learn/0.7)
- [Dioxus Router Guide](https://dioxuslabs.com/learn/0.7/router)
- [Dioxus Examples](https://github.com/DioxusLabs/dioxus/tree/master/examples)

### Docker

- [Docker Engine API](https://docs.docker.com/engine/api/)
- [Bollard Documentation](https://docs.rs/bollard/)
- [Docker CLI Reference](https://docs.docker.com/engine/reference/commandline/cli/)

### Rust

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Rust Book](https://rust-lang.github.io/async-book/)
- [Tokio Documentation](https://tokio.rs/tokio/tutorial)

## Documentation Roadmap

Future documentation additions:

- [ ] **Tutorials**: Step-by-step getting started guide
- [ ] **Deployment Guide**: Building and distributing Doctainr
- [ ] **Security Guide**: Security best practices
- [ ] **Performance Guide**: Optimization techniques
- [ ] **Plugin Development**: Guide for creating extensions (future)
- [ ] **Testing Guide**: Comprehensive testing documentation
- [ ] **Screenshots**: Visual guides and feature demonstrations
- [ ] **Video Tutorials**: Walkthrough videos
- [ ] **Internationalization**: Translated documentation

## License

Documentation is licensed under the same license as the project code.

---

**Last Updated**: 2026-02-14

**Maintained By**: Doctainr Contributors
