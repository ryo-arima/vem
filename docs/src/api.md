# API Documentation

VEM provides a comprehensive internal API for managing Vim environments, ctags, and configurations. This document describes the core APIs and their usage.

## Core APIs

### Environment Management API

#### Create Environment
```rust
pub fn create_environment(name: &str, template: Option<&str>) -> Result<Environment, VemError>
```

Creates a new Vim environment with the specified name and optional template.

**Parameters:**
- `name`: Environment name (must be unique)
- `template`: Optional template name (basic-vim, developer-vim, modern-nvim, ai-development)

**Returns:** Environment instance or error

#### Switch Environment  
```rust
pub fn switch_environment(name: &str) -> Result<(), VemError>
```

Switches to the specified environment by updating symlinks and configurations.

#### List Environments
```rust
pub fn list_environments() -> Result<Vec<Environment>, VemError>
```

Returns a list of all available environments.

### Ctags Management API

#### Generate Ctags
```rust
pub fn generate_ctags(
    repository: &str, 
    options: CtagsOptions
) -> Result<CtagsResult, VemError>
```

Generates ctags for the specified repository with given options.

**Parameters:**
- `repository`: Repository name from vem.toml configuration
- `options`: Ctags generation options (languages, exclude patterns, etc.)

#### Update Ctags
```rust
pub fn update_ctags(
    repository: &str,
    incremental: bool
) -> Result<CtagsResult, VemError>
```

Updates existing ctags for the repository.

#### List Ctags
```rust
pub fn list_ctags(format: ListFormat) -> Result<Vec<CtagsInfo>, VemError>
```

Lists all ctags with their status and metadata.

### Configuration API

#### Load Configuration
```rust
pub fn load_vem_config(path: &Path) -> Result<VemConfig, VemError>
```

Loads and parses a vem.toml configuration file.

#### Save Configuration
```rust
pub fn save_vem_config(config: &VemConfig, path: &Path) -> Result<(), VemError>
```

Saves configuration to a vem.toml file.

## Data Structures

### Environment
```rust
pub struct Environment {
    pub name: String,
    pub description: String,
    pub env_type: EnvironmentType,
    pub config_path: PathBuf,
    pub active: bool,
}
```

### CtagsInfo
```rust
pub struct CtagsInfo {
    pub name: String,
    pub repository: String,
    pub tag_file: PathBuf,
    pub last_updated: DateTime<Utc>,
    pub size: u64,
    pub status: CtagsStatus,
}
```

### VemConfig
```rust
pub struct VemConfig {
    pub environment: EnvironmentConfig,
    pub plugins: PluginConfig,
    pub ctags: CtagsConfig,
    pub theme: ThemeConfig,
}
```

## Error Handling

All APIs use the `VemError` type for consistent error handling:

```rust
pub enum VemError {
    IoError(std::io::Error),
    ConfigError(String),
    EnvironmentNotFound(String),
    CtagsError(String),
    ValidationError(String),
}
```

## Usage Examples

### Creating and Switching Environments

```rust
use vem::api::{create_environment, switch_environment};

// Create a new development environment
let env = create_environment("my-dev", Some("developer-vim"))?;
println!("Created environment: {}", env.name);

// Switch to the new environment
switch_environment("my-dev")?;
println!("Switched to my-dev environment");
```

### Managing Ctags

```rust
use vem::api::{generate_ctags, list_ctags, CtagsOptions, ListFormat};

// Generate ctags for main project
let options = CtagsOptions {
    languages: vec!["rust".to_string(), "python".to_string()],
    exclude_patterns: vec![".git".to_string(), "target".to_string()],
    ..Default::default()
};

generate_ctags("main_project", options)?;

// List all ctags
let ctags_list = list_ctags(ListFormat::Table)?;
for ctag in ctags_list {
    println!("{}: {} ({})", ctag.name, ctag.repository, ctag.status);
}
```

## Integration Notes

- All APIs are designed to be thread-safe
- Configuration changes are atomic where possible
- File operations include proper error handling and rollback
- APIs support both synchronous and asynchronous operations where appropriate
