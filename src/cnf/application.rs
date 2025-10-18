use crate::ent::VemError;
use dirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Default environment to activate on startup
    pub default_environment: Option<String>,
    /// Enable automatic environment switching
    pub auto_switch: bool,
    /// Enable automatic backups
    pub backup_enabled: bool,
    /// Backup retention period in days
    pub backup_retention_days: u32,
    /// Root directory for environments
    pub environment_root: PathBuf,
    /// Symlink mode (symbolic or hard)
    pub symlink_mode: SymlinkMode,
    /// Default editor command
    pub editor: String,
}

/// Symlink mode for environment switching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymlinkMode {
    #[serde(rename = "symbolic")]
    Symbolic,
    #[serde(rename = "hard")]
    Hard,
}

impl Default for AppConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

        Self {
            default_environment: None,
            auto_switch: false,
            backup_enabled: true,
            backup_retention_days: 30,
            environment_root: home_dir.join(".vem").join("environments"),
            symlink_mode: SymlinkMode::Symbolic,
            editor: "vim".to_string(),
        }
    }
}

impl AppConfig {
    /// Get the VEM home directory
    pub fn vem_home() -> PathBuf {
        if let Ok(vem_home) = std::env::var("VEM_HOME") {
            PathBuf::from(vem_home)
        } else {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".vem")
        }
    }

    /// Get the configuration file path
    pub fn config_path() -> PathBuf {
        if let Ok(config_path) = std::env::var("VEM_CONFIG") {
            PathBuf::from(config_path)
        } else {
            Self::vem_home().join("config.json")
        }
    }

    /// Get the current environment symlink path
    pub fn current_link_path() -> PathBuf {
        Self::vem_home().join("current")
    }

    /// Load configuration from file, creating default if not exists
    pub fn load() -> Result<Self, VemError> {
        let config_path = Self::config_path();

        if !config_path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), VemError> {
        let config_path = Self::config_path();

        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), VemError> {
        // Check if environment root is accessible
        if !self.environment_root.exists() {
            std::fs::create_dir_all(&self.environment_root)?;
        }

        // Validate editor command
        if self.editor.is_empty() {
            return Err(VemError::ConfigurationError(
                "Editor command cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}