#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LSP configuration for an environment
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LSP_CONFIG {
    pub environment_name: String,
    pub language_servers: Vec<LanguageServer>,
    pub global_settings: HashMap<String, serde_json::Value>,
}

/// Language server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageServer {
    pub name: String,
    pub language: String,
    pub command: String,
    pub args: Vec<String>,
    pub filetypes: Vec<String>,
    pub root_markers: Vec<String>,
    pub settings: HashMap<String, serde_json::Value>,
    pub enabled: bool,
    pub installed: bool,
}

/// Common language servers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LanguageServerType {
    // Programming languages
    RustAnalyzer,     // Rust
    Pyright,          // Python
    Pylsp,            // Python LSP
    Gopls,            // Go
    TsServer,         // TypeScript/JavaScript
    JavaLanguageServer, // Java
    Clangd,           // C/C++/Objective-C
    CSharpLs,         // C#
    
    // Web development
    VolarVue,         // Vue
    AngularLs,        // Angular
    HtmlLs,           // HTML
    CssLs,            // CSS/SCSS/Less
    JsonLs,           // JSON
    YamlLs,           // YAML
    
    // Other languages
    Solargraph,       // Ruby
    ElixirLs,         // Elixir
    HaskellLs,        // Haskell
    PhpActor,         // PHP
    LuaLs,            // Lua
    
    // Scripting and shell
    BashLs,           // Bash
    
    // Markup and config
    XmlLs,            // XML
    TomlLs,           // TOML
    
    // Custom language server
    Custom(String),
}

impl LanguageServerType {
    /// Get the command to start the language server
    pub fn command(&self) -> String {
        match self {
            LanguageServerType::RustAnalyzer => "rust-analyzer".to_string(),
            LanguageServerType::Pyright => "pyright-langserver".to_string(),
            LanguageServerType::Pylsp => "pylsp".to_string(),
            LanguageServerType::Gopls => "gopls".to_string(),
            LanguageServerType::TsServer => "typescript-language-server".to_string(),
            LanguageServerType::JavaLanguageServer => "jdtls".to_string(),
            LanguageServerType::Clangd => "clangd".to_string(),
            LanguageServerType::CSharpLs => "csharp-ls".to_string(),
            LanguageServerType::VolarVue => "vue-language-server".to_string(),
            LanguageServerType::AngularLs => "ngserver".to_string(),
            LanguageServerType::HtmlLs => "vscode-html-language-server".to_string(),
            LanguageServerType::CssLs => "vscode-css-language-server".to_string(),
            LanguageServerType::JsonLs => "vscode-json-language-server".to_string(),
            LanguageServerType::YamlLs => "yaml-language-server".to_string(),
            LanguageServerType::Solargraph => "solargraph".to_string(),
            LanguageServerType::ElixirLs => "elixir-ls".to_string(),
            LanguageServerType::HaskellLs => "haskell-language-server-wrapper".to_string(),
            LanguageServerType::PhpActor => "phpactor".to_string(),
            LanguageServerType::LuaLs => "lua-language-server".to_string(),
            LanguageServerType::BashLs => "bash-language-server".to_string(),
            LanguageServerType::XmlLs => "lemminx".to_string(),
            LanguageServerType::TomlLs => "taplo".to_string(),
            LanguageServerType::Custom(cmd) => cmd.clone(),
        }
    }

    /// Get default arguments for the language server
    pub fn default_args(&self) -> Vec<String> {
        match self {
            LanguageServerType::TsServer => vec!["--stdio".to_string()],
            LanguageServerType::Pyright => vec!["--stdio".to_string()],
            LanguageServerType::HtmlLs => vec!["--stdio".to_string()],
            LanguageServerType::CssLs => vec!["--stdio".to_string()],
            LanguageServerType::JsonLs => vec!["--stdio".to_string()],
            LanguageServerType::YamlLs => vec!["--stdio".to_string()],
            LanguageServerType::Solargraph => vec!["stdio".to_string()],
            LanguageServerType::BashLs => vec!["start".to_string()],
            _ => vec![],
        }
    }

    /// Get supported filetypes
    pub fn filetypes(&self) -> Vec<String> {
        match self {
            LanguageServerType::RustAnalyzer => vec!["rust".to_string()],
            LanguageServerType::Pyright | LanguageServerType::Pylsp => vec!["python".to_string()],
            LanguageServerType::Gopls => vec!["go".to_string()],
            LanguageServerType::TsServer => vec!["javascript".to_string(), "typescript".to_string(), "javascriptreact".to_string(), "typescriptreact".to_string()],
            LanguageServerType::JavaLanguageServer => vec!["java".to_string()],
            LanguageServerType::Clangd => vec!["c".to_string(), "cpp".to_string(), "objc".to_string(), "objcpp".to_string()],
            LanguageServerType::CSharpLs => vec!["cs".to_string()],
            LanguageServerType::VolarVue => vec!["vue".to_string()],
            LanguageServerType::AngularLs => vec!["html".to_string(), "typescript".to_string()],
            LanguageServerType::HtmlLs => vec!["html".to_string()],
            LanguageServerType::CssLs => vec!["css".to_string(), "scss".to_string(), "less".to_string()],
            LanguageServerType::JsonLs => vec!["json".to_string(), "jsonc".to_string()],
            LanguageServerType::YamlLs => vec!["yaml".to_string(), "yml".to_string()],
            LanguageServerType::Solargraph => vec!["ruby".to_string()],
            LanguageServerType::ElixirLs => vec!["elixir".to_string()],
            LanguageServerType::HaskellLs => vec!["haskell".to_string()],
            LanguageServerType::PhpActor => vec!["php".to_string()],
            LanguageServerType::LuaLs => vec!["lua".to_string()],
            LanguageServerType::BashLs => vec!["sh".to_string(), "bash".to_string()],
            LanguageServerType::XmlLs => vec!["xml".to_string()],
            LanguageServerType::TomlLs => vec!["toml".to_string()],
            LanguageServerType::Custom(_) => vec![],
        }
    }

