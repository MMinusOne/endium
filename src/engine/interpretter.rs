use crate::apis::features::increment::Increment;
use crate::{
    apis::type_variants::{js_number::JSNumber, js_string::JSString},
    engine::{heap::Heap, tokens::Token, value_variant::JSValueVariant},
    scope::Scope,
};
use std::{error::Error, sync::Mutex};

pub struct Interpretter {
    scope: Scope,
    instructions: Vec<Token>,
    interpretted_value: JSValueVariant,
    position: usize,
}

impl Interpretter {
    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        let scope_instructions = self.scope.instructions().clone();

        while let Some(token) = scope_instructions.get(self.position) {
            let _ = match token {
                Token::Const => self.handle_const(),

                Token::String(s) => {
                    self.position += 1;
                    self.interpretted_value = JSValueVariant::JSString(JSString::new(s.clone())); // Unhandled strings by other tokens are the interpretted_value    
                    return Ok(());
                }

                Token::Number(n) => {
                    self.position += 1;
                    let number = n.parse::<f64>()?;
                    self.interpretted_value = JSValueVariant::JSNumber(JSNumber::new(number));
                    return Ok(());
                }

                Token::Identifier(identifier) => self.handle_identifier(identifier),

                Token::Null => {
                    self.position += 1;
                    self.interpretted_value = JSValueVariant::Null;
                    return Ok(());
                }

                Token::Undefined => {
                    self.position += 1;
                    self.interpretted_value = JSValueVariant::Undefined;
                    return Ok(());
                }

                _ => {
                    self.position += 1;
                    return Ok(());
                }
            };
        }

        Ok(())
    }

    pub fn value_collector(&mut self) -> Vec<Token> {
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

        value_tokens
    }

    pub fn handle_const(&mut self) -> Result<(), Box<dyn Error>> {
        self.position += 1; // Skip the `const` keyword.

        let current_token = self.instructions[self.position].clone();

        match current_token {
            Token::Identifier(variable_name) => {
                self.position += 2; // Skip variable_name and `=`

                let value_tokens: Vec<Token> = self.value_collector();
                let parent_scope = self.scope.clone();

                let value_scope = Scope::new(Some(parent_scope), value_tokens);
                let mut value_interpretter = Interpretter::new(None, Some(value_scope)); // Make an interpretter with the value as instructions.
                value_interpretter.execute().unwrap();
                let value = value_interpretter.interpretted_value;

                self.scope.insert_state(variable_name.to_string(), value);
                println!("{:#?}", self.scope);
            }

            Token::LeftBrace => {
                //Manage destruction.
            }
            _ => {}
        }

        Ok(())
    }

    pub fn handle_identifier(&mut self, identifier: &String) -> Result<(), Box<dyn Error>> {
        self.position += 1;

        if let Some(operator) = self.instructions.get(self.position + 1) {
            match operator {
                Token::Increment => {
                    self.position += 1;
                    let value_tokens = self.value_collector();
                    let parent_scope = self.scope.clone();

                    let variable = self.scope.get_state(identifier).unwrap();

                    let value_scope = Scope::new(Some(parent_scope), value_tokens);
                    let mut value_interpretter = Interpretter::new(None, Some(value_scope));
                    value_interpretter.execute().unwrap();
                    let value = value_interpretter.interpretted_value;

                    // Handle the incrementations.
                    match variable {
                        JSValueVariant::JSNumber(js_number) => {
                            js_number.increment(value);
                        }

                        JSValueVariant::JSString(js_string) => {
                            js_string.increment(value);
                        }

                        _ => {}
                    };
                }
                _ => {}
            }
        } else {
            let variable = self.scope.get_state(identifier).unwrap();

            self.interpretted_value = variable.clone();
        }
        Ok(())
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
        }
    }
}
