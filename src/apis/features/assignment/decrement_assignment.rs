use crate::engine::value_variant::JSValueVariant;

pub trait DecrementAssignment {
    fn decrement_assignment(&mut self, value: &JSValueVariant);
}
