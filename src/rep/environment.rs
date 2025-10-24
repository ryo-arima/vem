#![allow(non_camel_case_types)]

use std::fs;
use std::ops::Deref;

use crate::cnf::application::app_config;
use crate::util::error::vem_error_t;
use crate::ent::model::environment::ENVIRONMENT;

/// Base configuration holder - similar to Go's embedded struct
pub struct RepositoryConfig {
    config: app_config,
}

impl RepositoryConfig {
    pub fn new(config: app_config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &app_config {
        &self.config
    }
}

/// Environment repository trait
pub trait EnvironmentRepository {
    fn create(&self, name: &str, description: Option<String>) -> (ENVIRONMENT, bool);
    fn list(&self) -> Vec<ENVIRONMENT>;
    fn get(&self, name: &str) -> ENVIRONMENT;
    fn update(&self, name: &str, description: Option<String>) -> (ENVIRONMENT, bool);
    fn delete(&self, name: &str) -> bool;
    fn get_current(&self) -> ENVIRONMENT;
    fn set_current(&self, name: &str) -> bool;
}

/// Environment repository implementation with embedded config
pub struct environment_repository {
    base: RepositoryConfig,  // Embedded base struct (like Go)
}

impl environment_repository {
    pub fn new(config: app_config) -> Self {
        Self {
            base: RepositoryConfig::new(config),
        }
    }
}

// Deref implementation for automatic access to base methods (like Go's embedding)
impl Deref for environment_repository {
    type Target = RepositoryConfig;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl EnvironmentRepository for environment_repository {
    /// Create a new environment
    fn create(&self, name: &str, description: Option<String>) -> (ENVIRONMENT, bool) {
        // Validate environment name
        if name.is_empty() || name.contains('/') || name.contains('\\') {
            let default_env = ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
            return (default_env, false);
        }

        // Access config through Deref (like Go's embedded field access)
        let env_path = self.config().environment_root().join(name);

        // Check if environment already exists
        if env_path.exists() {
            let default_env = ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
            return (default_env, false);
        }

        // Create environment directory structure
        if fs::create_dir_all(&env_path).is_err() {
            let default_env = ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
            return (default_env, false);
        }

        let env = ENVIRONMENT {
            name: name.to_string(),
            description,
            created: chrono::Utc::now(),
            update: chrono::Utc::now(),
            last_used: None,
            tags: Vec::new(),
            package_manager: None,
        };

        // Create .vimrc file
        let vimrc_path = env_path.join(".vimrc");
        if !vimrc_path.exists()
            && fs::write(&vimrc_path, format!("\" VEM Environment: {}\n", name)).is_err()
        {
            let default_env = ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
            return (default_env, false);
        }

        // Create .vim directory structure
        let vim_dir = env_path.join(".vim");
        if fs::create_dir_all(&vim_dir).is_err()
            || fs::create_dir_all(vim_dir.join("autoload")).is_err()
            || fs::create_dir_all(vim_dir.join("bundle")).is_err()
            || fs::create_dir_all(vim_dir.join("colors")).is_err()
            || fs::create_dir_all(vim_dir.join("plugin")).is_err()
        {
            let default_env = ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
            return (default_env, false);
        }

        // Save metadata
        if self.save_metadata(name, &env).is_err() {
            let default_env = ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
            return (default_env, false);
        }

        (env, true)
    }

    /// List all environments
    fn list(&self) -> Vec<ENVIRONMENT> {
        let mut environments = Vec::new();

        // Access config through Deref (like Go's embedded field)
        if !self.config().environment_root().exists() {
            return environments;
        }

        let Ok(entries) = fs::read_dir(self.config().environment_root()) else {
            return environments;
        };

        for entry in entries {
            let Ok(entry) = entry else { continue };
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let env = self.get(name);
                    if !env.name.is_empty() {
                        environments.push(env);
                    }
                }
            }
        }

        // Sort by name
        environments.sort_by(|a, b| a.name.cmp(&b.name));
        environments
    }

