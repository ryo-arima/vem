// Custom serialization helpers
// This module provides common serialization logic that can be reused across the codebase

use serde::Serializer;

// Serialization for symlink_mode_t
impl serde::Serialize for crate::cnf::application::symlink_mode_t {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use crate::cnf::application::symlink_mode_t;
        match self {
            symlink_mode_t::SYMBOLIC => serializer.serialize_str("symbolic"),
            symlink_mode_t::HARD => serializer.serialize_str("hard"),
        }
    }
}

// Serialization for app_config
impl serde::Serialize for crate::cnf::application::app_config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("app_config", 7)?;
        state.serialize_field("default_environment", &self.default_environment)?;
        state.serialize_field("auto_switch", &self.auto_switch)?;
        state.serialize_field("backup_enabled", &self.backup_enabled)?;
        state.serialize_field("backup_retention_days", &self.backup_retention_days)?;
        state.serialize_field("environment_root", &self.environment_root)?;
        state.serialize_field("symlink_mode", &self.symlink_mode)?;
        state.serialize_field("editor", &self.editor)?;
        state.end()
    }
}

// Serialization for ENVIRONMENT
impl serde::Serialize for crate::ent::model::environment::ENVIRONMENT {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ENVIRONMENT", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("created", &self.created)?;
        state.serialize_field("update", &self.update)?;
        state.serialize_field("last_used", &self.last_used)?;
        state.serialize_field("tags", &self.tags)?;
        state.end()
    }
}
