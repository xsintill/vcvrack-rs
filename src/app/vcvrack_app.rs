pub struct VcvRackApp {
    fullscreen: bool,
    rack_texture: Option<egui::TextureHandle>,
    blank_plate_plugin_texture: Option<egui::TextureHandle>,
    zoom_level: f32,
    placed_plugins: Vec<egui::Pos2>,
    plugin_to_delete: Option<egui::Pos2>,
    selected_plugin: Option<egui::Pos2>,
}

impl VcvRackApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load Rail SVG
        let rack_svg = std::fs::read_to_string("res/Rail.svg")
            .expect("Failed to load Rail.svg");

        // Parse Rail SVG
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_str(&rack_svg, &opt).unwrap();
        let pixmap_size = tree.size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32)
            .unwrap();
        resvg::render(&tree, usvg::Transform::default(), &mut pixmap.as_mut());

        let rack_image = egui::ColorImage::from_rgba_unmultiplied(
            [pixmap_size.width() as _, pixmap_size.height() as _],
            pixmap.data()
        );
        
        let rack_texture = cc.egui_ctx.load_texture(
            "rack",
            rack_image,
            egui::TextureOptions::default()
        );

        // Load BlankPlate SVG
        let blank_plate_svg = std::fs::read_to_string("res/BlankPlatePlugin.svg")
            .expect("Failed to load BlankPlatePlugin.svg");

        // Parse BlankPlate SVG
        let blank_plate_tree = usvg::Tree::from_str(&blank_plate_svg, &opt).unwrap();
        let blank_plate_size = blank_plate_tree.size();
        let mut blank_plate_pixmap = tiny_skia::Pixmap::new(
            blank_plate_size.width() as u32, 
            blank_plate_size.height() as u32
        ).unwrap();
        resvg::render(&blank_plate_tree, usvg::Transform::default(), &mut blank_plate_pixmap.as_mut());

        let blank_plate_image = egui::ColorImage::from_rgba_unmultiplied(
            [blank_plate_size.width() as _, blank_plate_size.height() as _],
            blank_plate_pixmap.data()
        );
        
        let blank_plate_plugin_texture = cc.egui_ctx.load_texture(
            "blank_plate_plugin",
            blank_plate_image,
            egui::TextureOptions::default()
        );

        Self {
            fullscreen: false,
            rack_texture: Some(rack_texture),
            blank_plate_plugin_texture: Some(blank_plate_plugin_texture),
            zoom_level: 1.0,
            placed_plugins: Vec::new(),
            plugin_to_delete: None,
            selected_plugin: None,
        }
    }

    pub fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        self.fullscreen = !self.fullscreen;
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
    }

    pub fn reset_zoom(&mut self) {
        self.zoom_level = 1.0;
    }

    pub fn zoom_in(&mut self) {
        if self.zoom_level >= 5.0 {
            self.zoom_level = 5.0;
        } else {
            self.zoom_level = (self.zoom_level * 1.1).min(5.0);
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom_level <= 0.1 {
            self.zoom_level = 0.1;
        } else {
            self.zoom_level = (self.zoom_level / 1.1).max(0.1);
        }
    }

    pub fn delete_plugin(&mut self, pos: egui::Pos2) {
        const TOLERANCE: f32 = 0.2;  // Smaller tolerance for more precise deletion
        if let Some(index) = self.placed_plugins.iter().position(|&p| {
            (p.x - pos.x).abs() < TOLERANCE && (p.y - pos.y).abs() < TOLERANCE
        }) {
            self.placed_plugins.remove(index);
        }
    }

    pub fn select_plugin(&mut self, pos: egui::Pos2) {
        const TOLERANCE: f32 = 0.2;
        self.selected_plugin = self.placed_plugins.iter().find(|&&p| {
            (p.x - pos.x).abs() < TOLERANCE && (p.y - pos.y).abs() < TOLERANCE
        }).copied();
    }

    pub fn deselect_plugin(&mut self) {
        self.selected_plugin = None;
    }

    fn update_menu(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    if ui.add(egui::Button::new("Exit").shortcut_text("Ctrl+Q")).clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
            
            ui.menu_button("View", |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    let rect = egui::Rect::from_min_size(
                        ui.cursor().min,
                        egui::vec2(200.0, 16.0)
                    );
                    
                    let painter = ui.painter();
                    
                    // Background bar border
                    painter.rect(
                        rect,
                        2.0,
                        egui::Color32::from_rgb(160, 180, 255),
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(140, 160, 235))
                    );
                    
                    // Progress bar
                    let progress_rect = egui::Rect::from_min_max(
                        rect.min,
                        egui::pos2(
                            rect.min.x + rect.width() * (self.zoom_level * 100.0) / 500.0,
                            rect.max.y
                        ),
                    );
                    
                    painter.rect(
                        progress_rect,
                        2.0,
                        egui::Color32::from_rgb(100, 149, 237),
                        egui::Stroke::NONE
                    );
                    
                    // Centered text
                    let text = format!("{:.0}%", self.zoom_level * 100.0);
                    painter.text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        text,
                        egui::FontId::proportional(14.0),
                        egui::Color32::WHITE,
                    );

                    ui.advance_cursor_after_rect(rect);
                });
                ui.separator();
                
                if ui.add(egui::Button::new("Zoom to 100%")).clicked() {
                    self.reset_zoom();
                }
                if ui.add(egui::Button::new("Zoom In").shortcut_text("Ctrl++")).clicked() {
                    self.zoom_in();
                }
                if ui.add(egui::Button::new("Zoom Out").shortcut_text("Ctrl+-")).clicked() {
                    self.zoom_out();
                }
                ui.separator();
                
                if ui.add(egui::Button::new("Fullscreen").shortcut_text("F11")).clicked() {
                    self.toggle_fullscreen(ctx);
                }
            });
        });
    }

    fn draw_rack(&mut self, ui: &mut egui::Ui) {
        // Handle zoom controls first
        if ui.input(|i| i.modifiers.ctrl) {
            if ui.input(|i| i.key_pressed(egui::Key::Plus)) {
                self.zoom_in();
            }
            if ui.input(|i| i.key_pressed(egui::Key::Minus)) {
                self.zoom_out();
            }
        }
        
        if ui.input(|i| i.modifiers.ctrl) {
            let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
            if scroll_delta != 0.0 {
                if scroll_delta > 0.0 {
                    self.zoom_in();
                } else {
                    self.zoom_out();
                }
            }
        }

        // Handle any pending plugin deletion
        if let Some(pos) = self.plugin_to_delete.take() {
            self.delete_plugin(pos);
            if Some(pos) == self.selected_plugin {
                self.deselect_plugin();
            }
        }

        // Check for clicks outside plugins to deselect
        let mut clicked_outside = false;
        if ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary)) {
            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                clicked_outside = !self.placed_plugins.iter().any(|&p| {
                    (p.x - pos.x).abs() < 0.2 && (p.y - pos.y).abs() < 0.2
                });
            }
        }

        if clicked_outside {
            self.deselect_plugin();
        }

        // Store clicked position outside the closure
        let mut clicked_pos = None;

        // Now handle the drawing
        if let Some(texture) = &self.rack_texture {
            let rail_width = texture.size_vec2().x * self.zoom_level;
            let rail_height = texture.size_vec2().y * self.zoom_level;
            
            let total_height = rail_height * 24.0;
            
            egui::ScrollArea::both()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .vertical_scroll_offset(0.0)
                .show(ui, |ui| {
                    ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::from_rgba_premultiplied(100, 100, 100, 180);
                    ui.visuals_mut().widgets.active.bg_fill = egui::Color32::from_rgba_premultiplied(120, 120, 120, 180);
                    ui.visuals_mut().widgets.hovered.bg_fill = egui::Color32::from_rgba_premultiplied(140, 140, 140, 180);
                    
                    ui.set_min_size(egui::vec2(rail_width * 200.0, total_height));

                    // First render all rails
                    for row in 0..24 {
                        for col in 0..200 {
                            let image = egui::widgets::Image::new(texture)
                                .fit_to_exact_size(egui::vec2(rail_width, rail_height));
                             

                            let pos = egui::pos2(col as f32 * rail_width, row as f32 * rail_height);
                            let rail_rect = egui::Rect::from_min_size(pos, egui::vec2(rail_width, rail_height));

                            // Render the rail
                            ui.put(rail_rect, image.clone());

                            // Handle plugin placement on click
                            if ui.rect_contains_pointer(rail_rect) && ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary)) {
                                if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                                    // Calculate the grid position based on the actual click position
                                    let grid_x = (pointer_pos.x / 30.4).floor() * 30.4;
                                    let plugin_pos = egui::pos2(grid_x, pos.y);
                                    
                                    // Only add if there isn't already a plugin at this position
                                    let already_placed = self.placed_plugins.iter().any(|&existing_pos| {
                                        (existing_pos.x - plugin_pos.x).abs() < 1.0 && 
                                        (existing_pos.y - plugin_pos.y).abs() < 1.0
                                    });

                                    if !already_placed {
                                        // Store the new plugin position
                                        self.placed_plugins.push(plugin_pos);
                                    }
                                }
                            }
                        }
                    }

                    // Then render all placed plugins in one pass
                    if let Some(blank_plate_plugin_texture) = &self.blank_plate_plugin_texture {
                        for &plugin_pos in &self.placed_plugins.clone() {  
                            let plugin_rect = egui::Rect::from_min_size(plugin_pos, egui::vec2(rail_width, rail_height));
                            let plugin_image = egui::widgets::Image::new(blank_plate_plugin_texture)
                                .fit_to_exact_size(egui::vec2(rail_width, rail_height));
                             

                            // Show context menu on right click
                            let response = ui.put(plugin_rect, plugin_image);

                            // Handle left click for selection
                            if response.clicked_by(egui::PointerButton::Primary) {
                                clicked_pos = Some(plugin_pos);
                            }

                            response.context_menu(|ui| {
                                if ui.button("Delete Plugin").clicked() {
                                    self.plugin_to_delete = Some(plugin_pos);
                                    ui.close_menu();
                                }
                            });

                            // Draw selection overlay if this plugin is selected
                            if Some(plugin_pos) == self.selected_plugin {
                                ui.painter().rect(
                                    plugin_rect,
                                    0.0,
                                    egui::Color32::from_rgba_premultiplied(255, 140, 0, 40),  // Transparent orange fill
                                    egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 140, 0)),  // Dark orange border
                                );
                            }
                        }
                    }
                });
        }

        // Handle selection after the closure
        if let Some(pos) = clicked_pos {
            self.select_plugin(pos);
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
                    self.update_menu(ctx, ui);
                });
            }
        } else {
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                self.update_menu(ctx, ui);
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_rack(ui);
        });
    }
}