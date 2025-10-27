# Contributing to VEM

Thank you for your interest in contributing to VEM! This guide will help you get started with contributing to the project.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Development Environment](#development-environment)
3. [Code Style and Guidelines](#code-style-and-guidelines)
4. [Testing](#testing)
5. [Submitting Changes](#submitting-changes)
6. [Release Process](#release-process)
7. [Community Guidelines](#community-guidelines)

## Getting Started

### Prerequisites

Ensure you have the required development tools:

```bash
# Rust toolchain (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable

# Development dependencies
sudo apt-get install universal-ctags vim neovim git

# Additional tools
cargo install cargo-tarpaulin   # Coverage
cargo install cargo-audit       # Security auditing
cargo install cargo-clippy      # Linting
```

### Fork and Clone

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR-USERNAME/vem.git
   cd vem
   ```

3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/ryo-arima/vem.git
   ```

4. **Verify setup**:
   ```bash
   git remote -v
   # origin    https://github.com/YOUR-USERNAME/vem.git (fetch)
   # origin    https://github.com/YOUR-USERNAME/vem.git (push)
   # upstream  https://github.com/ryo-arima/vem.git (fetch)
   # upstream  https://github.com/ryo-arima/vem.git (push)
   ```

## Development Environment

### Building VEM

```bash
# Debug build (development)
cargo build

# Release build (production)
cargo build --release

# Run from source
cargo run -- --help

# Install locally for testing
cargo install --path .
```

### Setting Up Test Environment

```bash
# Create test workspace
mkdir -p ~/vem-dev-test
cd ~/vem-dev-test

# Set VEM_HOME for testing
export VEM_HOME="$PWD/.vem"

# Initialize test environment
vem init

# Create sample environments for testing
cp -r ~/vem/etc/.vem/envs/* ~/.vem/environments/
```

### Development Workflow

1. **Create feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make changes** following code guidelines

3. **Test changes**:
   ```bash
   # Run tests
   make test
   
   # Run linting
   make lint
   
   # Test manually
   cargo run -- create test-env
   ```

4. **Commit changes**:
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

## Code Style and Guidelines

### Rust Style

VEM follows Rust standard conventions:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint code
cargo clippy -- -D warnings

# Check security
cargo audit
```

### Code Organization

```
src/
├── main.rs          # Entry point
├── base.rs          # Core application structure  
├── mod.rs           # Module declarations
├── cnf/             # Configuration management
├── ctl/             # Controllers (command handlers)
├── ent/             # Entities (data structures)
├── rep/             # Repositories (data access)
├── usc/             # Use cases (business logic)
└── util/            # Utilities and helpers
```

### Naming Conventions

- **Files**: Snake case (`environment_manager.rs`)
- **Functions**: Snake case (`create_environment`)
- **Structs**: Pascal case (`EnvironmentConfig`)
- **Constants**: Screaming snake case (`DEFAULT_CONFIG_PATH`)
- **Modules**: Snake case (`ctags_manager`)

### Error Handling

Use VEM's custom error types:

```rust
use crate::util::error::{VemError, Result};

fn example_function() -> Result<String> {
    let config = load_config()
        .map_err(|e| VemError::ConfigError(format!("Failed to load: {}", e)))?;
    
    Ok(config.name.clone())
}
```

### Documentation

- **Public functions**: Must have doc comments
- **Complex logic**: Inline comments explaining why, not what
- **Examples**: Include usage examples for public APIs

```rust
/// Creates a new environment with the specified configuration.
/// 
/// # Arguments
/// 
/// * `name` - The name of the environment to create
/// * `config` - Environment configuration
/// 
/// # Examples
/// 
/// ```
/// use vem::environment::create_environment;
/// 
/// let config = EnvironmentConfig::default();
/// create_environment("my-env", config)?;
/// ```
/// 
/// # Errors
/// 
/// Returns `VemError::EnvironmentExists` if environment already exists.
pub fn create_environment(name: &str, config: EnvironmentConfig) -> Result<()> {
    // Implementation
}
```

## Testing

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only  
cargo test --test '*'

# Specific test
cargo test test_create_environment

# With coverage
cargo tarpaulin --out html
```

### Writing Tests

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_environment_config() {
        let toml_content = r#"
            name = "test-env"
            description = "Test environment"
            editor = "vim"
        "#;
        
        let config = parse_config(toml_content).unwrap();
        assert_eq!(config.name, "test-env");
        assert_eq!(config.editor, "vim");
    }

    #[test]
    fn test_create_environment_with_temp_dir() {
        let temp_dir = TempDir::new().unwrap();
        let vem_home = temp_dir.path().join(".vem");
        
        let result = create_environment_at_path(&vem_home, "test", &Config::default());
        assert!(result.is_ok());
        assert!(vem_home.join("environments").join("test").exists());
    }
}
```

#### Integration Tests
```rust
// tests/integration/environment_management.rs
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_full_environment_workflow() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create environment
    let output = Command::new("cargo")
        .args(&["run", "--", "create", "test-env"])
        .env("VEM_HOME", temp_dir.path())
        .output()
        .unwrap();
    
    assert!(output.status.success());
    
    // Switch environment
    let output = Command::new("cargo")
        .args(&["run", "--", "switch", "test-env"])
        .env("VEM_HOME", temp_dir.path())
        .output()
        .unwrap();
        
    assert!(output.status.success());
    
    // Verify current environment
    let output = Command::new("cargo")
        .args(&["run", "--", "current"])
        .env("VEM_HOME", temp_dir.path())
        .output()
        .unwrap();
    
    let current = String::from_utf8(output.stdout).unwrap();
    assert!(current.contains("test-env"));
}
```

### Test Data Management

Create reusable test fixtures:

```rust
// tests/common/fixtures.rs
use tempfile::TempDir;
use std::path::PathBuf;

pub struct TestEnvironment {
    pub temp_dir: TempDir,
    pub vem_home: PathBuf,
}

impl TestEnvironment {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let vem_home = temp_dir.path().join(".vem");
        std::fs::create_dir_all(&vem_home).unwrap();
        
        Self { temp_dir, vem_home }
    }
    
    pub fn create_sample_config(&self, name: &str) -> PathBuf {
        let config_content = format!(r#"
            name = "{}"
            description = "Test environment"
            editor = "vim"
            
            [plugin_managers]
            vim-plug = true
        "#, name);
        
        let config_path = self.vem_home.join("environments").join(name).join("vem.toml");
        std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        std::fs::write(&config_path, config_content).unwrap();
        config_path
    }
}
```

## Submitting Changes

### Pull Request Process

1. **Ensure tests pass**:
   ```bash
   make test
   make lint
   ```

2. **Update documentation** if needed

3. **Create descriptive commits**:
   ```bash
   # Good commit messages
   git commit -m "feat: add multi-repository ctags support"
   git commit -m "fix: handle empty configuration files gracefully"
   git commit -m "docs: update installation guide for macOS"
   ```

4. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

5. **Create pull request** on GitHub:
   - Use descriptive title
   - Include detailed description
   - Reference related issues
   - Add screenshots for UI changes

### Commit Message Format

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
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
```bash
feat(ctags): add multi-repository support
fix(config): handle malformed TOML gracefully
docs(api): update environment management examples
refactor(cli): extract command parsing logic
test(integration): add environment switching tests
chore(deps): update dependencies to latest versions
```

### PR Review Process

1. **Automated checks**: CI must pass
2. **Code review**: At least one maintainer approval required
3. **Manual testing**: Reviewers may test changes locally
4. **Documentation**: Ensure docs are updated for user-facing changes

### Review Guidelines

**For contributors:**
- Respond promptly to review feedback
- Keep PRs focused and atomic
- Include tests for new functionality
- Update documentation for API changes

**For reviewers:**
- Be constructive and specific
- Test changes manually when appropriate
- Check for security implications
- Verify backwards compatibility

## Release Process

### Version Numbering

VEM follows [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes (backwards compatible)

### Release Steps

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with release notes
3. **Create release tag**:
   ```bash
   git tag -a v1.2.0 -m "Release v1.2.0"
   git push upstream v1.2.0
   ```
4. **GitHub Actions** automatically builds and publishes releases

### Release Notes Format

```markdown
## [1.2.0] - 2024-01-15

### Added
- Multi-repository ctags support
- AI-enhanced context for development tools
- New sample environments (ai-development, modern-nvim)

### Changed
- Improved configuration validation
- Enhanced error messages

### Fixed
- Environment switching on Windows
- TOML parsing edge cases

### Security
- Updated dependencies with security patches
```

## Community Guidelines

### Code of Conduct

VEM follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Key points:

- **Be respectful** and inclusive
- **Be constructive** in discussions and reviews
- **Focus on technical merit** of contributions
- **Help newcomers** learn and contribute

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and general discussion
- **Pull Requests**: Code contributions and technical discussions

### Getting Help

**For new contributors:**
1. Look for issues labeled `good first issue`
2. Ask questions in GitHub Discussions
3. Review existing code and documentation
4. Start with small improvements

**For development questions:**
1. Check existing documentation
2. Search GitHub Issues and Discussions
3. Create a discussion post with specific questions

### Recognition

Contributors are recognized through:
- **CONTRIBUTORS.md**: Listed alphabetically
- **Release notes**: Feature contributions acknowledged
- **GitHub**: Contributor statistics and graphs

## Development Best Practices

### Performance Considerations

- **Profile before optimizing**: Use `cargo bench` for benchmarking
- **Memory efficiency**: Avoid unnecessary allocations
- **I/O operations**: Use async where appropriate
- **Large datasets**: Consider streaming for file operations

### Security Guidelines

- **Input validation**: Sanitize all external inputs
- **File permissions**: Use appropriate file modes
- **Path traversal**: Validate paths to prevent directory escape
- **Dependencies**: Regularly audit with `cargo audit`

### Debugging

```bash
# Debug builds include more information
cargo build

# Enable debug logging
RUST_LOG=debug cargo run -- command

# Use debugger
rust-gdb target/debug/vem

# Memory debugging
valgrind --tool=memcheck target/debug/vem command
```

### IDE Setup

**VS Code**:
- Install `rust-analyzer` extension
- Configure `settings.json`:
  ```json
  {
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.cargo.features": "all"
  }
  ```

**Vim/Neovim**:
- Use `rust.vim` plugin
- Configure with Language Server Protocol (LSP)

### Documentation

- **Code documentation**: Use rustdoc comments (`///`)
- **User documentation**: Update `docs/src/` for user-facing changes  
- **API documentation**: Generate with `cargo doc --open`
- **Examples**: Include usage examples in documentation

## Thank You

Your contributions help make VEM better for everyone. Whether you're fixing bugs, adding features, improving documentation, or helping other users, every contribution is valued and appreciated!

For questions about contributing, please open a GitHub Discussion or contact the maintainers.
