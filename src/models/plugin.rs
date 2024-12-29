use eframe::egui;

pub struct Plugin {
    pub position: egui::Pos2,
    pub texture: Option<egui::TextureHandle>,
    pub selected: bool,
}

#[allow(dead_code)]  // Temporarily allow dead code until we implement the UI
impl Plugin {
    pub fn new(position: egui::Pos2, texture: Option<egui::TextureHandle>) -> Self {
        Self {
            position,
            texture,
            selected: false,
        }
    }

    pub fn is_at_position(&self, pos: egui::Pos2, tolerance: f32) -> bool {
        (self.position.x - pos.x).abs() < tolerance && 
        (self.position.y - pos.y).abs() < tolerance
    }

    pub fn draw(&self, ui: &mut egui::Ui, zoom_level: f32) -> egui::Response {
        if let Some(texture) = &self.texture {
            let rail_width = texture.size_vec2().x * zoom_level;
            let rail_height = texture.size_vec2().y * zoom_level;
            let plugin_rect = egui::Rect::from_min_size(
                self.position, 
                egui::vec2(rail_width, rail_height)
            );
            
            let plugin_image = egui::widgets::Image::new(texture)
                .fit_to_exact_size(egui::vec2(rail_width, rail_height));

            // Draw the plugin
            let response = ui.put(plugin_rect, plugin_image);

            // Draw selection overlay if selected
            if self.selected {
                ui.painter().rect(
                    plugin_rect,
                    0.0,
                    egui::Color32::from_rgba_premultiplied(255, 140, 0, 40),
                    egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 140, 0)),
                );
            }

            response
        } else {
            ui.allocate_response(egui::vec2(0.0, 0.0), egui::Sense::click())
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

pub struct PluginManager {
    plugins: Vec<Plugin>,
    plugin_to_delete: Option<usize>,
}

#[allow(dead_code)]  // Temporarily allow dead code until we implement the UI
impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            plugin_to_delete: None,
        }
    }

    pub fn add_plugin(&mut self, position: egui::Pos2, texture: Option<egui::TextureHandle>) {
        // Check if there's already a plugin at this position
        let already_placed = self.plugins.iter().any(|plugin| plugin.is_at_position(position, 1.0));

        if !already_placed {
            self.plugins.push(Plugin::new(position, texture));
        }
    }

    pub fn delete_plugin(&mut self, pos: egui::Pos2) {
        const TOLERANCE: f32 = 0.2;
        if let Some(index) = self.plugins.iter().position(|plugin| plugin.is_at_position(pos, TOLERANCE)) {
            self.plugins.remove(index);
        }
    }

    pub fn select_plugin(&mut self, pos: egui::Pos2) {
        const TOLERANCE: f32 = 0.2;
        // Deselect all plugins first
        for plugin in &mut self.plugins {
            plugin.set_selected(false);
        }
        // Select the clicked plugin
        if let Some(plugin) = self.plugins.iter_mut().find(|plugin| plugin.is_at_position(pos, TOLERANCE)) {
            plugin.set_selected(true);
        }
    }

    pub fn deselect_all(&mut self) {
        for plugin in &mut self.plugins {
            plugin.set_selected(false);
        }
    }

    pub fn get_plugin_at_position(&self, pos: egui::Pos2, tolerance: f32) -> Option<&Plugin> {
        self.plugins.iter().find(|plugin| plugin.is_at_position(pos, tolerance))
    }

    pub fn get_plugins(&self) -> Vec<egui::Pos2> {
        self.plugins.iter().map(|p| p.position).collect()
    }

    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub fn draw_plugins(&mut self, ui: &mut egui::Ui, zoom_level: f32) {
        let mut plugin_to_delete = None;
        let mut plugin_to_select = None;

        // First pass: draw plugins and collect actions
        for plugin in &self.plugins {
            let response = plugin.draw(ui, zoom_level);

            // Handle selection
            if response.clicked() {
                plugin_to_select = Some(plugin.position);
            }

            // Handle context menu deletion
            response.context_menu(|ui| {
                if ui.button("Delete Plugin").clicked() {
                    plugin_to_delete = Some(plugin.position);
                    ui.close_menu();
                }
            });
        }

        // Second pass: handle actions
        if let Some(pos) = plugin_to_select {
            self.select_plugin(pos);
        }
        if let Some(pos) = plugin_to_delete {
            self.delete_plugin(pos);
        }
    }
}