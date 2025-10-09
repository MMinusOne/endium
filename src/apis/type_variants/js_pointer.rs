use std::error::Error;

use crate::{engine::value_variant::JSValueVariant, utils::generate_memory_address};

#[derive(Clone, Debug)]
pub struct JSPointer {
    is_primitive: bool,
    ptr: String,
    ptr_value: Box<JSValueVariant>,
}

impl JSPointer {
    pub fn ptr(&self) -> &String {
        &self.ptr
    }

    pub fn ptr_value(&self) -> &JSValueVariant {
        &self.ptr_value
    }

    pub fn allocate_value(&mut self, value: JSValueVariant) -> Result<(), Box<dyn Error>> {
        self.ptr_value = Box::new(value);
        Ok(())
    }

    pub fn from(ptr: String) -> Self {
        Self {
            is_primitive: true,
            ptr,
            ptr_value: Box::new(JSValueVariant::Undefined),
        }
    }

    pub fn new() -> Self {
        Self {
            is_primitive: true,
            ptr: generate_memory_address(),
            ptr_value: Box::new(JSValueVariant::Undefined),
        }
    }
}
