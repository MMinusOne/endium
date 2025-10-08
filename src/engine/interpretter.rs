use std::{error::Error, sync::Mutex};

use crate::{
    apis::type_variants::js_string::JSString,
    engine::{heap::Heap, tokens::Token, value_variant::JSValueVariant},
    scope::Scope,
};

pub struct Interpretter {
    scope: Scope,
    instructions: Vec<Token>,
    interpretted_value: JSValueVariant,
    position: usize,
    heap: &'static Mutex<Heap>,
}

impl Interpretter {
    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        let scope_instructions = self.scope.instructions().clone();

        while let Some(token) = scope_instructions.get(self.position) {
            match token {
                Token::Const => self.handle_const(),

                Token::String(s) => {
                    self.position += 1;
                    self.interpretted_value = JSValueVariant::JSString(JSString::new(s.clone())); // Unhandled strings by other tokens are the interpretted_value
                }

                _ => {
                    self.position += 1;
                }
            }
        }

        Ok(())
    }

    pub fn handle_const(&mut self) {
        self.position += 1; // Skip the `const` keyword.
        match &self.instructions[self.position] {
            Token::Identifier(variable_name) => {
                self.position += 2; // Skip variable_name and `=`

                let mut value_tokens: Vec<Token> = vec![];

                while let Some(v_token) = self.instructions.get(self.position) {
                    let v_token = v_token.clone();

                    match v_token {
                        Token::Newline | Token::Semicolon => break,
                        _ => {}
                    }

                    value_tokens.push(v_token);
                    self.position += 1;
                }

                let value_scope = Scope::new(Some(self.scope.clone()), value_tokens);
                let mut value_interpretter = Interpretter::new(None, Some(value_scope));
                value_interpretter.execute().unwrap();
                let value = value_interpretter.interpretted_value;

                self.scope.insert_state(variable_name.to_string(), value);
                println!("{:#?}", self.scope);
            }

            Token::LeftBrace => {}
            _ => {}
        }
    }

    pub fn interpretted_value(&self) -> &JSValueVariant {
        &self.interpretted_value
    }

    pub fn new(tokens: Option<Vec<Token>>, scope: Option<Scope>) -> Self {
        let scope = match scope {
            Some(s) => s,
            None => Scope::new(None, tokens.unwrap()),
        };

        Self {
            instructions: scope.instructions().clone(),
            scope,
            interpretted_value: JSValueVariant::Undefined,
            position: 0,
            heap: Heap::instance(),
        }
    }
}
