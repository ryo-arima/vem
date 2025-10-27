# Testing Guide

This guide covers testing strategies, test execution, and quality assurance for VEM.

## Testing Philosophy

VEM follows a comprehensive testing approach:
- **Unit tests**: Test individual components and functions
- **Integration tests**: Test component interactions
- **End-to-end tests**: Test complete workflows
- **Configuration validation**: Test environment configurations
- **Cross-platform testing**: Ensure compatibility across operating systems

## Test Structure

```
tests/
├── unit/           # Unit tests
│   ├── config/     # Configuration parsing tests
│   ├── ctags/      # Ctags management tests
│   └── environment/# Environment operations tests
├── integration/    # Integration tests
│   ├── commands/   # CLI command tests
│   ├── workflows/  # Multi-step workflow tests
│   └── fixtures/   # Test data and configurations
└── e2e/           # End-to-end tests
    ├── scenarios/  # Real-world usage scenarios
    └── sample_envs/# Test environment configurations
```

## Running Tests

### Quick Test Run
```bash
# Run all tests
make test

# Run with coverage
make test-coverage

# Run specific test categories
cargo test unit
cargo test integration
cargo test e2e
```

### Detailed Test Execution

#### Unit Tests
```bash
# Test configuration parsing
cargo test config::tests

# Test ctags functionality
cargo test ctags::tests

# Test environment management
cargo test environment::tests

# Test with verbose output
cargo test --verbose

# Test specific function
cargo test test_parse_vem_config
```

#### Integration Tests
```bash
# Test command execution
cargo test --test commands

# Test environment workflows
cargo test --test workflows

# Test configuration validation
cargo test --test config_validation
```

#### End-to-End Tests
```bash
# Run E2E tests (requires test environments)
cargo test --test e2e

# Test specific scenarios
cargo test --test e2e test_create_and_switch_environment

# Test cross-platform compatibility
cargo test --test e2e --features cross-platform
```

## Test Environment Setup

### Prerequisites
```bash
# Install test dependencies
sudo apt-get install universal-ctags vim neovim

# Create test workspace
mkdir -p /tmp/vem-test
cd /tmp/vem-test

# Initialize test git repositories
git init test-repo-1
git init test-repo-2
```

### Sample Test Environments
```bash
# Use provided sample environments for testing
cp -r etc/.vem/envs/* /tmp/vem-test/

# Create minimal test environment
vem create test-env --minimal --test-mode

# Test environment switching
vem switch test-env
vem current | grep test-env
```

## Writing Tests

