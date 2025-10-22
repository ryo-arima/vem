#![allow(non_camel_case_types)]

#[allow(clippy::upper_case_acronyms)]
pub struct ENVIRONMENT {
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}