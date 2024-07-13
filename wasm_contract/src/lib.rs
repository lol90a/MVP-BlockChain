use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn contract_function(x: i32) -> i32 {
    x * 2
}
