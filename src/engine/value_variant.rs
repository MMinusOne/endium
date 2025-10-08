use crate::apis::type_variants::{js_number::*, js_pointer::*, js_string::*};

#[derive(Clone, Debug)]
pub enum JSValueVariant {
    JSString(JSString),
    JSNumber(JSNumber),
    // Boolean(JSBool),
    // Array(JSArray),
    // Object(JSObject),
    JSPointer(JSPointer),
    Null,
    Undefined,
}
