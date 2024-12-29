use eframe::egui;
use resvg::usvg::{self};

mod app;
use app::VcvRackApp;

#[cfg(test)]
mod tests;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title("VCV Rack Rust"),
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "VCV Rack Rust",
        native_options,
        Box::new(|cc| {
            let app = VcvRackApp::new(cc);
            Ok(Box::new(app))
        }),
    ) {
        eprintln!("Application error: {}", e);
    }
}
