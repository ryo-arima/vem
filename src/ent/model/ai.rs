#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI extension configuration for an environment
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AI_CONFIG {
    pub environment_name: String,
    pub ai_tools: Vec<AITool>,
    pub global_settings: HashMap<String, serde_json::Value>,
}

/// AI tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITool {
    pub name: String,
    pub tool_type: AIToolType,
    pub enabled: bool,
    pub installed: bool,
    pub api_key_env: Option<String>,
    pub settings: HashMap<String, serde_json::Value>,
}

/// Types of AI tools/extensions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum AIToolType {
    // GitHub Copilot
    GithubCopilot,           // Official GitHub Copilot
    
    // ChatGPT integrations
    ChatGPT,                 // ChatGPT.nvim
    GPT,                     // gpt.nvim
    NeoAI,                   // neoai.nvim
    
    // Code completion
    Tabnine,                 // Tabnine AI
    Codeium,                 // Codeium (free)
    
    // Code generation and assistance
    CodeWhisperer,           // Amazon CodeWhisperer
    Cody,                    // Sourcegraph Cody
    
    // Multi-model AI assistants
    Ollama,                  // Ollama local models
    LLM,                     // llm.nvim (supports multiple providers)
    GenNvim,                 // gen.nvim (Ollama integration)
    
    // Specific AI features
    Copilotchat,             // GitHub Copilot Chat
    AICommit,                // AI-powered commit messages
    AIDoc,                   // AI-powered documentation
    AIRefactor,              // AI-powered refactoring
    
    // Vim-specific
    VimCodeium,              // Codeium for Vim
    VimCopilot,              // vim-copilot
    
    // coc.nvim extensions
    CocCopilot,              // coc-copilot
    CocCodeium,              // coc-codeium
    CocTabnine,              // coc-tabnine
    
    // Custom AI tool
    Custom(String),
}

impl AIToolType {
    /// Get the plugin/extension name
    pub fn plugin_name(&self) -> String {
        match self {
            AIToolType::GithubCopilot => "github/copilot.vim".to_string(),
            AIToolType::ChatGPT => "jackMort/ChatGPT.nvim".to_string(),
            AIToolType::GPT => "robitx/gpt.nvim".to_string(),
            AIToolType::NeoAI => "Bryley/neoai.nvim".to_string(),
            AIToolType::Tabnine => "codota/tabnine-nvim".to_string(),
            AIToolType::Codeium => "Exafunction/codeium.vim".to_string(),
            AIToolType::CodeWhisperer => "aws/codewhisperer.vim".to_string(),
            AIToolType::Cody => "sourcegraph/sg.nvim".to_string(),
            AIToolType::Ollama => "nomnivore/ollama.nvim".to_string(),
            AIToolType::LLM => "huggingface/llm.nvim".to_string(),
            AIToolType::GenNvim => "David-Kunz/gen.nvim".to_string(),
            AIToolType::Copilotchat => "CopilotC-Nvim/CopilotChat.nvim".to_string(),
            AIToolType::AICommit => "jesseduffield/lazygit".to_string(),
            AIToolType::AIDoc => "danymat/neogen".to_string(),
            AIToolType::AIRefactor => "ThePrimeagen/refactoring.nvim".to_string(),
            AIToolType::VimCodeium => "Exafunction/codeium.vim".to_string(),
            AIToolType::VimCopilot => "github/copilot.vim".to_string(),
            AIToolType::CocCopilot => "coc-copilot".to_string(),
            AIToolType::CocCodeium => "coc-codeium".to_string(),
            AIToolType::CocTabnine => "coc-tabnine".to_string(),
            AIToolType::Custom(name) => name.clone(),
        }
    }

    /// Get the display name
    pub fn display_name(&self) -> &str {
        match self {
            AIToolType::GithubCopilot => "GitHub Copilot",
            AIToolType::ChatGPT => "ChatGPT",
            AIToolType::GPT => "GPT.nvim",
            AIToolType::NeoAI => "NeoAI",
            AIToolType::Tabnine => "Tabnine",
            AIToolType::Codeium => "Codeium",
            AIToolType::CodeWhisperer => "Amazon CodeWhisperer",
            AIToolType::Cody => "Sourcegraph Cody",
            AIToolType::Ollama => "Ollama",
            AIToolType::LLM => "LLM.nvim",
            AIToolType::GenNvim => "Gen.nvim",
            AIToolType::Copilotchat => "Copilot Chat",
            AIToolType::AICommit => "AI Commit",
            AIToolType::AIDoc => "AI Documentation",
            AIToolType::AIRefactor => "AI Refactoring",
            AIToolType::VimCodeium => "Codeium (Vim)",
            AIToolType::VimCopilot => "Copilot (Vim)",
            AIToolType::CocCopilot => "coc-copilot",
            AIToolType::CocCodeium => "coc-codeium",
            AIToolType::CocTabnine => "coc-tabnine",
            AIToolType::Custom(_) => "Custom AI Tool",
        }
    }

