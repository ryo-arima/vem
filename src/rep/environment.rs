use std::fs;

use crate::cnf::application::app_config_t;
use crate::util::error::vem_error_t;
use crate::ent::model::environment::{environment_meta_t, environment_t};

/// Repository for managing environment data
pub struct environment_repository_t {
    config: app_config_t,
}

impl environment_repository_t {
    pub fn new(config: app_config_t) -> Self {
        Self {
            config,
        }
    }

    /// Create a new environment
    pub fn create(&self, name: &str, description: Option<String>) -> Result<environment_t, vem_error_t> {
        // Validate environment name
        if !environment_t::is_valid_name(name) {
            return Err(vem_error_t::InvalidEnvironmentName(name.to_string()));
        }

        let env_path = self.config.environment_root().join(name);

        // Check if environment already exists
        if env_path.exists() {
            return Err(vem_error_t::EnvironmentAlreadyExists(name.to_string()));
        }

        // Create environment directory structure
        fs::create_dir_all(&env_path)?;

        let meta = environment_meta_t::new(description);
        let env = environment_t::new(name, env_path.clone(), meta);

        // Create .vimrc file
        let vimrc_path = env.vimrc_path();
        if !vimrc_path.exists() {
            fs::write(&vimrc_path, format!("\" VEM Environment: {}\n", name))?;
        }

        // Create .vim directory structure
        let vim_dir = env.vim_dir_path();
        fs::create_dir_all(&vim_dir)?;
        fs::create_dir_all(vim_dir.join("autoload"))?;
        fs::create_dir_all(vim_dir.join("bundle"))?;
        fs::create_dir_all(vim_dir.join("colors"))?;
        fs::create_dir_all(vim_dir.join("plugin"))?;

        // Save metadata
        self.save_metadata(&env)?;

        Ok(env)
    }

    /// List all environments
    pub fn list(&self) -> Result<Vec<environment_t>, vem_error_t> {
        let mut environments = Vec::new();

        if !self.config.environment_root().exists() {
            return Ok(environments);
        }

        let entries = fs::read_dir(self.config.environment_root())?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(env) = self.get(name) {
                        environments.push(env);
                    }
                }
            }
        }

        // Sort by name
        environments.sort_by(|a, b| a.name().cmp(b.name()));
        Ok(environments)
    }

    /// Get an environment by name
    pub fn get(&self, name: &str) -> Result<environment_t, vem_error_t> {
        let env_path = self.config.environment_root().join(name);

        if !env_path.exists() {
            return Err(vem_error_t::EnvironmentNotFound(name.to_string()));
        }

        let meta = self.load_metadata(&env_path)?;
        Ok(environment_t::new(name, env_path, meta))
    }

    /// Update an environment's metadata
    pub fn update(&self, name: &str, description: Option<String>) -> Result<environment_t, vem_error_t> {
        let mut env = self.get(name)?;
        env.meta_mut().set_description(description);
        self.save_metadata(&env)?;
        Ok(env)
    }

    /// Delete an environment
    pub fn delete(&self, name: &str) -> Result<(), vem_error_t> {
        let env_path = self.config.environment_root().join(name);

        if !env_path.exists() {
            return Err(vem_error_t::EnvironmentNotFound(name.to_string()));
        }

        // Check if it's the current environment
        if let Ok(current) = self.get_current() {
            if current.name() == name {
                return Err(vem_error_t::ConfigurationError(
                    "Cannot remove the currently active environment".to_string(),
                ));
            }
        }

        fs::remove_dir_all(&env_path)?;
        Ok(())
    }

    /// Get the current environment
    pub fn get_current(&self) -> Result<environment_t, vem_error_t> {
        let current_link = app_config_t::current_link_path();

        if !current_link.exists() {
            return Err(vem_error_t::NoCurrentEnvironment);
        }

        let target = fs::read_link(&current_link)?;

        if let Some(name) = target.file_name().and_then(|n| n.to_str()) {
            self.get(name)
        } else {
            Err(vem_error_t::NoCurrentEnvironment)
        }
    }

    /// Set the current environment
    pub fn set_current(&self, name: &str) -> Result<(), vem_error_t> {
        let mut env = self.get(name)?; // Validate environment exists
        let current_link = app_config_t::current_link_path();

        // Remove existing symlink if it exists
        if current_link.exists() {
            fs::remove_file(&current_link)?;
        }

        // Create symlink to the environment
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(env.path(), &current_link)?;
        }

        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_dir(env.path(), &current_link)?;
        }

        // Update last_used timestamp
        env.meta_mut().set_last_used(Some(chrono::Utc::now()));
        self.save_metadata(&env)?;

        Ok(())
    }

    /// Save environment metadata to meta.toml
    fn save_metadata(&self, env: &environment_t) -> Result<(), vem_error_t> {
        let meta_path = env.meta_path();
        let content = toml::to_string_pretty(env.meta())
            .map_err(|e| vem_error_t::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        fs::write(&meta_path, content)?;
        Ok(())
    }

    /// Load environment metadata from meta.toml
    fn load_metadata(&self, env_path: &std::path::Path) -> Result<environment_meta_t, vem_error_t> {
        let meta_path = env_path.join("meta.toml");

        if !meta_path.exists() {
            // Return default metadata if file doesn't exist
            return Ok(environment_meta_t::new(None));
        }

        let content = fs::read_to_string(&meta_path)?;
        let meta: environment_meta_t = toml::from_str(&content)
            .map_err(|e| vem_error_t::SerializationError(format!("Failed to parse metadata: {}", e)))?;
        Ok(meta)
    }
}
