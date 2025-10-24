#![allow(non_camel_case_types)]

use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;

use crate::cnf::application::app_config;
use crate::util::error::vem_error_t;
use crate::ent::model::lsp::{LSP_CONFIG, LanguageServer, LanguageServerType};

use super::environment::RepositoryConfig;

/// LSP repository trait
pub trait LspRepository {
    /// Initialize LSP configuration for an environment
    fn initialize(&self, env_name: &str) -> Result<LSP_CONFIG, vem_error_t>;
    
    /// Add a language server
    fn add_server(&self, env_name: &str, server_type: LanguageServerType) -> Result<(), vem_error_t>;
    
    /// Remove a language server
    fn remove_server(&self, env_name: &str, server_name: &str) -> Result<(), vem_error_t>;
    
    /// Enable/disable a language server
    fn toggle_server(&self, env_name: &str, server_name: &str, enabled: bool) -> Result<(), vem_error_t>;
    
    /// Check if a language server is installed
    fn check_server(&self, server_name: &str) -> bool;
    
    /// List all configured language servers
    fn list_servers(&self, env_name: &str) -> Result<Vec<LanguageServer>, vem_error_t>;
    
    /// Get LSP configuration
    fn get_config(&self, env_name: &str) -> Result<LSP_CONFIG, vem_error_t>;
    
    /// Save LSP configuration
    fn save_config(&self, env_name: &str, config: &LSP_CONFIG) -> Result<(), vem_error_t>;
    
    /// Generate LSP configuration file for vim-lsp or other LSP clients
    fn generate_vim_config(&self, env_name: &str) -> Result<String, vem_error_t>;
}

/// LSP repository implementation with embedded config (Go-style)
pub struct lsp_repository {
    base: RepositoryConfig,
}

impl lsp_repository {
    pub fn new(config: app_config) -> Self {
        Self {
            base: RepositoryConfig::new(config),
        }
    }

    /// Get LSP config file path
    fn get_config_file_path(&self, env_name: &str) -> PathBuf {
        self.config()
            .environment_root()
            .join(env_name)
            .join("lsp_config.toml")
    }

    /// Get vim LSP config file path
    fn get_vim_lsp_config_path(&self, env_name: &str) -> PathBuf {
        self.config()
            .environment_root()
            .join(env_name)
            .join(".vim")
            .join("lsp_servers.vim")
    }
}

impl Deref for lsp_repository {
    type Target = RepositoryConfig;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl LspRepository for lsp_repository {
    fn initialize(&self, env_name: &str) -> Result<LSP_CONFIG, vem_error_t> {
        let config = LSP_CONFIG {
            environment_name: env_name.to_string(),
            language_servers: Vec::new(),
            global_settings: std::collections::HashMap::new(),
        };

        self.save_config(env_name, &config)?;
        Ok(config)
    }

    fn add_server(&self, env_name: &str, server_type: LanguageServerType) -> Result<(), vem_error_t> {
        let mut config = self.get_config(env_name)?;
        
        let server = LanguageServer::from_type(server_type);
        
        // Check if server already exists
        if config.language_servers.iter().any(|s| s.name == server.name) {
            return Err(vem_error_t::ConfigurationError(
                format!("Language server '{}' already exists", server.name)
            ));
        }

        // Check if server command is available
        let installed = self.check_server(&server.command);
        let mut new_server = server;
        new_server.installed = installed;

        config.language_servers.push(new_server);
        self.save_config(env_name, &config)?;

        // Regenerate vim config
        self.generate_vim_config(env_name)?;

        Ok(())
    }

    fn remove_server(&self, env_name: &str, server_name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(env_name)?;
        
        let initial_len = config.language_servers.len();
        config.language_servers.retain(|s| s.name != server_name);

        if config.language_servers.len() == initial_len {
            return Err(vem_error_t::ConfigurationError(
                format!("Language server '{}' not found", server_name)
            ));
        }

        self.save_config(env_name, &config)?;

        // Regenerate vim config
        self.generate_vim_config(env_name)?;

        Ok(())
    }

    fn toggle_server(&self, env_name: &str, server_name: &str, enabled: bool) -> Result<(), vem_error_t> {
        let mut config = self.get_config(env_name)?;
        
        let server = config.language_servers.iter_mut()
            .find(|s| s.name == server_name)
            .ok_or_else(|| vem_error_t::ConfigurationError(
                format!("Language server '{}' not found", server_name)
            ))?;

        server.enabled = enabled;
        self.save_config(env_name, &config)?;

        // Regenerate vim config
        self.generate_vim_config(env_name)?;

        Ok(())
    }

    fn check_server(&self, server_name: &str) -> bool {
        Command::new("which")
            .arg(server_name)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn list_servers(&self, env_name: &str) -> Result<Vec<LanguageServer>, vem_error_t> {
        let config = self.get_config(env_name)?;
        Ok(config.language_servers)
    }

    fn get_config(&self, env_name: &str) -> Result<LSP_CONFIG, vem_error_t> {
        let config_file = self.get_config_file_path(env_name);
        
        if !config_file.exists() {
            return Ok(LSP_CONFIG {
                environment_name: env_name.to_string(),
                ..Default::default()
            });
        }

        let content = fs::read_to_string(&config_file)?;
        let config: LSP_CONFIG = toml::from_str(&content)
            .map_err(|e| vem_error_t::SerializationError(
                format!("Failed to parse LSP config: {}", e)
            ))?;

        Ok(config)
    }

    fn save_config(&self, env_name: &str, config: &LSP_CONFIG) -> Result<(), vem_error_t> {
        let config_file = self.get_config_file_path(env_name);
        
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(config)
            .map_err(|e| vem_error_t::SerializationError(
                format!("Failed to serialize LSP config: {}", e)
            ))?;

        fs::write(&config_file, content)?;

        Ok(())
    }

    fn generate_vim_config(&self, env_name: &str) -> Result<String, vem_error_t> {
        let config = self.get_config(env_name)?;
        let vim_config_path = self.get_vim_lsp_config_path(env_name);

        if let Some(parent) = vim_config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut vim_script = String::from("\" Auto-generated LSP configuration\n");
        vim_script.push_str("\" Do not edit manually - managed by VEM\n\n");

        // Generate configuration for each enabled server
        for server in config.language_servers.iter().filter(|s| s.enabled) {
            vim_script.push_str(&format!("\n\" {} Language Server\n", server.language));
            vim_script.push_str(&format!("if executable('{}')\n", server.command));
            
            // vim-lsp configuration
            vim_script.push_str("  au User lsp_setup call lsp#register_server({\n");
            vim_script.push_str(&format!("    \\ 'name': '{}',\n", server.name));
            vim_script.push_str(&format!("    \\ 'cmd': {{server_info->['{}'{args}]}},\n", 
                server.command,
                args = if server.args.is_empty() { 
                    String::new() 
                } else { 
                    format!(", '{}'", server.args.join("', '")) 
                }
            ));
            vim_script.push_str(&format!("    \\ 'allowlist': [{}],\n", 
                server.filetypes.iter()
                    .map(|ft| format!("'{}'", ft))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            vim_script.push_str("  \\ })\n");
            vim_script.push_str("endif\n");
        }

        fs::write(&vim_config_path, &vim_script)?;

        Ok(vim_script)
    }
}

/// Factory function to create LSP repository
pub fn new(config: app_config) -> impl LspRepository {
    lsp_repository::new(config)
}
