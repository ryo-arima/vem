use std::fmt;

/// VEM error types
#[derive(Debug)]
pub enum vem_error_t {
    /// Environment not found
    EnvironmentNotFound(String),
    /// Environment already exists
    EnvironmentAlreadyExists(String),
    /// Invalid environment name
    InvalidEnvironmentName(String),
    /// File system error
    FileSystemError(std::io::Error),
    /// Configuration error
    ConfigurationError(String),
    /// Serialization error
    SerializationError(String),
    /// No current environment set
    NoCurrentEnvironment,
}

impl fmt::Display for vem_error_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            vem_error_t::EnvironmentNotFound(name) => {
                write!(f, "Environment '{}' not found", name)
            }
            vem_error_t::EnvironmentAlreadyExists(name) => {
                write!(f, "Environment '{}' already exists", name)
            }
            vem_error_t::InvalidEnvironmentName(name) => {
                write!(f, "Invalid environment name: '{}'", name)
            }
            vem_error_t::FileSystemError(err) => {
                write!(f, "File system error: {}", err)
            }
            vem_error_t::ConfigurationError(msg) => {
                write!(f, "Configuration error: {}", msg)
            }
            vem_error_t::SerializationError(err) => {
                write!(f, "Serialization error: {}", err)
            }
            vem_error_t::NoCurrentEnvironment => {
                write!(f, "No current environment is set")
            }
        }
    }
}

impl std::error::Error for vem_error_t {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            vem_error_t::FileSystemError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for vem_error_t {
    fn from(err: std::io::Error) -> Self {
        vem_error_t::FileSystemError(err)
    }
}

/// Exit codes for VEM commands
#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
pub enum exit_code_t {
    SUCCESS = 0,
    GENERAL_ERROR = 1,
    INVALID_ARGUMENTS = 2,
    ENVIRONMENT_NOT_FOUND = 3,
    ENVIRONMENT_ALREADY_EXISTS = 4,
}

impl From<vem_error_t> for exit_code_t {
    fn from(err: vem_error_t) -> Self {
        match err {
            vem_error_t::EnvironmentNotFound(_) => exit_code_t::ENVIRONMENT_NOT_FOUND,
            vem_error_t::EnvironmentAlreadyExists(_) => exit_code_t::ENVIRONMENT_ALREADY_EXISTS,
            vem_error_t::InvalidEnvironmentName(_) => exit_code_t::INVALID_ARGUMENTS,
            _ => exit_code_t::GENERAL_ERROR,
        }
    }
}

impl From<&vem_error_t> for exit_code_t {
    fn from(err: &vem_error_t) -> Self {
        match err {
            vem_error_t::EnvironmentNotFound(_) => exit_code_t::ENVIRONMENT_NOT_FOUND,
            vem_error_t::EnvironmentAlreadyExists(_) => exit_code_t::ENVIRONMENT_ALREADY_EXISTS,
            vem_error_t::InvalidEnvironmentName(_) => exit_code_t::INVALID_ARGUMENTS,
            _ => exit_code_t::GENERAL_ERROR,
        }
    }
}
