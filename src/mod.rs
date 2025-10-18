// Centralized module declarations and re-exports

pub mod cnf {
	// application config module
	#[path = "cnf/application.rs"]
	pub mod application;

	// Re-exports and backward-compatible aliases
	pub use application::{app_config_t, symlink_mode_t};
	pub type AppConfig = app_config_t;
	pub type SymlinkMode = symlink_mode_t;
}

pub mod ent {
	// domain models
	pub mod model {
		pub mod environment;
	}
	
	pub mod request {
		pub mod environment;
	}
	
	pub mod response {
		pub mod environment;
	}
	
	// error handling
	pub mod error;
	
	// Re-export commonly used types
	pub use error::vem_error_t;
	pub use model::environment::{environment_meta_t, environment_t};
}

pub mod rep {
	// repositories
	#[path = "rep/environment.rs"]
	pub mod environment;

	// Re-exports and aliases to keep external API stable
	pub use environment::environment_repository_t;
	pub type EnvironmentRepository = environment_repository_t;
}

pub mod usc {
	// use-case layer
	#[path = "usc/environment.rs"]
	pub mod environment;

	// Re-exports and aliases
	pub use environment::environment_manager_t;
	pub type EnvironmentManager = environment_manager_t;
}

// CLI module (single file)
pub mod ctl;
