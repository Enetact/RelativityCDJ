use eframe::egui::{Color32, Ui};
use egui_plot::{Line, Plot, PlotPoints};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PadAction {
    Play,
    Cue,
    Sync,
    Loop,
}

pub struct CDJ {
    pub name: String,
    pub waveform_points: Vec<[f64; 2]>, // raw data
    pub pad_actions: Vec<PadAction>,
}


impl CDJ {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            waveform_points: vec![].into(), // ✅ now it's PlotPoints
            pad_actions: vec![],
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.heading(&self.name);
            let points: PlotPoints = self.waveform_points.clone().into();
            let line = Line::new(points); 
            Plot::new(format!("waveform_{}", self.name))
                .view_aspect(2.0)
                .show(ui, |plot_ui| plot_ui.line(line));
        });
    }
}