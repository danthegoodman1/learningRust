use wasm_bindgen::prelude::*;

// This attribute makes the function accessible from JavaScript
#[wasm_bindgen]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
