#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ENVIRONMENT {
    pub name: String,
    pub description: Option<String>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub update: chrono::DateTime<chrono::Utc>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub tags: Vec<String>,
}