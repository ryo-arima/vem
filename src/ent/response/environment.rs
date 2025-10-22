
#![allow(non_camel_case_types)]

use crate::ent::model::environment::ENVIRONMENT as ModelEnvironment;

pub struct ENVIRONMENT {
    pub mcode: String,
    pub messages: String,
    pub environment: ModelEnvironment
}