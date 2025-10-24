#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Ctags configuration for an environment
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTAGS_CONFIG {
    pub environment_name: String,
    pub target_paths: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub languages: Vec<String>,
    pub recursive: bool,
    pub extra_options: Vec<String>,
    pub output_format: CtagsOutputFormat,
}

/// Ctags index information
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTAGS_INDEX {
    pub tag_file_path: PathBuf,
    pub indexed_paths: Vec<PathBuf>,
    pub last_generated: chrono::DateTime<chrono::Utc>,
    pub tag_count: usize,
    pub languages: Vec<String>,
}

/// Ctags output format
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CtagsOutputFormat {
    #[default]
    UCTags,    // u-ctags (default)
    ECTags,    // e-ctags
    Etags,     // Emacs etags
    Xref,      // Cross-reference
    Json,      // JSON format
}

impl CtagsOutputFormat {
    pub fn as_str(&self) -> &str {
        match self {
            CtagsOutputFormat::UCTags => "u-ctags",
            CtagsOutputFormat::ECTags => "e-ctags",
            CtagsOutputFormat::Etags => "etags",
            CtagsOutputFormat::Xref => "xref",
            CtagsOutputFormat::Json => "json",
        }
    }
}

impl Default for CTAGS_CONFIG {
    fn default() -> Self {
        Self {
            environment_name: String::new(),
            target_paths: Vec::new(),
            exclude_patterns: Vec::new(),
            languages: Vec::new(),
            recursive: true,
            extra_options: Vec::new(),
            output_format: CtagsOutputFormat::default(),
        }
    }
}

impl Default for CTAGS_INDEX {
    fn default() -> Self {
        Self {
            tag_file_path: PathBuf::new(),
            indexed_paths: Vec::new(),
            last_generated: chrono::Utc::now(),
            tag_count: 0,
            languages: Vec::new(),
        }
    }
}
