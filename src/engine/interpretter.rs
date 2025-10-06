use std::error::Error;

use crate::{engine::{tokens::Token, value_variant::ValueVariant}, scope::Scope};

pub struct Interpretter<'a> {
    root_scope: Scope<'a>,
    interpretted_value: ValueVariant
}

impl<'a> Interpretter<'a> {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> { 
        Ok(())
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            root_scope: Scope::new(None, tokens),
            interpretted_value: ValueVariant::Undefined
        }
    }
}
