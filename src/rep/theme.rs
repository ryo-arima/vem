#![allow(non_camel_case_types)]

use crate::ent::model::theme::{Theme, ThemeBackground, THEME_CONFIG};
use crate::rep::RepositoryConfig;
use crate::util::error::vem_error_t;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

/// Theme repository with Go-style struct embedding
pub struct theme_repository {
    base: RepositoryConfig,
}

impl Deref for theme_repository {
    type Target = RepositoryConfig;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

pub trait ThemeRepository {
    fn initialize(&mut self, environment_name: &str) -> Result<(), vem_error_t>;
    fn add_theme(&mut self, environment_name: &str, theme: Theme) -> Result<(), vem_error_t>;
    fn remove_theme(&mut self, environment_name: &str, theme_name: &str)
        -> Result<(), vem_error_t>;
    fn set_active_theme(
        &mut self,
        environment_name: &str,
        theme_name: &str,
    ) -> Result<(), vem_error_t>;
    fn toggle_theme(
        &mut self,
        environment_name: &str,
        theme_name: &str,
    ) -> Result<(), vem_error_t>;
    fn check_theme(&self, theme_name: &str) -> Result<bool, vem_error_t>;
    fn list_themes(&self, environment_name: &str) -> Result<Vec<Theme>, vem_error_t>;
    fn get_active_theme(&self, environment_name: &str) -> Result<Option<Theme>, vem_error_t>;
    fn get_config(&self, environment_name: &str) -> Result<THEME_CONFIG, vem_error_t>;
    fn save_config(&self, config: &THEME_CONFIG) -> Result<(), vem_error_t>;
    fn generate_colorscheme_config(&self, environment_name: &str)
        -> Result<String, vem_error_t>;
    fn generate_plugin_config(&self, environment_name: &str) -> Result<String, vem_error_t>;
}

impl theme_repository {
    pub fn new() -> Self {
        Self {
            base: RepositoryConfig::new(),
        }
    }

    fn get_theme_config_path(&self, environment_name: &str) -> PathBuf {
        self.base
            .app_config
            .get_environment_path(environment_name)
            .join("theme_config.toml")
    }
}

impl ThemeRepository for theme_repository {
    fn initialize(&mut self, environment_name: &str) -> Result<(), vem_error_t> {
        let config = THEME_CONFIG {
            environment_name: environment_name.to_string(),
            active_theme: None,
            themes: Vec::new(),
            global_settings: std::collections::HashMap::new(),
        };

        self.save_config(&config)?;
        Ok(())
    }

