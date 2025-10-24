#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// VimScript configuration
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VIMSCRIPT_CONFIG {
    pub environment_name: String,
    pub scripts: Vec<VimScript>,
    pub functions: Vec<VimFunction>,
    pub commands: Vec<VimCommand>,
    pub autocommands: Vec<VimAutoCommand>,
    pub keymaps: Vec<VimKeymap>,
    pub global_settings: HashMap<String, serde_json::Value>,
}

/// Global VimScript configuration (shared across all environments)
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GLOBAL_VIMSCRIPT_CONFIG {
    pub scripts: Vec<VimScript>,
    pub functions: Vec<VimFunction>,
    pub commands: Vec<VimCommand>,
    pub autocommands: Vec<VimAutoCommand>,
    pub keymaps: Vec<VimKeymap>,
    pub settings: HashMap<String, serde_json::Value>,
}

/// VimScript file or snippet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VimScript {
    pub name: String,
    pub description: Option<String>,
    pub script_type: ScriptType,
    pub content: ScriptContent,
    pub enabled: bool,
    pub order: i32, // Loading order
}

/// Type of script
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptType {
    VimScript,  // .vim file
    Lua,        // .lua file
    Python,     // Python script (for Vim with Python support)
}

/// Script content source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptContent {
    Inline(String),       // Inline script content
    File(String),         // Path to script file
    Url(String),          // URL to download script from
}

/// Custom Vim function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VimFunction {
    pub name: String,
    pub description: Option<String>,
    pub function_type: FunctionType,
    pub parameters: Vec<String>,
    pub body: String,
    pub enabled: bool,
}

/// Function type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FunctionType {
    VimScript,    // VimScript function
    Lua,          // Lua function
    Python,       // Python function
}

/// Custom Vim command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VimCommand {
    pub name: String,
    pub description: Option<String>,
    pub command_type: CommandType,
    pub definition: String,
    pub attributes: CommandAttributes,
    pub enabled: bool,
}

/// Command type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommandType {
    Simple,       // Simple command mapping
    Function,     // Command that calls a function
    External,     // Command that calls external program
}

/// Command attributes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandAttributes {
    pub nargs: Option<String>,    // Number of arguments (0, 1, *, ?, +)
    pub range: bool,              // Accept range
    pub count: bool,              // Accept count
    pub bang: bool,               // Accept bang (!)
    pub register: bool,           // Accept register
    pub buffer: bool,             // Buffer-local command
    pub complete: Option<String>, // Completion type
}

/// Autocommand definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VimAutoCommand {
    pub name: String,
    pub description: Option<String>,
    pub events: Vec<String>,      // Autocmd events (BufRead, BufNewFile, etc.)
    pub pattern: String,          // File pattern
    pub command: String,          // Command to execute
    pub group: Option<String>,    // Autocommand group
    pub enabled: bool,
}

/// Keymap definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VimKeymap {
    pub name: String,
    pub description: Option<String>,
    pub mode: KeymapMode,
    pub key: String,              // Key combination
    pub mapping: String,          // Mapped command/action
    pub options: KeymapOptions,
    pub enabled: bool,
}

/// Keymap mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeymapMode {
    Normal,      // Normal mode (n)
    Insert,      // Insert mode (i)
    Visual,      // Visual mode (v)
    Command,     // Command mode (c)
    Terminal,    // Terminal mode (t)
    VisualBlock, // Visual block mode (x)
    Select,      // Select mode (s)
    Operator,    // Operator pending mode (o)
    All,         // All modes (map)
}

/// Keymap options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeymapOptions {
    pub noremap: bool,    // Non-recursive mapping
    pub silent: bool,     // Silent execution
    pub expr: bool,       // Expression mapping
    pub buffer: bool,     // Buffer-local mapping
    pub nowait: bool,     // Don't wait for more characters
    pub unique: bool,     // Only create if not already mapped
}

impl ScriptType {
    pub fn extension(&self) -> &str {
        match self {
            ScriptType::VimScript => "vim",
            ScriptType::Lua => "lua",
            ScriptType::Python => "py",
        }
    }
}

impl FunctionType {
    pub fn to_script_type(&self) -> ScriptType {
        match self {
            FunctionType::VimScript => ScriptType::VimScript,
            FunctionType::Lua => ScriptType::Lua,
            FunctionType::Python => ScriptType::Python,
        }
    }
}

