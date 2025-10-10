use crate::apis::type_variants::{js_bool::*, js_number::*, js_pointer::*, js_string::*};

#[derive(Clone, Debug)]
pub enum JSValueVariant {
    JSString(JSString),
    JSNumber(JSNumber),
    JSBoolean(JSBool),
    // Array(JSArray),
    // Object(JSObject),
    JSPointer(JSPointer),
    Null,
    Undefined,
}
