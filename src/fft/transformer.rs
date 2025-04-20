use crate::error::{Result, AudioError};
use rustfft::num_complex::Complex;
use rustfft::{FftDirection, FftPlanner};

pub fn audio_fft(audio_samples: &Vec<f32>) -> Result<Vec<Complex<f32>>> {
    if audio_samples.is_empty() {
        return Err(AudioError::InvalidData);
    }

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft(audio_samples.len(), FftDirection::Forward);

    let mut input: Vec<Complex<f32>> = audio_samples
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    fft.process(&mut input);

    Ok(input)
}
