use crate::cnf::AppConfig;
use crate::ent::{Environment, VemError};
use std::fs;

/// Repository for managing environment data
pub struct EnvironmentRepository {
    config: AppConfig,
}

impl EnvironmentRepository {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// Create a new environment
    pub fn create(&self, name: &str) -> Result<Environment, VemError> {
        // Validate environment name
        if !Environment::is_valid_name(name) {
            return Err(VemError::InvalidEnvironmentName(name.to_string()));
        }

        let env_path = self.config.environment_root.join(name);

        // Check if environment already exists
        if env_path.exists() {
            return Err(VemError::EnvironmentAlreadyExists(name.to_string()));
        }

        // Create environment directory structure
        fs::create_dir_all(&env_path)?;
        
        let env = Environment::new(name, env_path.clone());

        // Create .vimrc file
        let vimrc_path = env.vimrc_path();
        if !vimrc_path.exists() {
            fs::write(&vimrc_path, "\" VEM Environment: ".to_owned() + name + "\n")?;
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
    pub fn list(&self) -> Result<Vec<Environment>, VemError> {
        let mut environments = Vec::new();

        if !self.config.environment_root.exists() {
            return Ok(environments);
        }

        let entries = fs::read_dir(&self.config.environment_root)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(env) = self.load(name) {
                        environments.push(env);
                    }
                }
            }
        }

        // Sort by name
        environments.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(environments)
    }

    /// Load an environment by name
    pub fn load(&self, name: &str) -> Result<Environment, VemError> {
        let env_path = self.config.environment_root.join(name);
        
        if !env_path.exists() {
            return Err(VemError::EnvironmentNotFound(name.to_string()));
        }

        let mut env = Environment::new(name, env_path);
        
        // Load metadata if it exists
        let meta_path = env.meta_path();
        if meta_path.exists() {
            let content = fs::read_to_string(&meta_path)?;
            env.meta = serde_json::from_str(&content)?;
        }

        Ok(env)
    }

    /// Remove an environment
    pub fn remove(&self, name: &str) -> Result<(), VemError> {
        let env_path = self.config.environment_root.join(name);
        
        if !env_path.exists() {
            return Err(VemError::EnvironmentNotFound(name.to_string()));
        }

        // Check if it's the current environment
        if let Ok(current) = self.get_current() {
            if current.name == name {
                return Err(VemError::ConfigurationError(
                    "Cannot remove the currently active environment".to_string(),
                ));
            }
        }

        fs::remove_dir_all(&env_path)?;
        Ok(())
    }

    /// Get the current environment
    pub fn get_current(&self) -> Result<Environment, VemError> {
        let current_link = AppConfig::current_link_path();
        
        if !current_link.exists() {
            return Err(VemError::NoCurrentEnvironment);
        }

        let target = fs::read_link(&current_link)?;
        
        if let Some(name) = target.file_name().and_then(|n| n.to_str()) {
            self.load(name)
        } else {
            Err(VemError::NoCurrentEnvironment)
        }
    }

    /// Set the current environment
    pub fn set_current(&self, name: &str) -> Result<(), VemError> {
        let env = self.load(name)?; // Validate environment exists
        let current_link = AppConfig::current_link_path();
        
        // Remove existing symlink if it exists
        if current_link.exists() {
            fs::remove_file(&current_link)?;
        }

        // Create symlink to the environment
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&env.path, &current_link)?;
        }
        
        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_dir(&env.path, &current_link)?;
        }

        // Update last_used timestamp
        let mut env = env;
        env.meta.last_used = Some(chrono::Utc::now());
        self.save_metadata(&env)?;

        Ok(())
    }

    /// Save environment metadata
    fn save_metadata(&self, env: &Environment) -> Result<(), VemError> {
        let meta_path = env.meta_path();
        let content = serde_json::to_string_pretty(&env.meta)?;
        fs::write(&meta_path, content)?;
        Ok(())
    }
}