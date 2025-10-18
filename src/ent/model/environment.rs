use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Environment metadata stored in meta.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct environment_meta_t {
    description: Option<String>,
    created: chrono::DateTime<chrono::Utc>,
    last_used: Option<chrono::DateTime<chrono::Utc>>,
    tags: Vec<String>,
}

impl environment_meta_t {
    pub fn new(description: Option<String>) -> Self {
        Self {
            description,
            created: chrono::Utc::now(),
            last_used: None,
            tags: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    #[allow(dead_code)]
    pub fn created(&self) -> chrono::DateTime<chrono::Utc> {
        self.created
    }

    #[allow(dead_code)]
    pub fn last_used(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.last_used
    }

    pub fn set_last_used(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.last_used = ts;
    }

    #[allow(dead_code)]
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    #[allow(dead_code)]
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    #[allow(dead_code)]
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }
}

/// Represents a VEM environment
#[derive(Debug, Clone)]
pub struct environment_t {
    name: String,
    path: PathBuf,
    meta: environment_meta_t,
}

impl environment_t {
    pub fn new<S: Into<String>>(name: S, path: PathBuf, meta: environment_meta_t) -> Self {
        Self {
            name: name.into(),
            path,
            meta,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn meta(&self) -> &environment_meta_t {
        &self.meta
    }

    pub fn meta_mut(&mut self) -> &mut environment_meta_t {
        &mut self.meta
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
        self.path.join("meta.toml")
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
