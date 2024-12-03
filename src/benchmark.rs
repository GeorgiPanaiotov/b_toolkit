use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn check_memory() -> JsValue {
    let mem = wasm_bindgen::memory();
    mem
}
