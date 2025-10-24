#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme configuration for an environment
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct THEME_CONFIG {
    pub environment_name: String,
    pub active_theme: Option<String>,
    pub themes: Vec<Theme>,
    pub global_settings: HashMap<String, serde_json::Value>,
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub theme_type: ThemeType,
    pub enabled: bool,
    pub installed: bool,
    pub background: ThemeBackground,
    pub settings: HashMap<String, serde_json::Value>,
}

/// Theme background type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThemeBackground {
    Dark,
    Light,
}

/// Types of themes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThemeType {
    // Popular color schemes
    Gruvbox,                 // gruvbox
    Nord,                    // nord
    Dracula,                 // dracula
    OneDark,                 // onedark
    Solarized,               // solarized
    Monokai,                 // monokai
    TokyoNight,              // tokyonight
    Catppuccin,              // catppuccin
    Nightfox,                // nightfox
    Kanagawa,                // kanagawa
    
    // Material themes
    Material,                // material
    MaterialDeep,            // material-deep-ocean
    MaterialPalenight,       // material-palenight
    
    // Minimal themes
    Everforest,              // everforest
    Edge,                    // edge
    Sonokai,                 // sonokai
    
    // Classic themes
    Jellybeans,              // jellybeans
    Zenburn,                 // zenburn
    PaperColor,              // papercolor
    
    // Modern themes
    Nightfly,                // nightfly
    Moonfly,                 // moonfly
    OceanicNext,             // oceanic-next
    Palenight,               // palenight
    SpaceVim,                // spacevim
    
    // Pastel themes
    Ayu,                     // ayu
    Iceberg,                 // iceberg
    Apprentice,              // apprentice
    
    // High contrast
    OneDarkPro,              // onedark-pro
    GithubTheme,             // github-theme
    VscodeTheme,             // vscode-theme
    
    // Neovim specific
    NvimTreesitter,          // nvim-treesitter themes
    Nightfal,                // nightfal
    
    // Airline/lightline themes
    AirlineTheme,            // airline theme
    LightlineTheme,          // lightline theme
    
    // Custom theme
    Custom(String),
}

impl ThemeType {
    /// Get the plugin/repository name
    pub fn plugin_name(&self) -> String {
        match self {
            ThemeType::Gruvbox => "morhetz/gruvbox".to_string(),
            ThemeType::Nord => "arcticicestudio/nord-vim".to_string(),
            ThemeType::Dracula => "dracula/vim".to_string(),
            ThemeType::OneDark => "joshdick/onedark.vim".to_string(),
            ThemeType::Solarized => "altercation/vim-colors-solarized".to_string(),
            ThemeType::Monokai => "crusoexia/vim-monokai".to_string(),
            ThemeType::TokyoNight => "folke/tokyonight.nvim".to_string(),
            ThemeType::Catppuccin => "catppuccin/nvim".to_string(),
            ThemeType::Nightfox => "EdenEast/nightfox.nvim".to_string(),
            ThemeType::Kanagawa => "rebelot/kanagawa.nvim".to_string(),
            ThemeType::Material => "kaicataldo/material.vim".to_string(),
            ThemeType::MaterialDeep => "marko-cerovac/material.nvim".to_string(),
            ThemeType::MaterialPalenight => "drewtempelmeyer/palenight.vim".to_string(),
            ThemeType::Everforest => "sainnhe/everforest".to_string(),
            ThemeType::Edge => "sainnhe/edge".to_string(),
            ThemeType::Sonokai => "sainnhe/sonokai".to_string(),
            ThemeType::Jellybeans => "nanotech/jellybeans.vim".to_string(),
            ThemeType::Zenburn => "jnurmine/Zenburn".to_string(),
            ThemeType::PaperColor => "NLKNguyen/papercolor-theme".to_string(),
            ThemeType::Nightfly => "bluz71/vim-nightfly-colors".to_string(),
            ThemeType::Moonfly => "bluz71/vim-moonfly-colors".to_string(),
            ThemeType::OceanicNext => "mhartington/oceanic-next".to_string(),
            ThemeType::Palenight => "drewtempelmeyer/palenight.vim".to_string(),
            ThemeType::SpaceVim => "SpaceVim/SpaceVim".to_string(),
            ThemeType::Ayu => "ayu-theme/ayu-vim".to_string(),
            ThemeType::Iceberg => "cocopon/iceberg.vim".to_string(),
            ThemeType::Apprentice => "romainl/Apprentice".to_string(),
            ThemeType::OneDarkPro => "olimorris/onedarkpro.nvim".to_string(),
            ThemeType::GithubTheme => "projekt0n/github-nvim-theme".to_string(),
            ThemeType::VscodeTheme => "Mofiqul/vscode.nvim".to_string(),
            ThemeType::NvimTreesitter => "nvim-treesitter/nvim-treesitter".to_string(),
            ThemeType::Nightfal => "Yazeed1s/oh-lucy.nvim".to_string(),
            ThemeType::AirlineTheme => "vim-airline/vim-airline-themes".to_string(),
            ThemeType::LightlineTheme => "itchyny/lightline.vim".to_string(),
            ThemeType::Custom(name) => name.clone(),
        }
    }

