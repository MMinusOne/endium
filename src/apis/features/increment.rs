use crate::engine::value_variant::JSValueVariant;

pub trait Increment {
    fn increment(&self, value: JSValueVariant) -> JSValueVariant;
}
