use crate::apis::features::assignment::addition_assignment::AdditionAssignment;
use crate::apis::features::assignment::decrement_assignment::DecrementAssignment;
use crate::engine::state::State;
use crate::{
    apis::type_variants::{js_number::JSNumber, js_string::JSString},
    engine::{tokens::Token, value_variant::JSValueVariant},
    scope::Scope,
};
use std::error::Error;

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

                Token::Let => self.handle_let(),

                Token::String(s) => {
                    self.position += 1;
                    self.interpretted_value = JSValueVariant::JSString(JSString::new(s.clone())); // Unhandled strings by other tokens are the interpretted_value    
                }

                Token::Number(n) => {
                    self.position += 1;
                    let number = n.parse::<f64>()?;
                    self.interpretted_value = JSValueVariant::JSNumber(JSNumber::new(number));
                }

                Token::Identifier(identifier) => self.handle_identifier(identifier),

                Token::Null => {
                    self.position += 1;
                    self.interpretted_value = JSValueVariant::Null;
                }

                Token::Undefined => {
                    self.position += 1;
                    self.interpretted_value = JSValueVariant::Undefined;
                }

                Token::Eof => break,

                _ => {
                    self.position += 1;
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
                Token::Newline | Token::Semicolon => {
                    return value_tokens;
                }
                _ => {}
            }

            value_tokens.push(v_token);
            self.position += 1;
        }
        value_tokens
    }

    pub fn handle_const(&mut self) {
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
                let state: State = State::new(value, false);

                self.scope.insert_state(variable_name.to_string(), state);
            }

            Token::LeftBrace => {
                //Manage destruction.
            }
            _ => {}
        }
    }

    pub fn handle_let(&mut self) {
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
                let state: State = State::new(value, true);

                self.scope.insert_state(variable_name.to_string(), state);
            }

            Token::LeftBrace => {
                //Manage destruction.
            }
            _ => {}
        }
    }

    pub fn handle_identifier(&mut self, identifier: &String) {
        if let Some(operator) = self.instructions.get(self.position + 1) {
            self.position += 1;
            let operator = operator.clone();
            let value_tokens = self.value_collector();
            let parent_scope = self.scope.clone();

            match operator {
                Token::Increment => self.handle_increment(identifier),
                Token::Decrement => self.handle_decrement(identifier),
                Token::PlusAssign
                | Token::MinusAssign
                | Token::DivideAssign
                | Token::MultiplyAssign => {
                    let value_scope = Scope::new(Some(parent_scope), value_tokens);
                    let mut value_interpretter = Interpretter::new(None, Some(value_scope));
                    value_interpretter.execute().unwrap();
                    let value = value_interpretter.interpretted_value;

                    match operator {
                        Token::PlusAssign => self.handle_addition_assignment(identifier, value),
                        Token::MinusAssign => self.handle_subtraction_assignment(identifier, value),
                        Token::MultiplyAssign => {
                            self.handle_multiplication_assignment(identifier, value)
                        }
                        Token::DivideAssign => self.handle_division_assignment(identifier, value),
                        _ => {}
                    }
                }

                _ => {}
            };
        } else {
            let variable = self.scope.get_state(identifier).unwrap();

            self.interpretted_value = variable.value().clone();
        }
    }

    fn handle_increment(&mut self, variable_identifier: &String) {
        let variable_mut = self.scope.get_state_mut(variable_identifier).unwrap();

        if !variable_mut.is_mutable() {
            return; // Assignment to constant variable error.
        }

        match variable_mut.value_mut() {
            JSValueVariant::JSNumber(js_number) => {
                js_number.addition_assignment(&JSValueVariant::JSNumber(JSNumber::new(1.0)));
            }
            JSValueVariant::JSString(js_string) => {
                js_string.addition_assignment(&JSValueVariant::JSNumber(JSNumber::new(1.0)));
            }
            _ => {}
        }
    }

    fn handle_decrement(&mut self, variable_identifier: &String) {
        let variable_mut = self.scope.get_state_mut(variable_identifier).unwrap();

        if !variable_mut.is_mutable() {
            return; // Assignment to constant variable error.
        }

        match variable_mut.value_mut() {
            JSValueVariant::JSNumber(js_number) => {
                js_number.addition_assignment(&JSValueVariant::JSNumber(JSNumber::new(-1.0)));
            }
            JSValueVariant::JSString(js_string) => {
                js_string.addition_assignment(&JSValueVariant::JSNumber(JSNumber::new(-1.0)));
            }
            _ => {}
        }
    }

    fn handle_addition_assignment(&mut self, variable_identifier: &String, value: JSValueVariant) {
        let variable_mut = self.scope.get_state_mut(variable_identifier).unwrap();

        if !variable_mut.is_mutable() {
            return; // Assignment to constant variable error.
        }

        match variable_mut.value_mut() {
            JSValueVariant::JSNumber(js_number) => {
                js_number.addition_assignment(&value);
            }
            JSValueVariant::JSString(js_string) => {
                js_string.addition_assignment(&value);
            }
            _ => {}
        }
    }

    fn handle_subtraction_assignment(
        &mut self,
        variable_identifier: &String,
        value: JSValueVariant,
    ) {
        let variable_mut = self.scope.get_state_mut(variable_identifier).unwrap();

        if !variable_mut.is_mutable() {
            return; // Assignment to constant variable error.
        }

        match variable_mut.value_mut() {
            JSValueVariant::JSNumber(js_number) => {
                js_number.decrement_assignment(&value);
            }
            JSValueVariant::JSString(js_string) => {
                js_string.decrement_assignment(&value);
            }
            _ => {}
        }
    }

    fn handle_multiplication_assignment(
        &self,
        variable_identifier: &String,
        value: JSValueVariant,
    ) {
    }

    fn handle_division_assignment(&self, variable_identifier: &String, value: JSValueVariant) {}

    pub fn interpretted_value(&self) -> &JSValueVariant {
        &self.interpretted_value
    }

    pub fn scope(&self) -> &Scope {
        &self.scope
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
