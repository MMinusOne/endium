use crate::apis::type_variants::{js_bool::*, js_number::*, js_pointer::*, js_string::*, js_function::*};

#[derive(Clone, Debug)]
pub enum JSValueVariant {
    JSString(JSString),
    JSNumber(JSNumber),
    JSBoolean(JSBool),
    // Array(JSArray),
    // Object(JSObject),
    JSFunction(JSFunction),
    JSPointer(JSPointer),
    Null,
    Undefined,
}
