//! Regression tests for the symphonia/opus-decoder decode pipeline, using small
//! real-format fixtures checked into `tests/fixtures/` (generated with ffmpeg from
//! a 1s 440Hz sine tone).
//!
//! This crate's `.cargo/config.toml` defaults `cargo build`/`cargo test` to the
//! `wasm32-unknown-unknown` target (matching the actual wasm-pack deployment target),
//! but a `.wasm` test binary can't be executed directly on the host. Run this file
//! against your host target instead:
//!
//!   cargo test --target $(rustc -vV | sed -n 's/host: //p') --test decode_regression

use std::path::PathBuf;

fn fixture(name: &str) -> Vec<u8> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    path.push(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("failed to read fixture {:?}: {}", path, e))
}

/// Guards against a real bug we hit: `AudioBuffer::copy_to_vec_interleaved` resizes/
/// overwrites its destination on every call, so accumulating decoded packets into a
/// `Vec` by calling it directly (instead of appending via a scratch buffer) silently
/// keeps only the *last* decoded packet. For a 1s clip that collapses the decoded
/// audio down to a single ~26ms MP3 frame, which in turn collapses the RMS-windowed
/// output to a single point. Asserting more than one point here catches that class
/// of regression for any of the non-Opus codec paths.
fn assert_full_track_decoded(label: &str, bytes: &[u8]) {
    let peaks = wavy_sounds::parse_audio(bytes, 4)
        .unwrap_or_else(|e| panic!("{}: parse_audio failed: {:?}", label, e));

    assert!(
        peaks.len() > 1,
        "{}: expected multiple RMS-window peaks for a 1s clip, got {} \
         (a count of 1 usually means only the last decoded packet survived)",
        label,
        peaks.len()
    );
    assert!(peaks.iter().all(|v| v.is_finite()), "{}: all peaks should be finite", label);
}

#[test]
fn decodes_wav() {
    assert_full_track_decoded("wav", &fixture("tone.wav"));
}

#[test]
fn decodes_mp3() {
    assert_full_track_decoded("mp3", &fixture("tone.mp3"));
}

#[test]
fn decodes_webm_opus() {
    assert_full_track_decoded("webm/opus", &fixture("tone.webm"));
}
