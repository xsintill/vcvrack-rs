use eframe::egui;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct Plugin {
    #[allow(dead_code)]
    pub texture: Option<egui::TextureHandle>,
    pub position: egui::Pos2,
    pub selected: bool,
    pub id: usize,
    pub is_being_dragged: bool,
    pub drag_start_position: Option<egui::Pos2>, // Track where drag started
}

impl std::fmt::Debug for Plugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugin")
            .field("position", &self.position)
            .field("selected", &self.selected)
            .field("id", &self.id)
            .finish()
    }
}

impl Serialize for Plugin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let state = PluginState {
            x: self.position.x,
            y: self.position.y,
            selected: self.selected,
            id: self.id,
        };
        state.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Plugin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let state = PluginState::deserialize(deserializer)?;
        Ok(Plugin::from_state(state, None))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PluginState {
    pub x: f32,
    pub y: f32,
    pub selected: bool,
    pub id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RackState {
    pub plugins: Vec<PluginState>,
    pub next_id: usize,
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
            is_being_dragged: false,
            drag_start_position: None,
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

    pub fn draw(&mut self, ui: &mut egui::Ui, zoom_level: f32) -> (egui::Response, Option<usize>, bool, bool) {
        let mut delete_id = None;
        let mut position_changed = false;
        let mut drag_stopped = false;
        
        if let Some(texture) = &self.texture {
            let size = texture.size_vec2() / zoom_level;
            let rect = egui::Rect::from_min_size(self.position, size);
            
            // First allocate the response for the entire plugin area with drag sensing
            let response = ui.allocate_rect(rect, egui::Sense::drag());
            
            // Handle dragging
            if response.dragged() {
                println!("Plugin {} being dragged", self.id);
                if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                    // On drag start, save the initial position
                    if !self.is_being_dragged {
                        println!("Plugin {} drag started at {:?}", self.id, self.position);
                        self.drag_start_position = Some(self.position);
                        self.is_being_dragged = true;
                    }
                    
                    // Calculate grid position
                    let grid_x = (pointer_pos.x / (30.4 * zoom_level)).floor() * (30.4 * zoom_level);
                    let grid_y = self.position.y; // Keep same row
                    let new_pos = egui::pos2(grid_x, grid_y);
                    
                    if new_pos != self.position {
                        println!("Plugin {} position changed from {:?} to {:?}", self.id, self.position, new_pos);
                        self.position = new_pos;
                        position_changed = true;
                    }
                }
            } else if response.drag_released() {
                // Only set drag_stopped if we were actually dragging
                if self.is_being_dragged {
                    println!("Plugin {} drag released", self.id);
                    self.is_being_dragged = false;
                    // Check if position changed from start
                    if let Some(start_pos) = self.drag_start_position {
                        position_changed = start_pos != self.position;
                        drag_stopped = true;
                        println!("Plugin {} drag_stopped=true, position_changed={} (start_pos={:?}, current_pos={:?})", 
                            self.id, position_changed, start_pos, self.position);
                    }
                    self.drag_start_position = None;
                }
            }
            
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
            
            (response, delete_id, position_changed, drag_stopped)
        } else {
            (ui.allocate_response(egui::Vec2::ZERO, egui::Sense::click()), None, false, false)
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    #[cfg(test)]
    pub fn is_selected_for_test(&self) -> bool {
        self.selected
    }

    #[allow(dead_code)]
    pub fn get_width(&self) -> f32 {
        15.2
    }

    pub fn to_state(&self) -> PluginState {
        PluginState {
            x: self.position.x,
            y: self.position.y,
            selected: self.selected,
            id: self.id,
        }
    }

    pub fn from_state(state: PluginState, texture: Option<egui::TextureHandle>) -> Self {
        Self {
            texture,
            position: egui::pos2(state.x, state.y),
            selected: state.selected,
            id: state.id,
            is_being_dragged: false,
            drag_start_position: None,
        }
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
        println!("Adding plugin at position: {:?}", position);
        
        // Calculate grid position
        let grid_x = (position.x / 30.4).floor() * 30.4;
        let grid_y = 0.0; // Always place at top
        let grid_pos = egui::pos2(grid_x, grid_y);
        
        println!("Looking for plugin at pos: {:?}", grid_pos);
        
        // Check if there's already a plugin at this position
        if !self.plugins.iter().any(|p| p.is_at_grid_position(grid_x, grid_y)) {
            let plugin = Plugin::new(grid_pos, texture, self.next_id);
            self.next_id += 1;
            self.plugins.push(plugin);
            println!("Added plugin at position: {:?}", grid_pos);
        }
    }

    pub fn delete_plugin(&mut self, pos: egui::Pos2, zoom_level: f32) {
        if let Some(index) = self.plugins.iter().position(|p| p.is_at_position(pos, zoom_level)) {
            self.plugins.remove(index);
        }
    }

    pub fn select_plugin(&mut self, pos: egui::Pos2, zoom_level: f32) {
        if let Some(_) = self.get_plugin_at_position(pos, zoom_level) {
            if let Some(plugin) = self.plugins.iter_mut().find(|p| p.is_at_position(pos, zoom_level)) {
                // Toggle selection if clicking on a selected plugin
                plugin.set_selected(!plugin.is_selected());
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

    pub fn draw_plugins(&mut self, ui: &mut egui::Ui, zoom_level: f32, click_consumed: bool) -> (bool, bool) {
        let mut any_position_changed = false;
        let mut any_drag_stopped = false;
        let mut plugins_to_delete = Vec::new();

        for plugin in &mut self.plugins {
            let (response, delete_id, position_changed, drag_stopped) = plugin.draw(ui, zoom_level);
            println!("Plugin draw result: position_changed={}, drag_stopped={}", position_changed, drag_stopped);
            
            if position_changed {
                any_position_changed = true;
            }
            if drag_stopped {
                any_drag_stopped = true;
            }

            if let Some(id) = delete_id {
                plugins_to_delete.push(id);
            }
        }

        // Remove any plugins marked for deletion
        self.plugins.retain(|p| !plugins_to_delete.contains(&p.id));

        println!("draw_plugins result: any_position_changed={}, any_drag_stopped={}", any_position_changed, any_drag_stopped);
        (any_position_changed, any_drag_stopped)
    }

    pub fn delete_selected_plugins(&mut self) {
        self.plugins.retain(|plugin| !plugin.selected);
    }

    pub fn save_state(&self) -> RackState {
        RackState {
            plugins: self.plugins.iter().map(|p| p.to_state()).collect(),
            next_id: self.next_id,
        }
    }

    pub fn load_state(&mut self, state: RackState, texture: Option<egui::TextureHandle>) {
        self.plugins.clear();
        for plugin_state in state.plugins {
            self.plugins.push(Plugin::from_state(plugin_state, texture.clone()));
        }
        self.next_id = state.next_id;
    }

    pub fn get_selected_plugins(&self) -> Vec<&Plugin> {
        self.plugins.iter()
            .filter(|p| p.is_selected())
            .collect()
    }
}