use crate::error::{AudioError, Result};
use rodio::{Decoder, Source};
use std::io::Cursor;
use std::sync::Arc;

pub fn decode_bytes(bytes: Arc<Vec<u8>>) -> Result<Vec<f32>> {
    let bytes_vec = Arc::try_unwrap(bytes).map_err(|_| AudioError::InvalidData)?;

    let cursor = Cursor::new(bytes_vec);

    let decoder = Decoder::new(cursor).map_err(|_| AudioError::DecodingError)?;

    let sample_rate = decoder.sample_rate() as usize;

    if sample_rate == 0 {
        return Err(AudioError::InvalidData);
    }

    let samples: Vec<f32> = decoder.convert_samples::<f32>().collect();

    if samples.is_empty() {
        return Err(AudioError::ProcessingError);
    }

    let num_chunks = samples
        .len()
        .checked_div(sample_rate)
        .ok_or(AudioError::ProcessingError)?;

    let mut normalized_samples: Vec<f32> = Vec::with_capacity(num_chunks);

    for chunk in samples.chunks(sample_rate) {
        if chunk.len() != sample_rate {
            break;
        }

        let sum: f32 = chunk.iter().sum();
        let average = sum / sample_rate as f32;

        normalized_samples.push(average);
    }

    if normalized_samples.is_empty() {
        return Err(AudioError::ProcessingError);
    }

    Ok(normalized_samples)
}
