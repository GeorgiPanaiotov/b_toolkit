use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn encode_base_64(input: &str) -> String {
    let b64 = general_purpose::STANDARD.encode(input);
    b64
}

#[wasm_bindgen]
pub fn decode_base_64(input: &str) -> Vec<u8> {
    let bytes = general_purpose::STANDARD.decode(input).unwrap();
    bytes
}

#[wasm_bindgen]
pub fn decode_bytes(input: Vec<u8>) -> String {
    let mut string_result = String::new();
    for byte in input {
        let key = byte as char;
        string_result.push(key);
    }
    string_result
}

#[wasm_bindgen]
pub fn minify_json(input: &str) -> String {
    let value: Value = serde_json::from_str(input).expect("Unable to convert JSON");

    serde_json::to_string(&value).expect("Unable to process JSON")
}
