#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Vim package manager types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VimPackageManager {
    /// Vim-plug - https://github.com/junegunn/vim-plug
    VimPlug,
    /// Vundle - https://github.com/VundleVim/Vundle.vim
    Vundle,
    /// Pathogen - https://github.com/tpope/vim-pathogen
    Pathogen,
    /// Dein.vim - https://github.com/Shougo/dein.vim
    Dein,
    /// Native Vim 8+ package manager
    Native,
    /// Packer.nvim (Neovim) - https://github.com/wbthomason/packer.nvim
    Packer,
    /// Lazy.nvim (Neovim) - https://github.com/folke/lazy.nvim
    Lazy,
    /// vim-jetpack - https://github.com/tani/vim-jetpack
    Jetpack,
}

impl VimPackageManager {
    /// Get the package manager name as string
    pub fn as_str(&self) -> &str {
        match self {
            VimPackageManager::VimPlug => "vim-plug",
            VimPackageManager::Vundle => "vundle",
            VimPackageManager::Pathogen => "pathogen",
            VimPackageManager::Dein => "dein",
            VimPackageManager::Native => "native",
            VimPackageManager::Packer => "packer",
            VimPackageManager::Lazy => "lazy",
            VimPackageManager::Jetpack => "jetpack",
        }
    }

    /// Get the repository URL for the package manager
    pub fn repository_url(&self) -> &str {
        match self {
            VimPackageManager::VimPlug => "https://github.com/junegunn/vim-plug",
            VimPackageManager::Vundle => "https://github.com/VundleVim/Vundle.vim",
            VimPackageManager::Pathogen => "https://github.com/tpope/vim-pathogen",
            VimPackageManager::Dein => "https://github.com/Shougo/dein.vim",
            VimPackageManager::Native => "",
            VimPackageManager::Packer => "https://github.com/wbthomason/packer.nvim",
            VimPackageManager::Lazy => "https://github.com/folke/lazy.nvim",
            VimPackageManager::Jetpack => "https://github.com/tani/vim-jetpack",
        }
    }

    /// Get the installation path relative to Vim config directory
    pub fn install_path(&self) -> &str {
        match self {
            VimPackageManager::VimPlug => "autoload/plug.vim",
            VimPackageManager::Vundle => "bundle/Vundle.vim",
            VimPackageManager::Pathogen => "autoload/pathogen.vim",
            VimPackageManager::Dein => "repos/github.com/Shougo/dein.vim",
            VimPackageManager::Native => "pack",
            VimPackageManager::Packer => "pack/packer/start/packer.nvim",
            VimPackageManager::Lazy => "lazy/lazy.nvim",
            VimPackageManager::Jetpack => "pack/jetpack/opt/vim-jetpack",
        }
    }

    /// Check if this is a Neovim-specific package manager
    pub fn is_neovim_only(&self) -> bool {
        matches!(
            self,
            VimPackageManager::Packer | VimPackageManager::Lazy
        )
    }

    /// Get the command to install plugins
    pub fn install_command(&self) -> Vec<String> {
        match self {
            VimPackageManager::VimPlug => vec!["vim".to_string(), "+PlugInstall".to_string(), "+qall".to_string()],
            VimPackageManager::Vundle => vec!["vim".to_string(), "+PluginInstall".to_string(), "+qall".to_string()],
            VimPackageManager::Pathogen => vec![], // Manual installation
            VimPackageManager::Dein => vec!["vim".to_string(), "+call dein#install()".to_string(), "+qall".to_string()],
            VimPackageManager::Native => vec![], // Manual installation
            VimPackageManager::Packer => vec!["nvim".to_string(), "--headless".to_string(), "-c".to_string(), "PackerSync".to_string(), "-c".to_string(), "qa".to_string()],
            VimPackageManager::Lazy => vec!["nvim".to_string(), "--headless".to_string(), "+Lazy! sync".to_string(), "+qa".to_string()],
            VimPackageManager::Jetpack => vec!["vim".to_string(), "+JetpackSync".to_string(), "+qall".to_string()],
        }
    }

    /// Get the command to update plugins
    pub fn update_command(&self) -> Vec<String> {
        match self {
            VimPackageManager::VimPlug => vec!["vim".to_string(), "+PlugUpdate".to_string(), "+qall".to_string()],
            VimPackageManager::Vundle => vec!["vim".to_string(), "+PluginUpdate".to_string(), "+qall".to_string()],
            VimPackageManager::Pathogen => vec![], // Manual update
            VimPackageManager::Dein => vec!["vim".to_string(), "+call dein#update()".to_string(), "+qall".to_string()],
            VimPackageManager::Native => vec![], // Manual update
            VimPackageManager::Packer => vec!["nvim".to_string(), "--headless".to_string(), "-c".to_string(), "PackerSync".to_string(), "-c".to_string(), "qa".to_string()],
            VimPackageManager::Lazy => vec!["nvim".to_string(), "--headless".to_string(), "+Lazy! sync".to_string(), "+qa".to_string()],
            VimPackageManager::Jetpack => vec!["vim".to_string(), "+JetpackSync".to_string(), "+qall".to_string()],
        }
    }

    /// Get the command to clean unused plugins
    pub fn clean_command(&self) -> Vec<String> {
        match self {
            VimPackageManager::VimPlug => vec!["vim".to_string(), "+PlugClean!".to_string(), "+qall".to_string()],
            VimPackageManager::Vundle => vec!["vim".to_string(), "+PluginClean!".to_string(), "+qall".to_string()],
            VimPackageManager::Pathogen => vec![], // Manual cleanup
            VimPackageManager::Dein => vec!["vim".to_string(), "+call dein#recache_runtimepath()".to_string(), "+qall".to_string()],
            VimPackageManager::Native => vec![], // Manual cleanup
            VimPackageManager::Packer => vec!["nvim".to_string(), "--headless".to_string(), "-c".to_string(), "PackerClean".to_string(), "-c".to_string(), "qa".to_string()],
            VimPackageManager::Lazy => vec!["nvim".to_string(), "--headless".to_string(), "+Lazy! clean".to_string(), "+qa".to_string()],
            VimPackageManager::Jetpack => vec![], // No explicit clean command
        }
    }
}

/// Package manager configuration for an environment
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PACKAGE_MANAGER_CONFIG {
    pub environment_name: String,
    pub package_manager: VimPackageManager,
    pub installed: bool,
    pub install_path: PathBuf,
    pub plugins: Vec<PluginInfo>,
}

/// Information about an installed plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub source: String, // GitHub URL or other source
    pub installed: bool,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for PACKAGE_MANAGER_CONFIG {
    fn default() -> Self {
        Self {
            environment_name: String::new(),
            package_manager: VimPackageManager::VimPlug,
            installed: false,
            install_path: PathBuf::new(),
            plugins: Vec::new(),
        }
    }
}
