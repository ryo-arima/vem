// Custom Clone implementations
// This module provides common Clone trait implementations

// Clone implementation for symlink_mode_t
impl Clone for crate::cnf::application::symlink_mode_t {
    fn clone(&self) -> Self {
        use crate::cnf::application::symlink_mode_t;
        match self {
            symlink_mode_t::SYMBOLIC => symlink_mode_t::SYMBOLIC,
            symlink_mode_t::HARD => symlink_mode_t::HARD,
        }
    }
}

// Clone implementation for app_config
impl Clone for crate::cnf::application::app_config {
    fn clone(&self) -> Self {
        Self {
            default_environment: self.default_environment.clone(),
            auto_switch: self.auto_switch,
            backup_enabled: self.backup_enabled,
            backup_retention_days: self.backup_retention_days,
            environment_root: self.environment_root.clone(),
            symlink_mode: self.symlink_mode.clone(),
            editor: self.editor.clone(),
        }
    }
}

// Clone implementation for ENVIRONMENT
impl Clone for crate::ent::model::environment::ENVIRONMENT {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            description: self.description.clone(),
            created: self.created,
            update: self.update,
            last_used: self.last_used,
            tags: self.tags.clone(),
        }
    }
}

// Clone and Copy implementation for log_level_t
impl Clone for crate::util::mcode::log_level_t {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for crate::util::mcode::log_level_t {}
