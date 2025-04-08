# Audio Waveform Hash Generator

This project provides a WebAssembly module for generating audio waveforms from various audio file formats (MP3, OGG, WAV, FLAC, etc.).

## Features

- Generate waveform data from audio files as Uint8Array encoded in base64 string
- Configurable resolution and amplitude
- Should work in Node.js, Deno, and browsers (hopefully)
- Processes MP3, OGG, FLAC, WAV and other formats supported by Symphonia

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node (for testing)](https://nodejs.org/)

### Building the WebAssembly Module

```bash
# Clone the repository
git clone https://github.com/your-username/audio-waveform-hash-wasm.git
cd audio-waveform-hash-wasm

# Build the WebAssembly module
wasm-pack build --target bundler
```

This will create a `pkg` directory with the compiled WebAssembly module and JavaScript bindings.

## Usage

### In Node.js

```javascript
const fs = require("fs");
const wasm = require("./pkg/audio_waveform_hash.js");

// Read audio file
const audioData = fs.readFileSync("path/to/audio.mp3");

// Generate waveform
const waveform = wasm.generate_waveform_from_bytes(
  new Uint8Array(audioData),
  255, // max amplitude (0-255)
  100 // number of data points to generate
);

console.log(`Generated waveform: ${waveform}`);
```

### In Browsers

```html
<script type="module">
  import init, {
    generate_waveform_from_bytes,
  } from "./pkg/audio_waveform-hash-wasm.js";

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
      100 // points
    );

    // Use the waveform data (e.g., draw on canvas)
    console.log(`Generated waveform: ${waveform}`);
  }

  run();
</script>
```

## Publishing as an NPM Package

To publish this as an NPM package:

1. Build the WebAssembly module:

   ```bash
   wasm-pack build --target bundler
   ```

2. Navigate to the generated package directory:

   ```bash
   cd pkg
   ```

3. Publish to NPM:
   ```bash
   npm publish
   ```

## Example Projects

See the included examples:

- `example.js` - Node.js example
- `index.html` - Browser example

## License

[MIT License](LICENSE.md)
