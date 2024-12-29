use eframe::egui;

pub struct Plugin {
    pub position: egui::Pos2,
    pub texture: Option<egui::TextureHandle>,
    pub selected: bool,
}

impl Plugin {
    pub fn new(position: egui::Pos2, texture: Option<egui::TextureHandle>) -> Self {
        // Calculate the grid position
        let grid_unit = 15.2; // Width of one grid unit
        let relative_x = position.x - 100.0; // Adjust for rail start position
        
        // Calculate grid index
        let grid_index = if relative_x <= 0.0 {
            0
        } else {
            (relative_x / grid_unit).round() as i32
        };
        
        let grid_x = 100.0 + (grid_index as f32 * grid_unit);
        
        Self {
            position: egui::pos2(grid_x, position.y),
            texture,
            selected: false,
        }
    }

    pub fn is_at_position(&self, pos: egui::Pos2, _zoom_level: f32) -> bool {
        const DEFAULT_WIDTH: f32 = 15.2; // 1HP width
        const DEFAULT_HEIGHT: f32 = 380.0;

        // Get dimensions either from texture or use defaults
        let (width, height) = if let Some(texture) = &self.texture {
            (texture.size_vec2().x, texture.size_vec2().y)
        } else {
            (DEFAULT_WIDTH, DEFAULT_HEIGHT)
        };

        // Create a rect for hit detection
        let rect = egui::Rect::from_min_max(
            self.position,
            egui::pos2(self.position.x + width, self.position.y + height)
        );

        // Check if position is within plugin bounds
        rect.contains(pos)
    }

    pub fn draw(&self, ui: &mut egui::Ui, zoom_level: f32) -> egui::Response {
        let mut response = ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover());
        
        if let Some(texture) = &self.texture {
            // Scale size inversely with zoom level
            let size = texture.size_vec2() / zoom_level;
            let rect = egui::Rect::from_min_size(self.position, size);
            
            let mut mesh = egui::Mesh::with_texture(texture.id());
            let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
            
            mesh.add_rect_with_uv(rect, uv, if self.selected {
                egui::Color32::from_rgba_premultiplied(200, 200, 200, 255)
            } else {
                egui::Color32::WHITE
            });
            
            ui.painter().add(mesh);
            response = ui.allocate_rect(rect, egui::Sense::click());
        }
        
        response
    }

    pub fn set_selected(&mut self, selected: bool) {
        #[cfg(not(test))]
        println!("set_selected called with value: {}", selected);
        self.selected = selected;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn get_width(&self) -> f32 {
        15.2 // Always 1HP width
    }
}

pub struct PluginManager {
    plugins: Vec<Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, position: egui::Pos2, texture: Option<egui::TextureHandle>) {
        const GRID_UNIT: f32 = 15.2;
        
        // Calculate grid position for new plugin
        let relative_x = position.x - 100.0;
        let grid_index = if relative_x <= 0.0 {
            0
        } else {
            (relative_x / GRID_UNIT).round() as i32
        };

        // Check if there's already a plugin at this grid position
        for plugin in &self.plugins {
            let plugin_relative_x = plugin.position.x - 100.0;
            let plugin_grid_pos = (plugin_relative_x / GRID_UNIT).round() as i32;
            
            if plugin_grid_pos == grid_index {
                #[cfg(not(test))]
                println!("Cannot add plugin: grid position already occupied");
                return;
            }
        }

        // If we get here, it's safe to add the plugin
        self.plugins.push(Plugin::new(position, texture));
        #[cfg(not(test))]
        println!("Added plugin at position: {:?}", position);
    }

    pub fn delete_plugin(&mut self, pos: egui::Pos2, zoom_level: f32) {
        if let Some(index) = self.plugins.iter().position(|p| p.is_at_position(pos, zoom_level)) {
            self.plugins.remove(index);
        }
    }

    pub fn select_plugin(&mut self, pos: egui::Pos2, zoom_level: f32) {
        #[cfg(not(test))]
        println!("select_plugin called with pos: {:?}", pos);
        
        // First check if there's a plugin at the clicked position
        let clicked_index = self.plugins.iter().position(|p| p.is_at_position(pos, zoom_level));
        
        // Deselect all plugins if we clicked outside of any plugin
        if clicked_index.is_none() {
            for plugin in &mut self.plugins {
                plugin.set_selected(false);
            }
            return;
        }
        
        // If we found a plugin, toggle its selection and deselect others
        let index = clicked_index.unwrap();
        for (i, plugin) in self.plugins.iter_mut().enumerate() {
            if i == index {
                #[cfg(not(test))]
                println!("Found plugin to select, setting selected to true");
                plugin.set_selected(true);
            } else {
                plugin.set_selected(false);
            }
        }
    }

    pub fn deselect_all(&mut self) {
        #[cfg(not(test))]
        println!("deselect_all called");
        for plugin in &mut self.plugins {
            plugin.set_selected(false);
        }
    }

    pub fn get_plugin_at_position(&self, pos: egui::Pos2, zoom_level: f32) -> Option<&Plugin> {
        #[cfg(not(test))]
        println!("Looking for plugin at pos: {:?}", pos);
        self.plugins.iter().find(|plugin| {
            let is_at_pos = plugin.is_at_position(pos, zoom_level);
            #[cfg(not(test))]
            println!("Checking plugin at {:?}, is_at_pos: {}", plugin.position, is_at_pos);
            is_at_pos
        })
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
        let ctrl_pressed = ui.input(|i| i.modifiers.ctrl);
        #[cfg(not(test))]
        println!("draw_plugins - ctrl_pressed: {}", ctrl_pressed);

        // First pass: draw plugins and collect actions
        for plugin in &self.plugins {
            let response = plugin.draw(ui, zoom_level);

            // Handle selection
            if response.clicked() && ctrl_pressed {
                #[cfg(not(test))]
                println!("Ctrl+click detected at position: {:?}", plugin.position);
                plugin_to_select = Some(plugin.position);
            }

            // Handle deletion
            if response.clicked() && ui.input(|i| i.modifiers.shift) {
                plugin_to_delete = Some(plugin.position);
            }
        }

        // Second pass: handle actions
        if let Some(pos) = plugin_to_select {
            #[cfg(not(test))]
            println!("Attempting to select plugin at position: {:?}", pos);
            // Toggle selection directly without checking previous state
            self.deselect_all();
            self.select_plugin(pos, zoom_level);
        }

        if let Some(pos) = plugin_to_delete {
            self.delete_plugin(pos, zoom_level);
        }
    }
}