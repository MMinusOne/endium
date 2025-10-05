use crate::utils::generate_memory_address;

#[derive(Clone)]
pub struct JSPointer {
    is_primitive: bool,
    ptr_value: String,
}

impl JSPointer {
    pub fn from(ptr_value: String) -> Self {
        Self {
            is_primitive: false,
            ptr_value,
        }
    }

    pub fn new() -> Self {
        Self {
            is_primitive: false,
            ptr_value: generate_memory_address(),
        }
    }
}
