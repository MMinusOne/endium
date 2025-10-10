use std::collections::HashMap;

use crate::{
    apis::{
        features::assignment::{
            addition_assignment::AdditionAssignment, decrement_assignment::DecrementAssignment,
        },
        type_variants::js_number::JSNumber,
    },
    engine::value_variant::JSValueVariant,
};

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

    pub fn set_str_value(&mut self, str_addition: &String) {
        self.str_value = str_addition.to_string();
    }
}

impl AdditionAssignment for JSString {
    fn addition_assignment(&mut self, value: &JSValueVariant) {
        match value {
            JSValueVariant::JSNumber(js_num) => {
                self.str_value += &js_num.number_value().to_string()
            }
            _ => {}
        }
    }
}

impl DecrementAssignment for JSString {
    fn decrement_assignment(&mut self, value: &JSValueVariant) {
        match value {
            _ => {}
        }
    }
}
