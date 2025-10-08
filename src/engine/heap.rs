use std::{
    collections::HashMap,
    error::Error,
    sync::{Mutex, OnceLock},
};

use crate::{apis::type_variants::js_pointer::JSPointer, engine::value_variant::ValueVariant};

#[derive(Debug)]
pub struct Heap {
    state: HashMap<String, ValueVariant>,
}

static INSTANCE: OnceLock<Mutex<Heap>> = OnceLock::new();

impl Heap {
    pub fn instance() -> &'static Mutex<Self> {
        INSTANCE.get_or_init(|| Mutex::new(Self::new()))
    }

    pub fn get_ptr(&self, ptr: &String) -> Option<&ValueVariant> {
        self.state.get(ptr)
    }

    pub fn allocate_ptr(&mut self, ptr: JSPointer) {
        let value = ptr.ptr_value().clone();
        self.state.insert(ptr.ptr().clone(), value);
    }

    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }
}
