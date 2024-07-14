use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub fn contract_function(x: i32) -> i32 {
    x * 2
}

#[derive(Serialize, Deserialize)]
pub struct ContractState {
    pub value: i32,
}

#[wasm_bindgen]
pub fn get_state() -> JsValue {
    let state = ContractState { value: 42 };
    JsValue::from_serde(&state).unwrap()
}
