use eframe::egui;

use crate::{AppState, Image, WINDOW_TITLE};

impl AppState {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        AppState {
            image: Image::None,
            preview: true,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        WINDOW_TITLE,
        native_options,
        Box::new(|cc| Ok(Box::new(AppState::new(cc)))),
    );

    Ok(())
}
