use base64::Engine as _;
use js_sys::Array;
use std::io::Cursor;
use wasm_bindgen::{prelude::*, JsValue};

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `web_sys`'s global `window` function to get console access
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Helper macro for logging to the browser console
macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen(start)]
pub fn init() {
    // Set up better panic messages when debugging
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn generate_waveform_from_bytes(audio_data: &[u8], max_amplitude: u8, points: u32) -> String {
    // Call our implementation function and handle errors
    match generate_waveform_internal(audio_data, max_amplitude, points) {
        Ok(waveform) => {
            // Convert the Vec<u8> to base64 string
            base64::engine::general_purpose::STANDARD.encode(&waveform)
        }
        Err(e) => {
            console_log!("Error processing audio: {}", e);
            // Return an empty string if there was an error
            String::new()
        }
    }
}

#[wasm_bindgen]
pub fn get_supported_formats() -> Array {
    let formats = vec!["mp3", "aac", "ogg", "flac", "wav"];
    formats.into_iter().map(|s| JsValue::from_str(s)).collect()
}

// Internal implementation function that returns a Result
fn generate_waveform_internal(
    audio_data: &[u8],
    max_amplitude: u8,
    points_to_generate: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Copy the input slice into an owned Vec to ensure its lifetime matches MediaSourceStream's requirements.
    let owned_audio_data = audio_data.to_vec();
    // Create a cursor over our data
    let cursor = Cursor::new(owned_audio_data);
    let source = MediaSourceStream::new(Box::new(cursor), Default::default());

    // Create a hint to help the format registry
    let hint = Hint::new();

    // Use the default options
    let meta_opts = MetadataOptions::default();
    let fmt_opts = FormatOptions::default();
    let decoder_opts = DecoderOptions::default();

    // Probe the media source
    let probed = symphonia::default::get_probe().format(&hint, source, &fmt_opts, &meta_opts)?;

    // Get the format reader
    let mut format = probed.format;

    // Get the default track
    let track = format.default_track().ok_or("No default track found")?;

    // Create a decoder for the track
    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;

    // Store amplitude values for waveform
    let mut waveform = Vec::new();

    // Determine downsampling rate
    let mut sample_count = 0;
    let mut current_max = 0.0f32;
    let mut samples_per_point = 100; // Default

    // First pass to count total samples for better downsampling
    if let Some(n_frames) = track.codec_params.n_frames {
        let channels = track.codec_params.channels.map_or(1, |c| c.count()) as u64;
        let total_samples = n_frames * channels;
        samples_per_point = (total_samples / points_to_generate as u64).max(1) as usize;
    }

    // Process frames
    while let Ok(packet) = format.next_packet() {
        // Decode the packet
        match decoder.decode(&packet) {
            Ok(decoded) => {
                // Get the audio buffer
                let spec = *decoded.spec();
                let duration = decoded.capacity() as u64;
                let mut sample_buffer = SampleBuffer::<f32>::new(duration, spec);
                sample_buffer.copy_interleaved_ref(decoded);
                let samples = sample_buffer.samples();

                // Process samples for the waveform
                for &sample in samples {
                    let amplitude = sample.abs();
                    current_max = current_max.max(amplitude);

                    sample_count += 1;
                    if sample_count >= samples_per_point {
                        // Scale to 0-max_amplitude range and add to waveform
                        let scaled_amplitude = (current_max * max_amplitude as f32) as u8;
                        waveform.push(scaled_amplitude);

                        // Reset for next chunk
                        current_max = 0.0;
                        sample_count = 0;
                    }
                }
            }
            Err(e) => {
                console_log!("Error decoding packet: {}", e);
                continue;
            }
        }
    }

    // Add the last chunk if there's data remaining
    if sample_count > 0 {
        let scaled_amplitude = (current_max * max_amplitude as f32) as u8;
        waveform.push(scaled_amplitude);
    }

    Ok(waveform)
}
