use crate::apis::type_variants::js_string::JSString;
use crate::engine::interpretter;
use crate::engine::tokens::Token;
use crate::engine::value_variant::JSValueVariant;
use crate::scope::Scope;

#[derive(Clone, Debug)]
pub struct JSFunction {
    is_primitive: bool,
    argument_names: Vec<JSString>,
    scope: Scope,
}

impl JSFunction {
    pub fn scope(&self) -> &Scope {
        &self.scope
    }

    pub fn execute(&self) {
        let mut interpretter = interpretter::Interpretter::new(None, Some(self.scope.clone()));
        interpretter.execute();
    }

    pub fn new(
        instructions: Vec<Token>,
        argument_names: Vec<JSString>,
        parent_scope: Scope,
    ) -> Self {
        Self {
            is_primitive: true,
            argument_names,
            scope: Scope::new(Some(parent_scope), instructions),
        }
    }
}
