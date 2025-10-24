#![allow(non_camel_case_types)]

use std::fs;
use std::ops::Deref;
use std::path::{
    Path,
    PathBuf
};
use std::process::Command;

use crate::cnf::application::app_config;
use crate::util::error::vem_error_t;
use crate::ent::model::ctags::{
    CTAGS_CONFIG,
    CTAGS_INDEX,
};

// Re-use RepositoryConfig from environment.rs
use super::environment::RepositoryConfig;

/// Ctags repository trait
pub trait CtagsRepository {
    /// Generate tags for an environment
    fn generate_tags(&self, env_name: &str, config: &CTAGS_CONFIG) -> Result<CTAGS_INDEX, vem_error_t>;
    
    /// Update existing tags
    fn update_tags(&self, env_name: &str) -> Result<CTAGS_INDEX, vem_error_t>;
    
    /// Get tag index information
    fn get_tag_info(&self, env_name: &str) -> Result<CTAGS_INDEX, vem_error_t>;
    
    /// Clean/remove tags
    fn clean_tags(&self, env_name: &str) -> bool;
    
    /// Load ctags configuration
    fn load_config(&self, env_name: &str) -> Result<CTAGS_CONFIG, vem_error_t>;
    
    /// Save ctags configuration
    fn save_config(&self, env_name: &str, config: &CTAGS_CONFIG) -> Result<(), vem_error_t>;
}

/// Ctags repository implementation with embedded config (Go-style)
pub struct ctags_repository {
    base: RepositoryConfig,  // Embedded base struct (like Go)
}

impl ctags_repository {
    pub fn new(config: app_config) -> Self {
        Self {
            base: RepositoryConfig::new(config),
        }
    }

    /// Get tag file path for an environment
    fn get_tag_file_path(&self, env_name: &str) -> PathBuf {
        self.config()
            .environment_root()
            .join(env_name)
            .join(".vim")
            .join("tags")
            .join("tags")
    }

    /// Get config file path for an environment
    fn get_config_file_path(&self, env_name: &str) -> PathBuf {
        self.config()
            .environment_root()
            .join(env_name)
            .join("ctags_config.toml")
    }

    /// Count tags in a tag file
    fn count_tags(&self, tag_file: &Path) -> Result<usize, vem_error_t> {
        if !tag_file.exists() {
            return Ok(0);
        }

        let content = fs::read_to_string(tag_file)?;
        let count = content
            .lines()
            .filter(|line| !line.starts_with('!')) // Exclude pseudo tags
            .count();
        
        Ok(count)
    }

    /// Extract languages from tag file pseudo tags
    fn extract_languages(&self, tag_file: &Path) -> Result<Vec<String>, vem_error_t> {
        if !tag_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(tag_file)?;
        let mut languages = Vec::new();

        for line in content.lines() {
            if line.starts_with("!_TAG_KIND_DESCRIPTION!") {
                // Extract language from pseudo tag like: !_TAG_KIND_DESCRIPTION!C
                if let Some(lang_part) = line.strip_prefix("!_TAG_KIND_DESCRIPTION!") {
                    if let Some(lang) = lang_part.split('\t').next() {
                        if !languages.contains(&lang.to_string()) {
                            languages.push(lang.to_string());
                        }
                    }
                }
            }
        }

        Ok(languages)
    }

    /// Build ctags command
    fn build_ctags_command(&self, config: &CTAGS_CONFIG, output_file: &Path) -> Command {
        let mut cmd = Command::new("ctags");
        
        // Output format
        cmd.arg(format!("--output-format={}", config.output_format.as_str()));
        
        // Output file
        cmd.arg("-o");
        cmd.arg(output_file);
        
        // Recursive
        if config.recursive {
            cmd.arg("-R");
        }
        
        // Languages
        if !config.languages.is_empty() {
            cmd.arg(format!("--languages={}", config.languages.join(",")));
        }
        
        // Exclude patterns
        for pattern in &config.exclude_patterns {
            cmd.arg(format!("--exclude={}", pattern));
        }
        
        // Extra options
        for option in &config.extra_options {
            cmd.arg(option);
        }
        
        // Extra pseudo tags for information
        cmd.arg("--extras=+p");
        
        // Target paths
        for path in &config.target_paths {
            cmd.arg(path);
        }
        
        cmd
    }
}

