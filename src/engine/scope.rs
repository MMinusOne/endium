use crate::engine::heap::Heap;
use crate::engine::tokens::Token;
use crate::engine::value_variant::ValueVariant;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Scope {
    state: HashMap<String, ValueVariant>,
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
            for (key, value) in parent.state.iter() {
                match value {
                    ValueVariant::String(js_string) => {
                        self.state.insert(
                            key.to_string(),
                            ValueVariant::Pointer(js_string.heap_ptr().clone()),
                        );
                    }
                    ValueVariant::Pointer(js_ptr) => {
                        self.state
                            .insert(key.to_string(), ValueVariant::Pointer(js_ptr.clone()));
                    }
                    ValueVariant::Null => {}
                    ValueVariant::Undefined => {}
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

    pub fn insert_state(&mut self, key: String, value: ValueVariant) {
        self.state.insert(key, value);
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
