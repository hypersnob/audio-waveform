# Audio Waveform Generator

This project provides a WebAssembly module for generating audio waveforms from various audio file formats (MP3, OGG, WAV, FLAC, etc.).

## Features

- Generate waveform data from audio files as Uint8Array
- Configurable resolution and amplitude
- Works in Node.js, Deno, and browsers
- Processes MP3, OGG, FLAC, WAV and other formats supported by Symphonia

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Building the WebAssembly Module

```bash
# Clone the repository
git clone https://github.com/hypersnob/audio-waveform.git
cd audio-waveform

# Build the WebAssembly module
wasm-pack build --target bundler
```

This will create a `pkg` directory with the compiled WebAssembly module and JavaScript bindings.

## Usage

### In Node.js

```javascript
const fs = require("fs");
const wasm = require("./pkg/audio_waveform");

// Read audio file
const audioData = fs.readFileSync("path/to/audio.mp3");

// Generate waveform
const waveform = wasm.generate_waveform_from_bytes(
  new Uint8Array(audioData),
  255, // max amplitude (0-255)
  1000 // number of data points to generate
);

console.log(`Generated waveform with ${waveform.length} data points`);
```

### In Browsers

```html
<script type="module">
  import init, { generate_waveform_from_bytes } from "./pkg/audio_waveform.js";

  async function run() {
    // Initialize the WebAssembly module
    await init();

    // Load an audio file (e.g., from fetch or file input)
    const response = await fetch("audio.mp3");
    const audioData = await response.arrayBuffer();

    // Generate waveform
    const waveform = generate_waveform_from_bytes(
      new Uint8Array(audioData),
      255, // max amplitude
      1000 // points
    );

    // Use the waveform data (e.g., draw on canvas)
    console.log(`Generated waveform with ${waveform.length} data points`);
  }

  run();
</script>
```

## Example Projects

See the included examples:

- `example.js` - Node.js example

## License

[MIT License](LICENSE.md)