    /// Get description
    pub fn description(&self) -> &str {
        match self {
            AIToolType::GithubCopilot => "GitHub's AI pair programmer",
            AIToolType::ChatGPT => "ChatGPT integration for Neovim",
            AIToolType::GPT => "GPT-powered code assistance",
            AIToolType::NeoAI => "AI assistant for Neovim",
            AIToolType::Tabnine => "AI-powered code completion",
            AIToolType::Codeium => "Free AI code completion",
            AIToolType::CodeWhisperer => "Amazon's AI coding companion",
            AIToolType::Cody => "AI coding assistant by Sourcegraph",
            AIToolType::Ollama => "Local LLM integration via Ollama",
            AIToolType::LLM => "Multi-provider LLM integration",
            AIToolType::GenNvim => "Ollama-powered code generation",
            AIToolType::Copilotchat => "Chat with GitHub Copilot",
            AIToolType::AICommit => "AI-generated commit messages",
            AIToolType::AIDoc => "AI-powered documentation generation",
            AIToolType::AIRefactor => "AI-assisted refactoring",
            AIToolType::VimCodeium => "Codeium for Vim",
            AIToolType::VimCopilot => "GitHub Copilot for Vim",
            AIToolType::CocCopilot => "GitHub Copilot for coc.nvim",
            AIToolType::CocCodeium => "Codeium for coc.nvim",
            AIToolType::CocTabnine => "Tabnine for coc.nvim",
            AIToolType::Custom(_) => "Custom AI tool integration",
        }
    }

    /// Check if this tool requires an API key
    pub fn requires_api_key(&self) -> bool {
        matches!(
            self,
            AIToolType::GithubCopilot
                | AIToolType::ChatGPT
                | AIToolType::GPT
                | AIToolType::Tabnine
                | AIToolType::CodeWhisperer
                | AIToolType::Cody
        )
    }

    /// Get default API key environment variable name
    pub fn default_api_key_env(&self) -> Option<String> {
        match self {
            AIToolType::GithubCopilot => Some("GITHUB_COPILOT_TOKEN".to_string()),
            AIToolType::ChatGPT | AIToolType::GPT => Some("OPENAI_API_KEY".to_string()),
            AIToolType::Tabnine => Some("TABNINE_API_KEY".to_string()),
            AIToolType::CodeWhisperer => Some("AWS_ACCESS_KEY_ID".to_string()),
            AIToolType::Cody => Some("SRC_ACCESS_TOKEN".to_string()),
            _ => None,
        }
    }

    /// Check if this is a coc.nvim extension
    pub fn is_coc_extension(&self) -> bool {
        matches!(
            self,
            AIToolType::CocCopilot | AIToolType::CocCodeium | AIToolType::CocTabnine
        )
    }

    /// Check if this requires Neovim
    pub fn requires_neovim(&self) -> bool {
        matches!(
            self,
            AIToolType::ChatGPT
                | AIToolType::GPT
                | AIToolType::NeoAI
                | AIToolType::Tabnine
                | AIToolType::Ollama
                | AIToolType::LLM
                | AIToolType::GenNvim
                | AIToolType::Copilotchat
                | AIToolType::AIDoc
                | AIToolType::AIRefactor
                | AIToolType::Cody
        )
    }
}

impl AITool {
    pub fn from_type(tool_type: AIToolType) -> Self {
        Self {
            name: tool_type.display_name().to_string(),
            tool_type: tool_type.clone(),
            enabled: true,
            installed: false,
            api_key_env: tool_type.default_api_key_env(),
            settings: HashMap::new(),
        }
    }

    pub fn new(name: String, tool_type: AIToolType) -> Self {
        Self {
            name,
            tool_type: tool_type.clone(),
            enabled: true,
            installed: false,
            api_key_env: tool_type.default_api_key_env(),
            settings: HashMap::new(),
        }
    }
}
