use std::io::{Cursor};
use reqwest::{Client, Error};
use rodio::Decoder;
use rustfft::{FftDirection, FftPlanner};
use rustfft::num_complex::Complex;

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

async fn fetch_audio_bytes(url: &str) -> Result<Vec<u8>, Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;

    Ok(bytes.to_vec())
}

fn decode_bytes(bytes: &Vec<i8>) -> Vec<f32> {
    let cursor = Cursor::new(bytes);
    let decoder = Decoder::new(cursor).unwrap();

    let samples: Vec<f32> = decoder.into_sample().map(|s| s.unwrap()).collect();

    samples
}

fn audio_fft(audio_samples: &Vec<f32>) {
    let mut planner = FftPlanner::new();

    let fft = planner.plan_fft(audio_samples.len(), FftDirection::Forward);
    let mut input: Vec<Complex<f32>> = audio_samples.iter().map(|&x| Complex::new(x, 0.0)).collect();
    // let mut output: Vec<Complex<f32>> = vec![Complex::zero(); audio_samples.len()];
    let mut output = fft.process(&mut input);

    output
}