    /// Get an environment by name
    fn get(&self, name: &str) -> ENVIRONMENT {
        let env_path = self.config().environment_root().join(name);

        if !env_path.exists() {
            return ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
        }

        let Ok(env) = self.load_metadata(name) else {
            return ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
        };

        env
    }

    /// Update an environment's metadata
    fn update(&self, name: &str, description: Option<String>) -> (ENVIRONMENT, bool) {
        let env = self.get(name);
        
        if env.name.is_empty() {
            let default_env = ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
            return (default_env, false);
        }

        let updated_env = ENVIRONMENT {
            name: name.to_string(),
            description,
            created: env.created,
            update: chrono::Utc::now(),
            last_used: env.last_used,
            tags: env.tags,
            package_manager: env.package_manager,
        };
        
        if self.save_metadata(name, &updated_env).is_err() {
            let default_env = ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
            return (default_env, false);
        }

        (updated_env, true)
    }

    /// Delete an environment
    fn delete(&self, name: &str) -> bool {
        let env_path = self.config().environment_root().join(name);

        if !env_path.exists() {
            return false;
        }

        // Check if it's the current environment
        let current = self.get_current();
        if !current.name.is_empty() && current.name == name {
            return false;
        }

        fs::remove_dir_all(&env_path).is_ok()
    }

    /// Get the current environment
    fn get_current(&self) -> ENVIRONMENT {
        let current_link = app_config::current_link_path();

        if !current_link.exists() {
            return ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
        }

        let Ok(target) = fs::read_link(&current_link) else {
            return ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            };
        };

        if let Some(name) = target.file_name().and_then(|n| n.to_str()) {
            self.get(name)
        } else {
            ENVIRONMENT {
                name: String::new(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            }
        }
    }

    /// Set the current environment
    fn set_current(&self, name: &str) -> bool {
        let env = self.get(name);
        
        if env.name.is_empty() {
            return false;
        }

        let current_link = app_config::current_link_path();
        let env_path = self.config().environment_root().join(name);

        // Remove existing symlink if it exists
        if current_link.exists()
            && fs::remove_file(&current_link).is_err()
        {
            return false;
        }

        // Create symlink to the environment
        #[cfg(unix)]
        {
            if std::os::unix::fs::symlink(&env_path, &current_link).is_err() {
                return false;
            }
        }

        #[cfg(windows)]
        {
            if std::os::windows::fs::symlink_dir(&env_path, &current_link).is_err() {
                return false;
            }
        }

        // Update last_used timestamp
        let updated_env = ENVIRONMENT {
            name: env.name,
            description: env.description,
            created: env.created,
            update: chrono::Utc::now(),
            last_used: Some(chrono::Utc::now()),
            tags: env.tags,
            package_manager: env.package_manager,
        };
        if self.save_metadata(name, &updated_env).is_err() {
            return false;
        }

        true
    }
}

// Private helper methods
impl environment_repository {
    /// Save environment metadata to meta.toml
    fn save_metadata(&self, name: &str, env: &ENVIRONMENT) -> Result<(), vem_error_t> {
        let env_path = self.config().environment_root().join(name);
        let meta_path = env_path.join("meta.toml");
        let content = toml::to_string_pretty(env)
            .map_err(|e| vem_error_t::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        fs::write(&meta_path, content)?;
        Ok(())
    }

    /// Load environment metadata from meta.toml
    fn load_metadata(&self, name: &str) -> Result<ENVIRONMENT, vem_error_t> {
        let env_path = self.config().environment_root().join(name);
        let meta_path = env_path.join("meta.toml");

        if !meta_path.exists() {
            // Return default environment if file doesn't exist
            return Ok(ENVIRONMENT {
                name: name.to_string(),
                description: None,
                created: chrono::Utc::now(),
                update: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                package_manager: None,
            });
        }

        let content = fs::read_to_string(&meta_path)?;
        let env: ENVIRONMENT = toml::from_str(&content)
            .map_err(|e| vem_error_t::SerializationError(format!("Failed to parse metadata: {}", e)))?;
        Ok(env)
    }
}

/// Factory function to create environment repository
pub fn new(config: app_config) -> impl EnvironmentRepository {
    environment_repository::new(config)
}
