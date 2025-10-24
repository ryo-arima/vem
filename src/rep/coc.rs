#![allow(non_camel_case_types)]

use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;

use crate::cnf::application::app_config;
use crate::util::error::vem_error_t;
use crate::ent::model::coc::{COC_CONFIG, CocExtension, CocExtensionType};

use super::environment::RepositoryConfig;

/// coc.nvim repository trait
pub trait CocRepository {
    /// Initialize coc.nvim for an environment
    fn initialize(&self, env_name: &str) -> Result<COC_CONFIG, vem_error_t>;
    
    /// Install coc.nvim plugin
    fn install_coc(&self, env_name: &str) -> Result<(), vem_error_t>;
    
    /// Install coc extensions
    fn install_extensions(&self, env_name: &str, extensions: Vec<CocExtensionType>) -> Result<(), vem_error_t>;
    
    /// Uninstall coc extensions
    fn uninstall_extensions(&self, env_name: &str, extensions: Vec<String>) -> Result<(), vem_error_t>;
    
    /// Update coc extensions
    fn update_extensions(&self, env_name: &str) -> Result<(), vem_error_t>;
    
    /// List installed extensions
    fn list_extensions(&self, env_name: &str) -> Result<Vec<CocExtension>, vem_error_t>;
    
    /// Get coc.nvim configuration
    fn get_config(&self, env_name: &str) -> Result<COC_CONFIG, vem_error_t>;
    
    /// Save coc.nvim configuration
    fn save_config(&self, env_name: &str, config: &COC_CONFIG) -> Result<(), vem_error_t>;
    
    /// Update coc-settings.json
    fn update_settings(&self, env_name: &str, settings: serde_json::Value) -> Result<(), vem_error_t>;
}

/// coc.nvim repository implementation with embedded config (Go-style)
pub struct coc_repository {
    base: RepositoryConfig,
}

impl coc_repository {
    pub fn new(config: app_config) -> Self {
        Self {
            base: RepositoryConfig::new(config),
        }
    }

    /// Get coc config file path
    fn get_config_file_path(&self, env_name: &str) -> PathBuf {
        self.config()
            .environment_root()
            .join(env_name)
            .join("coc_config.toml")
    }

    /// Get coc-settings.json path
    fn get_coc_settings_path(&self, env_name: &str) -> PathBuf {
        self.config()
            .environment_root()
            .join(env_name)
            .join(".vim")
            .join("coc-settings.json")
    }

    /// Get coc data directory path
    fn get_coc_data_dir(&self, env_name: &str) -> PathBuf {
        self.config()
            .environment_root()
            .join(env_name)
            .join(".config")
            .join("coc")
    }

