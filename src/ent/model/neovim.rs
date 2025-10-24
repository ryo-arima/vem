#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Neovim configuration for an environment
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NEOVIM_CONFIG {
    pub environment_name: String,
    pub enabled: bool,
    pub version: Option<String>,
    pub lua_config_enabled: bool,
    pub init_vim_path: Option<String>,
    pub init_lua_path: Option<String>,
    pub plugin_manager: Option<PluginManager>,
    pub features: Vec<NeovimFeature>,
    pub global_settings: HashMap<String, serde_json::Value>,
}

/// Plugin manager types for Neovim
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginManager {
    VimPlug,           // vim-plug
    Packer,            // packer.nvim
    Lazy,              // lazy.nvim
    Dein,              // dein.vim
    Paq,               // paq-nvim
    Jetpack,           // jetpack.nvim
}

/// Neovim features/integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeovimFeature {
    pub name: String,
    pub feature_type: NeovimFeatureType,
    pub enabled: bool,
    pub installed: bool,
    pub config: HashMap<String, serde_json::Value>,
}

/// Types of Neovim features
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum NeovimFeatureType {
    // Core features
    Treesitter,              // nvim-treesitter
    LSP,                     // built-in LSP
    DAP,                     // nvim-dap (Debug Adapter Protocol)
    
    // UI enhancements
    Telescope,               // telescope.nvim
    NvimTree,                // nvim-tree.lua
    NeoTree,                 // neo-tree.nvim
    BufferLine,              // bufferline.nvim
    Lualine,                 // lualine.nvim
    NvimNotify,              // nvim-notify
    NoiceNvim,               // noice.nvim
    WhichKey,                // which-key.nvim
    
    // Completion
    NvimCmp,                 // nvim-cmp
    CoqNvim,                 // coq_nvim
    
    // Git integration
    Gitsigns,                // gitsigns.nvim
    Neogit,                  // neogit
    Diffview,                // diffview.nvim
    
    // File management
    Harpoon,                 // harpoon
    OilNvim,                 // oil.nvim
    
    // Terminal
    Toggleterm,              // toggleterm.nvim
    
    // Session management
    AutoSession,             // auto-session
    Possession,              // possession.nvim
    
    // Editing enhancements
    Comment,                 // Comment.nvim
    Surround,                // nvim-surround
    Autopairs,               // nvim-autopairs
    IndentBlankline,         // indent-blankline.nvim
    
    // Language specific
    RustTools,               // rust-tools.nvim
    GoNvim,                  // go.nvim
    PythonDap,               // nvim-dap-python
    TsAutotag,               // nvim-ts-autotag
    
    // Snippets
    LuaSnip,                 // LuaSnip
    FriendlySnippets,        // friendly-snippets
    
    // Project management
    ProjectNvim,             // project.nvim
    
    // Documentation
    Neogen,                  // neogen (documentation generator)
    
    // Testing
    NvimTest,                // nvim-test
    Neotest,                 // neotest
    
    // Custom feature
    Custom(String),
}

impl PluginManager {
    pub fn display_name(&self) -> &str {
        match self {
            PluginManager::VimPlug => "vim-plug",
            PluginManager::Packer => "packer.nvim",
            PluginManager::Lazy => "lazy.nvim",
            PluginManager::Dein => "dein.vim",
            PluginManager::Paq => "paq-nvim",
            PluginManager::Jetpack => "jetpack.nvim",
        }
    }

    pub fn config_file(&self) -> &str {
        match self {
            PluginManager::VimPlug => "init.vim",
            PluginManager::Packer => "lua/plugins.lua",
            PluginManager::Lazy => "lua/plugins/init.lua",
            PluginManager::Dein => "init.vim",
            PluginManager::Paq => "lua/plugins.lua",
            PluginManager::Jetpack => "init.vim",
        }
    }

    pub fn is_lua_based(&self) -> bool {
        matches!(
            self,
            PluginManager::Packer | PluginManager::Lazy | PluginManager::Paq
        )
    }
}