    fn add_theme(&mut self, environment_name: &str, mut theme: Theme) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        // Check if theme already exists
        if config.themes.iter().any(|t| t.name == theme.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Theme '{}' already exists",
                theme.name
            )));
        }

        // Check if theme is installed
        theme.installed = self.check_theme(&theme.name).unwrap_or(false);

        config.themes.push(theme);
        self.save_config(&config)?;

        Ok(())
    }

    fn remove_theme(
        &mut self,
        environment_name: &str,
        theme_name: &str,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        let original_len = config.themes.len();
        config.themes.retain(|t| t.name != theme_name);

        if config.themes.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Theme '{}' not found",
                theme_name
            )));
        }

        // Clear active theme if it was removed
        if config.active_theme.as_deref() == Some(theme_name) {
            config.active_theme = None;
        }

        self.save_config(&config)?;
        Ok(())
    }

    fn set_active_theme(
        &mut self,
        environment_name: &str,
        theme_name: &str,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        // Check if theme exists
        if !config.themes.iter().any(|t| t.name == theme_name) {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Theme '{}' not found",
                theme_name
            )));
        }

        config.active_theme = Some(theme_name.to_string());
        self.save_config(&config)?;

        Ok(())
    }

    fn toggle_theme(
        &mut self,
        environment_name: &str,
        theme_name: &str,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        let theme = config
            .themes
            .iter_mut()
            .find(|t| t.name == theme_name)
            .ok_or_else(|| {
                vem_error_t::EnvironmentNotFound(format!("Theme '{}' not found", theme_name))
            })?;

        theme.enabled = !theme.enabled;
        self.save_config(&config)?;

        Ok(())
    }

    fn check_theme(&self, theme_name: &str) -> Result<bool, vem_error_t> {
        // Check if theme is installed by looking for colorscheme files
        // vim: ~/.vim/colors/
        // neovim: ~/.config/nvim/colors/ or plugin directories

        let color_dirs = vec![
            dirs::home_dir()
                .map(|h| h.join(".vim/colors"))
                .unwrap_or_default(),
            dirs::home_dir()
                .map(|h| h.join(".config/nvim/colors"))
                .unwrap_or_default(),
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

        let theme_name_lower = theme_name.to_lowercase();

        for color_dir in color_dirs {
            if color_dir.exists() {
                // Check for .vim colorscheme file
                let vim_file = color_dir.join(format!("{}.vim", theme_name_lower));
                if vim_file.exists() {
                    return Ok(true);
                }

                // Check in plugin directories
                if let Ok(entries) = fs::read_dir(&color_dir) {
                    for entry in entries.flatten() {
                        if let Ok(name) = entry.file_name().into_string() {
                            if name.to_lowercase().contains(&theme_name_lower)
                                || theme_name_lower.contains(&name.to_lowercase())
                            {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    fn list_themes(&self, environment_name: &str) -> Result<Vec<Theme>, vem_error_t> {
        let config = self.get_config(environment_name)?;
        Ok(config.themes)
    }

    fn get_active_theme(&self, environment_name: &str) -> Result<Option<Theme>, vem_error_t> {
        let config = self.get_config(environment_name)?;

        if let Some(active_name) = &config.active_theme {
            let theme = config
                .themes
                .iter()
                .find(|t| &t.name == active_name)
                .cloned();
            Ok(theme)
        } else {
            Ok(None)
        }
    }

    fn get_config(&self, environment_name: &str) -> Result<THEME_CONFIG, vem_error_t> {
        let config_path = self.get_theme_config_path(environment_name);

        if !config_path.exists() {
            return Ok(THEME_CONFIG {
                environment_name: environment_name.to_string(),
                active_theme: None,
                themes: Vec::new(),
                global_settings: std::collections::HashMap::new(),
            });
        }

        let content = fs::read_to_string(&config_path).map_err(vem_error_t::FileSystemError)?;

        toml::from_str(&content).map_err(|e| {
            vem_error_t::SerializationError(format!("Failed to parse theme config: {}", e))
        })
    }

    fn save_config(&self, config: &THEME_CONFIG) -> Result<(), vem_error_t> {
        let config_path = self.get_theme_config_path(&config.environment_name);

        let content = toml::to_string_pretty(config).map_err(|e| {
            vem_error_t::SerializationError(format!("Failed to serialize theme config: {}", e))
        })?;

        fs::write(&config_path, content).map_err(vem_error_t::FileSystemError)?;

        Ok(())
    }

    fn generate_colorscheme_config(
        &self,
        environment_name: &str,
    ) -> Result<String, vem_error_t> {
        let config = self.get_config(environment_name)?;
        let mut vim_config = String::new();

        vim_config.push_str("\" Theme Configuration\n");
        vim_config.push_str("\" Generated by VEM\n\n");

        // Set background for active theme
        if let Some(active_theme_name) = &config.active_theme {
            if let Some(active_theme) = config.themes.iter().find(|t| &t.name == active_theme_name)
            {
                let bg = match active_theme.background {
                    ThemeBackground::Dark => "dark",
                    ThemeBackground::Light => "light",
                };
                vim_config.push_str(&format!("set background={}\n", bg));

                // Set colorscheme
                let colorscheme = active_theme.theme_type.colorscheme_name();
                if !colorscheme.is_empty() {
                    vim_config.push_str(&format!("colorscheme {}\n\n", colorscheme));
                }

                // Add theme-specific settings
                if !active_theme.settings.is_empty() {
                    vim_config.push_str("\" Theme-specific settings\n");
                    for (key, value) in &active_theme.settings {
                        vim_config.push_str(&format!("let g:{}_{} = {}\n", 
                            active_theme.name.to_lowercase().replace(' ', "_"),
                            key,
                            value
                        ));
                    }
                    vim_config.push('\n');
                }
            }
        }

        Ok(vim_config)
    }

    fn generate_plugin_config(&self, environment_name: &str) -> Result<String, vem_error_t> {
        let config = self.get_config(environment_name)?;
        let mut vim_config = String::new();

        vim_config.push_str("\" Theme Plugins Configuration\n");
        vim_config.push_str("\" Generated by VEM\n\n");

        for theme in config.themes.iter().filter(|t| t.enabled) {
            vim_config.push_str(&format!(
                "\" {}: {}\n",
                theme.name,
                theme.theme_type.description()
            ));

            // Generate plugin declaration
            let plugin_name = theme.theme_type.plugin_name();
            vim_config.push_str(&format!("\" Plug '{}'\n", plugin_name));

            vim_config.push('\n');
        }

        Ok(vim_config)
    }
}

impl Default for theme_repository {
    fn default() -> Self {
        Self::new()
    }
}