    /// Execute vim command with coc
    fn execute_vim_command(&self, env_name: &str, command: Vec<String>) -> Result<String, vem_error_t> {
        if command.is_empty() {
            return Ok(String::new());
        }

        let vim_dir = self.config()
            .environment_root()
            .join(env_name)
            .join(".vim");
        
        let output = Command::new(&command[0])
            .args(&command[1..])
            .env("HOME", self.config().environment_root().join(env_name))
            .env("VIMINIT", format!("source {}", vim_dir.join("vimrc").display()))
            .output()
            .map_err(|e| vem_error_t::CommandExecutionError(
                format!("Failed to execute {}: {}", command[0], e)
            ))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(vem_error_t::CommandExecutionError(
                format!("Command failed: {}", stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl Deref for coc_repository {
    type Target = RepositoryConfig;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl CocRepository for coc_repository {
    fn initialize(&self, env_name: &str) -> Result<COC_CONFIG, vem_error_t> {
        let config_path = self.get_coc_settings_path(env_name);
        
        let config = COC_CONFIG {
            environment_name: env_name.to_string(),
            installed: false,
            extensions: Vec::new(),
            settings: std::collections::HashMap::new(),
            config_path,
        };

        self.save_config(env_name, &config)?;
        Ok(config)
    }

    fn install_coc(&self, env_name: &str) -> Result<(), vem_error_t> {
        let config = self.get_config(env_name)?;
        
        if config.installed {
            return Ok(());
        }

        // Create coc data directory
        let coc_data_dir = self.get_coc_data_dir(env_name);
        fs::create_dir_all(&coc_data_dir)?;

        // Install coc.nvim via vim-plug or other package manager
        // This assumes coc.nvim is already in the vimrc
        // The actual installation happens through the package manager

        // Mark as installed
        let mut updated_config = config;
        updated_config.installed = true;
        self.save_config(env_name, &updated_config)?;

        Ok(())
    }

    fn install_extensions(&self, env_name: &str, extensions: Vec<CocExtensionType>) -> Result<(), vem_error_t> {
        let mut config = self.get_config(env_name)?;
        
        if !config.installed {
            return Err(vem_error_t::ConfigurationError(
                "coc.nvim not installed".to_string()
            ));
        }

        // Build extension list
        let extension_names: Vec<String> = extensions
            .iter()
            .map(|e| e.package_name())
            .collect();

        // Install extensions using CocInstall
        let command = vec![
            "nvim".to_string(),
            "--headless".to_string(),
            "-c".to_string(),
            format!("CocInstall -sync {}", extension_names.join(" ")),
            "-c".to_string(),
            "qa".to_string(),
        ];

        self.execute_vim_command(env_name, command)?;

        // Update config with new extensions
        for ext_type in extensions {
            let ext = CocExtension::from_extension_type(ext_type);
            if !config.extensions.iter().any(|e| e.name == ext.name) {
                config.extensions.push(ext);
            }
        }

        self.save_config(env_name, &config)?;

        Ok(())
    }

    fn uninstall_extensions(&self, env_name: &str, extensions: Vec<String>) -> Result<(), vem_error_t> {
        let mut config = self.get_config(env_name)?;
        
        if !config.installed {
            return Err(vem_error_t::ConfigurationError(
                "coc.nvim not installed".to_string()
            ));
        }

        // Uninstall extensions using CocUninstall
        let command = vec![
            "nvim".to_string(),
            "--headless".to_string(),
            "-c".to_string(),
            format!("CocUninstall {}", extensions.join(" ")),
            "-c".to_string(),
            "qa".to_string(),
        ];

        self.execute_vim_command(env_name, command)?;

        // Update config
        config.extensions.retain(|e| !extensions.contains(&e.name));
        self.save_config(env_name, &config)?;

        Ok(())
    }

    fn update_extensions(&self, env_name: &str) -> Result<(), vem_error_t> {
        let config = self.get_config(env_name)?;
        
        if !config.installed {
            return Err(vem_error_t::ConfigurationError(
                "coc.nvim not installed".to_string()
            ));
        }

        // Update all extensions using CocUpdate
        let command = vec![
            "nvim".to_string(),
            "--headless".to_string(),
            "-c".to_string(),
            "CocUpdate".to_string(),
            "-c".to_string(),
            "qa".to_string(),
        ];

        self.execute_vim_command(env_name, command)?;

        Ok(())
    }

    fn list_extensions(&self, env_name: &str) -> Result<Vec<CocExtension>, vem_error_t> {
        let config = self.get_config(env_name)?;
        
        if !config.installed {
            return Ok(Vec::new());
        }

        // List extensions using CocList
        let command = vec![
            "nvim".to_string(),
            "--headless".to_string(),
            "-c".to_string(),
            "CocList extensions".to_string(),
            "-c".to_string(),
            "qa".to_string(),
        ];

        let _output = self.execute_vim_command(env_name, command)?;
        
        // Parse output and update config
        // This is a simplified version - actual parsing would be more complex
        Ok(config.extensions)
    }

    fn get_config(&self, env_name: &str) -> Result<COC_CONFIG, vem_error_t> {
        let config_file = self.get_config_file_path(env_name);
        
        if !config_file.exists() {
            return Ok(COC_CONFIG {
                environment_name: env_name.to_string(),
                ..Default::default()
            });
        }

        let content = fs::read_to_string(&config_file)?;
        let config: COC_CONFIG = toml::from_str(&content)
            .map_err(|e| vem_error_t::SerializationError(
                format!("Failed to parse coc config: {}", e)
            ))?;

        Ok(config)
    }

    fn save_config(&self, env_name: &str, config: &COC_CONFIG) -> Result<(), vem_error_t> {
        let config_file = self.get_config_file_path(env_name);
        
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(config)
            .map_err(|e| vem_error_t::SerializationError(
                format!("Failed to serialize coc config: {}", e)
            ))?;

        fs::write(&config_file, content)?;

        Ok(())
    }

    fn update_settings(&self, env_name: &str, settings: serde_json::Value) -> Result<(), vem_error_t> {
        let settings_path = self.get_coc_settings_path(env_name);
        
        if let Some(parent) = settings_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&settings)
            .map_err(|e| vem_error_t::SerializationError(
                format!("Failed to serialize coc settings: {}", e)
            ))?;

        fs::write(&settings_path, content)?;

        Ok(())
    }
}

/// Factory function to create coc repository
pub fn new(config: app_config) -> impl CocRepository {
    coc_repository::new(config)
}
