use std::path::PathBuf;
use crate::util::error::vem_error_t;
use dirs;

// Symlink mode for environment switching
pub enum symlink_mode_t {
    SYMBOLIC,
    HARD,
}

// Trait for application configuration (Go-style interface)
pub trait AppConfig {
    fn load() -> Result<app_config, vem_error_t>;
    fn save(&self) -> Result<(), vem_error_t>;
    fn validate(&self) -> Result<(), vem_error_t>;
    fn environment_root(&self) -> &PathBuf;
    fn get_environment_path(&self, environment_name: &str) -> PathBuf;
    fn get_base_path(&self) -> PathBuf;
    fn default_environment(&self) -> Option<&str>;
    fn editor(&self) -> &str;
}

// Application configuration
pub struct app_config {
    pub default_environment: Option<String>, // Default environment name
    pub auto_switch: bool, // Auto-switch to default environment on startup
    pub backup_enabled: bool, // Enable backups
    pub backup_retention_days: u32, // Days to keep backups
    pub environment_root: PathBuf, // Root directory for environments
    pub symlink_mode: symlink_mode_t, // Symlink mode for environment switching
    pub editor: String, // Default editor
}

pub fn new_app_config() -> app_config {
    let vem_home = get_vem_home();
    app_config {
        default_environment: None,
        auto_switch: false,
        backup_enabled: true,
        backup_retention_days: 30,
        environment_root: vem_home.join("environments"),
        symlink_mode: symlink_mode_t::SYMBOLIC,
        editor: "vim".to_string(),
    }
}

pub fn get_vem_home() -> PathBuf {
    if let Ok(vem_home_env) = std::env::var("VEM_HOME") {
        PathBuf::from(vem_home_env)
    } else {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".vem")
    }
}

// Get the configuration file path
pub fn config_path() -> PathBuf {
    if let Ok(config_path) = std::env::var("VEM_CONFIG") {
        PathBuf::from(config_path)
    } else {
        get_vem_home().join("config.toml")
    }
}

// Get the current environment symlink path
pub fn current_link_path() -> PathBuf {
    get_vem_home().join("current")
}

impl app_config {
    // Load configuration from file, creating default if not exists
    pub fn load() -> Result<Self, vem_error_t> {
        let config_path = config_path();

        if !config_path.exists() {
            let config = new_app_config();
            config.save()?;
            return Ok(config);
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: Self = toml::from_str(&content).map_err(|e| {
            vem_error_t::ConfigurationError(format!("Failed to parse config: {}", e))
        })?;
        Ok(config)
    }

    // Save configuration to file
    pub fn save(&self) -> Result<(), vem_error_t> {
        let config_path = config_path();

        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self).map_err(|e| {
            vem_error_t::ConfigurationError(format!("Failed to serialize config: {}", e))
        })?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    // Validate configuration
    pub fn validate(&self) -> Result<(), vem_error_t> {
        // Check if environment root is accessible
        if !self.environment_root.exists() {
            std::fs::create_dir_all(&self.environment_root)?;
        }

        // Validate editor command
        if self.editor.is_empty() {
            return Err(vem_error_t::ConfigurationError(
                "Editor command cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    pub fn environment_root(&self) -> &PathBuf {
        &self.environment_root
    }

    pub fn get_environment_path(&self, environment_name: &str) -> PathBuf {
        self.environment_root.join(environment_name)
    }

    pub fn get_base_path(&self) -> PathBuf {
        get_vem_home()
    }

    #[allow(dead_code)]
    pub fn default_environment(&self) -> Option<&str> {
        self.default_environment.as_deref()
    }

    #[allow(dead_code)]
    pub fn editor(&self) -> &str {
        &self.editor
    }
}
