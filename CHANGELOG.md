# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0]

### Added
- WebM/Opus decoding support (e.g. audio recorded in-browser via `MediaRecorder`), via a pure-Rust `opus-decoder` alongside `symphonia`'s WebM/Matroska demuxer — `symphonia` itself has no working Opus decoder.
- Real regression tests (`tests/decode_regression.rs`) against committed WAV/MP3/WebM fixtures, run in CI before every publish. Previously the crate had no tests running in CI at all.
- `CLAUDE.md` project documentation.

### Changed
- **Breaking:** `parse_audio(audio_data)` is now `parse_audio(audio_data, group_size)` — wires the previously-unused `normalize_peaks` reduction step into the pipeline. `group_size` controls how many consecutive RMS peaks are reduced (by max) into one output point.
- Replaced `rodio` with `symphonia` (+ `opus-decoder` for Opus) for decoding. Decoding is now zero-copy — no owned `Vec<u8>` of the input audio is ever created.
- Compiled `.wasm` trimmed from a full `symphonia` feature set down to just MP3, WAV, and WebM/Opus — the formats this library's browser use case actually needs. Shrinks the release binary from 1.42MB to ~560KB (60% smaller). FLAC, Vorbis/OGG, ADPCM, and AAC/MP4 are no longer supported (`parse_audio` returns `AudioError::UnsupportedCodec` for them).
- CI (`ci.yml`) now actually builds, tests, and publishes: npm publishing uses trusted publishing (OIDC) instead of a stored token; GitHub Pages deployment uses the official `actions/deploy-pages` flow instead of `peaceiris/actions-gh-pages`; all third-party actions pinned to commit SHAs.

### Fixed
- A packet-accumulation bug in the MP3/WAV/FLAC/Vorbis/AAC decode path: `AudioBuffer::copy_to_vec_interleaved` overwrites its destination buffer on every call rather than appending, so only the last decoded packet of a file was ever kept. A multi-second file was silently analyzed as if it were only its last ~26ms. Found via real-format regression testing, not by inspection.
- `example/wavy-sound-example`: removed calls to a default `init()` export that no longer exists with current `wasm-bindgen` (0.2.100) — the bundler target self-initializes on import.
- `example/wavy-sound-example`: added `vite-plugin-wasm` — Vite 6 can't load the raw ESM `.wasm` import `wasm-pack`'s `bundler` target emits without it, in dev or production build.

### Removed
- Dropped `rodio`, and with it its transitive dependencies unused by this crate: `cpal` (audio *playback*, unreachable in a browser), `claxon` (FLAC), `hound` (WAV), `lewton` (Vorbis) — decoding for the formats this crate keeps now comes directly from `symphonia`.

## [1.1.0] and earlier

Not documented retroactively; see git history.
