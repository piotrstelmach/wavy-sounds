use std::io::{Cursor};
use std::sync::Arc;
use rodio::{Decoder, Source};
use rustfft::{FftDirection, FftPlanner};
use rustfft::num_complex::Complex;

extern crate web_sys;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn decode_bytes(bytes: Arc<Vec<u8>>) -> Vec<f32> {
    let bytes_vec = match Arc::try_unwrap(bytes) {
        Ok(vec) => vec,
        Err(_) => panic!("More than one strong reference to the data"),
    };
    let cursor = Cursor::new(bytes_vec);
    let decoder = Decoder::new(cursor).unwrap();
    let sample_rate = decoder.sample_rate() as usize;
    let samples: Vec<f32> = decoder.convert_samples::<f32>().collect();
    let num_chunks = samples.len() / sample_rate;

    let mut normalized_samples: Vec<f32> = Vec::with_capacity(num_chunks);
    for i in 0..num_chunks {
        let start_index = i * sample_rate;
        let end_index = (i + 1) * sample_rate;
        let average = samples[start_index..end_index].iter().sum::<f32>() / sample_rate as f32;
        normalized_samples.push(average);
    }


    normalized_samples
}

pub fn normalize_peaks(peaks: &Vec<f32>, group_size: usize) -> Vec<f32> {
    let mut reduced_data: Vec<f32> = Vec::new();
    for i in (0..peaks.len()).step_by(group_size) {
        let end_index = (i + group_size).min(peaks.len());
        let group = &peaks[i..i + end_index];
        let max_value = group.iter().fold(f32::NEG_INFINITY, |acc, &x| acc.max(x));
        reduced_data.push(max_value); // Push the maximum (or minimum) value to the reduced data array
    }

    reduced_data
}

pub fn audio_fft(audio_samples: &Vec<f32>) -> Vec<Complex<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft(audio_samples.len(), FftDirection::Forward);
    let mut input: Vec<Complex<f32>> = audio_samples.iter().map(|&x| Complex::new(x, 0.0)).collect();

    fft.process(&mut input);

    input
}
