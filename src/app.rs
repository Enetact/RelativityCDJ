use eframe::egui::{Context, CentralPanel, TopBottomPanel};

use crate::ui::{cdj::CDJ, footer::Footer, library::Library, mixer::Mixer};

pub struct DJEmulatorApp {
    deck1: CDJ,
    deck2: CDJ,
    mixer: Mixer,
    footer: Footer,
    library: Library,
}

impl Default for DJEmulatorApp {
    fn default() -> Self {
        let deck1 = CDJ::new("Deck 1");
        let deck2 = CDJ::new("Deck 2");

        Self {
            deck1,
            deck2,
            mixer: Mixer::default(),
            footer: Footer::default(),
            library: Library::default(),
        }
    }
}

impl eframe::App for DJEmulatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("library_panel").show(ctx, |ui| {
            self.library.ui(ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            self.deck1.ui(ui);
            self.deck2.ui(ui);
            self.mixer.ui(ui);
        });

        TopBottomPanel::bottom("footer_panel").show(ctx, |ui| {
            self.footer.ui(ui);
        });
    }
}