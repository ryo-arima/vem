#![allow(non_camel_case_types)]

use crate::cnf::application::app_config_t;
use crate::ent::model::environment::environment_t;
use crate::rep::environment::{environment_repository_t, EnvironmentRepository};
use crate::util::error::vem_error_t;

/// Environment manager interface trait
pub trait EnvironmentManager {
    /// Create a new environment
    fn create_environment(&self, name: &str, description: Option<String>) -> Result<environment_t, vem_error_t>;
    
    /// List all environments
    fn list_environments(&self) -> Result<Vec<environment_t>, vem_error_t>;
    
    /// Get an environment by name
    fn get_environment(&self, name: &str) -> Result<environment_t, vem_error_t>;
    
    /// Update an environment's metadata
    fn update_environment(&self, name: &str, description: Option<String>) -> Result<environment_t, vem_error_t>;
    
    /// Switch to an environment
    fn switch_environment(&self, name: &str) -> Result<(), vem_error_t>;
    
    /// Get the current environment
    fn get_current_environment(&self) -> Result<environment_t, vem_error_t>;
    
    /// Remove an environment
    fn remove_environment(&self, name: &str) -> Result<(), vem_error_t>;
}

/// High-level environment management operations
pub struct environment_manager_t<R>
where
    R: EnvironmentRepository,
{
    repository: R,
}

impl<R> environment_manager_t<R>
where
    R: EnvironmentRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Create a new environment
    pub fn create_environment(&self, name: &str, description: Option<String>) -> Result<environment_t, vem_error_t> {
        self.repository.create(name, description)
    }

    /// List all environments
    pub fn list_environments(&self) -> Result<Vec<environment_t>, vem_error_t> {
        self.repository.list()
    }

    /// Get an environment by name
    #[allow(dead_code)]
    pub fn get_environment(&self, name: &str) -> Result<environment_t, vem_error_t> {
        self.repository.get(name)
    }

    /// Update an environment's metadata
    pub fn update_environment(&self, name: &str, description: Option<String>) -> Result<environment_t, vem_error_t> {
        self.repository.update(name, description)
    }

    /// Switch to an environment
    pub fn switch_environment(&self, name: &str) -> Result<(), vem_error_t> {
        self.repository.set_current(name)?;
        Ok(())
    }

    /// Get the current environment
    pub fn get_current_environment(&self) -> Result<environment_t, vem_error_t> {
        self.repository.get_current()
    }

    /// Remove an environment
    pub fn remove_environment(&self, name: &str) -> Result<(), vem_error_t> {
        self.repository.delete(name)?;
        Ok(())
    }
}

impl<R> EnvironmentManager for environment_manager_t<R>
where
    R: EnvironmentRepository,
{
    fn create_environment(&self, name: &str, description: Option<String>) -> Result<environment_t, vem_error_t> {
        self.repository.create(name, description)
    }
    
    fn list_environments(&self) -> Result<Vec<environment_t>, vem_error_t> {
        self.repository.list()
    }
    
    fn get_environment(&self, name: &str) -> Result<environment_t, vem_error_t> {
        self.repository.get(name)
    }
    
    fn update_environment(&self, name: &str, description: Option<String>) -> Result<environment_t, vem_error_t> {
        self.repository.update(name, description)
    }
    
    fn switch_environment(&self, name: &str) -> Result<(), vem_error_t> {
        self.repository.set_current(name)?;
        Ok(())
    }
    
    fn get_current_environment(&self) -> Result<environment_t, vem_error_t> {
        self.repository.get_current()
    }
    
    fn remove_environment(&self, name: &str) -> Result<(), vem_error_t> {
        self.repository.delete(name)?;
        Ok(())
    }
}

/// Factory function to create environment manager
pub fn create_environment_manager() -> Result<impl EnvironmentManager, vem_error_t> {
    let config = app_config_t::load()?;
    let repository = environment_repository_t::new(config);
    Ok(environment_manager_t::new(repository))
}