impl NeovimFeatureType {
    pub fn plugin_name(&self) -> String {
        match self {
            NeovimFeatureType::Treesitter => "nvim-treesitter/nvim-treesitter".to_string(),
            NeovimFeatureType::LSP => "neovim/nvim-lspconfig".to_string(),
            NeovimFeatureType::DAP => "mfussenegger/nvim-dap".to_string(),
            NeovimFeatureType::Telescope => "nvim-telescope/telescope.nvim".to_string(),
            NeovimFeatureType::NvimTree => "nvim-tree/nvim-tree.lua".to_string(),
            NeovimFeatureType::NeoTree => "nvim-neo-tree/neo-tree.nvim".to_string(),
            NeovimFeatureType::BufferLine => "akinsho/bufferline.nvim".to_string(),
            NeovimFeatureType::Lualine => "nvim-lualine/lualine.nvim".to_string(),
            NeovimFeatureType::NvimNotify => "rcarriga/nvim-notify".to_string(),
            NeovimFeatureType::NoiceNvim => "folke/noice.nvim".to_string(),
            NeovimFeatureType::WhichKey => "folke/which-key.nvim".to_string(),
            NeovimFeatureType::NvimCmp => "hrsh7th/nvim-cmp".to_string(),
            NeovimFeatureType::CoqNvim => "ms-jpq/coq_nvim".to_string(),
            NeovimFeatureType::Gitsigns => "lewis6991/gitsigns.nvim".to_string(),
            NeovimFeatureType::Neogit => "TimUntersberger/neogit".to_string(),
            NeovimFeatureType::Diffview => "sindrets/diffview.nvim".to_string(),
            NeovimFeatureType::Harpoon => "ThePrimeagen/harpoon".to_string(),
            NeovimFeatureType::OilNvim => "stevearc/oil.nvim".to_string(),
            NeovimFeatureType::Toggleterm => "akinsho/toggleterm.nvim".to_string(),
            NeovimFeatureType::AutoSession => "rmagatti/auto-session".to_string(),
            NeovimFeatureType::Possession => "jedrzejboczar/possession.nvim".to_string(),
            NeovimFeatureType::Comment => "numToStr/Comment.nvim".to_string(),
            NeovimFeatureType::Surround => "kylechui/nvim-surround".to_string(),
            NeovimFeatureType::Autopairs => "windwp/nvim-autopairs".to_string(),
            NeovimFeatureType::IndentBlankline => "lukas-reineke/indent-blankline.nvim".to_string(),
            NeovimFeatureType::RustTools => "simrat39/rust-tools.nvim".to_string(),
            NeovimFeatureType::GoNvim => "ray-x/go.nvim".to_string(),
            NeovimFeatureType::PythonDap => "mfussenegger/nvim-dap-python".to_string(),
            NeovimFeatureType::TsAutotag => "windwp/nvim-ts-autotag".to_string(),
            NeovimFeatureType::LuaSnip => "L3MON4D3/LuaSnip".to_string(),
            NeovimFeatureType::FriendlySnippets => "rafamadriz/friendly-snippets".to_string(),
            NeovimFeatureType::ProjectNvim => "ahmedkhalf/project.nvim".to_string(),
            NeovimFeatureType::Neogen => "danymat/neogen".to_string(),
            NeovimFeatureType::NvimTest => "vim-test/vim-test".to_string(),
            NeovimFeatureType::Neotest => "nvim-neotest/neotest".to_string(),
            NeovimFeatureType::Custom(name) => name.clone(),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            NeovimFeatureType::Treesitter => "Treesitter",
            NeovimFeatureType::LSP => "LSP",
            NeovimFeatureType::DAP => "Debug Adapter",
            NeovimFeatureType::Telescope => "Telescope",
            NeovimFeatureType::NvimTree => "NvimTree",
            NeovimFeatureType::NeoTree => "NeoTree",
            NeovimFeatureType::BufferLine => "BufferLine",
            NeovimFeatureType::Lualine => "Lualine",
            NeovimFeatureType::NvimNotify => "Notify",
            NeovimFeatureType::NoiceNvim => "Noice",
            NeovimFeatureType::WhichKey => "WhichKey",
            NeovimFeatureType::NvimCmp => "nvim-cmp",
            NeovimFeatureType::CoqNvim => "coq.nvim",
            NeovimFeatureType::Gitsigns => "Gitsigns",
            NeovimFeatureType::Neogit => "Neogit",
            NeovimFeatureType::Diffview => "Diffview",
            NeovimFeatureType::Harpoon => "Harpoon",
            NeovimFeatureType::OilNvim => "Oil",
            NeovimFeatureType::Toggleterm => "Toggleterm",
            NeovimFeatureType::AutoSession => "Auto-Session",
            NeovimFeatureType::Possession => "Possession",
            NeovimFeatureType::Comment => "Comment",
            NeovimFeatureType::Surround => "Surround",
            NeovimFeatureType::Autopairs => "Auto Pairs",
            NeovimFeatureType::IndentBlankline => "Indent Blankline",
            NeovimFeatureType::RustTools => "Rust Tools",
            NeovimFeatureType::GoNvim => "Go.nvim",
            NeovimFeatureType::PythonDap => "Python DAP",
            NeovimFeatureType::TsAutotag => "TS Autotag",
            NeovimFeatureType::LuaSnip => "LuaSnip",
            NeovimFeatureType::FriendlySnippets => "Friendly Snippets",
            NeovimFeatureType::ProjectNvim => "Project.nvim",
            NeovimFeatureType::Neogen => "Neogen",
            NeovimFeatureType::NvimTest => "Nvim Test",
            NeovimFeatureType::Neotest => "Neotest",
            NeovimFeatureType::Custom(_) => "Custom Feature",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            NeovimFeatureType::Treesitter => "Advanced syntax highlighting and code understanding",
            NeovimFeatureType::LSP => "Built-in Language Server Protocol support",
            NeovimFeatureType::DAP => "Debug Adapter Protocol for debugging",
            NeovimFeatureType::Telescope => "Fuzzy finder and picker",
            NeovimFeatureType::NvimTree => "File explorer tree",
            NeovimFeatureType::NeoTree => "Modern file explorer",
            NeovimFeatureType::BufferLine => "Buffer line with tabs",
            NeovimFeatureType::Lualine => "Fast and customizable statusline",
            NeovimFeatureType::NvimNotify => "Notification manager",
            NeovimFeatureType::NoiceNvim => "UI for messages, cmdline and popupmenu",
            NeovimFeatureType::WhichKey => "Display keybindings",
            NeovimFeatureType::NvimCmp => "Completion engine",
            NeovimFeatureType::CoqNvim => "Fast completion engine",
            NeovimFeatureType::Gitsigns => "Git integration for buffers",
            NeovimFeatureType::Neogit => "Magit-like Git interface",
            NeovimFeatureType::Diffview => "Git diff viewer",
            NeovimFeatureType::Harpoon => "Quick file navigation",
            NeovimFeatureType::OilNvim => "File explorer as a buffer",
            NeovimFeatureType::Toggleterm => "Terminal management",
            NeovimFeatureType::AutoSession => "Automatic session management",
            NeovimFeatureType::Possession => "Session management",
            NeovimFeatureType::Comment => "Smart commenting",
            NeovimFeatureType::Surround => "Surround text objects",
            NeovimFeatureType::Autopairs => "Automatic bracket pairing",
            NeovimFeatureType::IndentBlankline => "Indent guides",
            NeovimFeatureType::RustTools => "Enhanced Rust support",
            NeovimFeatureType::GoNvim => "Enhanced Go support",
            NeovimFeatureType::PythonDap => "Python debugging support",
            NeovimFeatureType::TsAutotag => "Auto close and rename HTML tags",
            NeovimFeatureType::LuaSnip => "Snippet engine",
            NeovimFeatureType::FriendlySnippets => "Snippet collection",
            NeovimFeatureType::ProjectNvim => "Project management",
            NeovimFeatureType::Neogen => "Documentation generator",
            NeovimFeatureType::NvimTest => "Test runner",
            NeovimFeatureType::Neotest => "Testing framework",
            NeovimFeatureType::Custom(_) => "Custom Neovim feature",
        }
    }

