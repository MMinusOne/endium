use crate::apis::type_variants::{js_pointer::*, js_string::*};

#[derive(Clone)]
pub enum ValueVariant {
    String(JSString),
    // Number(JSNumber),
    // Boolean(JSBool),
    // Array(JSArray),
    // Object(JSObject),
    Pointer(JSPointer),
    Null,
    Undefined,
}
