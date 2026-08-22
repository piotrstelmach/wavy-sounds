# Wavy Sounds

## Description
Simple library written in Rust for analyze sounds and generate wave data to display it. Now it's experimental use library and I've implemeneted only simple approach. It's experiment for analyze sounds in Rust and gain some knowledge of Web Assembly as a new big web feature. 

See [CHANGELOG.md](./CHANGELOG.md) for what's changed between versions.

## How to use

For use you need to install library from npm:
```bash
npm install wavy-sounds
```
Then you can use it in your project:
```js
import { parse_audio } from 'wavy-sounds';

async function downloadAudioFile(url) {
    const response = await fetch(url);
    const arrayBuffer = await response.arrayBuffer();
    return new Uint8Array(arrayBuffer);
}

async function getSoundwave(audio_url) {
    const audio = await downloadAudioFile(audio_url);
    const groupSize = 8; // number of peaks to reduce into one point
    const result = parse_audio(audio, groupSize);
    console.log(result);
}

```

Remember to parse data for parse_audio function as Uint8Array, it's important for correct analyze data.

`group_size` controls how many consecutive peaks are reduced (by max) into a single output point — a larger value produces a shorter, coarser waveform.

## Example output
```json
[
    0.1853021979331970,
    0.6672140955924988,
    0.9481893181800842,
    1.0,
    0.9284461140632629,
    0.8760479688644409,
    0.9147800803184509,
    0.7912532687187195,
    0.9998148083686829
]
```

It's an array of floats in the `[0, 1]` range — each value is a normalized RMS amplitude peak for one time window of the audio. You can use it to display a sound wave in your project.

## Supported formats

- MP3
- WAV
- WebM/Opus (e.g. audio recorded in-browser via `MediaRecorder`)

Other formats (FLAC, Vorbis/OGG, AAC/MP4, etc.) aren't currently supported — `parse_audio` returns an `UnsupportedCodec` error for them. This set was chosen deliberately to keep the compiled `.wasm` small; see [CLAUDE.md](./CLAUDE.md) for the size trade-offs if you need a format that's missing.

## Performance

Sound analyzing is quite high-consuming process. With big sound files I don't really know how time it will take to analyze it. I've tested it with small files which not take much time. I will try check and optimize it in the future.

## Browser vs. Node

This library is built for the browser: it's a WASM library (`wasm-pack build --target bundler`), and its whole point is doing the waveform-decoding work in an environment that can't run native code otherwise. If you're running in Node, a native addon (e.g. via `napi-rs`) would outperform this — no WASM-boundary copy, no sandboxing overhead — since Node already has access to the OS. Node isn't unsupported, but the browser is the intended environment.