### Unit Test Example
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_vem_config() {
        let config_content = r#"
            name = "test-env"
            description = "Test environment"
            editor = "vim"
            
            [plugin_managers]
            vim-plug = true
            
            [ctags.repositories.main]
            path = "."
        "#;
        
        let config = parse_vem_config(config_content).unwrap();
        assert_eq!(config.name, "test-env");
        assert_eq!(config.editor, "vim");
        assert!(config.plugin_managers.vim_plug);
    }

    #[test]
    fn test_ctags_generation() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().to_str().unwrap();
        
        // Create test source file
        std::fs::write(
            format!("{}/test.rs", repo_path),
            "fn test_function() {}"
        ).unwrap();
        
        let result = generate_ctags(repo_path, &["rust"]).unwrap();
        assert!(result.tags_generated > 0);
    }
}
```

### Integration Test Example
```rust
// tests/integration/commands.rs
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_create_environment_command() {
    let temp_dir = TempDir::new().unwrap();
    let vem_home = temp_dir.path().join(".vem");
    
    let output = Command::new("vem")
        .env("VEM_HOME", vem_home)
        .args(&["create", "test-env", "--template=basic-vim"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    assert!(vem_home.join("environments").join("test-env").exists());
}

#[test]
fn test_environment_switching() {
    // Test environment creation, switching, and validation
    let temp_dir = TempDir::new().unwrap();
    let vem_home = temp_dir.path().join(".vem");
    
    // Create environment
    Command::new("vem")
        .env("VEM_HOME", vem_home)
        .args(&["create", "test1"])
        .output()
        .unwrap();
    
    // Switch to environment
    let output = Command::new("vem")
        .env("VEM_HOME", vem_home)
        .args(&["switch", "test1"])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    
    // Verify current environment
    let current = Command::new("vem")
        .env("VEM_HOME", vem_home)
        .args(&["current"])
        .output()
        .unwrap();
    
    let current_name = String::from_utf8(current.stdout).unwrap();
    assert!(current_name.contains("test1"));
}
```

### End-to-End Test Example
```rust
// tests/e2e/scenarios.rs
#[test]
fn test_developer_workflow() {
    // Simulate complete developer workflow
    
    // 1. Create new environment
    run_command(&["vem", "create", "dev-env", "--template=developer-vim"]);
    
    // 2. Switch to environment
    run_command(&["vem", "switch", "dev-env"]);
    
    // 3. Generate ctags for project
    run_command(&["vem", "generate", "ctags", "main_project"]);
    
    // 4. Verify ctags were created
    let tags_list = run_command(&["vem", "list", "ctags"]);
    assert!(tags_list.contains("main_project"));
    
    // 5. Update environment configuration
    update_vem_config("dev-env", |config| {
        config.ctags.auto_generate = true;
    });
    
    // 6. Verify configuration persistence
    let current_config = get_environment_config("dev-env");
    assert!(current_config.ctags.auto_generate);
}
```

## Configuration Testing

### Valid Configuration Tests
```rust
#[test]
fn test_valid_configurations() {
    let configs = vec![
        "etc/.vem/envs/basic-vim/vem.toml",
        "etc/.vem/envs/developer-vim/vem.toml", 
        "etc/.vem/envs/modern-nvim/vem.toml",
        "etc/.vem/envs/ai-development/vem.toml",
    ];
    
    for config_path in configs {
        let content = std::fs::read_to_string(config_path).unwrap();
        let config = parse_vem_config(&content);
        assert!(config.is_ok(), "Invalid config: {}", config_path);
        
        let config = config.unwrap();
        validate_configuration(&config).unwrap();
    }
}

#[test]
fn test_configuration_schemas() {
    // Test TOML schema validation
    let invalid_configs = vec![
        // Missing required fields
        r#"description = "Missing name""#,
        
        // Invalid plugin manager
        r#"
            name = "test"
            [plugin_managers]
            invalid_manager = true
        "#,
        
        // Invalid ctags configuration
        r#"
            name = "test"
            [ctags.repositories.test]
            # Missing path
        "#,
    ];
    
    for invalid in invalid_configs {
        let result = parse_vem_config(invalid);
        assert!(result.is_err());
    }
}
```

## Performance Testing

### Benchmark Tests
```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_environment_switching(c: &mut Criterion) {
    c.bench_function("switch environment", |b| {
        b.iter(|| {
            switch_environment("test-env")
        })
    });
}

fn benchmark_ctags_generation(c: &mut Criterion) {
    c.bench_function("generate ctags", |b| {
        b.iter(|| {
            generate_ctags("./test-repo", &["rust", "python"])
        })
    });
}

criterion_group!(benches, benchmark_environment_switching, benchmark_ctags_generation);
criterion_main!(benches);
```

### Load Testing
```bash
# Test with multiple environments
for i in {1..50}; do
    vem create "test-env-$i" --template=basic-vim &
done
wait

# Test rapid environment switching
for i in {1..20}; do
    vem switch "test-env-$((RANDOM % 50 + 1))"
done

# Test concurrent ctags generation
for repo in repo1 repo2 repo3; do
    vem generate ctags "$repo" &
done
wait
```

## Cross-Platform Testing

### Platform-Specific Tests
```bash
# Linux-specific tests
make test-linux

# macOS-specific tests  
make test-macos

# Windows-specific tests (if supported)
make test-windows

# Container-based testing
docker run --rm -v $(pwd):/vem ubuntu:latest bash -c "
    cd /vem && 
    apt-get update && 
    apt-get install -y build-essential &&
    make test
"
```

### Compatibility Testing
```rust
#[cfg(target_os = "linux")]
#[test]
fn test_linux_paths() {
    assert_eq!(get_config_dir(), "/home/user/.vem");
}

#[cfg(target_os = "macos")]  
#[test]
fn test_macos_paths() {
    assert_eq!(get_config_dir(), "/Users/user/.vem");
}

#[test]
fn test_cross_platform_compatibility() {
    // Test path handling across platforms
    let path = normalize_path("/some/path");
    assert!(path.is_absolute());
    
    // Test command execution
    let result = execute_command("echo", &["test"]);
    assert!(result.is_ok());
}
```

## Continuous Integration

### GitHub Actions Configuration
```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
        rust: [stable, beta]
        
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y universal-ctags vim
        
    - name: Run tests
      run: make test
      
    - name: Run integration tests
      run: make test-integration
      
    - name: Upload coverage
      uses: codecov/codecov-action@v3
```

### Test Coverage
```bash
# Generate coverage report
cargo tarpaulin --out html

# View coverage
open target/tarpaulin/coverage.html

# Check coverage thresholds
cargo tarpaulin --fail-under 80
```

## Debugging Tests

### Test Debugging
```bash
# Run tests with debug output
RUST_LOG=debug cargo test

# Run single test with debugging
cargo test test_name -- --nocapture

# Debug test with gdb
rust-gdb target/debug/deps/vem-test

# Memory debugging with valgrind
valgrind --tool=memcheck cargo test
```

### Test Data Management
```bash
# Clean test artifacts
make clean-test

# Reset test environments
rm -rf /tmp/vem-test/*
vem init --test-mode

# Generate test fixtures
make generate-fixtures

# Validate test data
make validate-test-data
```

## Quality Assurance

### Code Quality Checks
```bash
# Linting
cargo clippy -- -D warnings

# Formatting
cargo fmt --check

# Security audit
cargo audit

# License compliance
cargo license

# Documentation tests
cargo test --doc
```

### Performance Monitoring
```bash
# Memory usage profiling
heaptrack vem switch environment-name

# CPU profiling
perf record vem generate ctags large-repo
perf report

# I/O monitoring
iotop -p $(pgrep vem)
```

This comprehensive testing guide ensures VEM maintains high quality and reliability across all supported platforms and use cases.
