# Contributing to logly-rs

Thank you for your interest in contributing to logly-rs! We welcome contributions of all kinds.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Code Style](#code-style)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Reporting Bugs](#reporting-bugs)
- [Feature Requests](#feature-requests)

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

### Prerequisites

- Rust 1.70.0 or later
- Cargo (latest stable)
- Git
- (Optional) CUDA Toolkit for GPU features

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:

```bash
git clone https://github.com/your-username/logly-rs.git
cd logly-rs
```

3. Add upstream remote:

```bash
git remote add upstream https://github.com/muhammad-fiaz/logly-rs.git
```

## Development Setup

### Build the Project

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Run Examples

```bash
cargo run --example basic
cargo run --example advanced
```

### Build Documentation

```bash
cargo doc --open
```

### Run Benchmarks

```bash
cargo bench
```

## How to Contribute

### 1. Create a Branch

```bash
git checkout -b feature/my-feature
# or
git checkout -b fix/my-bugfix
```

### 2. Make Changes

- Write clear, concise code
- Follow Rust best practices
- Add tests for new features
- Update documentation
- Ensure all tests pass

### 3. Commit Changes

```bash
git add .
git commit -m "feat: add new feature"
# or
git commit -m "fix: resolve issue #123"
```

Commit message format:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test additions/changes
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `chore:` Maintenance tasks

### 4. Push Changes

```bash
git push origin feature/my-feature
```

### 5. Create Pull Request

- Go to GitHub and create a pull request
- Fill in the PR template
- Link related issues
- Wait for review

## Code Style

### Rust Style Guidelines

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting:

```bash
cargo fmt
```

- Use `clippy` for linting:

```bash
cargo clippy -- -D warnings
```

### Code Quality

- Write idiomatic Rust code
- Avoid unsafe code unless absolutely necessary
- Document unsafe blocks with safety comments
- Use meaningful variable and function names
- Keep functions small and focused
- Minimize dependencies

### Documentation

- Add doc comments for public APIs:

```rust
/// Logs a message at the specified level.
///
/// # Arguments
///
/// * `level` - The log level
/// * `message` - The message to log
///
/// # Examples
///
/// ```
/// logger.log(Level::Info, "Hello".to_string())?;
/// ```
pub fn log(&self, level: Level, message: String) -> Result<()> {
    // ...
}
```

## Testing

### Writing Tests

- Add unit tests in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test code
    }
}
```

- Add integration tests in `tests/`:

```rust
use logly::prelude::*;

#[test]
fn test_integration() {
    // Integration test
}
```

### Test Coverage

- Aim for high test coverage
- Test edge cases
- Test error conditions
- Test concurrent scenarios

### Running Specific Tests

```bash
# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in specific file
cargo test --test integration_tests
```

## Documentation

### Types of Documentation

1. **API Documentation**: Doc comments in code
2. **Guides**: Markdown files in `docs/guides/`
3. **Examples**: Working examples in `examples/`
4. **README**: Main project documentation

### Updating Documentation

- Update relevant docs when changing features
- Add examples for new features
- Keep README.md up to date
- Update CHANGELOG.md

## Pull Request Process

### Before Submitting

1. **Run all checks**:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --all-features
```

2. **Update documentation**
3. **Add tests**
4. **Update CHANGELOG.md**

### PR Requirements

- Clear description of changes
- Link to related issues
- All tests passing
- No compiler warnings
- Documentation updated
- Code formatted with rustfmt
- Clippy checks passing

### Review Process

1. Maintainer reviews PR
2. Address feedback
3. Update PR as needed
4. Approval and merge

## Reporting Bugs

### Before Reporting

- Check existing issues
- Verify it's reproducible
- Test with latest version

### Bug Report Template

```markdown
**Description**
Clear description of the bug

**To Reproduce**
1. Step 1
2. Step 2
3. See error

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Environment**
- OS: [e.g., Windows 11]
- Rust version: [e.g., 1.70.0]
- logly version: [e.g., 0.0.4]

**Additional Context**
Any other relevant information
```

## Feature Requests

### Proposing Features

1. Check existing feature requests
2. Open an issue with `[Feature Request]` prefix
3. Describe the feature clearly
4. Explain use cases
5. Discuss implementation approach

### Feature Request Template

```markdown
**Feature Description**
Clear description of the feature

**Use Case**
Why is this feature needed?

**Proposed Solution**
How should it work?

**Alternatives**
Other approaches considered

**Additional Context**
Any other relevant information
```

## Areas for Contribution

### High Priority

- Bug fixes
- Performance improvements
- Documentation improvements
- Test coverage

### Feature Ideas

- Network sinks (TCP/UDP)
- Syslog integration
- Windows Event Log support
- Log compression
- Log encryption
- Metrics and statistics
- Rate limiting

### Documentation Needs

- More examples
- Tutorial videos
- Blog posts
- Translations

## Development Tips

### Debugging

```rust
// Enable debug mode
logger.enable_debug();

// Use debug_log_file
config.debug_log_file = Some(PathBuf::from("debug.log"));
```

### Performance Testing

```bash
cargo bench
```

### Memory Profiling

```bash
cargo build --release
valgrind --tool=massif target/release/your_binary
```

## Getting Help

- **Documentation**: https://muhammad-fiaz.github.io/logly-rs
- **Issues**: https://github.com/muhammad-fiaz/logly-rs/issues
- **Discussions**: https://github.com/muhammad-fiaz/logly-rs/discussions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Credited in documentation

Thank you for contributing to logly-rs! 🎉
