# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

`wavy-sounds` is a Rust library compiled to WebAssembly (via `wasm-bindgen`) that decodes an audio file and produces a normalized array of amplitude values (`Vec<f32>`) suitable for rendering a waveform in JS/canvas. It's published to npm as `wavy-sounds`. The `example/` directory is a Vite + TypeScript app that consumes the built package to demonstrate drawing a waveform.

## Commands

Build/test the Rust crate directly:
```bash
cargo build
cargo test                 # only wasm32-gated tests exist (see tests/web.rs); this alone won't run them
```

Build the WASM package for npm/JS consumption (what CI does, matches `.github/workflows/ci.yml`):
```bash
wasm-pack build --release --target bundler
```
This generates `pkg/` (gitignored) containing the JS bindings and `.wasm` binary — required before running the example app, since `example/wavy-sound-example` depends on it via `"wavy-sounds": "file:../../pkg/"`.

Run the wasm-bindgen browser test suite (`tests/web.rs`) — requires a headless browser:
```bash
wasm-pack test --headless --chrome
# or --firefox
```

Run the example app (after building `pkg/` above):
```bash
cd example/wavy-sound-example
npm install
npm run dev      # vite dev server
npm run build    # tsc && vite build
```

## Architecture

Entry point `src/lib.rs` exposes a single `#[wasm_bindgen]` function, `parse_audio(audio_data: &[u8], group_size: usize) -> Result<Vec<f32>>`, which is the only public API surface consumed by JS.

Pipeline modules under `src/`:
- `audio/decoder.rs` (`decode_bytes`) — decodes raw bytes with `rodio::Decoder`, converts to `f32` samples, chunks them to ~20 points/second, computes RMS per chunk, and normalizes against the max RMS value.
- `audio/processor.rs` (`normalize_peaks`) — takes the RMS-normalized peaks from `decode_bytes` and further reduces them by grouping every `group_size` peaks and taking the max per group. `parse_audio` calls `decode_bytes` then `normalize_peaks` in sequence.
- `fft/transformer.rs` (`audio_fft`) — performs an FFT over samples using `rustfft`. Exported but not currently called from `lib.rs`. Treat it as available infrastructure for frequency-domain features, not part of the active pipeline.
- `error.rs` — defines `AudioError` (a `#[wasm_bindgen]` enum) and the crate's `Result<T>` alias; errors cross the WASM boundary as this enum.
- `utils/panic.rs` — installs `console_error_panic_hook` (gated by the `console_error_panic_hook` feature, on by default) so Rust panics surface as browser console errors instead of opaque WASM traps.

## CI/CD (`.github/workflows/ci.yml`)

- Every push/PR to `main`: builds the crate to WASM (`wasm-pack build --release --target bundler`).
- On a `v*` tag push (with `NODE_AUTH_TOKEN` secret set): publishes the built package to npm.
- On every build: deploys `example/wavy-sound-example` (built with Vite) to GitHub Pages.

`.appveyor.yml` and `.travis.yml` are leftover from the original wasm-pack project template and are not part of the active CI (GitHub Actions is authoritative).
