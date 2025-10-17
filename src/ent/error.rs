use std::fmt;

/// VEM error types
#[derive(Debug)]
pub enum VemError {
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
    SerializationError(serde_json::Error),
    /// No current environment set
    NoCurrentEnvironment,
}

impl fmt::Display for VemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VemError::EnvironmentNotFound(name) => {
                write!(f, "Environment '{}' not found", name)
            }
            VemError::EnvironmentAlreadyExists(name) => {
                write!(f, "Environment '{}' already exists", name)
            }
            VemError::InvalidEnvironmentName(name) => {
                write!(f, "Invalid environment name: '{}'", name)
            }
            VemError::FileSystemError(err) => {
                write!(f, "File system error: {}", err)
            }
            VemError::ConfigurationError(msg) => {
                write!(f, "Configuration error: {}", msg)
            }
            VemError::SerializationError(err) => {
                write!(f, "Serialization error: {}", err)
            }
            VemError::NoCurrentEnvironment => {
                write!(f, "No current environment is set")
            }
        }
    }
}

impl std::error::Error for VemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            VemError::FileSystemError(err) => Some(err),
            VemError::SerializationError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for VemError {
    fn from(err: std::io::Error) -> Self {
        VemError::FileSystemError(err)
    }
}

impl From<serde_json::Error> for VemError {
    fn from(err: serde_json::Error) -> Self {
        VemError::SerializationError(err)
    }
}

/// Exit codes for VEM commands
pub enum ExitCode {
    Success = 0,
    GeneralError = 1,
    InvalidArguments = 2,
    EnvironmentNotFound = 3,
    EnvironmentAlreadyExists = 4,
}

impl From<VemError> for ExitCode {
    fn from(err: VemError) -> Self {
        match err {
            VemError::EnvironmentNotFound(_) => ExitCode::EnvironmentNotFound,
            VemError::EnvironmentAlreadyExists(_) => ExitCode::EnvironmentAlreadyExists,
            VemError::InvalidEnvironmentName(_) => ExitCode::InvalidArguments,
            _ => ExitCode::GeneralError,
        }
    }
}

impl From<&VemError> for ExitCode {
    fn from(err: &VemError) -> Self {
        match err {
            VemError::EnvironmentNotFound(_) => ExitCode::EnvironmentNotFound,
            VemError::EnvironmentAlreadyExists(_) => ExitCode::EnvironmentAlreadyExists,
            VemError::InvalidEnvironmentName(_) => ExitCode::InvalidArguments,
            _ => ExitCode::GeneralError,
        }
    }
}