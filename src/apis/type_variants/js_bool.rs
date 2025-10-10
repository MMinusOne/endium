#[derive(Clone, Debug)]
pub struct JSBool {
    is_primitive: bool,
    bool_value: bool,
}

impl JSBool {
    pub fn bool_value(&self) -> bool {
        self.bool_value
    }

    fn new(bool_value: bool) -> Self {
        Self {
            is_primitive: true,
            bool_value,
        }
    }
}
