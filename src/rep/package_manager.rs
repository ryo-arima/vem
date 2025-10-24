#![allow(non_camel_case_types)]

use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cnf::application::app_config;
use crate::util::error::vem_error_t;
use crate::ent::model::package_manager::{PACKAGE_MANAGER_CONFIG, VimPackageManager, PluginInfo};

use super::environment::RepositoryConfig;

/// Package manager repository trait
pub trait PackageManagerRepository {
    /// Initialize a package manager for an environment
    fn initialize(&self, env_name: &str, package_manager: VimPackageManager) -> Result<PACKAGE_MANAGER_CONFIG, vem_error_t>;
    
    /// Install the package manager itself
    fn install_manager(&self, env_name: &str) -> Result<(), vem_error_t>;
    
    /// Install plugins using the package manager
    fn install_plugins(&self, env_name: &str) -> Result<(), vem_error_t>;
    
    /// Update plugins
    fn update_plugins(&self, env_name: &str) -> Result<(), vem_error_t>;
    
    /// Clean unused plugins
    fn clean_plugins(&self, env_name: &str) -> Result<(), vem_error_t>;
    
    /// Get package manager configuration
    fn get_config(&self, env_name: &str) -> Result<PACKAGE_MANAGER_CONFIG, vem_error_t>;
    
    /// Save package manager configuration
    fn save_config(&self, env_name: &str, config: &PACKAGE_MANAGER_CONFIG) -> Result<(), vem_error_t>;
    
    /// List installed plugins
    fn list_plugins(&self, env_name: &str) -> Result<Vec<PluginInfo>, vem_error_t>;
}

/// Package manager repository implementation with embedded config (Go-style)
pub struct package_manager_repository {
    base: RepositoryConfig,
}

impl package_manager_repository {
    pub fn new(config: app_config) -> Self {
        Self {
            base: RepositoryConfig::new(config),
        }
    }

    /// Get config file path
    fn get_config_file_path(&self, env_name: &str) -> PathBuf {
        self.config()
            .environment_root()
            .join(env_name)
            .join("package_manager.toml")
    }

    /// Get vim config directory path
    fn get_vim_config_dir(&self, env_name: &str) -> PathBuf {
        self.config()
            .environment_root()
            .join(env_name)
            .join(".vim")
    }

    /// Get package manager install path
    fn get_install_path(&self, env_name: &str, package_manager: &VimPackageManager) -> PathBuf {
        self.get_vim_config_dir(env_name)
            .join(package_manager.install_path())
    }

    /// Clone repository using git
    fn clone_repository(&self, url: &str, dest: &Path) -> Result<(), vem_error_t> {
        if dest.exists() {
            return Ok(());
        }

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }

        let output = Command::new("git")
            .args(["clone", "--depth", "1", url, dest.to_str().unwrap()])
            .output()
            .map_err(|e| vem_error_t::CommandExecutionError(format!("Failed to execute git: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(vem_error_t::CommandExecutionError(
                format!("Git clone failed: {}", stderr)
            ));
        }

        Ok(())
    }

    /// Execute vim/nvim command
    fn execute_vim_command(&self, env_name: &str, command: Vec<String>) -> Result<(), vem_error_t> {
        if command.is_empty() {
            return Ok(());
        }

        let vim_dir = self.get_vim_config_dir(env_name);
        
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

        Ok(())
    }
}

impl Deref for package_manager_repository {
    type Target = RepositoryConfig;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl PackageManagerRepository for package_manager_repository {
    fn initialize(&self, env_name: &str, package_manager: VimPackageManager) -> Result<PACKAGE_MANAGER_CONFIG, vem_error_t> {
        let install_path = self.get_install_path(env_name, &package_manager);
        
        let config = PACKAGE_MANAGER_CONFIG {
            environment_name: env_name.to_string(),
            package_manager,
            installed: false,
            install_path,
            plugins: Vec::new(),
        };

        self.save_config(env_name, &config)?;
        Ok(config)
    }

    fn install_manager(&self, env_name: &str) -> Result<(), vem_error_t> {
        let config = self.get_config(env_name)?;
        
        if config.installed {
            return Ok(());
        }

        let repo_url = config.package_manager.repository_url();
        if repo_url.is_empty() {
            // Native package manager doesn't need installation
            let mut updated_config = config;
            updated_config.installed = true;
            self.save_config(env_name, &updated_config)?;
            return Ok(());
        }

        // Clone the package manager repository
        self.clone_repository(repo_url, &config.install_path)?;

        // Mark as installed
        let mut updated_config = config;
        updated_config.installed = true;
        self.save_config(env_name, &updated_config)?;

        Ok(())
    }

    fn install_plugins(&self, env_name: &str) -> Result<(), vem_error_t> {
        let config = self.get_config(env_name)?;
        
        if !config.installed {
            return Err(vem_error_t::ConfigurationError(
                "Package manager not installed".to_string()
            ));
        }

        let command = config.package_manager.install_command();
        self.execute_vim_command(env_name, command)?;

        Ok(())
    }

    fn update_plugins(&self, env_name: &str) -> Result<(), vem_error_t> {
        let config = self.get_config(env_name)?;
        
        if !config.installed {
            return Err(vem_error_t::ConfigurationError(
                "Package manager not installed".to_string()
            ));
        }

        let command = config.package_manager.update_command();
        self.execute_vim_command(env_name, command)?;

        Ok(())
    }

    fn clean_plugins(&self, env_name: &str) -> Result<(), vem_error_t> {
        let config = self.get_config(env_name)?;
        
        if !config.installed {
            return Err(vem_error_t::ConfigurationError(
                "Package manager not installed".to_string()
            ));
        }

        let command = config.package_manager.clean_command();
        self.execute_vim_command(env_name, command)?;

        Ok(())
    }

    fn get_config(&self, env_name: &str) -> Result<PACKAGE_MANAGER_CONFIG, vem_error_t> {
        let config_file = self.get_config_file_path(env_name);
        
        if !config_file.exists() {
            return Ok(PACKAGE_MANAGER_CONFIG {
                environment_name: env_name.to_string(),
                ..Default::default()
            });
        }

        let content = fs::read_to_string(&config_file)?;
        let config: PACKAGE_MANAGER_CONFIG = toml::from_str(&content)
            .map_err(|e| vem_error_t::SerializationError(
                format!("Failed to parse package manager config: {}", e)
            ))?;

        Ok(config)
    }

    fn save_config(&self, env_name: &str, config: &PACKAGE_MANAGER_CONFIG) -> Result<(), vem_error_t> {
        let config_file = self.get_config_file_path(env_name);
        
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(config)
            .map_err(|e| vem_error_t::SerializationError(
                format!("Failed to serialize package manager config: {}", e)
            ))?;

        fs::write(&config_file, content)?;

        Ok(())
    }

    fn list_plugins(&self, env_name: &str) -> Result<Vec<PluginInfo>, vem_error_t> {
        let config = self.get_config(env_name)?;
        Ok(config.plugins)
    }
}

/// Factory function to create package manager repository
pub fn new(config: app_config) -> impl PackageManagerRepository {
    package_manager_repository::new(config)
}
