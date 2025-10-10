use crate::engine::value_variant::JSValueVariant;

pub trait MultiplicationAssignment {
    fn multiplication_assignment(&mut self, value: &JSValueVariant);
}
