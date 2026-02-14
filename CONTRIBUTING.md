# Contributing to Doctainr

Thank you for your interest in contributing! We welcome contributions of all kinds.

## Quick Links

- 📖 **[Full Contributing Guide](docs/contributing.md)** - Detailed guidelines
- 🏗️ **[Development Guide](docs/development.md)** - Setup and development workflow
- 🏛️ **[Architecture](docs/architecture.md)** - System design and structure
- 🔧 **[Troubleshooting](docs/troubleshooting.md)** - Common issues and solutions

## Quick Start

````bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/doctainr.git
cd doctainr

# Build and run
dx serve --platform desktop
````

## How to Contribute

- 🐛 **Report bugs** - [Create an issue](https://github.com/MH0386/doctainr/issues/new)
- 💡 **Suggest features** - [Start a discussion](https://github.com/MH0386/doctainr/discussions/new)
- 📝 **Improve docs** - Documentation PRs are always welcome
- 💻 **Submit code** - Fork, branch, commit, and PR

## Coding Standards

- **Use `dx` not `cargo`** for all commands
- **Format code** with `dx fmt` before committing
- **Run linter** with `cargo clippy`
- **Add tests** for new functionality
- **Document** public APIs with rustdoc comments

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

````
feat(containers): add restart functionality
fix(docker): handle connection timeout
docs(api): add rustdoc comments
````

## Code of Conduct

Be respectful and constructive. We're building a welcoming community.

## Need Help?

- 💬 [GitHub Discussions](https://github.com/MH0386/doctainr/discussions) - Ask questions
- 🐛 [GitHub Issues](https://github.com/MH0386/doctainr/issues) - Report bugs

---

For complete guidelines, see **[docs/contributing.md](docs/contributing.md)**
