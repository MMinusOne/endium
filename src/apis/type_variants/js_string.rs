use std::{collections::HashMap, error::Error};

use crate::{
    apis::type_variants::js_pointer::JSPointer,
    engine::{heap::Heap, value_variant::ValueVariant},
    utils::generate_memory_address,
};

#[derive(Clone, Debug)]
enum JSStringProperty {
    Length,
}

#[derive(Clone, Debug)]
pub struct JSString {
    is_primitive: bool,
    properties: HashMap<JSStringProperty, ValueVariant>,
    str_value: String,
}

impl JSString {
    pub fn is_primitive(&self) -> bool {
        self.is_primitive
    }

    pub fn str_value(&self) -> &String {
        &self.str_value
    }

    pub fn new(str_value: String) -> Self {
        let heap_ptr = Box::new(JSPointer::new());

        let str_len = str_value.len();

        let str_self = Self {
            is_primitive: false,
            str_value,
            // properties: HashMap::from([(
            //     JSStringProperty::Length,
            //     ValueVariant::Number(str_value.len()),
            // )]),
        };

        str_self
    }
}
