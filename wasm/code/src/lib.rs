use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[wasm_bindgen]
pub fn uppercase(input: &str) -> String {
    input.to_uppercase()
}