    /// Get root markers for project detection
    pub fn root_markers(&self) -> Vec<String> {
        match self {
            LanguageServerType::RustAnalyzer => vec!["Cargo.toml".to_string()],
            LanguageServerType::Pyright | LanguageServerType::Pylsp => vec!["pyproject.toml".to_string(), "setup.py".to_string(), "setup.cfg".to_string(), "requirements.txt".to_string(), "Pipfile".to_string()],
            LanguageServerType::Gopls => vec!["go.mod".to_string(), "go.sum".to_string()],
            LanguageServerType::TsServer => vec!["package.json".to_string(), "tsconfig.json".to_string()],
            LanguageServerType::JavaLanguageServer => vec!["pom.xml".to_string(), "build.gradle".to_string()],
            LanguageServerType::CSharpLs => vec![".csproj".to_string(), ".sln".to_string()],
            LanguageServerType::VolarVue => vec!["package.json".to_string()],
            LanguageServerType::Solargraph => vec!["Gemfile".to_string()],
            LanguageServerType::ElixirLs => vec!["mix.exs".to_string()],
            LanguageServerType::PhpActor => vec!["composer.json".to_string()],
            _ => vec![".git".to_string()],
        }
    }

    /// Get language name
    pub fn language(&self) -> String {
        match self {
            LanguageServerType::RustAnalyzer => "rust".to_string(),
            LanguageServerType::Pyright | LanguageServerType::Pylsp => "python".to_string(),
            LanguageServerType::Gopls => "go".to_string(),
            LanguageServerType::TsServer => "typescript".to_string(),
            LanguageServerType::JavaLanguageServer => "java".to_string(),
            LanguageServerType::Clangd => "c/c++".to_string(),
            LanguageServerType::CSharpLs => "csharp".to_string(),
            LanguageServerType::VolarVue => "vue".to_string(),
            LanguageServerType::AngularLs => "angular".to_string(),
            LanguageServerType::HtmlLs => "html".to_string(),
            LanguageServerType::CssLs => "css".to_string(),
            LanguageServerType::JsonLs => "json".to_string(),
            LanguageServerType::YamlLs => "yaml".to_string(),
            LanguageServerType::Solargraph => "ruby".to_string(),
            LanguageServerType::ElixirLs => "elixir".to_string(),
            LanguageServerType::HaskellLs => "haskell".to_string(),
            LanguageServerType::PhpActor => "php".to_string(),
            LanguageServerType::LuaLs => "lua".to_string(),
            LanguageServerType::BashLs => "bash".to_string(),
            LanguageServerType::XmlLs => "xml".to_string(),
            LanguageServerType::TomlLs => "toml".to_string(),
            LanguageServerType::Custom(name) => name.clone(),
        }
    }

    /// Get server name
    pub fn server_name(&self) -> String {
        match self {
            LanguageServerType::RustAnalyzer => "rust-analyzer".to_string(),
            LanguageServerType::Pyright => "pyright".to_string(),
            LanguageServerType::Pylsp => "pylsp".to_string(),
            LanguageServerType::Gopls => "gopls".to_string(),
            LanguageServerType::TsServer => "tsserver".to_string(),
            LanguageServerType::JavaLanguageServer => "jdtls".to_string(),
            LanguageServerType::Clangd => "clangd".to_string(),
            LanguageServerType::CSharpLs => "csharp-ls".to_string(),
            LanguageServerType::VolarVue => "volar".to_string(),
            LanguageServerType::AngularLs => "angular".to_string(),
            LanguageServerType::HtmlLs => "html".to_string(),
            LanguageServerType::CssLs => "css".to_string(),
            LanguageServerType::JsonLs => "json".to_string(),
            LanguageServerType::YamlLs => "yaml".to_string(),
            LanguageServerType::Solargraph => "solargraph".to_string(),
            LanguageServerType::ElixirLs => "elixir-ls".to_string(),
            LanguageServerType::HaskellLs => "haskell-ls".to_string(),
            LanguageServerType::PhpActor => "phpactor".to_string(),
            LanguageServerType::LuaLs => "lua-ls".to_string(),
            LanguageServerType::BashLs => "bash-ls".to_string(),
            LanguageServerType::XmlLs => "xml-ls".to_string(),
            LanguageServerType::TomlLs => "taplo".to_string(),
            LanguageServerType::Custom(name) => name.clone(),
        }
    }
}

impl LanguageServer {
    pub fn from_type(server_type: LanguageServerType) -> Self {
        Self {
            name: server_type.server_name(),
            language: server_type.language(),
            command: server_type.command(),
            args: server_type.default_args(),
            filetypes: server_type.filetypes(),
            root_markers: server_type.root_markers(),
            settings: HashMap::new(),
            enabled: true,
            installed: false,
        }
    }

    pub fn new(name: String, language: String, command: String) -> Self {
        Self {
            name,
            language,
            command,
            args: Vec::new(),
            filetypes: Vec::new(),
            root_markers: vec![".git".to_string()],
            settings: HashMap::new(),
            enabled: true,
            installed: false,
        }
    }
}
