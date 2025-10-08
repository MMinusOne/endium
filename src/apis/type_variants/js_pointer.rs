use std::error::Error;

use crate::{
    engine::{heap::Heap, value_variant::ValueVariant},
    utils::generate_memory_address,
};

#[derive(Clone, Debug)]
pub struct JSPointer {
    is_primitive: bool,
    ptr: String,
    ptr_value: Box<ValueVariant>,
}

impl JSPointer {
    pub fn ptr(&self) -> &String {
        &self.ptr
    }

    pub fn ptr_value(&self) -> &ValueVariant {
        &self.ptr_value
    }

    pub fn allocate_value(&mut self, value: ValueVariant) -> Result<(), Box<dyn Error>> {
        self.ptr_value = Box::new(value);
        Ok(())
    }

    pub fn from(ptr: String) -> Self {
        Self {
            is_primitive: false,
            ptr,
            ptr_value: Box::new(ValueVariant::Undefined),
        }
    }

    pub fn new() -> Self {
        Self {
            is_primitive: false,
            ptr: generate_memory_address(),
            ptr_value: Box::new(ValueVariant::Undefined),
        }
    }
}
