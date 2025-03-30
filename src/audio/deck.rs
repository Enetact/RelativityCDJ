use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};

use crate::audio::{beatgrid::{self, BeatGrid}, waveform};

pub struct Deck {
    pub name: String,
    pub pitch: f32,
    pub waveform: Option<Vec<[f32; 2]>>,
    pub start_time: Option<Instant>,
    pub duration: f32,
    pub cue_point: Option<f32>,
    pub loop_in: Option<f32>,
    pub loop_out: Option<f32>,
    pub hot_cues: [Option<f32>; 4],
    pub beatgrid: Option<BeatGrid>,
    pub is_master: bool,
    pub bpm_override: Option<f32>,
    pub output_buffer: Arc<Mutex<Vec<f32>>>,
    pub _stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
}

impl Deck {
    pub fn new(name: &str) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            name: name.to_string(),
            pitch: 0.0,
            waveform: None,
            start_time: None,
            duration: 0.0,
            cue_point: None,
            loop_in: None,
            loop_out: None,
            hot_cues: [None; 4],
            beatgrid: None,
            is_master: false,
            bpm_override: None,
            output_buffer: Arc::new(Mutex::new(Vec::new())),
            _stream,
            stream_handle,
        }
    }

    pub fn load_file(&mut self, path: &Path) {
        if let Some(samples) = read_audio_samples(path) {
            let duration_secs = samples.len() as f32 / 44100.0;
            let buffer_clone = Arc::clone(&self.output_buffer);

            thread::spawn(move || {
                let mut cursor = 0;
                while cursor < samples.len() {
                    let mut buf = buffer_clone.lock().unwrap();
                    for _ in 0..1024 {
                        if cursor >= samples.len() {
                            break;
                        }
                        let sample = samples[cursor] as f32 / i16::MAX as f32;
                        buf.push(sample);
                        cursor += 1;
                    }
                    drop(buf);
                    thread::sleep(Duration::from_millis(10));
                }
            });

            self.duration = duration_secs;
            self.start_time = Some(Instant::now());
        }

        self.waveform = waveform::extract_waveform(path);
        if let Some(samples) = &self.waveform {
            let flat_samples: Vec<f32> = samples.iter().map(|pair| pair[1]).collect();
            self.beatgrid = Some(beatgrid::detect_beats(&flat_samples, 44100));
        }
    }

    pub fn play(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn pause(&mut self) {
        // Placeholder for pause logic
    }

    pub fn playback_position(&mut self) -> f32 {
        let pos = if let Some(start) = self.start_time {
            (Instant::now() - start).as_secs_f32()
        } else {
            0.0
        };

        match (self.loop_in, self.loop_out) {
            (Some(start), Some(end)) if start < end => {
                if pos > end {
                    self.start_time = Some(Instant::now() - Duration::from_secs_f32(start));
                    start
                } else {
                    pos
                }
            }
            _ => pos.min(self.duration),
        }
    }

    pub fn set_cue(&mut self) {
        self.cue_point = Some(self.playback_position());
    }

    pub fn jump_to_cue(&mut self) {
        if let Some(pos) = self.cue_point {
            let snapped = self.snap_to_beat(pos);
            self.start_time = Some(Instant::now() - Duration::from_secs_f32(snapped));
        }
    }

    pub fn set_loop_in(&mut self) {
        self.loop_in = Some(self.playback_position());
    }

    pub fn set_loop_out(&mut self) {
        self.loop_out = Some(self.playback_position());
    }

    pub fn jump_to_hot_cue(&mut self, index: usize) {
        if let Some(pos) = self.hot_cues[index] {
            let snapped = self.snap_to_beat(pos);
            self.start_time = Some(Instant::now() - Duration::from_secs_f32(snapped));
        }
    }

    pub fn set_hot_cue(&mut self, index: usize) {
        self.hot_cues[index] = Some(self.playback_position());
    }

    pub fn snap_to_beat(&self, position: f32) -> f32 {
        if let Some(grid) = &self.beatgrid {
            let mut closest = grid.beats.first().copied().unwrap_or(0.0);
            for &b in &grid.beats {
                if (b - position).abs() < (closest - position).abs() {
                    closest = b;
                }
            }
            closest
        } else {
            position
        }
    }

    pub fn effective_bpm(&self) -> f32 {
        if let Some(b) = self.bpm_override {
            b
        } else if let Some(grid) = &self.beatgrid {
            grid.bpm
        } else {
            120.0
        }
    }
}

/// Reads and decodes a file into raw audio samples (i16)
pub fn read_audio_samples<P: AsRef<Path>>(path: P) -> Option<Vec<i16>> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);
    let source = Decoder::new(reader).ok()?;

    let samples: Vec<i16> = source.convert_samples().collect();
    Some(samples)
}
