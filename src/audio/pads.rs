use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PadAction {
    Play,
    Cue,
    Sync,
    Loop,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PadPreset {
    pub name: String,
    pub actions: Vec<PadAction>,
}

pub fn save_pad_preset(deck_name: &str, preset: &PadPreset) {
    let path = format!("{}_pad_preset.json", deck_name);
    if let Ok(json) = serde_json::to_string_pretty(preset) {
        std::fs::write(path, json).ok();
    }
}

pub fn load_pad_preset(deck_name: &str) -> Option<PadPreset> {
    let path = format!("{}_pad_preset.json", deck_name);
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}