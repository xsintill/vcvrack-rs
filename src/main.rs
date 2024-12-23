use eframe::egui;
use resvg::usvg::{self};

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

struct VcvRackApp {
    fullscreen: bool,
    rack_texture: Option<egui::TextureHandle>,
}

impl VcvRackApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load SVG from file
        let rack_svg = std::fs::read_to_string("res/Rail.svg")
            .expect("Failed to load Rail.svg");

        // Parse SVG
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_str(&rack_svg, &opt).unwrap();
        // Convert to pixels
        let pixmap_size = tree.size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32)
            .unwrap();
        resvg::render(&tree, usvg::Transform::default(), &mut pixmap.as_mut());

        // Convert to egui texture
        let image = egui::ColorImage::from_rgba_unmultiplied(
            [pixmap_size.width() as _, pixmap_size.height() as _],
            pixmap.data()
        );
        
        let texture = cc.egui_ctx.load_texture(
            "rack",
            image,
            egui::TextureOptions::default()
        );

        Self {
            fullscreen: false,
            rack_texture: Some(texture),
        }
    }

    fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        self.fullscreen = !self.fullscreen;
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
    }

    fn draw_rack(&self, ui: &mut egui::Ui) {
        if let Some(texture) = &self.rack_texture {
            let available_size = ui.available_size();
            let aspect_ratio = texture.aspect_ratio();
            let rail_width = available_size.x / 8.0;  // Use full window width
            let rail_height = rail_width / aspect_ratio;
            
            let total_height = rail_height * 4.0;  // Calculate total height needed
            
            // Create a scrollable area if needed
            egui::ScrollArea::both().show(ui, |ui| {
                // Set minimum size to ensure we use full width
                ui.set_min_size(egui::vec2(available_size.x, total_height));
                
                // Create 4 rows
                for row in 0..4 {
                    // Create 8 rails per row
                    for col in 0..8 {
                        let image = egui::widgets::Image::new(texture)
                            .fit_to_exact_size(egui::vec2(rail_width, rail_height));
                        
                        let pos = egui::pos2(col as f32 * rail_width, row as f32 * rail_height);
                        ui.put(egui::Rect::from_min_size(pos, egui::vec2(rail_width, rail_height)), image);
                    }
                }
            });
        }
    }
}

impl eframe::App for VcvRackApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::F11)) {
            self.toggle_fullscreen(ctx);
        }

        if self.fullscreen {
            let menu_area = egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(ctx.screen_rect().width(), 24.0)
            );
            
            if menu_area.contains(ctx.pointer_hover_pos().unwrap_or_default()) {
                egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Exit").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });                       
                        ui.menu_button("View", |ui| {
                            if ui.button(format!("Fullscreen\t{}", "F11")).clicked() {
                                self.toggle_fullscreen(ctx);
                            }
                        });
                    });
                });
            }
        } else {
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Exit").clicked() {
                          ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.menu_button("View", |ui| {
                        if ui.button(format!("Fullscreen\t{}", "F11")).clicked() {
                            self.toggle_fullscreen(ctx);
                        }
                    });
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_rack(ui);
        });
    }
} 