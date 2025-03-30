pub trait FxPlugin: Send + Sync {
    fn apply(&mut self, sample: f32) -> f32;
}

pub struct LowPassFilter {
    pub cutoff: f32,
    state: f32,
}

impl LowPassFilter {
    pub fn new(cutoff: f32) -> Self {
        Self { cutoff, state: 0.0 }
    }
}

impl FxPlugin for LowPassFilter {
    fn apply(&mut self, input: f32) -> f32 {
        let alpha = self.cutoff.clamp(0.01, 0.99);
        self.state = self.state + alpha * (input - self.state);
        self.state
    }
}

#[derive(Clone)]
pub struct EchoFx {
    pub division: f32,
    pub feedback: f32,
    pub sample_rate: u32,
    pub bpm: f32,
    buffer: Vec<f32>,
    index: usize,
}

impl EchoFx {
    pub fn new_sync(bpm: f32, division: f32, sample_rate: u32, feedback: f32) -> Self {
        let beat_duration = 60.0 / bpm * division;
        let delay_samples = (sample_rate as f32 * beat_duration).round() as usize;

        Self {
            division,
            feedback,
            sample_rate,
            bpm,
            buffer: vec![0.0; delay_samples],
            index: 0,
        }
    }

    pub fn update_bpm(&mut self, new_bpm: f32) {
        self.bpm = new_bpm;
        let beat_duration = 60.0 / self.bpm * self.division;
        let delay_samples = (self.sample_rate as f32 * beat_duration).round() as usize;
        self.buffer = vec![0.0; delay_samples];
        self.index = 0;
    }
}

impl FxPlugin for EchoFx {
    fn apply(&mut self, input: f32) -> f32 {
        let echo = self.buffer[self.index];
        let out = input + echo;
        self.buffer[self.index] = input + echo * self.feedback;
        self.index = (self.index + 1) % self.buffer.len();
        out
    }
}

pub struct ReverbFx {
    feedback: f32,
    last: f32,
}

impl ReverbFx {
    pub fn new(feedback: f32) -> Self {
        Self { feedback, last: 0.0 }
    }
}

impl FxPlugin for ReverbFx {
    fn apply(&mut self, input: f32) -> f32 {
        let out = input + self.last * self.feedback;
        self.last = out;
        out
    }
}
