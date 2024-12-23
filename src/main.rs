use eframe::egui;

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
}

impl VcvRackApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            fullscreen: false,
        }
    }

    fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        self.fullscreen = !self.fullscreen;
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
    }

    fn draw_rack(&self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();
        let rack_width = available_size.x.min(800.0); // Maximum width of 800px
        let hp_width = rack_width / 20.0; // Standard Eurorack width is 20HP
        let rail_height = 20.0;
        let rack_height = 400.0; // Adjust as needed

        let painter = ui.painter();
        
        // Draw the rack background
        let rack_rect = egui::Rect::from_min_size(
            ui.cursor().min,
            egui::vec2(rack_width, rack_height)
        );
        
        // Background panel
        painter.rect_filled(
            rack_rect,
            0.0,
            egui::Color32::from_gray(40)
        );

        // Top rail
        painter.rect_filled(
            egui::Rect::from_min_size(
                rack_rect.min,
                egui::vec2(rack_width, rail_height)
            ),
            0.0,
            egui::Color32::from_gray(60)
        );

        // Bottom rail
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(rack_rect.min.x, rack_rect.max.y - rail_height),
                egui::vec2(rack_width, rail_height)
            ),
            0.0,
            egui::Color32::from_gray(60)
        );

        // Draw mounting holes
        let hole_radius = 2.0;
        let hole_margin = 5.0;
        let holes_per_hp = 2; // Two holes per HP unit
        
        for hp in 0..20 { // 20 HP wide
            let x = rack_rect.min.x + (hp as f32 * hp_width);
            
            // Top holes
            painter.circle_filled(
                egui::pos2(x + hole_margin, rack_rect.min.y + hole_margin),
                hole_radius,
                egui::Color32::DARK_GRAY
            );

            // Bottom holes
            painter.circle_filled(
                egui::pos2(x + hole_margin, rack_rect.max.y - hole_margin),
                hole_radius,
                egui::Color32::DARK_GRAY
            );
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