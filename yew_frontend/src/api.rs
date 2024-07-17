use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::callback::Callback;

#[derive(Deserialize, Debug, Clone)]
pub struct BlockchainData {
    pub data: Vec<String>, // Replace with actual data structure
}

pub fn fetch_blockchain_data(callback: Callback<BlockchainData>) {
    spawn_local(async move {
        let window = window().expect("no global `window` exists");
        let resp = window.fetch_with_str("/api/blockchain").await.unwrap();
        let json = resp.json().await.unwrap();
        let data: BlockchainData = json.into_serde().unwrap();
        callback.emit(data);
    });
}