    pub fn requires_dependencies(&self) -> Vec<String> {
        match self {
            NeovimFeatureType::Telescope => vec![
                "nvim-lua/plenary.nvim".to_string(),
                "nvim-telescope/telescope-fzf-native.nvim".to_string(),
            ],
            NeovimFeatureType::NeoTree => vec![
                "nvim-lua/plenary.nvim".to_string(),
                "nvim-tree/nvim-web-devicons".to_string(),
                "MunifTanjim/nui.nvim".to_string(),
            ],
            NeovimFeatureType::Neogit => vec![
                "nvim-lua/plenary.nvim".to_string(),
                "sindrets/diffview.nvim".to_string(),
            ],
            NeovimFeatureType::NoiceNvim => vec![
                "MunifTanjim/nui.nvim".to_string(),
                "rcarriga/nvim-notify".to_string(),
            ],
            NeovimFeatureType::BufferLine => vec![
                "nvim-tree/nvim-web-devicons".to_string(),
            ],
            NeovimFeatureType::Lualine => vec![
                "nvim-tree/nvim-web-devicons".to_string(),
            ],
            NeovimFeatureType::GoNvim => vec![
                "ray-x/guihua.lua".to_string(),
            ],
            _ => Vec::new(),
        }
    }
}

impl NeovimFeature {
    pub fn from_type(feature_type: NeovimFeatureType) -> Self {
        Self {
            name: feature_type.display_name().to_string(),
            feature_type,
            enabled: true,
            installed: false,
            config: HashMap::new(),
        }
    }

    pub fn new(name: String, feature_type: NeovimFeatureType) -> Self {
        Self {
            name,
            feature_type,
            enabled: true,
            installed: false,
            config: HashMap::new(),
        }
    }
}
