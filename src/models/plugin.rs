use eframe::egui;

pub struct Plugin {
    pub position: egui::Pos2,
    pub texture: Option<egui::TextureHandle>,
    pub selected: bool,
    pub id: usize,
}

impl Plugin {
    pub fn new(position: egui::Pos2, texture: Option<egui::TextureHandle>, id: usize) -> Self {
        let grid_unit = 15.2;
        let relative_x = position.x - 100.0;
        
        let grid_index = if relative_x <= 0.0 {
            0
        } else {
            (relative_x / grid_unit).round() as i32
        };
        
        let grid_x = 100.0 + (grid_index as f32 * grid_unit);
        
        Self {
            position: egui::pos2(grid_x, position.y),
            texture,
            selected: false,  // Explicitly set to false
            id,
        }
    }

    pub fn is_at_position(&self, pos: egui::Pos2, _zoom_level: f32) -> bool {
        const GRID_UNIT: f32 = 15.2;
        
        // Calculate grid boundaries
        let half_grid = GRID_UNIT / 2.0;
        let min_x = self.position.x - half_grid;
        let max_x = self.position.x + half_grid;
        
        // Check if position is within grid boundaries
        pos.x >= min_x && pos.x < max_x && 
        pos.y >= self.position.y && pos.y < self.position.y + GRID_UNIT
    }

    pub fn is_at_grid_position(&self, grid_x: f32, grid_y: f32) -> bool {
        const GRID_UNIT: f32 = 15.2;
        const RAIL_HEIGHT: f32 = 380.0;
        
        let plugin_relative_x = self.position.x - 100.0;
        let plugin_grid_x = (plugin_relative_x / GRID_UNIT).round() as i32;
        let plugin_grid_y = ((self.position.y - 100.0) / RAIL_HEIGHT).round() as i32;
        
        let target_grid_x = ((grid_x - 100.0) / GRID_UNIT).round() as i32;
        let target_grid_y = ((grid_y - 100.0) / RAIL_HEIGHT).round() as i32;
        
        plugin_grid_x == target_grid_x && plugin_grid_y == target_grid_y
    }

    pub fn draw(&self, ui: &mut egui::Ui, zoom_level: f32) -> (egui::Response, Option<usize>) {
        let mut delete_id = None;
        let mut response = ui.allocate_response(egui::Vec2::ZERO, egui::Sense::click());
        
        if let Some(texture) = &self.texture {
            let size = texture.size_vec2() / zoom_level;
            let rect = egui::Rect::from_min_size(self.position, size);
            
            // First allocate the response for the entire plugin area
            response = ui.allocate_rect(rect, egui::Sense::click());
            
            // Then draw the plugin texture
            let mut mesh = egui::Mesh::with_texture(texture.id());
            let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
            
            mesh.add_rect_with_uv(rect, uv, if self.selected {
                egui::Color32::from_rgba_premultiplied(200, 200, 200, 255)
            } else {
                egui::Color32::WHITE
            });
            
            ui.painter().add(mesh);

            // Handle context menu
            response.context_menu(|ui| {
                if ui.button("Delete").clicked() {
                    ui.close_menu();
                    delete_id = Some(self.id);
                }
            });
            
            #[cfg(not(test))]
            if response.clicked() {
                println!("Plugin {} clicked", self.id);
            }
        }
        
        (response, delete_id)
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    #[cfg(test)]
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    #[allow(dead_code)]
    pub fn get_width(&self) -> f32 {
        15.2
    }
}

pub struct PluginManager {
    plugins: Vec<Plugin>,
    next_id: usize,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_plugin(&mut self, position: egui::Pos2, texture: Option<egui::TextureHandle>) {
        const GRID_UNIT: f32 = 15.2;
        const RAIL_HEIGHT: f32 = 380.0;
        
        let relative_x = position.x - 100.0;
        let grid_index = if relative_x <= 0.0 {
            0
        } else {
            (relative_x / GRID_UNIT).round() as i32
        };

        let grid_x = 100.0 + (grid_index as f32 * GRID_UNIT);
        let rail_index = ((position.y - 100.0) / RAIL_HEIGHT).round() as i32;
        let grid_y = 100.0 + (rail_index as f32 * RAIL_HEIGHT);

        // Check for existing plugins using grid position
        for plugin in &self.plugins {
            if plugin.is_at_grid_position(grid_x, grid_y) {
                #[cfg(not(test))]
                println!("Cannot add plugin: grid position already occupied on this rail");
                return;
            }
        }

        let id = self.next_id;
        self.next_id += 1;

        // Create new plugin and ensure it's not selected
        let mut new_plugin = Plugin::new(position, texture, id);
        new_plugin.set_selected(false);
        self.plugins.push(new_plugin);

        #[cfg(not(test))]
        println!("Added plugin at position: {:?}", position);
    }

    pub fn delete_plugin(&mut self, pos: egui::Pos2, zoom_level: f32) {
        if let Some(index) = self.plugins.iter().position(|p| p.is_at_position(pos, zoom_level)) {
            self.plugins.remove(index);
        }
    }

    pub fn select_plugin(&mut self, pos: egui::Pos2, zoom_level: f32) {
        // First check if we clicked on any plugin
        let clicked_on_plugin = self.get_plugin_at_position(pos, zoom_level).is_some();
        
        // If we clicked on a plugin, select it
        if clicked_on_plugin {
            if let Some(plugin) = self.plugins.iter_mut().find(|p| p.is_at_position(pos, zoom_level)) {
                plugin.set_selected(true);
            }
        } else {
            // If we clicked outside any plugin, deselect all
            self.deselect_all();
        }
    }

    pub fn deselect_all(&mut self) {
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

    #[allow(dead_code)]
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub fn draw_plugins(&mut self, ui: &mut egui::Ui, zoom_level: f32, ignore_clicks: bool) {
        let mut plugins_to_delete = Vec::new();
        let mut plugin_to_toggle: Option<usize> = None;
        
        // First pass: Draw plugins and collect actions
        for plugin in self.plugins.iter_mut() {
            let (response, delete_id) = plugin.draw(ui, zoom_level);
            
            // Handle selection on click, but only if we're not ignoring clicks
            if !ignore_clicks && response.clicked() {
                #[cfg(not(test))]
                println!("Click detected on plugin {}", plugin.id);
                plugin_to_toggle = Some(plugin.id);
            }
            
            if let Some(id) = delete_id {
                plugins_to_delete.push(id);
            }
        }
        
        // Second pass: Handle selection changes
        if let Some(toggle_id) = plugin_to_toggle {
            #[cfg(not(test))]
            println!("Toggling plugin {}", toggle_id);
            // First deselect all plugins
            for plugin in self.plugins.iter_mut() {
                plugin.set_selected(false);
            }
            // Then select the clicked plugin
            if let Some(plugin) = self.plugins.iter_mut().find(|p| p.id == toggle_id) {
                plugin.set_selected(true);
            }
        }
        
        // Finally: Remove deleted plugins
        self.plugins.retain(|plugin| !plugins_to_delete.contains(&plugin.id));
    }

    pub fn delete_selected_plugins(&mut self) {
        self.plugins.retain(|plugin| !plugin.selected);
    }
}