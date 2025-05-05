use crate::error::{AudioError, Result};
use rodio::{Decoder, Source};
use std::io::Cursor;
use std::sync::Arc;

pub fn decode_bytes(bytes: Arc<Vec<u8>>) -> Result<Vec<f32>> {
    let bytes_vec = Arc::try_unwrap(bytes).map_err(|_| AudioError::InvalidData)?;
    let cursor = Cursor::new(bytes_vec);
    let decoder = Decoder::new(cursor).map_err(|_| AudioError::DecodingError)?;

    let sample_rate = decoder.sample_rate() as usize;
    let samples: Vec<f32> = decoder.convert_samples::<f32>().collect();

    if samples.is_empty() {
        return Err(AudioError::ProcessingError);
    }

    let duration_sec = samples.len() as f32 / sample_rate as f32;
    let points_per_second = 20;
    let desired_points = (duration_sec * points_per_second as f32).round() as usize;
    let chunk_size = if desired_points == 0 { samples.len() } else { samples.len() / desired_points };

    if chunk_size == 0 {
        return Err(AudioError::ProcessingError);
    }

    let mut rms_values = Vec::with_capacity(desired_points);

    for chunk in samples.chunks(chunk_size) {
        if chunk.is_empty() {
            continue;
        }

        let rms = (chunk.iter().map(|x| x * x).sum::<f32>() / chunk.len() as f32).sqrt();
        rms_values.push(rms);
    }

    if rms_values.is_empty() {
        return Err(AudioError::ProcessingError);
    }

    let max_rms = rms_values.iter().cloned().fold(0.0, f32::max);
    if max_rms == 0.0 {
        return Err(AudioError::ProcessingError);
    }

    let normalized_samples: Vec<f32> = rms_values.iter().map(|v| v / max_rms).collect();

    Ok(normalized_samples)
}
