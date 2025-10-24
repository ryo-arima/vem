#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// coc.nvim configuration for an environment
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct COC_CONFIG {
    pub environment_name: String,
    pub installed: bool,
    pub extensions: Vec<CocExtension>,
    pub settings: HashMap<String, serde_json::Value>,
    pub config_path: PathBuf,
}

/// Information about a coc.nvim extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CocExtension {
    pub name: String,
    pub version: Option<String>,
    pub installed: bool,
    pub enabled: bool,
    pub description: Option<String>,
}

/// Common coc.nvim extensions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CocExtensionType {
    // Language servers
    CocTsserver,      // TypeScript/JavaScript
    CocJson,          // JSON
    CocHtml,          // HTML
    CocCss,           // CSS/SCSS/Less
    CocPython,        // Python (Jedi)
    CocPylsp,         // Python LSP
    CocRust,          // Rust (deprecated, use rust-analyzer)
    CocRustAnalyzer,  // Rust Analyzer
    CocGo,            // Go
    CocJava,          // Java
    CocClangd,        // C/C++/Objective-C
    CocSolargraph,    // Ruby
    CocYaml,          // YAML
    CocXml,           // XML
    CocPhp,           // PHP
    CocElixir,        // Elixir
    
    // Snippets and completion
    CocSnippets,      // Snippets support
    CocUltisnips,     // UltiSnips integration
    CocPairs,         // Auto pairs
    CocEmmet,         // Emmet
    
    // Linters and formatters
    CocPrettier,      // Prettier formatter
    CocEslint,        // ESLint
    CocStylelint,     // Stylelint
    CocDiagnostic,    // Diagnostic support
    
    // Git
    CocGit,           // Git integration
    
    // Tools
    CocExplorer,      // File explorer
    CocLists,         // List support
    CocYank,          // Yank history
    CocMarketplace,   // Extension marketplace
    
    // Custom extension
    Custom(String),
}

impl CocExtensionType {
    /// Get the npm package name for the extension
    pub fn package_name(&self) -> String {
        match self {
            CocExtensionType::CocTsserver => "coc-tsserver".to_string(),
            CocExtensionType::CocJson => "coc-json".to_string(),
            CocExtensionType::CocHtml => "coc-html".to_string(),
            CocExtensionType::CocCss => "coc-css".to_string(),
            CocExtensionType::CocPython => "coc-python".to_string(),
            CocExtensionType::CocPylsp => "coc-pylsp".to_string(),
            CocExtensionType::CocRust => "coc-rust".to_string(),
            CocExtensionType::CocRustAnalyzer => "coc-rust-analyzer".to_string(),
            CocExtensionType::CocGo => "coc-go".to_string(),
            CocExtensionType::CocJava => "coc-java".to_string(),
            CocExtensionType::CocClangd => "coc-clangd".to_string(),
            CocExtensionType::CocSolargraph => "coc-solargraph".to_string(),
            CocExtensionType::CocYaml => "coc-yaml".to_string(),
            CocExtensionType::CocXml => "coc-xml".to_string(),
            CocExtensionType::CocPhp => "coc-php".to_string(),
            CocExtensionType::CocElixir => "coc-elixir".to_string(),
            CocExtensionType::CocSnippets => "coc-snippets".to_string(),
            CocExtensionType::CocUltisnips => "coc-ultisnips".to_string(),
            CocExtensionType::CocPairs => "coc-pairs".to_string(),
            CocExtensionType::CocEmmet => "coc-emmet".to_string(),
            CocExtensionType::CocPrettier => "coc-prettier".to_string(),
            CocExtensionType::CocEslint => "coc-eslint".to_string(),
            CocExtensionType::CocStylelint => "coc-stylelint".to_string(),
            CocExtensionType::CocDiagnostic => "coc-diagnostic".to_string(),
            CocExtensionType::CocGit => "coc-git".to_string(),
            CocExtensionType::CocExplorer => "coc-explorer".to_string(),
            CocExtensionType::CocLists => "coc-lists".to_string(),
            CocExtensionType::CocYank => "coc-yank".to_string(),
            CocExtensionType::CocMarketplace => "coc-marketplace".to_string(),
            CocExtensionType::Custom(name) => name.clone(),
        }
    }

    /// Get description for the extension
    pub fn description(&self) -> &str {
        match self {
            CocExtensionType::CocTsserver => "TypeScript/JavaScript language server",
            CocExtensionType::CocJson => "JSON language server",
            CocExtensionType::CocHtml => "HTML language server",
            CocExtensionType::CocCss => "CSS/SCSS/Less language server",
            CocExtensionType::CocPython => "Python language server (Jedi)",
            CocExtensionType::CocPylsp => "Python LSP language server",
            CocExtensionType::CocRust => "Rust language server (deprecated)",
            CocExtensionType::CocRustAnalyzer => "Rust Analyzer language server",
            CocExtensionType::CocGo => "Go language server",
            CocExtensionType::CocJava => "Java language server",
            CocExtensionType::CocClangd => "C/C++/Objective-C language server",
            CocExtensionType::CocSolargraph => "Ruby language server",
            CocExtensionType::CocYaml => "YAML language server",
            CocExtensionType::CocXml => "XML language server",
            CocExtensionType::CocPhp => "PHP language server",
            CocExtensionType::CocElixir => "Elixir language server",
            CocExtensionType::CocSnippets => "Snippets support",
            CocExtensionType::CocUltisnips => "UltiSnips integration",
            CocExtensionType::CocPairs => "Auto pairs",
            CocExtensionType::CocEmmet => "Emmet support",
            CocExtensionType::CocPrettier => "Prettier formatter",
            CocExtensionType::CocEslint => "ESLint integration",
            CocExtensionType::CocStylelint => "Stylelint integration",
            CocExtensionType::CocDiagnostic => "Diagnostic support",
            CocExtensionType::CocGit => "Git integration",
            CocExtensionType::CocExplorer => "File explorer",
            CocExtensionType::CocLists => "List support",
            CocExtensionType::CocYank => "Yank history",
            CocExtensionType::CocMarketplace => "Extension marketplace",
            CocExtensionType::Custom(_) => "Custom extension",
        }
    }
}

impl Default for COC_CONFIG {
    fn default() -> Self {
        Self {
            environment_name: String::new(),
            installed: false,
            extensions: Vec::new(),
            settings: HashMap::new(),
            config_path: PathBuf::new(),
        }
    }
}

impl CocExtension {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: None,
            installed: false,
            enabled: true,
            description: None,
        }
    }

    pub fn from_extension_type(ext_type: CocExtensionType) -> Self {
        Self {
            name: ext_type.package_name(),
            version: None,
            installed: false,
            enabled: true,
            description: Some(ext_type.description().to_string()),
        }
    }
}
