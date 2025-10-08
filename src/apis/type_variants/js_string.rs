use std::collections::HashMap;

use crate::{apis::type_variants::js_number::JSNumber, engine::value_variant::JSValueVariant};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum JSStringProperty {
    Length,
}

#[derive(Clone, Debug)]
pub struct JSString {
    is_primitive: bool,
    properties: HashMap<JSStringProperty, JSValueVariant>,
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
        let str_len = str_value.len();

        let str_self = Self {
            is_primitive: false,
            str_value,
            properties: HashMap::from([(
                JSStringProperty::Length,
                JSValueVariant::JSNumber(JSNumber::new(str_len as f64)),
            )]),
        };

        str_self
    }
}
