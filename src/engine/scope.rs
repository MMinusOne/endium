use crate::apis::type_variants::js_string::JSString;
use crate::engine::heap::Heap;
use crate::engine::state::State;
use crate::engine::tokens::Token;
use crate::engine::value_variant::JSValueVariant;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Scope {
    state: HashMap<String, State>,
    depth: usize,
    instructions: Vec<Token>,
    parent: Option<Rc<RefCell<Scope>>>,
    children: Vec<Rc<RefCell<Scope>>>,

    intialized_parent_state: bool,
}

impl Scope {
    pub fn initialize_parent_state(&mut self) {
        if let Some(parent) = &self.parent {
            let parent = parent.borrow();
            if self.intialized_parent_state == true {
                return;
            }
            for (key, state) in parent.state.iter() {
                match state.value() {
                    JSValueVariant::JSString(js_string) => {
                        // self.state.insert(
                        //     key.to_string(),
                        //     ValueVariant::Pointer(js_string.heap_ptr().clone()),
                        // );
                        self.state.insert(key.to_string(), state.clone());
                    }
                    JSValueVariant::JSPointer(js_ptr) => {
                        // self.state
                        //     .insert(key.to_string(), JSValueVariant::JSPointer(js_ptr.clone()));
                    }
                    JSValueVariant::JSBoolean(js_bool) => {}
                    JSValueVariant::JSNumber(js_number) => {}
                    JSValueVariant::Null => {}
                    JSValueVariant::Undefined => {}
                }
            }
        }
    }

    pub fn add_child(&mut self, scope: Scope) {
        self.children.push(Rc::new(RefCell::new(scope)));
    }

    pub fn instructions(&self) -> &Vec<Token> {
        &self.instructions
    }

    pub fn insert_state(&mut self, key: String, state: State) {
        self.state.insert(key, state);
    }

    pub fn get_state(&self, key: &String) -> Option<&State> {
        self.state.get(key)
    }

    pub fn get_state_mut(&mut self, key: &String) -> Option<&mut State> {
        self.state.get_mut(key)
    }

    pub fn new(parent: Option<Scope>, instructions: Vec<Token>) -> Self {
        let parent_depth = parent.as_ref().map(|p| p.depth).unwrap_or(0);
        let parent_rc = parent.map(|p| Rc::new(RefCell::new(p)));

        let mut scope_self = Self {
            parent: parent_rc.clone(),
            intialized_parent_state: false,
            state: HashMap::new(),
            depth: parent_depth,
            children: vec![],
            instructions,
        };

        scope_self.initialize_parent_state();

        if let Some(scope_parent) = &scope_self.parent {
            let mut scope_parent = scope_parent.borrow_mut();
            scope_parent.add_child(scope_self.clone());
        }

        scope_self
    }
}
