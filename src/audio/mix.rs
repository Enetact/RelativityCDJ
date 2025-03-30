use rodio::Source;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use crate::audio::fx::FxPlugin;

pub struct FxSlot {
    pub enabled: bool,
    pub momentary: bool,
    pub fx: Box<dyn FxPlugin>,
    pub dry_wet: f32,
}

pub struct SharedDeck {
    pub buffer: Arc<Mutex<Vec<f32>>>,
    pub volume: f32,
    pub gain: f32,
    pub eq: [f32; 3],
    pub filter: f32,
    pub fx_enabled: bool,
    pub fx_slots: Vec<FxSlot>,
}

pub struct MixerEngine {
    pub deck1: SharedDeck,
    pub deck2: SharedDeck,
    pub crossfader: f32,
    pub sample_rate: u32,
    pub channels: u16,
}

fn apply_eq(sample: f32, eq: &[f32; 3]) -> f32 {
    sample * (eq[0] + eq[1] + eq[2]) / 3.0
}

fn apply_fx_chain(sample: f32, fx_slots: &mut [FxSlot]) -> f32 {
    let mut out = sample;
    for fx in fx_slots.iter_mut() {
        if fx.enabled || fx.momentary {
            let wet = fx.fx.apply(out);
            out = out * (1.0 - fx.dry_wet) + wet * fx.dry_wet;
        }
    }
    out
}

impl Iterator for MixerEngine {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut d1 = self.deck1.buffer.lock().unwrap();
        let mut d2 = self.deck2.buffer.lock().unwrap();

        let mut s1 = d1.pop().unwrap_or(0.0) * self.deck1.volume * self.deck1.gain;
        let mut s2 = d2.pop().unwrap_or(0.0) * self.deck2.volume * self.deck2.gain;

        s1 = apply_eq(s1, &self.deck1.eq);
        s2 = apply_eq(s2, &self.deck2.eq);

        s1 *= 1.0 - self.deck1.filter.abs();
        s2 *= 1.0 - self.deck2.filter.abs();

        if self.deck1.fx_enabled {
            s1 = apply_fx_chain(s1, &mut self.deck1.fx_slots);
        }

        if self.deck2.fx_enabled {
            s2 = apply_fx_chain(s2, &mut self.deck2.fx_slots);
        }

        let mix1 = 0.5 * (1.0 - self.crossfader);
        let mix2 = 0.5 * (1.0 + self.crossfader);
        let mixed = s1 * mix1 + s2 * mix2;

        Some(mixed)
    }
}

impl Source for MixerEngine {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
