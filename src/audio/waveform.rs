use symphonia::core::audio::Signal;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::default::get_probe;
use std::fs::File;
use std::path::Path;

/// Extracts a waveform as Vec<[f32; 2]>, where:
/// - `[0]` = time/sample index as float
/// - `[1]` = amplitude value
pub fn extract_waveform<P: AsRef<Path>>(path: P) -> Option<Vec<[f32; 2]>> {
    let file = File::open(path).ok()?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe().format(
        &Default::default(), // Hint
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    ).ok()?;

    let mut format = probed.format;

    let track = format.default_track()?;
    let decoder_opts = DecoderOptions::default();
    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts).ok()?;

    let mut waveform = Vec::new();

    while let Ok(packet) = format.next_packet() {
        if let Ok(decoded) = decoder.decode(&packet) {
            let samples = decoded.make_equivalent::<f32>();
            let channel_samples = samples.chan(0); // Mono: 1st channel only

            for (i, sample) in channel_samples.iter().enumerate().step_by(512) {
                waveform.push([i as f32, *sample]);
            }
        }
    }

    Some(waveform)
}
