import fs from "fs";

// Import the WASM module
import * as wasm from "./pkg/audio_waveform_bg.wasm";
import { __wbg_set_wasm, generate_waveform_from_bytes } from "./pkg/audio_waveform_bg.js";

// Initialize the WASM module
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();

async function main() {
  // Get command line arguments
  const args = process.argv.slice(2);
  if (args.length < 1) {
    console.log(
      "Usage: node example.ts <path-to-audio-file> [max-amplitude] [points]"
    );
    return;
  }

  const audioPath = args[0];
  const maxAmplitude = parseInt(args[1] || "255", 10);
  const points = parseInt(args[2] || "100", 10);

  console.log(`Reading audio file: ${audioPath}`);
  console.log(`Max amplitude: ${maxAmplitude}`);
  console.log(`Waveform points: ${points}`);

  try {
    // Read the audio file
    const audioData = fs.readFileSync(audioPath);
    console.log(`Read ${audioData.length} bytes of audio data`);

    // Generate the waveform
    console.time("Waveform generation");
    const wavehash = generate_waveform_from_bytes(
      new Uint8Array(audioData),
      maxAmplitude,
      points
    );
    console.timeEnd("Waveform generation");

    // Whole string
    console.log(`Wavehash: ${wavehash}`);

  } catch (error) {
    console.error("Error:", error);
  }
}

main();
