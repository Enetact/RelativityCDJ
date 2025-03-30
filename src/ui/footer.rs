use eframe::egui::Ui;

#[derive(Default)]
pub struct Footer;

impl Footer {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("DJ Emulator Footer - CPU: 3%, Memory: 120MB");
        });
    }
}