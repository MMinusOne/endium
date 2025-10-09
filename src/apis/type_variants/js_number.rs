use crate::{apis::features::increment::Increment, engine::value_variant::JSValueVariant};

#[derive(Clone, Debug)]
pub struct JSNumber {
    is_primitive: bool,
    number_value: f64,
}

impl JSNumber {
    pub fn new(number_value: f64) -> Self {
        Self {
            is_primitive: true,
            number_value,
        }
    }
}

impl Increment for JSNumber {
    fn increment(&self, value: JSValueVariant) -> JSValueVariant {
        match value {
            JSValueVariant::JSNumber(js_num) => {
                JSValueVariant::JSNumber(JSNumber::new(self.number_value + js_num.number_value))
            }
            _ => JSValueVariant::JSNumber(self.clone()),
        }
    }
}