    /// Get the colorscheme command name
    pub fn colorscheme_name(&self) -> String {
        match self {
            ThemeType::Gruvbox => "gruvbox".to_string(),
            ThemeType::Nord => "nord".to_string(),
            ThemeType::Dracula => "dracula".to_string(),
            ThemeType::OneDark => "onedark".to_string(),
            ThemeType::Solarized => "solarized".to_string(),
            ThemeType::Monokai => "monokai".to_string(),
            ThemeType::TokyoNight => "tokyonight".to_string(),
            ThemeType::Catppuccin => "catppuccin".to_string(),
            ThemeType::Nightfox => "nightfox".to_string(),
            ThemeType::Kanagawa => "kanagawa".to_string(),
            ThemeType::Material => "material".to_string(),
            ThemeType::MaterialDeep => "material".to_string(),
            ThemeType::MaterialPalenight => "palenight".to_string(),
            ThemeType::Everforest => "everforest".to_string(),
            ThemeType::Edge => "edge".to_string(),
            ThemeType::Sonokai => "sonokai".to_string(),
            ThemeType::Jellybeans => "jellybeans".to_string(),
            ThemeType::Zenburn => "zenburn".to_string(),
            ThemeType::PaperColor => "PaperColor".to_string(),
            ThemeType::Nightfly => "nightfly".to_string(),
            ThemeType::Moonfly => "moonfly".to_string(),
            ThemeType::OceanicNext => "OceanicNext".to_string(),
            ThemeType::Palenight => "palenight".to_string(),
            ThemeType::SpaceVim => "SpaceVim".to_string(),
            ThemeType::Ayu => "ayu".to_string(),
            ThemeType::Iceberg => "iceberg".to_string(),
            ThemeType::Apprentice => "apprentice".to_string(),
            ThemeType::OneDarkPro => "onedarkpro".to_string(),
            ThemeType::GithubTheme => "github_dark".to_string(),
            ThemeType::VscodeTheme => "vscode".to_string(),
            ThemeType::NvimTreesitter => "".to_string(),
            ThemeType::Nightfal => "oh-lucy".to_string(),
            ThemeType::AirlineTheme => "".to_string(),
            ThemeType::LightlineTheme => "".to_string(),
            ThemeType::Custom(name) => name.clone(),
        }
    }

    /// Get the display name
    pub fn display_name(&self) -> &str {
        match self {
            ThemeType::Gruvbox => "Gruvbox",
            ThemeType::Nord => "Nord",
            ThemeType::Dracula => "Dracula",
            ThemeType::OneDark => "One Dark",
            ThemeType::Solarized => "Solarized",
            ThemeType::Monokai => "Monokai",
            ThemeType::TokyoNight => "Tokyo Night",
            ThemeType::Catppuccin => "Catppuccin",
            ThemeType::Nightfox => "Nightfox",
            ThemeType::Kanagawa => "Kanagawa",
            ThemeType::Material => "Material",
            ThemeType::MaterialDeep => "Material Deep Ocean",
            ThemeType::MaterialPalenight => "Material Palenight",
            ThemeType::Everforest => "Everforest",
            ThemeType::Edge => "Edge",
            ThemeType::Sonokai => "Sonokai",
            ThemeType::Jellybeans => "Jellybeans",
            ThemeType::Zenburn => "Zenburn",
            ThemeType::PaperColor => "PaperColor",
            ThemeType::Nightfly => "Nightfly",
            ThemeType::Moonfly => "Moonfly",
            ThemeType::OceanicNext => "Oceanic Next",
            ThemeType::Palenight => "Palenight",
            ThemeType::SpaceVim => "SpaceVim",
            ThemeType::Ayu => "Ayu",
            ThemeType::Iceberg => "Iceberg",
            ThemeType::Apprentice => "Apprentice",
            ThemeType::OneDarkPro => "One Dark Pro",
            ThemeType::GithubTheme => "GitHub Theme",
            ThemeType::VscodeTheme => "VSCode Theme",
            ThemeType::NvimTreesitter => "Treesitter Themes",
            ThemeType::Nightfal => "Nightfal",
            ThemeType::AirlineTheme => "Airline Themes",
            ThemeType::LightlineTheme => "Lightline Themes",
            ThemeType::Custom(_) => "Custom Theme",
        }
    }

