mod audio;
mod fft;
mod utils;
mod error;

use crate::audio::{decode_bytes, normalize_peaks};
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use error::Result;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse_audio(audio_data: &[u8]) -> Result<Vec<f32>> {
    let decoded_bytes = decode_bytes(Arc::new(audio_data.to_vec()))?;
    Ok(decoded_bytes)
}