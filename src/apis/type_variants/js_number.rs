use crate::{
    apis::features::assignment::addition_assignment::AdditionAssignment,
    engine::value_variant::JSValueVariant,
};

#[derive(Clone, Debug)]
pub struct JSNumber {
    is_primitive: bool,
    number_value: f64,
}

impl JSNumber {
    pub fn number_value(&self) -> f64 {
        self.number_value
    }

    pub fn new(number_value: f64) -> Self {
        Self {
            is_primitive: true,
            number_value,
        }
    }
}

impl AdditionAssignment for JSNumber {
    fn addition_assignment(&mut self, value: &JSValueVariant) {
        match value {
            JSValueVariant::JSNumber(js_num) => {
                self.number_value = self.number_value + js_num.number_value;
            }
            _ => {}
        }
    }
}