    /// Get description
    pub fn description(&self) -> &str {
        match self {
            ThemeType::Gruvbox => "Retro groove color scheme",
            ThemeType::Nord => "Arctic, north-bluish color palette",
            ThemeType::Dracula => "Dark theme with vibrant colors",
            ThemeType::OneDark => "Atom's One Dark theme",
            ThemeType::Solarized => "Precision colors for machines and people",
            ThemeType::Monokai => "Sublime Text's iconic color scheme",
            ThemeType::TokyoNight => "Clean, dark theme inspired by Tokyo's night",
            ThemeType::Catppuccin => "Soothing pastel theme",
            ThemeType::Nightfox => "Highly customizable theme",
            ThemeType::Kanagawa => "Theme inspired by Kanagawa's famous painting",
            ThemeType::Material => "Material design color scheme",
            ThemeType::MaterialDeep => "Material deep ocean variant",
            ThemeType::MaterialPalenight => "Material palenight variant",
            ThemeType::Everforest => "Comfortable & pleasant theme",
            ThemeType::Edge => "Clean & elegant theme",
            ThemeType::Sonokai => "High contrast & vivid theme",
            ThemeType::Jellybeans => "Colorful, dark theme",
            ThemeType::Zenburn => "Low-contrast color scheme",
            ThemeType::PaperColor => "Light & dark theme inspired by Google's design",
            ThemeType::Nightfly => "Dark theme with modern aesthetics",
            ThemeType::Moonfly => "Dark charcoal theme",
            ThemeType::OceanicNext => "Oceanic color scheme",
            ThemeType::Palenight => "Material palenight variant",
            ThemeType::SpaceVim => "SpaceVim color scheme",
            ThemeType::Ayu => "Simple, bright and elegant theme",
            ThemeType::Iceberg => "Dark blue color scheme",
            ThemeType::Apprentice => "Dark, low-contrast colorscheme",
            ThemeType::OneDarkPro => "One Dark Pro theme for Neovim",
            ThemeType::GithubTheme => "GitHub's color schemes",
            ThemeType::VscodeTheme => "VSCode's color schemes",
            ThemeType::NvimTreesitter => "Treesitter-based themes",
            ThemeType::Nightfal => "Minimalist dark theme",
            ThemeType::AirlineTheme => "Themes for vim-airline",
            ThemeType::LightlineTheme => "Themes for lightline",
            ThemeType::Custom(_) => "Custom theme",
        }
    }

    /// Check if this requires Neovim
    pub fn requires_neovim(&self) -> bool {
        matches!(
            self,
            ThemeType::TokyoNight
                | ThemeType::Catppuccin
                | ThemeType::Nightfox
                | ThemeType::Kanagawa
                | ThemeType::MaterialDeep
                | ThemeType::OneDarkPro
                | ThemeType::GithubTheme
                | ThemeType::VscodeTheme
                | ThemeType::NvimTreesitter
                | ThemeType::Nightfal
        )
    }

    /// Get default background
    pub fn default_background(&self) -> ThemeBackground {
        match self {
            ThemeType::PaperColor | ThemeType::Solarized => ThemeBackground::Light,
            _ => ThemeBackground::Dark,
        }
    }
}

impl Theme {
    pub fn from_type(theme_type: ThemeType) -> Self {
        let background = theme_type.default_background();
        Self {
            name: theme_type.display_name().to_string(),
            theme_type: theme_type.clone(),
            enabled: true,
            installed: false,
            background,
            settings: HashMap::new(),
        }
    }

    pub fn new(name: String, theme_type: ThemeType, background: ThemeBackground) -> Self {
        Self {
            name,
            theme_type,
            enabled: true,
            installed: false,
            background,
            settings: HashMap::new(),
        }
    }
}
