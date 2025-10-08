#[derive(Clone, Debug)]
pub struct JSNumber {
    is_primitive: bool,
    number: f64,
}

impl JSNumber {
    pub fn new(number: f64) -> Self {
        Self {
            is_primitive: true,
            number,
        }
    }
}
