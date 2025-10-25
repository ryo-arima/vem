pub mod cnf {
	pub mod application;
	pub use application::{
		app_config,
		symlink_mode_t,
		new_app_config
	};
	pub type AppConfig = app_config;
	pub type SymlinkMode = symlink_mode_t;
}

pub mod ent {
	pub mod model {
		pub mod environment;
	}
	pub mod request {
		pub mod environment;
	}
	pub mod response {
		pub mod environment;
	}
	pub use crate::util::error::vem_error_t;
	pub use model::environment::ENVIRONMENT;
}

pub mod rep {
	pub mod environment;
	pub use environment::environment_repository;
	pub type EnvironmentRepository = dyn environment::EnvironmentRepository;
}

pub mod usc {
	pub mod environment;
	pub use environment::EnvironmentUsecase;
}

pub mod ctl {
	pub mod environment;
}

pub mod util {
	pub mod error;
	pub mod mcode;
	pub mod logger;
	pub mod serialize;
	pub mod deserialize;
	pub mod debug;
	pub mod clone;
	pub mod eq;
}