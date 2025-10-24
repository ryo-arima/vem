#![allow(non_camel_case_types)]

use crate::ent::model::neovim::{NeovimFeature, PluginManager, NEOVIM_CONFIG};
use crate::rep::RepositoryConfig;
use crate::util::error::vem_error_t;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;

/// Neovim repository with Go-style struct embedding
pub struct neovim_repository {
    base: RepositoryConfig,
}

impl Deref for neovim_repository {
    type Target = RepositoryConfig;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

pub trait NeovimRepository {
    fn initialize(&mut self, environment_name: &str) -> Result<(), vem_error_t>;
    fn enable_neovim(&mut self, environment_name: &str) -> Result<(), vem_error_t>;
    fn disable_neovim(&mut self, environment_name: &str) -> Result<(), vem_error_t>;
    fn set_plugin_manager(
        &mut self,
        environment_name: &str,
        plugin_manager: PluginManager,
    ) -> Result<(), vem_error_t>;
    fn add_feature(
        &mut self,
        environment_name: &str,
        feature: NeovimFeature,
    ) -> Result<(), vem_error_t>;
    fn remove_feature(
        &mut self,
        environment_name: &str,
        feature_name: &str,
    ) -> Result<(), vem_error_t>;
    fn toggle_feature(
        &mut self,
        environment_name: &str,
        feature_name: &str,
    ) -> Result<(), vem_error_t>;
    fn check_neovim_installed(&self) -> Result<bool, vem_error_t>;
    fn get_neovim_version(&self) -> Result<Option<String>, vem_error_t>;
    fn list_features(&self, environment_name: &str) -> Result<Vec<NeovimFeature>, vem_error_t>;
    fn get_config(&self, environment_name: &str) -> Result<NEOVIM_CONFIG, vem_error_t>;
    fn save_config(&self, config: &NEOVIM_CONFIG) -> Result<(), vem_error_t>;
    fn generate_init_vim(&self, environment_name: &str) -> Result<String, vem_error_t>;
    fn generate_init_lua(&self, environment_name: &str) -> Result<String, vem_error_t>;
    fn generate_plugin_config(&self, environment_name: &str) -> Result<String, vem_error_t>;
}

impl neovim_repository {
    pub fn new() -> Self {
        Self {
            base: RepositoryConfig::new(),
        }
    }

    fn get_neovim_config_path(&self, environment_name: &str) -> PathBuf {
        self.base
            .app_config
            .get_environment_path(environment_name)
            .join("neovim_config.toml")
    }

    fn get_nvim_config_dir(&self, environment_name: &str) -> PathBuf {
        self.base
            .app_config
            .get_environment_path(environment_name)
            .join("nvim")
    }
}

impl NeovimRepository for neovim_repository {
    fn initialize(&mut self, environment_name: &str) -> Result<(), vem_error_t> {
        let nvim_installed = self.check_neovim_installed().unwrap_or(false);
        let version = if nvim_installed {
            self.get_neovim_version().unwrap_or(None)
        } else {
            None
        };

        let config = NEOVIM_CONFIG {
            environment_name: environment_name.to_string(),
            enabled: nvim_installed,
            version,
            lua_config_enabled: true,
            init_vim_path: None,
            init_lua_path: None,
            plugin_manager: None,
            features: Vec::new(),
            global_settings: std::collections::HashMap::new(),
        };

        self.save_config(&config)?;

        // Create nvim config directory
        let nvim_dir = self.get_nvim_config_dir(environment_name);
        fs::create_dir_all(&nvim_dir).map_err(vem_error_t::FileSystemError)?;
        fs::create_dir_all(nvim_dir.join("lua")).map_err(vem_error_t::FileSystemError)?;

        Ok(())
    }

    fn enable_neovim(&mut self, environment_name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        config.enabled = true;
        self.save_config(&config)?;
        Ok(())
    }

    fn disable_neovim(&mut self, environment_name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        config.enabled = false;
        self.save_config(&config)?;
        Ok(())
    }

    fn set_plugin_manager(
        &mut self,
        environment_name: &str,
        plugin_manager: PluginManager,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        config.plugin_manager = Some(plugin_manager);
        self.save_config(&config)?;
        Ok(())
    }

