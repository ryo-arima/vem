#![allow(non_camel_case_types)]

use crate::ent::model::ai::{AITool, AI_CONFIG};
use crate::rep::RepositoryConfig;
use crate::util::error::vem_error_t;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

/// AI repository with Go-style struct embedding
pub struct ai_repository {
    base: RepositoryConfig,
}

impl Deref for ai_repository {
    type Target = RepositoryConfig;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

pub trait AiRepository {
    fn initialize(&mut self, environment_name: &str) -> Result<(), vem_error_t>;
    fn add_tool(&mut self, environment_name: &str, tool: AITool) -> Result<(), vem_error_t>;
    fn remove_tool(&mut self, environment_name: &str, tool_name: &str) -> Result<(), vem_error_t>;
    fn toggle_tool(&mut self, environment_name: &str, tool_name: &str) -> Result<(), vem_error_t>;
    fn check_tool(&self, tool_name: &str) -> Result<bool, vem_error_t>;
    fn list_tools(&self, environment_name: &str) -> Result<Vec<AITool>, vem_error_t>;
    fn get_config(&self, environment_name: &str) -> Result<AI_CONFIG, vem_error_t>;
    fn save_config(&self, config: &AI_CONFIG) -> Result<(), vem_error_t>;
    fn generate_plugin_config(&self, environment_name: &str) -> Result<String, vem_error_t>;
    fn generate_coc_extensions_config(
        &self,
        environment_name: &str,
    ) -> Result<Vec<String>, vem_error_t>;
}

impl ai_repository {
    pub fn new() -> Self {
        Self {
            base: RepositoryConfig::new(),
        }
    }

    fn get_ai_config_path(&self, environment_name: &str) -> PathBuf {
        self.base
            .app_config
            .get_environment_path(environment_name)
            .join("ai_config.toml")
    }
}

impl AiRepository for ai_repository {
    fn initialize(&mut self, environment_name: &str) -> Result<(), vem_error_t> {
        let config = AI_CONFIG {
            environment_name: environment_name.to_string(),
            ai_tools: Vec::new(),
            global_settings: std::collections::HashMap::new(),
        };

        self.save_config(&config)?;
        Ok(())
    }

