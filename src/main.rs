use crate::app::GigaChessApp;

mod app;
mod traits;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Giga Chess",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(GigaChessApp::new(cc)))
        }),
    )
    .expect("Failed to run egui application.");
}
