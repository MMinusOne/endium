use crate::engine::value_variant::JSValueVariant;

pub trait DivisionAssignment {
    fn division_assignment(&mut self, value: &JSValueVariant);
}
