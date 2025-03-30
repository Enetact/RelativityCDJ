use eframe::egui::Ui;
use eframe::egui::ScrollArea;

#[derive(Default)]
pub struct Library {
    pub tracks: Vec<String>,
}

impl Library {
    pub fn ui(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            for track in &self.tracks {
                ui.label(track);
            }
        });
    }
}