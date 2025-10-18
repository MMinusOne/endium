use crate::{
    apis::features::object_features::ObjectFeatures, engine::value_variant::JSValueVariant,
    utils::ListNode,
};

pub struct PropertyPath {
    head: ListNode<String>,
    tail: Box<ListNode<String>>,
}

impl PropertyPath {
    pub fn add_path(&mut self, path_name: String) {
        let new_tail = ListNode::new(path_name, None);
        self.tail.as_mut().chain(new_tail.clone());
        self.tail = Box::new(new_tail);
    }
    pub fn resolve_path(&self, object: Box<&dyn ObjectFeatures>) {
        let mut current_value: Option<JSValueVariant> = None;
        for property_key in self.head.clone() {
            match current_value {
                Some(ref value) => match value {
                    JSValueVariant::JSString(js_string) => {
                        let property = js_string.get_property(&property_key);
                        current_value = property.cloned();
                    }
                    _ => {}
                },
                None => {
                    let property = object.get_property(&property_key);
                    current_value = property.cloned();
                }
            };
        }
    }
}
