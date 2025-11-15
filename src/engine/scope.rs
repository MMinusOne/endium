use crate::apis::type_variants::js_string::JSString;
use crate::engine::state::State;
use crate::engine::tokens::Token;
use crate::engine::value_variant::JSValueVariant;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Scope {
    state: HashMap<String, Rc<RefCell<State>>>,
    parent_state: HashMap<String, Rc<RefCell<State>>>,
    depth: usize,
    instructions: Vec<Token>,
    children: Vec<Rc<RefCell<Scope>>>,
    intialized_parent_state: bool,
}

impl Scope {
    pub fn initialize_parent_state(&mut self) {
        if self.intialized_parent_state == true {
            return;
        }

        for (key, state) in self.parent_state.iter() {
            self.state.insert(key.into(), Rc::clone(state));
        }
    }

    pub fn add_child(&mut self, scope: Scope) {
        self.children.push(Rc::new(RefCell::new(scope)));
    }

    pub fn instructions(&self) -> &Vec<Token> {
        &self.instructions
    }

    pub fn insert_state(&mut self, key: String, state: State) {
        self.state.insert(key, Rc::new(RefCell::new(state)));
    }

    pub fn get_state(&self, key: &String) -> Option<&Rc<RefCell<State>>> {
        self.state.get(key)
    }

    pub fn get_state_mut(&mut self, key: &String) -> Option<&mut Rc<RefCell<State>>> {
        self.state.get_mut(key)
    }

    pub fn state(&self) -> &HashMap<String, Rc<RefCell<State>>> {
        &self.state
    }

    pub fn clear_state(&mut self) {
        self.state.clear();
    }

    pub fn new(parent: Option<Rc<RefCell<Scope>>>, instructions: Vec<Token>) -> Self {
        let parent_depth = parent.as_ref().map(|p| p.borrow().depth).unwrap_or(0);
        let parent_state = match &parent {
            Some(p) => p.borrow().state().clone(),
            None => HashMap::new(),
        };

        let mut scope_self = Self {
            parent_state,
            intialized_parent_state: false,
            state: HashMap::new(),
            depth: parent_depth,
            children: vec![],
            instructions,
        };

        scope_self.initialize_parent_state();

        if let Some(scope_parent_rc) = parent {
            scope_parent_rc.borrow_mut().add_child(scope_self.clone());
        }

        scope_self
    }
}
