use std::cell::RefCell;
use std::rc::Rc;

use crate::apis::type_variants::js_string::JSString;
use crate::engine::interpretter;
use crate::engine::state::State;
use crate::engine::tokens::Token;
use crate::engine::value_variant::JSValueVariant;
use crate::scope::Scope;

#[derive(Clone, Debug)]
pub struct JSFunction {
    is_primitive: bool,
    argument_names: Vec<String>,
    scope: Rc<RefCell<Scope>>,
}

impl JSFunction {
    pub fn scope(&self) -> &Rc<RefCell<Scope>> {
        &self.scope
    }

    pub fn execute(&mut self, arguments: Vec<JSValueVariant>) {
        self.argument_names
            .iter()
            .zip(arguments)
            .for_each(|(key, state)| {
                self.scope
                    .borrow_mut()
                    .insert_state(key.to_string(), State::new(state, true));
            });
        let mut interpretter = interpretter::Interpretter::new(None, Some(self.scope.clone()));
        let _ = interpretter.execute();
        println!("Function scope {:#?}", self.scope);
        self.scope.borrow_mut().clear_state();
    }

    pub fn new(
        instructions: Vec<Token>,
        argument_names: Vec<String>,
        parent_scope: Rc<RefCell<Scope>>,
    ) -> Self {
        Self {
            is_primitive: true,
            argument_names,
            scope: Rc::new(RefCell::new(Scope::new(Some(parent_scope), instructions))),
        }
    }
}
