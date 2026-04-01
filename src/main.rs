mod app;
mod voicebank;
mod io;

use eframe::egui;
use crate::app::UtaulariaApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "UTAULARIA - Lojinha de Voicebanks",
        options,
        Box::new(|cc| Ok(Box::new(UtaulariaApp::new(cc)))),
    )
}
