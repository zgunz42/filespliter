# Contributing to File Splitter

Thank you for your interest in contributing to File Splitter! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- A code editor (VS Code, IntelliJ IDEA, or similar)

### Clone and Build

```bash
git clone https://github.com/YOUR_USERNAME/filesplitter.git
cd filesplitter
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Code Style

We use the standard Rust formatting and linting tools:

### Format Code

```bash
cargo fmt
```

### Lint Code

```bash
cargo clippy -- -D warnings
```

**All code must pass `cargo fmt` and `cargo clippy` before being merged.**

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

Branch naming conventions:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions or modifications

### 2. Make Your Changes

- Write clear, concise commit messages
- Keep commits focused and atomic
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Build in release mode
cargo build --release
```

### 4. Commit Your Changes

```bash
git add .
git commit -m "feat: add new feature"
```

Commit message format:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Test changes
- `chore:` - Build/tooling changes

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then open a Pull Request on GitHub.

## Pull Request Guidelines

### Before Submitting

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy passes with no warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Commit messages are clear
- [ ] Branch is up to date with main

### PR Description

Include in your PR description:
1. **What** - What does this PR do?
2. **Why** - Why is this change needed?
3. **How** - How does it work?
4. **Testing** - How was it tested?

### Example

```markdown
## What
Adds support for resumable file splitting

## Why
Users want to resume interrupted split operations

## How
Implements checkpoint system that tracks progress

## Testing
- Added unit tests for checkpoint logic
- Tested with 10GB file with interruptions
```

## Code Review Process

1. Automated checks run via GitHub Actions
2. Maintainers review code
3. Feedback is provided
4. Changes are requested if needed
5. Once approved, PR is merged

## Coding Standards

### Error Handling

- Use `anyhow::Result` for functions that can fail
- Provide context with `.context()` for errors
- Never use `unwrap()` or `expect()` in production code

### Performance

- Use buffered I/O for file operations
- Avoid loading entire files into memory
- Profile code for performance-critical sections

### Documentation

- Add doc comments for public APIs
- Include examples in doc comments
- Keep README up to date

### Testing

- Write unit tests for new functions
- Add integration tests for features
- Ensure tests are deterministic

## Feature Requests and Bug Reports

### Bug Reports

Include:
- Operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected behavior
- Actual behavior
- Error messages (if any)

### Feature Requests

Include:
- Description of the feature
- Use case / motivation
- Example usage
- Alternative solutions considered

## Questions?

Feel free to open an issue for questions or discussion!

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