    fn add_feature(
        &mut self,
        environment_name: &str,
        feature: NeovimFeature,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        // Check if feature already exists
        if config.features.iter().any(|f| f.name == feature.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Feature '{}' already exists",
                feature.name
            )));
        }

        config.features.push(feature);
        self.save_config(&config)?;

        Ok(())
    }

    fn remove_feature(
        &mut self,
        environment_name: &str,
        feature_name: &str,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        let original_len = config.features.len();
        config.features.retain(|f| f.name != feature_name);

        if config.features.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Feature '{}' not found",
                feature_name
            )));
        }

        self.save_config(&config)?;
        Ok(())
    }

    fn toggle_feature(
        &mut self,
        environment_name: &str,
        feature_name: &str,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        let feature = config
            .features
            .iter_mut()
            .find(|f| f.name == feature_name)
            .ok_or_else(|| {
                vem_error_t::EnvironmentNotFound(format!("Feature '{}' not found", feature_name))
            })?;

        feature.enabled = !feature.enabled;
        self.save_config(&config)?;

        Ok(())
    }

    fn check_neovim_installed(&self) -> Result<bool, vem_error_t> {
        match Command::new("nvim").arg("--version").output() {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
    }

    fn get_neovim_version(&self) -> Result<Option<String>, vem_error_t> {
        match Command::new("nvim").arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    let version_output = String::from_utf8_lossy(&output.stdout);
                    // Parse version from first line: "NVIM v0.9.5"
                    if let Some(first_line) = version_output.lines().next() {
                        if let Some(version) = first_line.split_whitespace().nth(1) {
                            return Ok(Some(version.to_string()));
                        }
                    }
                }
                Ok(None)
            }
            Err(_) => Ok(None),
        }
    }

    fn list_features(&self, environment_name: &str) -> Result<Vec<NeovimFeature>, vem_error_t> {
        let config = self.get_config(environment_name)?;
        Ok(config.features)
    }

    fn get_config(&self, environment_name: &str) -> Result<NEOVIM_CONFIG, vem_error_t> {
        let config_path = self.get_neovim_config_path(environment_name);

        if !config_path.exists() {
            return Ok(NEOVIM_CONFIG {
                environment_name: environment_name.to_string(),
                enabled: false,
                version: None,
                lua_config_enabled: true,
                init_vim_path: None,
                init_lua_path: None,
                plugin_manager: None,
                features: Vec::new(),
                global_settings: std::collections::HashMap::new(),
            });
        }

        let content = fs::read_to_string(&config_path).map_err(vem_error_t::FileSystemError)?;

        toml::from_str(&content).map_err(|e| {
            vem_error_t::SerializationError(format!("Failed to parse Neovim config: {}", e))
        })
    }

    fn save_config(&self, config: &NEOVIM_CONFIG) -> Result<(), vem_error_t> {
        let config_path = self.get_neovim_config_path(&config.environment_name);

        let content = toml::to_string_pretty(config).map_err(|e| {
            vem_error_t::SerializationError(format!("Failed to serialize Neovim config: {}", e))
        })?;

        fs::write(&config_path, content).map_err(vem_error_t::FileSystemError)?;

        Ok(())
    }

    fn generate_init_vim(&self, environment_name: &str) -> Result<String, vem_error_t> {
        let config = self.get_config(environment_name)?;
        let mut vim_config = String::new();

        vim_config.push_str("\" Neovim Configuration\n");
        vim_config.push_str("\" Generated by VEM\n\n");

        // Basic settings
        vim_config.push_str("\" Basic Settings\n");
        vim_config.push_str("set number\n");
        vim_config.push_str("set relativenumber\n");
        vim_config.push_str("set expandtab\n");
        vim_config.push_str("set tabstop=2\n");
        vim_config.push_str("set shiftwidth=2\n");
        vim_config.push_str("set smartindent\n");
        vim_config.push_str("set termguicolors\n\n");

        // Plugin manager configuration
        if let Some(pm) = &config.plugin_manager {
            vim_config.push_str(&format!("\" Plugin Manager: {}\n", pm.display_name()));
            
            match pm {
                PluginManager::VimPlug => {
                    vim_config.push_str("call plug#begin('~/.local/share/nvim/plugged')\n\n");
                    
                    // Add enabled features
                    for feature in config.features.iter().filter(|f| f.enabled) {
                        vim_config.push_str(&format!("Plug '{}'\n", feature.feature_type.plugin_name()));
                        
                        // Add dependencies
                        for dep in feature.feature_type.requires_dependencies() {
                            vim_config.push_str(&format!("Plug '{}'\n", dep));
                        }
                    }
                    
                    vim_config.push_str("\ncall plug#end()\n\n");
                }
                _ => {
                    vim_config.push_str("\" Please configure your plugin manager manually\n\n");
                }
            }
        }

        // Feature configurations
        vim_config.push_str("\" Feature Configurations\n");
        for feature in config.features.iter().filter(|f| f.enabled) {
            vim_config.push_str(&format!("\" {}\n", feature.name));
            if !feature.config.is_empty() {
                for (key, value) in &feature.config {
                    vim_config.push_str(&format!("let g:{}_{} = {}\n", 
                        feature.name.to_lowercase().replace(' ', "_"),
                        key,
                        value
                    ));
                }
            }
            vim_config.push('\n');
        }

        Ok(vim_config)
    }

    fn generate_init_lua(&self, environment_name: &str) -> Result<String, vem_error_t> {
        let config = self.get_config(environment_name)?;
        let mut lua_config = String::new();

        lua_config.push_str("-- Neovim Configuration (Lua)\n");
        lua_config.push_str("-- Generated by VEM\n\n");

        // Basic settings
        lua_config.push_str("-- Basic Settings\n");
        lua_config.push_str("vim.opt.number = true\n");
        lua_config.push_str("vim.opt.relativenumber = true\n");
        lua_config.push_str("vim.opt.expandtab = true\n");
        lua_config.push_str("vim.opt.tabstop = 2\n");
        lua_config.push_str("vim.opt.shiftwidth = 2\n");
        lua_config.push_str("vim.opt.smartindent = true\n");
        lua_config.push_str("vim.opt.termguicolors = true\n\n");

        // Plugin manager configuration
        if let Some(pm) = &config.plugin_manager {
            if pm.is_lua_based() {
                lua_config.push_str(&format!("-- Plugin Manager: {}\n", pm.display_name()));
                
                match pm {
                    PluginManager::Packer => {
                        lua_config.push_str("require('packer').startup(function(use)\n");
                        lua_config.push_str("  use 'wbthomason/packer.nvim'\n\n");
                        
                        for feature in config.features.iter().filter(|f| f.enabled) {
                            lua_config.push_str(&format!("  use '{}'\n", feature.feature_type.plugin_name()));
                            
                            for dep in feature.feature_type.requires_dependencies() {
                                lua_config.push_str(&format!("  use '{}'\n", dep));
                            }
                        }
                        
                        lua_config.push_str("end)\n\n");
                    }
                    PluginManager::Lazy => {
                        lua_config.push_str("local lazypath = vim.fn.stdpath('data') .. '/lazy/lazy.nvim'\n");
                        lua_config.push_str("require('lazy').setup({\n");
                        
                        for feature in config.features.iter().filter(|f| f.enabled) {
                            lua_config.push_str(&format!("  '{}',\n", feature.feature_type.plugin_name()));
                        }
                        
                        lua_config.push_str("})\n\n");
                    }
                    _ => {}
                }
            }
        }

        // Feature configurations
        lua_config.push_str("-- Feature Configurations\n");
        for feature in config.features.iter().filter(|f| f.enabled) {
            lua_config.push_str(&format!("-- {}\n", feature.name));
            lua_config.push('\n');
        }

        Ok(lua_config)
    }

    fn generate_plugin_config(&self, environment_name: &str) -> Result<String, vem_error_t> {
        let config = self.get_config(environment_name)?;
        let mut plugin_config = String::new();

        plugin_config.push_str("# Neovim Plugins\n");
        plugin_config.push_str("# Generated by VEM\n\n");

        if let Some(pm) = &config.plugin_manager {
            plugin_config.push_str(&format!("Plugin Manager: {}\n\n", pm.display_name()));
        }

        plugin_config.push_str("## Enabled Features:\n");
        for feature in config.features.iter().filter(|f| f.enabled) {
            plugin_config.push_str(&format!(
                "- {} ({}): {}\n",
                feature.name,
                feature.feature_type.plugin_name(),
                feature.feature_type.description()
            ));
        }

        Ok(plugin_config)
    }
}

impl Default for neovim_repository {
    fn default() -> Self {
        Self::new()
    }
}
