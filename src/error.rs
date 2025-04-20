use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug)]
pub enum AudioError {
    DecodingError,
    EncodingError,
    IOError,
    InvalidData,
    ProcessingError,
}

impl Display for AudioError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioError::DecodingError => write!(f, "Decoding error"),
            AudioError::EncodingError => write!(f, "Encoding error"),
            AudioError::IOError => write!(f, "IO error"),
            AudioError::InvalidData => write!(f, "Invalid data"),
            AudioError::ProcessingError => write!(f, "Processing error"),
        }
    }
}

pub type Result<T> = std::result::Result<T, AudioError>;
