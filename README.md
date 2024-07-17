# Wavy Sounds

## Description
Simple library written in Rust for analyze sounds and generate wave data to display it. Now it's experimental use library and I've implemeneted only simple approach. It's experiment for analyze sounds in Rust and gain some knowledge of Web Assembly as a new big web feature. 

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
    const result = parse_audio(audio);
    console.log(result);
}

```

Remember to parse data for parse_audio function as Uint8Array, it's important for correct analyze data.

## Example output
```json
[
    0.06449686735868454,
    0,
    0.009737508371472359,
    -0.0006639050552621484,
    0.0021148957312107086,
    0.0008121852297335863,
    -0.0003689117729663849,
    -0.005668351426720619,
    0.004233232699334621
]
```

As you see, it's array of floats, which represents sound wave peaks. You can use it to display sound wave in your project.

## Performance

Sound analyzing is quite high-consuming process. With big sound files I don't really know how time it will take to analyze it. I've tested it with small files which not take much time. I will try check and optimize it in the future.
## Node

It will be good approach to use it in Node.js environment, not especially in browser. It's because of performance and memory usage. Feel free to experiment and use it in your node.js projects.
