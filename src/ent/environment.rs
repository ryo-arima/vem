use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a VEM environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Environment name
    pub name: String,
    /// Path to the environment directory
    pub path: PathBuf,
    /// Environment metadata
    pub meta: EnvironmentMeta,
}

/// Environment metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentMeta {
    /// Human-readable description
    pub description: Option<String>,
    /// Creation timestamp
    pub created: chrono::DateTime<chrono::Utc>,
    /// Last used timestamp
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    /// Environment tags
    pub tags: Vec<String>,
    /// Custom settings
    pub settings: serde_json::Value,
}

impl Environment {
    pub fn new<S: Into<String>>(name: S, path: PathBuf) -> Self {
        Self {
            name: name.into(),
            path,
            meta: EnvironmentMeta {
                description: None,
                created: chrono::Utc::now(),
                last_used: None,
                tags: Vec::new(),
                settings: serde_json::Value::Object(serde_json::Map::new()),
            },
        }
    }

    /// Get the path to the .vimrc file
    pub fn vimrc_path(&self) -> PathBuf {
        self.path.join(".vimrc")
    }

    /// Get the path to the .vim directory
    pub fn vim_dir_path(&self) -> PathBuf {
        self.path.join(".vim")
    }

    /// Get the path to the metadata file
    pub fn meta_path(&self) -> PathBuf {
        self.path.join("meta.json")
    }

    /// Validate environment name
    pub fn is_valid_name(name: &str) -> bool {
        !name.is_empty()
            && !name.contains('/')
            && !name.contains('\\')
            && !name.contains(' ')
            && name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}