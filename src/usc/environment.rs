use crate::cnf::application::app_config_t;
use crate::ent::model::environment::environment_t;
use crate::rep::environment::environment_repository_t;
use crate::util::error::vem_error_t;

/// High-level environment management operations
pub struct environment_manager_t {
    repository: environment_repository_t,
}

impl environment_manager_t {
    pub fn new() -> Result<Self, vem_error_t> {
        let config = app_config_t::load()?;
        config.validate()?;

        let repository = environment_repository_t::new(config);

        Ok(Self {
            repository,
        })
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
