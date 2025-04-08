import fs from "fs";

// Import our wasm module
// Note: In actual usage, you'd import from your published package like:
// const wasm = require('audio-waveform-hash');
const wasm = await import("./pkg/audio_waveform_hash.js");

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
    const wavehash = wasm.generate_waveform_from_bytes(
      new Uint8Array(audioData),
      maxAmplitude,
      points
    );
    console.timeEnd("Waveform generation");

    // Output information about the waveform
    console.log(`Wavehash: ${wavehash}`);

    // Show supported formats
    const formats = wasm.get_supported_formats();
    console.log(`Supported formats: ${formats.join(", ")}`);
  } catch (error) {
    console.error("Error:", error);
  }
}

main();
