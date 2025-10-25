// Custom Debug implementations
// This module provides common Debug trait implementations

// Debug implementation for symlink_mode_t
impl std::fmt::Debug for crate::cnf::application::symlink_mode_t {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::cnf::application::symlink_mode_t;
        match self {
            symlink_mode_t::SYMBOLIC => write!(f, "SYMBOLIC"),
            symlink_mode_t::HARD => write!(f, "HARD"),
        }
    }
}

// Debug implementation for app_config
impl std::fmt::Debug for crate::cnf::application::app_config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("app_config")
            .field("default_environment", &self.default_environment)
            .field("auto_switch", &self.auto_switch)
            .field("backup_enabled", &self.backup_enabled)
            .field("backup_retention_days", &self.backup_retention_days)
            .field("environment_root", &self.environment_root)
            .field("symlink_mode", &self.symlink_mode)
            .field("editor", &self.editor)
            .finish()
    }
}

// Debug implementation for ENVIRONMENT
impl std::fmt::Debug for crate::ent::model::environment::ENVIRONMENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ENVIRONMENT")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("created", &self.created)
            .field("update", &self.update)
            .field("last_used", &self.last_used)
            .field("tags", &self.tags)
            .finish()
    }
}

// Debug implementation for vem_error_t
impl std::fmt::Debug for crate::util::error::vem_error_t {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::util::error::vem_error_t;
        match self {
            vem_error_t::EnvironmentNotFound(s) => f.debug_tuple("EnvironmentNotFound").field(s).finish(),
            vem_error_t::EnvironmentAlreadyExists(s) => f.debug_tuple("EnvironmentAlreadyExists").field(s).finish(),
            vem_error_t::InvalidEnvironmentName(s) => f.debug_tuple("InvalidEnvironmentName").field(s).finish(),
            vem_error_t::FileSystemError(e) => f.debug_tuple("FileSystemError").field(e).finish(),
            vem_error_t::ConfigurationError(s) => f.debug_tuple("ConfigurationError").field(s).finish(),
            vem_error_t::SerializationError(s) => f.debug_tuple("SerializationError").field(s).finish(),
            vem_error_t::NoCurrentEnvironment => write!(f, "NoCurrentEnvironment"),
        }
    }
}

// Debug implementation for log_level_t
impl std::fmt::Debug for crate::util::mcode::log_level_t {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::util::mcode::log_level_t;
        match self {
            log_level_t::EMERG => write!(f, "EMERG"),
            log_level_t::ALERT => write!(f, "ALERT"),
            log_level_t::CRIT => write!(f, "CRIT"),
            log_level_t::ERROR => write!(f, "ERROR"),
            log_level_t::WARN => write!(f, "WARN"),
            log_level_t::NOTICE => write!(f, "NOTICE"),
            log_level_t::INFO => write!(f, "INFO"),
            log_level_t::DEBUG => write!(f, "DEBUG"),
        }
    }
}
