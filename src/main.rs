use eframe::egui;

mod app;
mod audio;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DJ Emulator",
        options,
        Box::new(|_cc| Box::new(app::DJEmulatorApp::default())),
    )
}