mod app;
mod models;

use app::VcvRackApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title("VCV Rack Rust"),
        ..Default::default()
    };

    eframe::run_native(
        "VCV Rack Rust",
        native_options,
        Box::new(|cc| Ok(Box::new(VcvRackApp::new(cc)))),
    )
}