impl KeymapMode {
    pub fn to_vim_mode(&self) -> &str {
        match self {
            KeymapMode::Normal => "n",
            KeymapMode::Insert => "i",
            KeymapMode::Visual => "v",
            KeymapMode::Command => "c",
            KeymapMode::Terminal => "t",
            KeymapMode::VisualBlock => "x",
            KeymapMode::Select => "s",
            KeymapMode::Operator => "o",
            KeymapMode::All => "",
        }
    }
}

impl VimScript {
    pub fn new_inline(name: String, script_type: ScriptType, content: String) -> Self {
        Self {
            name,
            description: None,
            script_type,
            content: ScriptContent::Inline(content),
            enabled: true,
            order: 0,
        }
    }

    pub fn new_file(name: String, script_type: ScriptType, file_path: String) -> Self {
        Self {
            name,
            description: None,
            script_type,
            content: ScriptContent::File(file_path),
            enabled: true,
            order: 0,
        }
    }
}

impl VimFunction {
    pub fn new(name: String, function_type: FunctionType, body: String) -> Self {
        Self {
            name,
            description: None,
            function_type,
            parameters: Vec::new(),
            body,
            enabled: true,
        }
    }

    pub fn generate_definition(&self) -> String {
        match self.function_type {
            FunctionType::VimScript => {
                let params = if self.parameters.is_empty() {
                    String::new()
                } else {
                    self.parameters.join(", ")
                };
                format!(
                    "function! {}({})\n{}\nendfunction",
                    self.name, params, self.body
                )
            }
            FunctionType::Lua => {
                let params = self.parameters.join(", ");
                format!("function {}({})\n{}\nend", self.name, params, self.body)
            }
            FunctionType::Python => {
                let params = self.parameters.join(", ");
                format!("def {}({}):\n{}", self.name, params, self.body)
            }
        }
    }
}

impl VimCommand {
    pub fn new_simple(name: String, definition: String) -> Self {
        Self {
            name,
            description: None,
            command_type: CommandType::Simple,
            definition,
            attributes: CommandAttributes::default(),
            enabled: true,
        }
    }

    pub fn generate_definition(&self) -> String {
        let mut cmd = "command!".to_string();

        // Add attributes
        if let Some(ref nargs) = self.attributes.nargs {
            cmd.push_str(&format!(" -nargs={}", nargs));
        }
        if self.attributes.range {
            cmd.push_str(" -range");
        }
        if self.attributes.count {
            cmd.push_str(" -count");
        }
        if self.attributes.bang {
            cmd.push_str(" -bang");
        }
        if self.attributes.register {
            cmd.push_str(" -register");
        }
        if self.attributes.buffer {
            cmd.push_str(" -buffer");
        }
        if let Some(ref complete) = self.attributes.complete {
            cmd.push_str(&format!(" -complete={}", complete));
        }

        cmd.push_str(&format!(" {} {}", self.name, self.definition));
        cmd
    }
}

impl VimAutoCommand {
    pub fn new(events: Vec<String>, pattern: String, command: String) -> Self {
        Self {
            name: format!("autocmd_{}", pattern.replace('*', "all").replace('.', "_")),
            description: None,
            events,
            pattern,
            command,
            group: None,
            enabled: true,
        }
    }

    pub fn generate_definition(&self) -> String {
        let events = self.events.join(",");
        if let Some(ref group) = self.group {
            format!(
                "augroup {}\n  autocmd! {} {} {}\naugroup END",
                group, events, self.pattern, self.command
            )
        } else {
            format!("autocmd {} {} {}", events, self.pattern, self.command)
        }
    }
}

impl VimKeymap {
    pub fn new(mode: KeymapMode, key: String, mapping: String) -> Self {
        let clean_key = key.replace(['<', '>'], "");
        Self {
            name: format!("{}_{}", mode.to_vim_mode(), clean_key),
            description: None,
            mode,
            key,
            mapping,
            options: KeymapOptions::default(),
            enabled: true,
        }
    }

    pub fn generate_definition(&self) -> String {
        let mode = self.mode.to_vim_mode();
        let map_cmd = if self.options.noremap {
            format!("{}noremap", mode)
        } else {
            format!("{}map", mode)
        };

        let mut opts = Vec::new();
        if self.options.silent {
            opts.push("<silent>");
        }
        if self.options.expr {
            opts.push("<expr>");
        }
        if self.options.buffer {
            opts.push("<buffer>");
        }
        if self.options.nowait {
            opts.push("<nowait>");
        }
        if self.options.unique {
            opts.push("<unique>");
        }

        let opts_str = if opts.is_empty() {
            String::new()
        } else {
            format!(" {}", opts.join(" "))
        };

        format!("{}{} {} {}", map_cmd, opts_str, self.key, self.mapping)
    }
}
