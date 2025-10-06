use std::error::Error;

use crate::{
    apis::type_variants::js_pointer::JSPointer,
    engine::{heap::Heap, value_variant::ValueVariant},
    utils::generate_memory_address,
};

#[derive(Clone)]
pub struct JSString {
    is_primitive: bool,
    length: usize,
    str_value: String,
    heap_ptr: Box<JSPointer>,
}

impl JSString {
    pub fn is_primitive(&self) -> bool {
        self.is_primitive
    }

    pub fn heap_ptr(&self) -> &JSPointer {
        &self.heap_ptr
    }

    pub fn str_value(&self) -> &String {
        &self.str_value
    }

    pub fn new(str_value: String) -> Self {
        let heap_ptr = Box::new(JSPointer::new());

        let str_len = str_value.len();

        let str_self = Self {
            is_primitive: false,
            heap_ptr,
            str_value,
            length: str_len,
        };

        str_self
    }
}
