// Custom deserialization helpers
// This module provides common deserialization logic that can be reused across the codebase

use serde::Deserializer;
use serde::de::Deserialize;

// Deserialization for symlink_mode_t
impl<'de> Deserialize<'de> for crate::cnf::application::symlink_mode_t {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use crate::cnf::application::symlink_mode_t;
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "symbolic" => Ok(symlink_mode_t::SYMBOLIC),
            "hard" => Ok(symlink_mode_t::HARD),
            _ => Err(serde::de::Error::unknown_variant(&s, &["symbolic", "hard"])),
        }
    }
}

// Deserialization for app_config
impl<'de> Deserialize<'de> for crate::cnf::application::app_config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct AppConfigVisitor;

        impl<'de> Visitor<'de> for AppConfigVisitor {
            type Value = crate::cnf::application::app_config;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct app_config")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut default_environment = None;
                let mut auto_switch = None;
                let mut backup_enabled = None;
                let mut backup_retention_days = None;
                let mut environment_root = None;
                let mut symlink_mode = None;
                let mut editor = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "default_environment" => {
                            default_environment = Some(map.next_value()?);
                        }
                        "auto_switch" => {
                            auto_switch = Some(map.next_value()?);
                        }
                        "backup_enabled" => {
                            backup_enabled = Some(map.next_value()?);
                        }
                        "backup_retention_days" => {
                            backup_retention_days = Some(map.next_value()?);
                        }
                        "environment_root" => {
                            environment_root = Some(map.next_value()?);
                        }
                        "symlink_mode" => {
                            symlink_mode = Some(map.next_value()?);
                        }
                        "editor" => {
                            editor = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                Ok(crate::cnf::application::app_config {
                    default_environment: default_environment.ok_or_else(|| de::Error::missing_field("default_environment"))?,
                    auto_switch: auto_switch.ok_or_else(|| de::Error::missing_field("auto_switch"))?,
                    backup_enabled: backup_enabled.ok_or_else(|| de::Error::missing_field("backup_enabled"))?,
                    backup_retention_days: backup_retention_days.ok_or_else(|| de::Error::missing_field("backup_retention_days"))?,
                    environment_root: environment_root.ok_or_else(|| de::Error::missing_field("environment_root"))?,
                    symlink_mode: symlink_mode.ok_or_else(|| de::Error::missing_field("symlink_mode"))?,
                    editor: editor.ok_or_else(|| de::Error::missing_field("editor"))?,
                })
            }
        }

        deserializer.deserialize_struct("app_config", &["default_environment", "auto_switch", "backup_enabled", "backup_retention_days", "environment_root", "symlink_mode", "editor"], AppConfigVisitor)
    }
}

// Deserialization for ENVIRONMENT
impl<'de> Deserialize<'de> for crate::ent::model::environment::ENVIRONMENT {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct EnvironmentVisitor;

        impl<'de> Visitor<'de> for EnvironmentVisitor {
            type Value = crate::ent::model::environment::ENVIRONMENT;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ENVIRONMENT")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut description = None;
                let mut created = None;
                let mut update = None;
                let mut last_used = None;
                let mut tags = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "name" => name = Some(map.next_value()?),
                        "description" => description = Some(map.next_value()?),
                        "created" => created = Some(map.next_value()?),
                        "update" => update = Some(map.next_value()?),
                        "last_used" => last_used = Some(map.next_value()?),
                        "tags" => tags = Some(map.next_value()?),
                        _ => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                Ok(crate::ent::model::environment::ENVIRONMENT {
                    name: name.ok_or_else(|| de::Error::missing_field("name"))?,
                    description: description.ok_or_else(|| de::Error::missing_field("description"))?,
                    created: created.ok_or_else(|| de::Error::missing_field("created"))?,
                    update: update.ok_or_else(|| de::Error::missing_field("update"))?,
                    last_used: last_used.ok_or_else(|| de::Error::missing_field("last_used"))?,
                    tags: tags.ok_or_else(|| de::Error::missing_field("tags"))?,
                })
            }
        }

        deserializer.deserialize_struct("ENVIRONMENT", &["name", "description", "created", "update", "last_used", "tags"], EnvironmentVisitor)
    }
}
