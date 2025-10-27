# Building from Source

This guide explains how to build VEM from source code, set up the development environment, and contribute to the project.

## Prerequisites

### System Requirements

- **Rust**: 1.70 or higher (nightly toolchain recommended for development)
- **Git**: For source code management
- **Make**: For running build tasks
- **System Dependencies**:
  - `ctags`: Universal Ctags for tag generation
  - `fzf`: Fuzzy finder (for some sample environments)
  - `ripgrep` or `ag`: Fast text search tools

### Platform-Specific Requirements

#### macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies via Homebrew
brew install universal-ctags fzf ripgrep git make

# Install nightly toolchain (optional, for development)
rustup toolchain install nightly
rustup default nightly
```

#### Ubuntu/Debian
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
sudo apt update
sudo apt install -y universal-ctags fzf ripgrep git make build-essential

# Install nightly toolchain (optional)
rustup toolchain install nightly
rustup default nightly
```

#### Windows (WSL recommended)
```bash
# Follow Ubuntu/Debian instructions in WSL
# Or use Windows-specific tools:
# - Install Rust via rustup-init.exe
# - Install dependencies via package managers like Chocolatey
```

## Building VEM

### Quick Build

```bash
# Clone the repository
git clone https://github.com/ryo-arima/vem.git
cd vem

# Build in release mode
cargo build --release

# The binary will be available at target/release/vem
```

### Development Build

```bash
# Clone and setup
git clone https://github.com/ryo-arima/vem.git
cd vem

# Install development dependencies
cargo install cargo-watch cargo-edit cargo-audit

# Build in debug mode (faster compilation)
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- --help
```

## Development Environment Setup

### Using the Makefile

The project includes a Makefile for common development tasks:

```bash
# Show available targets
make help

# Run debug builds of sample environments
make debug-basic      # Test basic Vim environment
make debug-developer  # Test developer Vim environment  
make debug-modern     # Test modern Neovim environment
make debug-ai         # Test AI-enhanced environment

# Run all tests
make test-all

# Setup temporary directories for testing
make setup-temp

# Clean build artifacts
make clean
```

### Code Organization

```
src/
├── main.rs           # Application entry point
├── base.rs           # Base utilities and common code
├── mod.rs            # Module declarations
├── cnf/              # Configuration management
│   └── application.rs
├── ctl/              # Control layer (CLI commands)
│   └── environment.rs
├── ent/              # Entity definitions
│   ├── model/
│   ├── request/
│   └── response/
├── rep/              # Repository layer (data access)
│   └── environment.rs
├── usc/              # Use case layer (business logic)
│   └── environment.rs
└── util/             # Utilities and helper functions
    ├── clone.rs
    ├── debug.rs
    ├── deserialize.rs
    ├── eq.rs
    ├── error.rs
    ├── logger.rs
    ├── mcode.rs
    └── serialize.rs
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test environment

# Run integration tests
cargo test --test integration

# Run tests with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

### Development Workflow

1. **Create Feature Branch**
   ```bash
   git checkout -b feature/new-feature
   ```

2. **Make Changes**
   - Follow the existing code style and patterns
   - Add tests for new functionality
   - Update documentation as needed

3. **Test Changes**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

4. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: add new feature description"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/new-feature
   # Create pull request on GitHub
   ```

## Building Documentation

### mdBook Documentation

```bash
# Install mdBook
cargo install mdbook

# Serve documentation locally
cd docs
mdbook serve

# Build documentation
mdbook build

# Open documentation
open book/index.html
```

### API Documentation

```bash
# Generate API docs
cargo doc --no-deps

# Open API docs
cargo doc --no-deps --open

# Generate docs with private items
cargo doc --no-deps --document-private-items
```

## Cross-Platform Compilation

### Target Platforms

VEM supports multiple platforms:

```bash
# Add compilation targets
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Compile for specific target
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-apple-darwin
```

### Using Cross for Linux Targets

```bash
# Install cross for easier cross-compilation
cargo install cross

# Build for Linux on macOS/Windows
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu
```

## Packaging

### Create Distribution Packages

```bash
# Build release binary
cargo build --release

# Create packages (requires packaging scripts)
./scripts/packages/apt/pack.sh      # Debian/Ubuntu package
./scripts/packages/rpm/pack.sh      # RPM package  
./scripts/packages/brew/pack.sh     # Homebrew formula
```

## Troubleshooting

### Common Build Issues

1. **Rust Version Issues**
   ```bash
   # Update Rust
   rustup update
   
   # Check version
   rustc --version
   ```

2. **Missing System Dependencies**
   ```bash
   # Install ctags
   brew install universal-ctags  # macOS
   apt install universal-ctags   # Ubuntu
   ```

3. **Permission Issues**
   ```bash
   # Fix cargo permissions
   sudo chown -R $USER ~/.cargo
   ```

### Getting Help

- Check the [FAQ](faq.md) for common issues
- Review [troubleshooting guide](troubleshooting.md)
- Open an issue on [GitHub](https://github.com/ryo-arima/vem/issues)
- Join discussions in GitHub Discussions