// Deref implementation for automatic access to base methods (like Go's embedding)
impl Deref for ctags_repository {
    type Target = RepositoryConfig;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl CtagsRepository for ctags_repository {
    /// Generate tags for an environment
    fn generate_tags(&self, env_name: &str, config: &CTAGS_CONFIG) -> Result<CTAGS_INDEX, vem_error_t> {
        let tag_file = self.get_tag_file_path(env_name);
        
        // Ensure tag directory exists
        if let Some(parent) = tag_file.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Build and execute ctags command
        let mut cmd = self.build_ctags_command(config, &tag_file);
        
        let output = cmd.output()
            .map_err(|e| vem_error_t::CommandExecutionError(format!("Failed to execute ctags: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(vem_error_t::CommandExecutionError(
                format!("ctags failed: {}", stderr)
            ));
        }
        
        // Count tags and extract information
        let tag_count = self.count_tags(&tag_file)?;
        let languages = self.extract_languages(&tag_file)?;
        
        let index = CTAGS_INDEX {
            tag_file_path: tag_file,
            indexed_paths: config.target_paths.clone(),
            last_generated: chrono::Utc::now(),
            tag_count,
            languages,
        };
        
        Ok(index)
    }
    
    /// Update existing tags (re-generate with saved config)
    fn update_tags(&self, env_name: &str) -> Result<CTAGS_INDEX, vem_error_t> {
        let config = self.load_config(env_name)?;
        self.generate_tags(env_name, &config)
    }
    
    /// Get tag index information
    fn get_tag_info(&self, env_name: &str) -> Result<CTAGS_INDEX, vem_error_t> {
        let tag_file = self.get_tag_file_path(env_name);
        
        if !tag_file.exists() {
            return Ok(CTAGS_INDEX::default());
        }
        
        let tag_count = self.count_tags(&tag_file)?;
        let languages = self.extract_languages(&tag_file)?;
        
        let metadata = fs::metadata(&tag_file)?;
        let modified = metadata.modified()?;
        let last_generated: chrono::DateTime<chrono::Utc> = modified.into();
        
        let config = self.load_config(env_name).unwrap_or_default();
        
        Ok(CTAGS_INDEX {
            tag_file_path: tag_file,
            indexed_paths: config.target_paths,
            last_generated,
            tag_count,
            languages,
        })
    }
    
    /// Clean/remove tags
    fn clean_tags(&self, env_name: &str) -> bool {
        let tag_file = self.get_tag_file_path(env_name);
        
        if !tag_file.exists() {
            return true;
        }
        
        fs::remove_file(&tag_file).is_ok()
    }
    
    /// Load ctags configuration
    fn load_config(&self, env_name: &str) -> Result<CTAGS_CONFIG, vem_error_t> {
        let config_file = self.get_config_file_path(env_name);
        
        if !config_file.exists() {
            return Ok(CTAGS_CONFIG {
                environment_name: env_name.to_string(),
                ..Default::default()
            });
        }
        
        let content = fs::read_to_string(&config_file)?;
        let config: CTAGS_CONFIG = toml::from_str(&content)
            .map_err(|e| vem_error_t::SerializationError(format!("Failed to parse ctags config: {}", e)))?;
        
        Ok(config)
    }
    
    /// Save ctags configuration
    fn save_config(&self, env_name: &str, config: &CTAGS_CONFIG) -> Result<(), vem_error_t> {
        let config_file = self.get_config_file_path(env_name);
        
        // Ensure parent directory exists
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(config)
            .map_err(|e| vem_error_t::SerializationError(format!("Failed to serialize ctags config: {}", e)))?;
        
        fs::write(&config_file, content)?;
        
        Ok(())
    }
}

/// Factory function to create ctags repository
pub fn new(config: app_config) -> impl CtagsRepository {
    ctags_repository::new(config)
}
