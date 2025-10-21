// Centralized module declarations and re-exports

pub mod cnf {
	// application config module
	#[path = "application.rs"]
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
	
	// Re-export commonly used types
	pub use crate::util::error::vem_error_t;
	pub use model::environment::{environment_meta_t, environment_t};
}

pub mod rep {
	// repositories
	#[path = "environment.rs"]
	pub mod environment;

	// Re-exports and aliases to keep external API stable
	pub use environment::environment_repository_t;
	pub type EnvironmentRepository = environment_repository_t;
}

pub mod usc {
	// use-case layer
	#[path = "environment.rs"]
	pub mod environment;

	// Re-exports and aliases
	pub use environment::environment_manager_t;
	// Note: EnvironmentManager is generic, use environment_manager_t<R> directly
}

pub mod ctl {
	#[path = "environment.rs"]
	pub mod environment;
}

pub mod util {
	#[path = "error.rs"]
	pub mod error;
	
	#[path = "mcode.rs"]
	pub mod mcode;

	#[path = "logger.rs"]
	pub mod logger;
}