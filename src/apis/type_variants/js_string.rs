use std::collections::HashMap;

use crate::{
    apis::{
        features::{
            assignment::{
                addition_assignment::AdditionAssignment, decrement_assignment::DecrementAssignment,
                division_assignment::DivisionAssignment,
                multiplication_assignment::MultiplicationAssignment,
            },
            object_features::ObjectFeatures,
        },
        type_variants::js_number::JSNumber,
    },
    engine::value_variant::JSValueVariant,
};

#[derive(Clone, Debug)]
pub struct JSString {
    is_primitive: bool,
    properties: HashMap<String, JSValueVariant>,
    str_value: String,
}

impl JSString {
    pub fn is_primitive(&self) -> bool {
        self.is_primitive
    }

    pub fn str_value(&self) -> &String {
        &self.str_value
    }

    pub fn new() -> Self {
        Self {
            is_primitive: false,
            str_value: String::new(),
            properties: HashMap::from([(
                String::from("length"),
                JSValueVariant::JSNumber(JSNumber::new(0f64)),
            )]),
        }
    }

    pub fn from(str_value: String) -> Self {
        let str_len = str_value.len();

        let str_self = Self {
            is_primitive: false,
            str_value,
            properties: HashMap::from([(
                String::from("length"),
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
                self.str_value += &js_num.number_value().to_string();
            }
            JSValueVariant::JSString(string) => {
                self.str_value += &string.str_value;
            }
            _ => {}
        }
        if let Some(length) = self.properties.get_mut("length") {
            *length = JSValueVariant::JSNumber(JSNumber::new(self.str_value.len() as f64));
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

impl MultiplicationAssignment for JSString {
    fn multiplication_assignment(&mut self, value: &JSValueVariant) {}
}

impl DivisionAssignment for JSString {
    fn division_assignment(&mut self, value: &JSValueVariant) {}
}

impl ObjectFeatures for JSString {
    fn get_property(&self, property_key: &String) -> Option<&JSValueVariant> {
        self.properties.get(property_key)
    }
    fn get_property_mut(&mut self, property_key: &String) -> Option<&mut JSValueVariant> {
        self.properties.get_mut(property_key)
    }
}
