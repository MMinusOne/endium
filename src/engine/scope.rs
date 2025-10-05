use crate::engine::heap::Heap;
use crate::engine::tokens::Token;
use crate::engine::value_variant::ValueVariant;
use std::collections::HashMap;

pub struct Scope<'a> {
    state: HashMap<String, ValueVariant>,
    depth: usize,
    instructions: Vec<Token>,
    parent: Option<&'a Scope<'a>>,

    intialized_parent_state: bool,
    heap: &'a Heap,
}

impl<'a> Scope<'a> {
    pub fn initialize_parent_state(&mut self) {
        if let Some(parent) = self.parent {
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
                }
            }
        }
    }

    pub fn new(parent: Option<&'a Scope>, instructions: Vec<Token>) -> Self {
        Self {
            heap: Heap::instance(),
            parent: parent,
            intialized_parent_state: false,
            state: HashMap::new(),
            depth: match parent {
                Some(p) => p.depth - 1,
                None => 0,
            },
            instructions,
        }
    }
}
