use eframe::egui::Ui;
use eframe::egui::{Slider, ComboBox};

#[derive(Default)]
pub struct Mixer {
    pub crossfader: f32,
    pub volume: [f32; 2],
    pub gain: [f32; 2],
    pub eq: [[f32; 3]; 2],
    pub filter: [f32; 2],
    pub fx_dry_wet: [[f32; 2]; 2],
    pub fx_division: [[String; 2]; 2],
}

impl Mixer {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.add(Slider::new(&mut self.crossfader, -1.0..=1.0).text("Crossfader"));

            for i in 0..2 {
                ui.group(|ui| {
                    ui.label(format!("Deck {}", i + 1));
                    ui.add(Slider::new(&mut self.volume[i], 0.0..=1.0).text("Volume"));
                    ui.add(Slider::new(&mut self.gain[i], 0.0..=2.0).text("Gain"));
                    ui.add(Slider::new(&mut self.filter[i], -1.0..=1.0).text("Filter"));
                    ui.add(Slider::new(&mut self.eq[i][2], 0.0..=2.0).text("High"));
                    ui.add(Slider::new(&mut self.eq[i][1], 0.0..=2.0).text("Mid"));
                    ui.add(Slider::new(&mut self.eq[i][0], 0.0..=2.0).text("Low"));

                    for fx_i in 0..2 {
                        ComboBox::from_label("Division")
                            .selected_text(&self.fx_division[i][fx_i])
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.fx_division[i][fx_i], "1/4".to_string(), "1/4");
                                ui.selectable_value(&mut self.fx_division[i][fx_i], "1/2".to_string(), "1/2");
                                ui.selectable_value(&mut self.fx_division[i][fx_i], "1/1".to_string(), "1/1");
                            });

                        ui.add(Slider::new(&mut self.fx_dry_wet[i][fx_i], 0.0..=1.0).text("Wet"));
                    }
                });
            }
        });
    }
}