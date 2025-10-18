use crate::engine::value_variant::JSValueVariant;

pub trait ObjectFeatures {
    fn get_property(&self, property_key: &String) -> Option<&JSValueVariant>;
    fn get_property_mut(&mut self, property_key: &String) -> Option<&mut JSValueVariant>;
}
