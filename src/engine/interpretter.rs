use std::error::Error;

use crate::{engine::{tokens::Token, value_variant::ValueVariant}, scope::Scope};

pub struct Interpretter {
    root_scope: Scope,
    interpretted_value: ValueVariant
}

impl Interpretter {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> { 
       
        Ok(())
    }

    pub fn interpretted_value(&self) -> &ValueVariant { 
        &self.interpretted_value
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            root_scope: Scope::new(None, tokens),
            interpretted_value: ValueVariant::Undefined
        }
    }
}
