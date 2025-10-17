use crate::cnf::AppConfig;
use crate::ent::{Environment, VemError};
use crate::rep::EnvironmentRepository;

/// High-level environment management operations
pub struct EnvironmentManager {
    repository: EnvironmentRepository,
}

impl EnvironmentManager {
    pub fn new() -> Result<Self, VemError> {
        let config = AppConfig::load()?;
        config.validate()?;
        
        let repository = EnvironmentRepository::new(config);
        
        Ok(Self { repository })
    }

    /// Create a new environment
    pub fn create_environment(&self, name: &str) -> Result<Environment, VemError> {
        println!("Creating environment: {}", name);
        let env = self.repository.create(name)?;
        println!("Created environment: {}", name);
        Ok(env)
    }

    /// List all environments
    pub fn list_environments(&self) -> Result<Vec<Environment>, VemError> {
        self.repository.list()
    }

    /// Switch to an environment
    pub fn switch_environment(&self, name: &str) -> Result<(), VemError> {
        println!("Switching to environment: {}", name);
        self.repository.set_current(name)?;
        println!("Switched to environment: {}", name);
        Ok(())
    }

    /// Get the current environment
    pub fn get_current_environment(&self) -> Result<Environment, VemError> {
        self.repository.get_current()
    }

    /// Remove an environment
    pub fn remove_environment(&self, name: &str) -> Result<(), VemError> {
        println!("Removing environment: {}", name);
        self.repository.remove(name)?;
        println!("Removed environment: {}", name);
        Ok(())
    }
}