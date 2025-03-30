use rustfft::{FftPlanner, num_complex::Complex};

#[derive(Debug, Clone)]
pub struct BeatGrid {
    pub beats: Vec<f32>,
    pub bpm: f32,
}

pub fn detect_beats(samples: &[f32], sample_rate: usize) -> BeatGrid {
    let window_size = 1024;
    let hop = 512;

    let mut energy = vec![];
    for i in (0..samples.len().saturating_sub(window_size)).step_by(hop) {
        let window = &samples[i..i + window_size];
        let sum: f32 = window.iter().map(|x| x * x).sum();
        energy.push(sum.sqrt());
    }

    let mut onset_strength = vec![0.0];
    for i in 1..energy.len() {
        onset_strength.push((energy[i] - energy[i - 1]).max(0.0));
    }

    let mut peaks = vec![];
    for i in 1..onset_strength.len() - 1 {
        if onset_strength[i] > onset_strength[i - 1] && onset_strength[i] > onset_strength[i + 1] {
            peaks.push(i);
        }
    }

    let mut intervals = vec![];
    for i in 1..peaks.len() {
        let diff = (peaks[i] - peaks[i - 1]) as f32 * hop as f32 / sample_rate as f32;
        if diff > 0.2 && diff < 2.0 {
            intervals.push(diff);
        }
    }

    let avg_interval = intervals.iter().copied().sum::<f32>() / intervals.len().max(1) as f32;
    let bpm = if avg_interval > 0.0 { 60.0 / avg_interval } else { 120.0 };

    let beats = peaks.iter().map(|i| (*i as f32 * hop as f32 / sample_rate as f32)).collect();

    BeatGrid { beats, bpm }
}
