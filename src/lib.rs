pub mod app;
pub mod components;
pub mod error_template;
pub mod models;
pub mod pages;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Dummy {
    pub dummy_val: i64,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
