mod utils;

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use crate::utils::{audio_fft, decode_bytes };

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wavy-sounds!");
}

#[wasm_bindgen]
pub fn parse_audio(audio_data: &[u8]) -> Result<JsValue, JsValue> {
    let decoded_bytes = decode_bytes(Arc::new(audio_data.to_vec()));
    let result = audio_fft(&decoded_bytes);

    let result_flattened: Vec<f32> = result.iter().flat_map(|c| vec![c.re, c.im]).collect();

    Ok(serde_wasm_bindgen::to_value(&result_flattened).unwrap())
}