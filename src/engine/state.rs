use crate::engine::value_variant::JSValueVariant;

#[derive(Debug, Clone)]
pub struct State {
    value: JSValueVariant,
    mutable: bool,
}

impl State {
    pub fn value(&self) -> &JSValueVariant {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut JSValueVariant {
        &mut self.value
    }

    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    pub fn new(value: JSValueVariant, mutable: bool) -> Self {
        Self { value, mutable }
    }
}
