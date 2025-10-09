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
