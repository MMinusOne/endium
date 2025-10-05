use crate::{apis::type_variants::js_pointer::JSPointer, utils::generate_memory_address};

#[derive(Clone)]
pub struct JSString {
    is_primitive: bool,
    heap_ptr: JSPointer,
    str_value: String,
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
        Self {
            is_primitive: false,
            heap_ptr: JSPointer::new(),
            str_value,
        }
    }
}