    fn add_tool(&mut self, environment_name: &str, mut tool: AITool) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        // Check if tool already exists
        if config.ai_tools.iter().any(|t| t.name == tool.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "AI tool '{}' already exists",
                tool.name
            )));
        }

        // Check if tool is installed
        tool.installed = self.check_tool(&tool.name).unwrap_or(false);

        config.ai_tools.push(tool);
        self.save_config(&config)?;

        Ok(())
    }

    fn remove_tool(&mut self, environment_name: &str, tool_name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        let original_len = config.ai_tools.len();
        config.ai_tools.retain(|t| t.name != tool_name);

        if config.ai_tools.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "AI tool '{}' not found",
                tool_name
            )));
        }

        self.save_config(&config)?;
        Ok(())
    }

    fn toggle_tool(&mut self, environment_name: &str, tool_name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        let tool = config
            .ai_tools
            .iter_mut()
            .find(|t| t.name == tool_name)
            .ok_or_else(|| vem_error_t::EnvironmentNotFound(format!("AI tool '{}' not found", tool_name)))?;

        tool.enabled = !tool.enabled;
        self.save_config(&config)?;

        Ok(())
    }

    fn check_tool(&self, tool_name: &str) -> Result<bool, vem_error_t> {
        // Check if plugin/extension is installed by looking for common plugin managers
        // This is a simplified check - in reality, you'd check specific plugin manager directories

        // For coc extensions, check node_modules
        if tool_name.starts_with("coc-") {
            let coc_extensions_dir = dirs::home_dir()
                .ok_or(vem_error_t::ConfigurationError("Home directory not found".to_string()))?
                .join(".config/coc/extensions/node_modules")
                .join(tool_name);
            return Ok(coc_extensions_dir.exists());
        }

        // For vim/neovim plugins, this would check plugin manager directories
        // vim-plug: ~/.vim/plugged or ~/.local/share/nvim/plugged
        // packer: ~/.local/share/nvim/site/pack/packer
        let plugin_dirs = vec![
            dirs::home_dir()
                .map(|h| h.join(".vim/plugged"))
                .unwrap_or_default(),
            dirs::home_dir()
                .map(|h| h.join(".local/share/nvim/plugged"))
                .unwrap_or_default(),
            dirs::home_dir()
                .map(|h| h.join(".local/share/nvim/site/pack/packer/start"))
                .unwrap_or_default(),
        ];

        for plugin_dir in plugin_dirs {
            if plugin_dir.exists() {
                // Check if any directory in plugin_dir contains the tool name
                if let Ok(entries) = fs::read_dir(&plugin_dir) {
                    for entry in entries.flatten() {
                        if let Ok(name) = entry.file_name().into_string() {
                            if name.contains(tool_name) || tool_name.contains(&name) {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    fn list_tools(&self, environment_name: &str) -> Result<Vec<AITool>, vem_error_t> {
        let config = self.get_config(environment_name)?;
        Ok(config.ai_tools)
    }

    fn get_config(&self, environment_name: &str) -> Result<AI_CONFIG, vem_error_t> {
        let config_path = self.get_ai_config_path(environment_name);

        if !config_path.exists() {
            return Ok(AI_CONFIG {
                environment_name: environment_name.to_string(),
                ai_tools: Vec::new(),
                global_settings: std::collections::HashMap::new(),
            });
        }

        let content = fs::read_to_string(&config_path).map_err(|e| {
            vem_error_t::FileSystemError(e)
        })?;

        toml::from_str(&content)
            .map_err(|e| vem_error_t::SerializationError(format!("Failed to parse AI config: {}", e)))
    }

    fn save_config(&self, config: &AI_CONFIG) -> Result<(), vem_error_t> {
        let config_path = self.get_ai_config_path(&config.environment_name);

        let content = toml::to_string_pretty(config)
            .map_err(|e| vem_error_t::SerializationError(format!("Failed to serialize AI config: {}", e)))?;

        fs::write(&config_path, content)
            .map_err(vem_error_t::FileSystemError)?;

        Ok(())
    }

    fn generate_plugin_config(&self, environment_name: &str) -> Result<String, vem_error_t> {
        let config = self.get_config(environment_name)?;
        let mut vim_config = String::new();

        vim_config.push_str("\" AI Tools Configuration\n");
        vim_config.push_str("\" Generated by VEM\n\n");

        for tool in config.ai_tools.iter().filter(|t| t.enabled) {
            // Skip coc extensions as they're handled separately
            if tool.tool_type.is_coc_extension() {
                continue;
            }

            vim_config.push_str(&format!(
                "\" {}: {}\n",
                tool.name,
                tool.tool_type.description()
            ));

            // Generate plugin declaration based on plugin manager
            // This is a generic example - would need to be adapted per plugin manager
            let plugin_name = tool.tool_type.plugin_name();
            vim_config.push_str(&format!("\" Plug '{}'\n", plugin_name));

            // Add API key configuration if required
            if let Some(api_key_env) = &tool.api_key_env {
                vim_config.push_str(&format!(
                    "\" Make sure {} is set in your environment\n",
                    api_key_env
                ));
            }

            // Add tool-specific settings
            if !tool.settings.is_empty() {
                vim_config.push_str("\" Settings:\n");
                for (key, value) in &tool.settings {
                    vim_config.push_str(&format!("\" let g:{}_{} = {}\n", 
                        tool.name.to_lowercase().replace(' ', "_"),
                        key,
                        value
                    ));
                }
            }

            vim_config.push('\n');
        }

        Ok(vim_config)
    }

    fn generate_coc_extensions_config(
        &self,
        environment_name: &str,
    ) -> Result<Vec<String>, vem_error_t> {
        let config = self.get_config(environment_name)?;
        let extensions: Vec<String> = config
            .ai_tools
            .iter()
            .filter(|t| t.enabled && t.tool_type.is_coc_extension())
            .map(|t| t.tool_type.plugin_name())
            .collect();

        Ok(extensions)
    }
}

impl Default for ai_repository {
    fn default() -> Self {
        Self::new()
    }
}
