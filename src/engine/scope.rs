use std::collections::HashMap;

use crate::engine::value_variant::ValueVariant;

pub struct Scope {
    state: HashMap<String, ValueVariant>,
}

impl Scope {}
