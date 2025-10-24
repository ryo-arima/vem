#![allow(non_camel_case_types)]

use crate::ent::model::vimscript::{
    VimAutoCommand, VimCommand, VimFunction, VimKeymap, VimScript, GLOBAL_VIMSCRIPT_CONFIG,
    VIMSCRIPT_CONFIG,
};
use crate::rep::RepositoryConfig;
use crate::util::error::vem_error_t;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

/// VimScript repository with Go-style struct embedding
pub struct vimscript_repository {
    base: RepositoryConfig,
}

impl Deref for vimscript_repository {
    type Target = RepositoryConfig;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

pub trait VimScriptRepository {
    // Environment-specific operations
    fn initialize(&mut self, environment_name: &str) -> Result<(), vem_error_t>;
    fn add_script(
        &mut self,
        environment_name: &str,
        script: VimScript,
    ) -> Result<(), vem_error_t>;
    fn add_function(
        &mut self,
        environment_name: &str,
        function: VimFunction,
    ) -> Result<(), vem_error_t>;
    fn add_command(
        &mut self,
        environment_name: &str,
        command: VimCommand,
    ) -> Result<(), vem_error_t>;
    fn add_autocommand(
        &mut self,
        environment_name: &str,
        autocommand: VimAutoCommand,
    ) -> Result<(), vem_error_t>;
    fn add_keymap(
        &mut self,
        environment_name: &str,
        keymap: VimKeymap,
    ) -> Result<(), vem_error_t>;

    fn remove_script(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t>;
    fn remove_function(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t>;
    fn remove_command(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t>;
    fn remove_autocommand(
        &mut self,
        environment_name: &str,
        name: &str,
    ) -> Result<(), vem_error_t>;
    fn remove_keymap(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t>;

    fn toggle_script(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t>;
    fn toggle_function(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t>;
    fn toggle_command(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t>;
    fn toggle_autocommand(
        &mut self,
        environment_name: &str,
        name: &str,
    ) -> Result<(), vem_error_t>;
    fn toggle_keymap(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t>;

    fn get_config(&self, environment_name: &str) -> Result<VIMSCRIPT_CONFIG, vem_error_t>;
    fn save_config(&self, config: &VIMSCRIPT_CONFIG) -> Result<(), vem_error_t>;

    // Global operations
    fn initialize_global(&mut self) -> Result<(), vem_error_t>;
    fn add_global_script(&mut self, script: VimScript) -> Result<(), vem_error_t>;
    fn add_global_function(&mut self, function: VimFunction) -> Result<(), vem_error_t>;
    fn add_global_command(&mut self, command: VimCommand) -> Result<(), vem_error_t>;
    fn add_global_autocommand(&mut self, autocommand: VimAutoCommand) -> Result<(), vem_error_t>;
    fn add_global_keymap(&mut self, keymap: VimKeymap) -> Result<(), vem_error_t>;

    fn remove_global_script(&mut self, name: &str) -> Result<(), vem_error_t>;
    fn remove_global_function(&mut self, name: &str) -> Result<(), vem_error_t>;
    fn remove_global_command(&mut self, name: &str) -> Result<(), vem_error_t>;
    fn remove_global_autocommand(&mut self, name: &str) -> Result<(), vem_error_t>;
    fn remove_global_keymap(&mut self, name: &str) -> Result<(), vem_error_t>;

    fn get_global_config(&self) -> Result<GLOBAL_VIMSCRIPT_CONFIG, vem_error_t>;
    fn save_global_config(&self, config: &GLOBAL_VIMSCRIPT_CONFIG) -> Result<(), vem_error_t>;

    // Generation
    fn generate_vimrc(&self, environment_name: &str) -> Result<String, vem_error_t>;
    fn generate_global_vimrc(&self) -> Result<String, vem_error_t>;
    fn generate_combined_vimrc(&self, environment_name: &str) -> Result<String, vem_error_t>;
}

impl vimscript_repository {
    pub fn new() -> Self {
        Self {
            base: RepositoryConfig::new(),
        }
    }

    fn get_vimscript_config_path(&self, environment_name: &str) -> PathBuf {
        self.base
            .app_config
            .get_environment_path(environment_name)
            .join("vimscript_config.toml")
    }

    fn get_global_vimscript_config_path(&self) -> PathBuf {
        self.base.base_path.join("global_vimscript_config.toml")
    }

    fn get_scripts_dir(&self, environment_name: &str) -> PathBuf {
        self.base
            .app_config
            .get_environment_path(environment_name)
            .join("scripts")
    }

    fn get_global_scripts_dir(&self) -> PathBuf {
        self.base.base_path.join("scripts")
    }
}

impl VimScriptRepository for vimscript_repository {
    fn initialize(&mut self, environment_name: &str) -> Result<(), vem_error_t> {
        let config = VIMSCRIPT_CONFIG {
            environment_name: environment_name.to_string(),
            scripts: Vec::new(),
            functions: Vec::new(),
            commands: Vec::new(),
            autocommands: Vec::new(),
            keymaps: Vec::new(),
            global_settings: std::collections::HashMap::new(),
        };

        self.save_config(&config)?;

        // Create scripts directory
        let scripts_dir = self.get_scripts_dir(environment_name);
        fs::create_dir_all(&scripts_dir).map_err(vem_error_t::FileSystemError)?;

        Ok(())
    }

    fn add_script(
        &mut self,
        environment_name: &str,
        script: VimScript,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        if config.scripts.iter().any(|s| s.name == script.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Script '{}' already exists",
                script.name
            )));
        }

        config.scripts.push(script);
        self.save_config(&config)?;
        Ok(())
    }

    fn add_function(
        &mut self,
        environment_name: &str,
        function: VimFunction,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        if config.functions.iter().any(|f| f.name == function.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Function '{}' already exists",
                function.name
            )));
        }

        config.functions.push(function);
        self.save_config(&config)?;
        Ok(())
    }

    fn add_command(
        &mut self,
        environment_name: &str,
        command: VimCommand,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        if config.commands.iter().any(|c| c.name == command.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Command '{}' already exists",
                command.name
            )));
        }

