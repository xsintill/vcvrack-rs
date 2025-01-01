use crate::models::plugin::{PluginManager, RackState};
use eframe::egui;
use std::path::PathBuf;
use directories::ProjectDirs;
use std::fs;
use serde_json;

pub struct VcvRackApp {
    fullscreen: bool,
    rack_texture: Option<egui::TextureHandle>,
    blank_plate_plugin_texture: Option<egui::TextureHandle>,
    zoom_level: f32,
    pub plugin_manager: PluginManager,
}

#[allow(dead_code)]  // Temporarily allow dead code until we implement the UI
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
            plugin_manager: PluginManager::new(),
        }
    }

    #[cfg(test)]
    pub fn new_test(_ctx: &egui::Context) -> Self {
        Self {
            fullscreen: false,
            plugin_manager: PluginManager::new(),
            blank_plate_plugin_texture: None,
            zoom_level: 1.0,
            rack_texture: None,
        }
    }

    pub fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        self.fullscreen = !self.fullscreen;
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
    }

    pub fn reset_zoom(&mut self) {
        self.zoom_level = 1.0;
    }

    const MIN_ZOOM: f32 = 0.4;
    const MAX_ZOOM: f32 = 2.6;
    const ZOOM_STEP: f32 = 0.2;

    pub fn zoom_in(&mut self) {
        let new_zoom = self.zoom_level + Self::ZOOM_STEP;
        self.zoom_level = new_zoom.min(Self::MAX_ZOOM);
    }

    pub fn zoom_out(&mut self) {
        let new_zoom = self.zoom_level - Self::ZOOM_STEP;
        self.zoom_level = new_zoom.max(Self::MIN_ZOOM);
    }

    fn update_menu(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Save").clicked() {
                    if let Ok(()) = self.save_rack_state("default") {
                        println!("Rack state saved successfully");
                    }
                    ui.close_menu();
                }
                if ui.button("Load").clicked() {
                    if let Ok(()) = self.load_rack_state("default") {
                        println!("Rack state loaded successfully");
                    }
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
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
                            rect.min.x + rect.width() * (self.zoom_level * 100.0) / 260.0,
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

    pub fn draw_rack(&mut self, ui: &mut egui::Ui) {
        let rail_width = 304.0;
        let rail_height = 380.0;
        let total_height = rail_height * 24.0;

        if let Some(texture) = &self.rack_texture {
            egui::ScrollArea::both()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .vertical_scroll_offset(0.0)
                .show(ui, |ui| {
                    ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::from_rgba_premultiplied(100, 100, 100, 180);
                    ui.visuals_mut().widgets.active.bg_fill = egui::Color32::from_rgba_premultiplied(120, 120, 120, 180);
                    ui.visuals_mut().widgets.hovered.bg_fill = egui::Color32::from_rgba_premultiplied(140, 140, 140, 180);
                    
                    ui.set_min_size(egui::vec2(rail_width * 200.0, total_height));

                    let mut click_consumed = false;

                    // Handle delete key press
                    if ui.input(|i| i.key_pressed(egui::Key::Delete)) {
                        self.plugin_manager.delete_selected_plugins();
                    }

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
                            if ui.rect_contains_pointer(rail_rect) {
                                if ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary)) {
                                    if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                                        #[cfg(not(test))]
                                        println!("Adding plugin at position: {:?}", pointer_pos);
                                        
                                        // Calculate the grid position based on the actual click position
                                        let grid_x = (pointer_pos.x / (30.4 * self.zoom_level)).floor() * (30.4 * self.zoom_level);
                                        let plugin_pos = egui::pos2(grid_x, pos.y);
                                        
                                        let ctrl_pressed = ui.input(|i| i.modifiers.ctrl);
                                        
                                        // Check if there's already a plugin at this position
                                        if let Some(clicked_plugin) = self.plugin_manager.get_plugin_at_position(plugin_pos, self.zoom_level) {
                                            // Store the selection state before modifying plugin manager
                                            let was_selected = clicked_plugin.is_selected();
                                            
                                            // If there's already a plugin, handle selection/deselection
                                            if !ctrl_pressed {
                                                // Normal click: deselect all others and toggle this one
                                                self.plugin_manager.deselect_all();
                                                if !was_selected {
                                                    self.plugin_manager.select_plugin(plugin_pos, self.zoom_level);
                                                }
                                            } else {
                                                // Ctrl+click: toggle this plugin's selection without affecting others
                                                self.plugin_manager.select_plugin(plugin_pos, self.zoom_level);
                                            }
                                        } else if let Some(texture) = &self.blank_plate_plugin_texture {
                                            // If position is free, add a new plugin
                                            self.plugin_manager.deselect_all();
                                            self.plugin_manager.add_plugin(plugin_pos, Some(texture.clone()));
                                        }
                                        click_consumed = true;
                                    }
                                }
                            }
                        }
                    }

                    // Always draw plugins, but pass click_consumed to control click handling
                    if self.blank_plate_plugin_texture.is_some() {
                        self.plugin_manager.draw_plugins(ui, self.zoom_level, click_consumed);
                    }
                });
        }
    }

    pub fn add_plugin(&mut self, pos: egui::Pos2) {
        if let Some(texture) = &self.blank_plate_plugin_texture {
            self.plugin_manager.add_plugin(pos, Some(texture.clone()));
        }
    }

    pub fn delete_plugin(&mut self, pos: egui::Pos2) {
        self.plugin_manager.delete_plugin(pos, self.zoom_level);
    }

    pub fn get_plugins(&self) -> Vec<egui::Pos2> {
        self.plugin_manager.get_plugins()
    }

    pub fn get_zoom_level(&self) -> f32 {
        self.zoom_level
    }

    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen
    }

    #[cfg(test)]
    pub fn get_rack_texture(&self) -> Option<&egui::TextureHandle> {
        self.rack_texture.as_ref()
    }

    #[cfg(test)]
    pub fn get_blank_plate_plugin_texture(&self) -> Option<&egui::TextureHandle> {
        self.blank_plate_plugin_texture.as_ref()
    }

    pub fn get_save_directory() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "vcvrack", "vcvrack-rs") {
            let data_dir = proj_dirs.data_dir();
            if !data_dir.exists() {
                fs::create_dir_all(data_dir).ok()?;
            }
            Some(data_dir.to_path_buf())
        } else {
            None
        }
    }

    pub fn save_rack_state(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let save_dir = Self::get_save_directory().ok_or("Could not get save directory")?;
        let file_path = save_dir.join(format!("{}.json", name));
        
        let state = self.plugin_manager.save_state();
        let json = serde_json::to_string_pretty(&state)?;
        fs::write(file_path, json)?;
        
        Ok(())
    }

    pub fn load_rack_state(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let save_dir = Self::get_save_directory().ok_or("Could not get save directory")?;
        let file_path = save_dir.join(format!("{}.json", name));
        
        let json = fs::read_to_string(file_path)?;
        let state: RackState = serde_json::from_str(&json)?;
        
        self.plugin_manager.load_state(state, self.blank_plate_plugin_texture.clone());
        
        Ok(())
    }

    pub fn list_saved_states() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let save_dir = Self::get_save_directory().ok_or("Could not get save directory")?;
        let mut states = Vec::new();
        
        for entry in fs::read_dir(save_dir)? {
            if let Ok(entry) = entry {
                if let Some(name) = entry.path().file_stem() {
                    if let Some(name) = name.to_str() {
                        states.push(name.to_string());
                    }
                }
            }
        }
        
        Ok(states)
    }
}

impl eframe::App for VcvRackApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.update_menu(ctx, ui);
        });

        if self.fullscreen {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.draw_rack(ui);
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.draw_rack(ui);
            });
        }

        if ctx.input(|i| i.key_pressed(egui::Key::F11)) {
            self.toggle_fullscreen(ctx);
        }
    }
}