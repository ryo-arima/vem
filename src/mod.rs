// Centralized module declarations and re-exports

pub mod cnf {
	// application config module
	#[path = "application.rs"]
	pub mod application;

	// Re-exports and backward-compatible aliases
	pub use application::{
		app_config,
		symlink_mode_t
	};
	pub type AppConfig = app_config;
	pub type SymlinkMode = symlink_mode_t;
}

pub mod ent {
	// domain models
	pub mod model {
		pub mod environment;
		pub mod ctags;
	}
	
	pub mod request {
		pub mod environment;
	}
	
	pub mod response {
		pub mod environment;
	}
	
	// Re-export commonly used types
	pub use crate::util::error::vem_error_t;
	pub use model::environment::ENVIRONMENT;
	pub use model::ctags::{CTAGS_CONFIG, CTAGS_INDEX, CtagsOutputFormat};
}

pub mod rep {
	// repositories
	pub mod environment;
	pub mod ctags;

	// Re-exports and aliases to keep external API stable
	pub use environment::environment_repository;
	pub use ctags::ctags_repository;
	pub type EnvironmentRepository = dyn environment::EnvironmentRepository;
	pub type CtagsRepository = dyn ctags::CtagsRepository;
}

pub mod usc {
	// use-case layer
	#[path = "environment.rs"]
	pub mod environment;

	// Re-exports and aliases
	pub use environment::EnvironmentUsecase;
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