        config.commands.push(command);
        self.save_config(&config)?;
        Ok(())
    }

    fn add_autocommand(
        &mut self,
        environment_name: &str,
        autocommand: VimAutoCommand,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        if config
            .autocommands
            .iter()
            .any(|a| a.name == autocommand.name)
        {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Autocommand '{}' already exists",
                autocommand.name
            )));
        }

        config.autocommands.push(autocommand);
        self.save_config(&config)?;
        Ok(())
    }

    fn add_keymap(
        &mut self,
        environment_name: &str,
        keymap: VimKeymap,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;

        if config.keymaps.iter().any(|k| k.name == keymap.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Keymap '{}' already exists",
                keymap.name
            )));
        }

        config.keymaps.push(keymap);
        self.save_config(&config)?;
        Ok(())
    }

    fn remove_script(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        let original_len = config.scripts.len();
        config.scripts.retain(|s| s.name != name);

        if config.scripts.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Script '{}' not found",
                name
            )));
        }

        self.save_config(&config)?;
        Ok(())
    }

    fn remove_function(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        let original_len = config.functions.len();
        config.functions.retain(|f| f.name != name);

        if config.functions.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Function '{}' not found",
                name
            )));
        }

        self.save_config(&config)?;
        Ok(())
    }

    fn remove_command(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        let original_len = config.commands.len();
        config.commands.retain(|c| c.name != name);

        if config.commands.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Command '{}' not found",
                name
            )));
        }

        self.save_config(&config)?;
        Ok(())
    }

    fn remove_autocommand(
        &mut self,
        environment_name: &str,
        name: &str,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        let original_len = config.autocommands.len();
        config.autocommands.retain(|a| a.name != name);

        if config.autocommands.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Autocommand '{}' not found",
                name
            )));
        }

        self.save_config(&config)?;
        Ok(())
    }

    fn remove_keymap(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        let original_len = config.keymaps.len();
        config.keymaps.retain(|k| k.name != name);

        if config.keymaps.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Keymap '{}' not found",
                name
            )));
        }

        self.save_config(&config)?;
        Ok(())
    }

    fn toggle_script(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        let script = config
            .scripts
            .iter_mut()
            .find(|s| s.name == name)
            .ok_or_else(|| {
                vem_error_t::EnvironmentNotFound(format!("Script '{}' not found", name))
            })?;

        script.enabled = !script.enabled;
        self.save_config(&config)?;
        Ok(())
    }

    fn toggle_function(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        let function = config
            .functions
            .iter_mut()
            .find(|f| f.name == name)
            .ok_or_else(|| {
                vem_error_t::EnvironmentNotFound(format!("Function '{}' not found", name))
            })?;

        function.enabled = !function.enabled;
        self.save_config(&config)?;
        Ok(())
    }

    fn toggle_command(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        let command = config
            .commands
            .iter_mut()
            .find(|c| c.name == name)
            .ok_or_else(|| {
                vem_error_t::EnvironmentNotFound(format!("Command '{}' not found", name))
            })?;

        command.enabled = !command.enabled;
        self.save_config(&config)?;
        Ok(())
    }

    fn toggle_autocommand(
        &mut self,
        environment_name: &str,
        name: &str,
    ) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        let autocommand = config
            .autocommands
            .iter_mut()
            .find(|a| a.name == name)
            .ok_or_else(|| {
                vem_error_t::EnvironmentNotFound(format!("Autocommand '{}' not found", name))
            })?;

        autocommand.enabled = !autocommand.enabled;
        self.save_config(&config)?;
        Ok(())
    }

    fn toggle_keymap(&mut self, environment_name: &str, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_config(environment_name)?;
        let keymap = config
            .keymaps
            .iter_mut()
            .find(|k| k.name == name)
            .ok_or_else(|| {
                vem_error_t::EnvironmentNotFound(format!("Keymap '{}' not found", name))
            })?;

        keymap.enabled = !keymap.enabled;
        self.save_config(&config)?;
        Ok(())
    }

    fn get_config(&self, environment_name: &str) -> Result<VIMSCRIPT_CONFIG, vem_error_t> {
        let config_path = self.get_vimscript_config_path(environment_name);

        if !config_path.exists() {
            return Ok(VIMSCRIPT_CONFIG {
                environment_name: environment_name.to_string(),
                scripts: Vec::new(),
                functions: Vec::new(),
                commands: Vec::new(),
                autocommands: Vec::new(),
                keymaps: Vec::new(),
                global_settings: std::collections::HashMap::new(),
            });
        }

        let content = fs::read_to_string(&config_path).map_err(vem_error_t::FileSystemError)?;

        toml::from_str(&content).map_err(|e| {
            vem_error_t::SerializationError(format!("Failed to parse VimScript config: {}", e))
        })
    }

    fn save_config(&self, config: &VIMSCRIPT_CONFIG) -> Result<(), vem_error_t> {
        let config_path = self.get_vimscript_config_path(&config.environment_name);

        let content = toml::to_string_pretty(config).map_err(|e| {
            vem_error_t::SerializationError(format!("Failed to serialize VimScript config: {}", e))
        })?;

        fs::write(&config_path, content).map_err(vem_error_t::FileSystemError)?;

        Ok(())
    }

    fn initialize_global(&mut self) -> Result<(), vem_error_t> {
        let config = GLOBAL_VIMSCRIPT_CONFIG {
            scripts: Vec::new(),
            functions: Vec::new(),
            commands: Vec::new(),
            autocommands: Vec::new(),
            keymaps: Vec::new(),
            settings: std::collections::HashMap::new(),
        };

        self.save_global_config(&config)?;

        // Create global scripts directory
        let scripts_dir = self.get_global_scripts_dir();
        fs::create_dir_all(&scripts_dir).map_err(vem_error_t::FileSystemError)?;

        Ok(())
    }

    fn add_global_script(&mut self, script: VimScript) -> Result<(), vem_error_t> {
        let mut config = self.get_global_config()?;

        if config.scripts.iter().any(|s| s.name == script.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Global script '{}' already exists",
                script.name
            )));
        }

        config.scripts.push(script);
        self.save_global_config(&config)?;
        Ok(())
    }

    fn add_global_function(&mut self, function: VimFunction) -> Result<(), vem_error_t> {
        let mut config = self.get_global_config()?;

        if config.functions.iter().any(|f| f.name == function.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Global function '{}' already exists",
                function.name
            )));
        }

        config.functions.push(function);
        self.save_global_config(&config)?;
        Ok(())
    }

    fn add_global_command(&mut self, command: VimCommand) -> Result<(), vem_error_t> {
        let mut config = self.get_global_config()?;

        if config.commands.iter().any(|c| c.name == command.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Global command '{}' already exists",
                command.name
            )));
        }

        config.commands.push(command);
        self.save_global_config(&config)?;
        Ok(())
    }

    fn add_global_autocommand(&mut self, autocommand: VimAutoCommand) -> Result<(), vem_error_t> {
        let mut config = self.get_global_config()?;

        if config
            .autocommands
            .iter()
            .any(|a| a.name == autocommand.name)
        {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Global autocommand '{}' already exists",
                autocommand.name
            )));
        }

        config.autocommands.push(autocommand);
        self.save_global_config(&config)?;
        Ok(())
    }

    fn add_global_keymap(&mut self, keymap: VimKeymap) -> Result<(), vem_error_t> {
        let mut config = self.get_global_config()?;

        if config.keymaps.iter().any(|k| k.name == keymap.name) {
            return Err(vem_error_t::EnvironmentAlreadyExists(format!(
                "Global keymap '{}' already exists",
                keymap.name
            )));
        }

        config.keymaps.push(keymap);
        self.save_global_config(&config)?;
        Ok(())
    }

    fn remove_global_script(&mut self, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_global_config()?;
        let original_len = config.scripts.len();
        config.scripts.retain(|s| s.name != name);

        if config.scripts.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Global script '{}' not found",
                name
            )));
        }

        self.save_global_config(&config)?;
        Ok(())
    }

    fn remove_global_function(&mut self, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_global_config()?;
        let original_len = config.functions.len();
        config.functions.retain(|f| f.name != name);

        if config.functions.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Global function '{}' not found",
                name
            )));
        }

        self.save_global_config(&config)?;
        Ok(())
    }

    fn remove_global_command(&mut self, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_global_config()?;
        let original_len = config.commands.len();
        config.commands.retain(|c| c.name != name);

        if config.commands.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Global command '{}' not found",
                name
            )));
        }

        self.save_global_config(&config)?;
        Ok(())
    }

    fn remove_global_autocommand(&mut self, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_global_config()?;
        let original_len = config.autocommands.len();
        config.autocommands.retain(|a| a.name != name);

        if config.autocommands.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Global autocommand '{}' not found",
                name
            )));
        }

        self.save_global_config(&config)?;
        Ok(())
    }

    fn remove_global_keymap(&mut self, name: &str) -> Result<(), vem_error_t> {
        let mut config = self.get_global_config()?;
        let original_len = config.keymaps.len();
        config.keymaps.retain(|k| k.name != name);

        if config.keymaps.len() == original_len {
            return Err(vem_error_t::EnvironmentNotFound(format!(
                "Global keymap '{}' not found",
                name
            )));
        }

        self.save_global_config(&config)?;
        Ok(())
    }

    fn get_global_config(&self) -> Result<GLOBAL_VIMSCRIPT_CONFIG, vem_error_t> {
        let config_path = self.get_global_vimscript_config_path();

        if !config_path.exists() {
            return Ok(GLOBAL_VIMSCRIPT_CONFIG::default());
        }

        let content = fs::read_to_string(&config_path).map_err(vem_error_t::FileSystemError)?;

        toml::from_str(&content).map_err(|e| {
            vem_error_t::SerializationError(format!(
                "Failed to parse global VimScript config: {}",
                e
            ))
        })
    }

    fn save_global_config(&self, config: &GLOBAL_VIMSCRIPT_CONFIG) -> Result<(), vem_error_t> {
        let config_path = self.get_global_vimscript_config_path();

        let content = toml::to_string_pretty(config).map_err(|e| {
            vem_error_t::SerializationError(format!(
                "Failed to serialize global VimScript config: {}",
                e
            ))
        })?;

        fs::write(&config_path, content).map_err(vem_error_t::FileSystemError)?;

        Ok(())
    }

    fn generate_vimrc(&self, environment_name: &str) -> Result<String, vem_error_t> {
        let config = self.get_config(environment_name)?;
        let mut vimrc = String::new();

        vimrc.push_str("\" Environment-specific VimScript Configuration\n");
        vimrc.push_str(&format!("\" Environment: {}\n", environment_name));
        vimrc.push_str("\" Generated by VEM\n\n");

        // Scripts
        if !config.scripts.is_empty() {
            vimrc.push_str("\" === Scripts ===\n");
            for script in config.scripts.iter().filter(|s| s.enabled) {
                if let Some(desc) = &script.description {
                    vimrc.push_str(&format!("\" {}\n", desc));
                }
                match &script.content {
                    crate::ent::model::vimscript::ScriptContent::Inline(content) => {
                        vimrc.push_str(content);
                        vimrc.push('\n');
                    }
                    crate::ent::model::vimscript::ScriptContent::File(path) => {
                        vimrc.push_str(&format!("source {}\n", path));
                    }
                    crate::ent::model::vimscript::ScriptContent::Url(_) => {
                        vimrc.push_str(&format!("\" TODO: Download script from URL: {}\n", script.name));
                    }
                }
                vimrc.push('\n');
            }
        }

        // Functions
        if !config.functions.is_empty() {
            vimrc.push_str("\" === Functions ===\n");
            for function in config.functions.iter().filter(|f| f.enabled) {
                if let Some(desc) = &function.description {
                    vimrc.push_str(&format!("\" {}\n", desc));
                }
                vimrc.push_str(&function.generate_definition());
                vimrc.push_str("\n\n");
            }
        }

        // Commands
        if !config.commands.is_empty() {
            vimrc.push_str("\" === Commands ===\n");
            for command in config.commands.iter().filter(|c| c.enabled) {
                if let Some(desc) = &command.description {
                    vimrc.push_str(&format!("\" {}\n", desc));
                }
                vimrc.push_str(&command.generate_definition());
                vimrc.push_str("\n\n");
            }
        }

        // Autocommands
        if !config.autocommands.is_empty() {
            vimrc.push_str("\" === Autocommands ===\n");
            for autocmd in config.autocommands.iter().filter(|a| a.enabled) {
                if let Some(desc) = &autocmd.description {
                    vimrc.push_str(&format!("\" {}\n", desc));
                }
                vimrc.push_str(&autocmd.generate_definition());
                vimrc.push_str("\n\n");
            }
        }

        // Keymaps
        if !config.keymaps.is_empty() {
            vimrc.push_str("\" === Keymaps ===\n");
            for keymap in config.keymaps.iter().filter(|k| k.enabled) {
                if let Some(desc) = &keymap.description {
                    vimrc.push_str(&format!("\" {}\n", desc));
                }
                vimrc.push_str(&keymap.generate_definition());
                vimrc.push('\n');
            }
        }

        Ok(vimrc)
    }

    fn generate_global_vimrc(&self) -> Result<String, vem_error_t> {
        let config = self.get_global_config()?;
        let mut vimrc = String::new();

        vimrc.push_str("\" Global VimScript Configuration\n");
        vimrc.push_str("\" Shared across all environments\n");
        vimrc.push_str("\" Generated by VEM\n\n");

        // Scripts
        if !config.scripts.is_empty() {
            vimrc.push_str("\" === Global Scripts ===\n");
            for script in &config.scripts {
                if let Some(desc) = &script.description {
                    vimrc.push_str(&format!("\" {}\n", desc));
                }
                match &script.content {
                    crate::ent::model::vimscript::ScriptContent::Inline(content) => {
                        vimrc.push_str(content);
                        vimrc.push('\n');
                    }
                    crate::ent::model::vimscript::ScriptContent::File(path) => {
                        vimrc.push_str(&format!("source {}\n", path));
                    }
                    crate::ent::model::vimscript::ScriptContent::Url(_) => {
                        vimrc.push_str(&format!("\" TODO: Download script from URL: {}\n", script.name));
                    }
                }
                vimrc.push('\n');
            }
        }

        // Functions
        if !config.functions.is_empty() {
            vimrc.push_str("\" === Global Functions ===\n");
            for function in &config.functions {
                if let Some(desc) = &function.description {
                    vimrc.push_str(&format!("\" {}\n", desc));
                }
                vimrc.push_str(&function.generate_definition());
                vimrc.push_str("\n\n");
            }
        }

        // Commands
        if !config.commands.is_empty() {
            vimrc.push_str("\" === Global Commands ===\n");
            for command in &config.commands {
                if let Some(desc) = &command.description {
                    vimrc.push_str(&format!("\" {}\n", desc));
                }
                vimrc.push_str(&command.generate_definition());
                vimrc.push_str("\n\n");
            }
        }

        // Autocommands
        if !config.autocommands.is_empty() {
            vimrc.push_str("\" === Global Autocommands ===\n");
            for autocmd in &config.autocommands {
                if let Some(desc) = &autocmd.description {
                    vimrc.push_str(&format!("\" {}\n", desc));
                }
                vimrc.push_str(&autocmd.generate_definition());
                vimrc.push_str("\n\n");
            }
        }

        // Keymaps
        if !config.keymaps.is_empty() {
            vimrc.push_str("\" === Global Keymaps ===\n");
            for keymap in &config.keymaps {
                if let Some(desc) = &keymap.description {
                    vimrc.push_str(&format!("\" {}\n", desc));
                }
                vimrc.push_str(&keymap.generate_definition());
                vimrc.push('\n');
            }
        }

        Ok(vimrc)
    }

    fn generate_combined_vimrc(&self, environment_name: &str) -> Result<String, vem_error_t> {
        let mut vimrc = String::new();

        vimrc.push_str("\" Combined VimScript Configuration\n");
        vimrc.push_str(&format!("\" Environment: {}\n", environment_name));
        vimrc.push_str("\" Generated by VEM\n\n");

        // Global configuration first
        vimrc.push_str(&self.generate_global_vimrc()?);
        vimrc.push_str("\n\" ========================================\n\n");

        // Environment-specific configuration
        vimrc.push_str(&self.generate_vimrc(environment_name)?);

        Ok(vimrc)
    }
}

impl Default for vimscript_repository {
    fn default() -> Self {
        Self::new()
    }
}
