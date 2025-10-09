use crate::engine::value_variant::JSValueVariant;

pub trait AdditionAssignment {
    fn addition_assignment(&mut self, value: &JSValueVariant);
}
