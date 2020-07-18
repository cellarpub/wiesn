use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello wasm".into()
}


#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b
}