#[cfg(test)]
mod gui_tests {
    use eframe::egui;
    use crate::models::plugin::PluginManager;

    const TEST_ZOOM: f32 = 1.0;
    const RAIL_START_X: f32 = 100.0;
    const RAIL_Y: f32 = 100.0;
    const MOCK_RAIL_WIDTH: u32 = 304;
    const MOCK_RAIL_HEIGHT: u32 = 380;
    const GRID_UNIT: f32 = 15.2;  // Width of one grid unit (1HP)
    const MOCK_PLUGIN_WIDTH: f32 = GRID_UNIT;  // Default plugin width (1HP)
    const RAIL_HEIGHT: f32 = 380.0;

    struct MockTextures {
        rail: egui::TextureHandle,
        blank_plate: egui::TextureHandle,
    }

    impl MockTextures {
        fn new(ctx: &egui::Context) -> Self {
            // Create a mock rail texture
            let rail_pixels = vec![255u8; (MOCK_RAIL_WIDTH * MOCK_RAIL_HEIGHT * 4) as usize];
            let rail_texture = ctx.load_texture(
                "mock_rail",
                egui::ColorImage::from_rgba_unmultiplied(
                    [MOCK_RAIL_WIDTH as usize, MOCK_RAIL_HEIGHT as usize],
                    &rail_pixels,
                ),
                egui::TextureOptions::default(),
            );

            // Create a mock blank plate texture
            let plate_pixels = vec![200u8; (MOCK_PLUGIN_WIDTH as u32 * MOCK_RAIL_HEIGHT * 4) as usize];
            let blank_plate = ctx.load_texture(
                "mock_blank_plate",
                egui::ColorImage::from_rgba_unmultiplied(
                    [MOCK_PLUGIN_WIDTH as usize, MOCK_RAIL_HEIGHT as usize],
                    &plate_pixels,
                ),
                egui::TextureOptions::default(),
            );

            MockTextures {
                rail: rail_texture,
                blank_plate,
            }
        }
    }

    struct Rail {
        position: egui::Pos2,
        width: u32,
        height: u32,
        rail_texture: egui::TextureHandle,
        plugins: Vec<Plugin>,
    }

    impl Rail {
        fn new(position: egui::Pos2, width: u32, height: u32, rail_texture: egui::TextureHandle) -> Self {
            Rail {
                position,
                width,
                height,
                rail_texture,
                plugins: Vec::new(),
            }
        }

        fn add_plugin(&mut self, click_pos: egui::Pos2, plugin_texture: egui::TextureHandle) -> Option<egui::Pos2> {
            // Calculate relative position from rail start
            let relative_x = click_pos.x - self.position.x;
            
            // Determine which grid point is closest
            let grid_f = relative_x / GRID_UNIT;
            let prev_grid = grid_f.floor();
            let next_grid = prev_grid + 1.0;
            
            // Calculate distances to previous and next grid points
            let dist_to_prev = (relative_x - (prev_grid * GRID_UNIT)).abs();
            let dist_to_next = (relative_x - (next_grid * GRID_UNIT)).abs();
            
            // Choose the closest grid point
            let grid_index = if relative_x <= 0.0 {
                0
            } else if dist_to_prev <= dist_to_next {
                prev_grid as i32
            } else {
                next_grid as i32
            };
            
            let grid_x = self.position.x + (grid_index as f32 * GRID_UNIT);
            
            println!("Adding plugin: click_x={}, relative_x={}, grid_index={}, grid_x={}", 
                    click_pos.x, relative_x, grid_index, grid_x);
            
            // Check if position is already occupied
            for plugin in &self.plugins {
                if (plugin.position.x - grid_x).abs() < 0.1 {
                    println!("Position {} already occupied by plugin at {}", 
                            grid_x, plugin.position.x);
                    return None;
                }
            }

            // Add plugin at grid position
            let new_plugin = Plugin {
                position: egui::pos2(grid_x, self.position.y),
                texture: plugin_texture,
                id: self.plugins.len(),
                selected: false,
            };
            println!("Added plugin at {}", grid_x);
            self.plugins.push(new_plugin);
            Some(egui::pos2(grid_x, self.position.y))
        }

        fn get_plugin_at_position(&self, pos: egui::Pos2) -> Option<&Plugin> {
            self.plugins.iter().find(|plugin| plugin.is_at_position(pos, TEST_ZOOM))
        }

        fn get_plugin_at_position_mut(&mut self, pos: egui::Pos2) -> Option<&mut Plugin> {
            self.plugins.iter_mut().find(|plugin| plugin.is_at_position(pos, TEST_ZOOM))
        }

        fn toggle_plugin_selection(&mut self, pos: egui::Pos2) -> bool {
            if let Some(plugin) = self.get_plugin_at_position_mut(pos) {
                plugin.toggle_selection();
                true
            } else {
                false
            }
        }
    }

    struct Plugin {
        position: egui::Pos2,
        texture: egui::TextureHandle,
        id: usize,
        selected: bool,
    }

    impl Plugin {
        fn is_at_position(&self, pos: egui::Pos2, _zoom: f32) -> bool {
            // Check if the position is within the plugin's grid unit
            let half_grid = GRID_UNIT / 2.0;
            let min_x = self.position.x - half_grid;
            let max_x = self.position.x + half_grid;
            
            println!("Plugin at {:?}, checking if {:?} is between {} and {}", 
                    self.position, pos, min_x, max_x);
            
            // Use strict inequalities to avoid overlap at grid boundaries
            pos.x > min_x && pos.x < max_x && 
            pos.y >= self.position.y && pos.y < self.position.y + RAIL_HEIGHT
        }

        fn toggle_selection(&mut self) {
            self.selected = !self.selected;
        }

        fn is_selected(&self) -> bool {
            self.selected
        }
    }

    fn create_test_context() -> (egui::Context, PluginManager, MockTextures) {
        let ctx = egui::Context::default();
        let mock_textures = MockTextures::new(&ctx);
        let manager = PluginManager::new();
        (ctx, manager, mock_textures)
    }

    #[test]
    fn test_plugin_selection() {
        let (_ctx, mut manager, mock_textures) = create_test_context();
        let pos = egui::pos2(100.0, 100.0);
        
        // Add a plugin with mock texture
        manager.add_plugin(pos, Some(mock_textures.blank_plate));
        
        // Simulate Ctrl+Click to select
        manager.select_plugin(pos, TEST_ZOOM);
        
        let plugin = manager.get_plugin_at_position(pos, TEST_ZOOM).unwrap();
        assert!(plugin.is_selected(), "Plugin should be selected after Ctrl+Click");
    }

    #[test]
    fn test_plugin_deselection() {
        let (_ctx, mut manager, mock_textures) = create_test_context();
        let pos = egui::pos2(100.0, 100.0);
        
        println!("Adding first plugin at {}", pos.x);
        manager.add_plugin(pos, Some(mock_textures.blank_plate));
        manager.select_plugin(pos, TEST_ZOOM);
        
        let plugin = manager.get_plugin_at_position(pos, TEST_ZOOM).unwrap();
        assert!(plugin.is_selected(), "Plugin should be selected after initial selection");
        
        println!("Test case 0");
        println!("Click position: {}", pos.x);
        println!("Expected position: {}", pos.x);
        println!("Actual position: {}", plugin.position.x);
        
        // Click outside to deselect
        manager.select_plugin(egui::pos2(200.0, 200.0), TEST_ZOOM);
        
        let plugin = manager.get_plugin_at_position(pos, TEST_ZOOM).unwrap();
        assert!(!plugin.is_selected(), "Plugin should be deselected after clicking outside");
    }

    #[test]
    fn test_plugin_context_menu() {
        let (_ctx, mut manager, mock_textures) = create_test_context();
        let pos = egui::pos2(100.0, 100.0);
        
        manager.add_plugin(pos, Some(mock_textures.blank_plate));
        assert_eq!(manager.get_plugins().len(), 1);
    }

    #[test]
    fn test_multiple_plugins_on_rail() {
        let ctx = egui::Context::default();
        let mock_textures = MockTextures::new(&ctx);
        let mut rail = Rail::new(
            egui::pos2(RAIL_START_X, RAIL_Y),
            MOCK_RAIL_WIDTH,
            MOCK_RAIL_HEIGHT,
            mock_textures.rail.clone(),
        );

        println!("\nAdding plugins...");
        // Add 5 plugins at exact grid positions
        for i in 0..5 {
            let x = RAIL_START_X + (i as f32 * GRID_UNIT);
            println!("Adding plugin {} at x={}", i, x);
            rail.add_plugin(egui::pos2(x, RAIL_Y), mock_textures.blank_plate.clone());
        }

        // Click between plugins 2 and 3
        let click_pos = egui::pos2(RAIL_START_X + (2.5 * GRID_UNIT), RAIL_Y);
        
        println!("\nPlugin positions after adding:");
        for (i, plugin) in rail.plugins.iter().enumerate() {
            println!("Plugin {} position: {:?}", i, plugin.position);
        }
        println!("\nTesting click at position: {:?}", click_pos);

        // Verify no plugin is selected at this position
        for (i, plugin) in rail.plugins.iter().enumerate() {
            let is_at_pos = plugin.is_at_position(click_pos, TEST_ZOOM);
            println!("Plugin {} at {:?}: is_at_pos={}", i, plugin.position, is_at_pos);
            assert!(!is_at_pos, 
                   "Plugin {} at position {:?} should not be selected when clicking at {:?}", 
                   i, plugin.position, click_pos);
        }
    }

    #[test]
    fn test_plugin_grid_positioning() {
        let ctx = egui::Context::default();
        let mock_textures = MockTextures::new(&ctx);
        let mut rail = Rail::new(
            egui::pos2(RAIL_START_X, RAIL_Y),
            MOCK_RAIL_WIDTH,
            MOCK_RAIL_HEIGHT,
            mock_textures.rail.clone(),
        );

        println!("\nStarting grid positioning test...");
        // Test cases with input click positions and expected grid-snapped positions
        let test_cases = vec![
            // Exact grid positions
            (RAIL_START_X, RAIL_START_X),                    // Grid 0
            (RAIL_START_X + GRID_UNIT, RAIL_START_X + GRID_UNIT),      // Grid 1
            (RAIL_START_X + 2.0 * GRID_UNIT, RAIL_START_X + 2.0 * GRID_UNIT),  // Grid 2
            
            // Positions that should snap to nearest grid
            (RAIL_START_X + 0.3 * GRID_UNIT, RAIL_START_X),            // Snaps to Grid 0
            (RAIL_START_X + 1.7 * GRID_UNIT, RAIL_START_X + 2.0 * GRID_UNIT),  // Snaps to Grid 2
            (RAIL_START_X + 2.4 * GRID_UNIT, RAIL_START_X + 2.0 * GRID_UNIT),  // Snaps to Grid 2
        ];

        for (i, (click_x, expected_x)) in test_cases.iter().enumerate() {
            println!("\nTest case {}", i);
            println!("Click position: {}", click_x);
            println!("Expected position: {}", expected_x);
            
            let click_pos = egui::pos2(*click_x, RAIL_Y);
            if let Some(actual_pos) = rail.add_plugin(click_pos, mock_textures.blank_plate.clone()) {
                println!("Actual position: {}", actual_pos.x);
                assert!((actual_pos.x - expected_x).abs() < 0.1, 
                       "Click at {} should place plugin at {}, but was placed at {}", 
                       click_x, expected_x, actual_pos.x);
            } else {
                // If position is occupied, it should be the same as expected_x
                assert!((click_x - expected_x).abs() < GRID_UNIT / 2.0,
                       "Click at {} should snap to {}, but position was occupied", 
                       click_x, expected_x);
            }
        }
    }

    #[test]
    fn test_plugins_on_different_rails() {
        let ctx = egui::Context::default();
        let mock_textures = MockTextures::new(&ctx);

        // Create two rails, one above the other
        let mut top_rail = Rail::new(
            egui::pos2(RAIL_START_X, RAIL_Y),
            MOCK_RAIL_WIDTH,
            MOCK_RAIL_HEIGHT,
            mock_textures.rail.clone(),
        );

        let mut bottom_rail = Rail::new(
            egui::pos2(RAIL_START_X, RAIL_Y + MOCK_RAIL_HEIGHT as f32),
            MOCK_RAIL_WIDTH,
            MOCK_RAIL_HEIGHT,
            mock_textures.rail.clone(),
        );

        // Simulate user clicking on the first position of each rail
        // These positions match what we see in the app logs
        let click_positions = vec![
            egui::pos2(100.0, 100.0),     // Top rail, first position
            egui::pos2(100.0, 480.0),     // Bottom rail, first position (100.0 + 380.0)
        ];

        // Add plugins by simulating clicks
        for &click_pos in &click_positions {
            let rail = if click_pos.y < RAIL_Y + MOCK_RAIL_HEIGHT as f32 {
                &mut top_rail
            } else {
                &mut bottom_rail
            };
            rail.add_plugin(click_pos, mock_textures.blank_plate.clone());
            
            // Verify plugin was added at the clicked position
            let plugin = rail.get_plugin_at_position(click_pos).unwrap();
            
            // X position should be snapped to grid
            let expected_x = 100.0; // First grid position
            assert!((plugin.position.x - expected_x).abs() < 0.1,
                   "Plugin should be at x={}, but was at x={}", 
                   expected_x, plugin.position.x);
            
            // Y position should match the rail
            assert!((plugin.position.y - click_pos.y).abs() < 0.1,
                   "Plugin should be at y={}, but was at y={}", 
                   click_pos.y, plugin.position.y);
        }

        // Verify both plugins exist and are at different heights
        let top_plugin = top_rail.get_plugin_at_position(click_positions[0]).unwrap();
        let bottom_plugin = bottom_rail.get_plugin_at_position(click_positions[1]).unwrap();

        // Verify plugins are at the same x position but different y positions
        assert!((top_plugin.position.x - bottom_plugin.position.x).abs() < 0.1,
                "Plugins should have the same x position");
        assert!((bottom_plugin.position.y - top_plugin.position.y - MOCK_RAIL_HEIGHT as f32).abs() < 0.1,
                "Plugins should be separated by rail height");

        // Print positions for debugging
        println!("Top plugin position: {:?}", top_plugin.position);
        println!("Bottom plugin position: {:?}", bottom_plugin.position);
    }

    #[test]
    fn test_plugin_deletion() {
        let (_ctx, mut plugin_manager, mock_textures) = create_test_context();

        // Add two plugins next to each other
        let plugin1_pos = egui::pos2(RAIL_START_X, RAIL_Y);
        let plugin2_pos = egui::pos2(RAIL_START_X + GRID_UNIT, RAIL_Y);
        
        plugin_manager.add_plugin(plugin1_pos, Some(mock_textures.blank_plate.clone()));
        plugin_manager.add_plugin(plugin2_pos, Some(mock_textures.blank_plate));

        // Verify plugins were added
        assert_eq!(plugin_manager.plugin_count(), 2);

        // Delete the second plugin
        plugin_manager.delete_plugin(plugin2_pos, TEST_ZOOM);

        // Verify only the second plugin was deleted
        assert_eq!(plugin_manager.plugin_count(), 1);
        assert!(plugin_manager.get_plugin_at_position(plugin1_pos, TEST_ZOOM).is_some());
        assert!(plugin_manager.get_plugin_at_position(plugin2_pos, TEST_ZOOM).is_none());

        // Try deleting at an empty position
        let empty_pos = egui::pos2(RAIL_START_X + 100.0, RAIL_Y);
        plugin_manager.delete_plugin(empty_pos, TEST_ZOOM);

        // Verify no changes when deleting at empty position
        assert_eq!(plugin_manager.plugin_count(), 1);
        assert!(plugin_manager.get_plugin_at_position(plugin1_pos, TEST_ZOOM).is_some());
    }

    #[test]
    fn test_plugin_selection_toggle() {
        let ctx = egui::Context::default();
        let mock_textures = MockTextures::new(&ctx);
        let mut rail = Rail::new(
            egui::pos2(RAIL_START_X, RAIL_Y),
            MOCK_RAIL_WIDTH,
            MOCK_RAIL_HEIGHT,
            mock_textures.rail.clone(),
        );

        // Add a plugin
        let plugin_pos = egui::pos2(RAIL_START_X, RAIL_Y);
        rail.add_plugin(plugin_pos, mock_textures.blank_plate.clone());

        // Initially plugin should not be selected
        let plugin = rail.get_plugin_at_position(plugin_pos).unwrap();
        assert!(!plugin.is_selected(), "Plugin should not be selected initially");

        // Click on plugin to select it
        assert!(rail.toggle_plugin_selection(plugin_pos), "Toggle selection should succeed");
        let plugin = rail.get_plugin_at_position(plugin_pos).unwrap();
        assert!(plugin.is_selected(), "Plugin should be selected after first click");

        // Click again to deselect
        assert!(rail.toggle_plugin_selection(plugin_pos), "Toggle selection should succeed");
        let plugin = rail.get_plugin_at_position(plugin_pos).unwrap();
        assert!(!plugin.is_selected(), "Plugin should not be selected after second click");

        // Click away from plugin should not change selection
        let away_pos = egui::pos2(RAIL_START_X + 2.0 * GRID_UNIT, RAIL_Y);
        assert!(!rail.toggle_plugin_selection(away_pos), "Toggle selection should fail for invalid position");
        let plugin = rail.get_plugin_at_position(plugin_pos).unwrap();
        assert!(!plugin.is_selected(), "Plugin selection should not change when clicking away");
    }

    #[test]
    fn test_select_rightmost_plugin() {
        let ctx = egui::Context::default();
        let mock_textures = MockTextures::new(&ctx);
        let mut rail = Rail::new(
            egui::pos2(RAIL_START_X, RAIL_Y),
            MOCK_RAIL_WIDTH,
            MOCK_RAIL_HEIGHT,
            mock_textures.rail.clone(),
        );

        // Add two plugins next to each other
        let left_pos = egui::pos2(RAIL_START_X, RAIL_Y);
        let right_pos = egui::pos2(RAIL_START_X + GRID_UNIT, RAIL_Y);
        
        println!("Adding left plugin at {}", left_pos.x);
        rail.add_plugin(left_pos, mock_textures.blank_plate.clone());
        println!("Adding right plugin at {}", right_pos.x);
        rail.add_plugin(right_pos, mock_textures.blank_plate.clone());

        // Click on the right plugin
        let click_pos = egui::pos2(right_pos.x + (GRID_UNIT * 0.25), RAIL_Y); // Click slightly right of center
        println!("Clicking at {}", click_pos.x);

        // Toggle selection at click position
        assert!(rail.toggle_plugin_selection(click_pos), "Toggle selection should succeed");

        // Verify the right plugin is selected and left is not
        let left_plugin = rail.get_plugin_at_position(left_pos).unwrap();
        let right_plugin = rail.get_plugin_at_position(right_pos).unwrap();

        assert!(!left_plugin.is_selected(), "Left plugin should not be selected");
        assert!(right_plugin.is_selected(), "Right plugin should be selected");

        // Print plugin positions and boundaries for debugging
        println!("\nPlugin positions and boundaries:");
        println!("Left plugin: pos={}, boundaries=[{}, {}]", 
                left_pos.x, 
                left_pos.x - GRID_UNIT/2.0,
                left_pos.x + GRID_UNIT/2.0);
        println!("Right plugin: pos={}, boundaries=[{}, {}]", 
                right_pos.x,
                right_pos.x - GRID_UNIT/2.0,
                right_pos.x + GRID_UNIT/2.0);
        println!("Click position: {}", click_pos.x);
    }

    #[test]
    fn test_plugin_selection_states_after_adding() {
        let ctx = egui::Context::default();
        let mock_textures = MockTextures::new(&ctx);
        let mut rail = Rail::new(
            egui::pos2(RAIL_START_X, RAIL_Y),
            MOCK_RAIL_WIDTH,
            MOCK_RAIL_HEIGHT,
            mock_textures.rail.clone(),
        );

        // Add first plugin
        let first_pos = egui::pos2(RAIL_START_X, RAIL_Y);
        println!("Adding first plugin at {}", first_pos.x);
        rail.add_plugin(first_pos, mock_textures.blank_plate.clone());

        // Verify first plugin is not selected
        let first_plugin = rail.get_plugin_at_position(first_pos).unwrap();
        assert!(!first_plugin.is_selected(), "First plugin should not be selected after adding");

        // Add second plugin
        let second_pos = egui::pos2(RAIL_START_X + GRID_UNIT, RAIL_Y);
        println!("Adding second plugin at {}", second_pos.x);
        
        // Debug output to match production behavior
        println!("Checking for existing plugins at second position:");
        for plugin in &rail.plugins {
            let is_at_pos = plugin.is_at_position(second_pos, TEST_ZOOM);
            println!("Plugin at {:?}, is_at_pos: {}", plugin.position, is_at_pos);
        }
        
        rail.add_plugin(second_pos, mock_textures.blank_plate.clone());

        // Verify plugin states - this should fail to match production behavior
        let first_plugin = rail.get_plugin_at_position(first_pos).unwrap();
        let second_plugin = rail.get_plugin_at_position(second_pos).unwrap();
        
        println!("\nPlugin states after adding second plugin:");
        println!("First plugin at {}: selected={}", first_pos.x, first_plugin.is_selected());
        println!("Second plugin at {}: selected={}", second_pos.x, second_plugin.is_selected());
        
        // This assertion will fail because in production the first plugin gets selected
        assert!(!first_plugin.is_selected(), 
               "First plugin should not be selected after adding second plugin (but it is in production)");
        assert!(!second_plugin.is_selected(), 
               "Second plugin should not be selected when added");
    }

    #[test]
    fn test_plugin_selection_issue_when_adding() {
        let ctx = egui::Context::default();
        let mock_textures = MockTextures::new(&ctx);
        let mut rail = Rail::new(
            egui::pos2(RAIL_START_X, RAIL_Y),
            MOCK_RAIL_WIDTH,
            MOCK_RAIL_HEIGHT,
            mock_textures.rail.clone(),
        );

        // Add first plugin
        let first_pos = egui::pos2(RAIL_START_X, RAIL_Y);
        println!("\nAdding first plugin at {}", first_pos.x);
        rail.add_plugin(first_pos, mock_textures.blank_plate.clone());

        // Verify first plugin is not selected
        let first_plugin = rail.get_plugin_at_position(first_pos).unwrap();
        assert!(!first_plugin.is_selected(), "First plugin should not be selected after adding");

        // Add second plugin
        let second_pos = egui::pos2(RAIL_START_X + GRID_UNIT, RAIL_Y);
        println!("\nAdding second plugin at {}", second_pos.x);
        
        // Debug output to match production behavior
        println!("Checking for existing plugins at second position:");
        for plugin in &rail.plugins {
            let is_at_pos = plugin.is_at_position(second_pos, TEST_ZOOM);
            println!("Plugin at {:?}, is_at_pos: {}", plugin.position, is_at_pos);
        }
        
        rail.add_plugin(second_pos, mock_textures.blank_plate.clone());

        // Verify plugin states - this should fail to match production behavior
        let first_plugin = rail.get_plugin_at_position(first_pos).unwrap();
        let second_plugin = rail.get_plugin_at_position(second_pos).unwrap();
        
        println!("\nPlugin states after adding second plugin:");
        println!("First plugin at {}: selected={}", first_pos.x, first_plugin.is_selected());
        println!("Second plugin at {}: selected={}", second_pos.x, second_plugin.is_selected());
        
        // This assertion will fail because in production the first plugin gets selected
        assert!(!first_plugin.is_selected(), 
               "First plugin should not be selected after adding second plugin (but it is in production)");
        assert!(!second_plugin.is_selected(), 
               "Second plugin should not be selected when added");
    }

    #[test]
    fn test_plugin_selection_via_click() {
        let (_ctx, mut manager, mock_textures) = create_test_context();
        let pos = egui::pos2(100.0, 100.0);
        
        // Add a plugin with mock texture
        manager.add_plugin(pos, Some(mock_textures.blank_plate.clone()));
        
        // Select the plugin by clicking on it
        manager.select_plugin(pos, TEST_ZOOM);
        
        // Verify the plugin is selected
        let plugin = manager.get_plugin_at_position(pos, TEST_ZOOM).unwrap();
        assert!(plugin.is_selected(), "Plugin should be selected after clicking");
    }

    #[test]
    fn test_delete_selected_plugins() {
        let (_ctx, mut manager, mock_textures) = create_test_context();
        let pos1 = egui::pos2(100.0, 100.0);
        let pos2 = egui::pos2(130.4, 100.0);
        
        // Add two plugins
        manager.add_plugin(pos1, Some(mock_textures.blank_plate.clone()));
        manager.add_plugin(pos2, Some(mock_textures.blank_plate.clone()));
        assert_eq!(manager.get_plugins().len(), 2, "Should have 2 plugins initially");
        
        // Select first plugin
        manager.select_plugin(pos1, TEST_ZOOM);
        let plugin = manager.get_plugin_at_position(pos1, TEST_ZOOM).unwrap();
        assert!(plugin.is_selected(), "First plugin should be selected");
        
        // Delete selected plugins
        manager.delete_selected_plugins();
        
        // Verify
        assert_eq!(manager.get_plugins().len(), 1, "Should have 1 plugin after deletion");
        assert!(manager.get_plugin_at_position(pos1, TEST_ZOOM).is_none(), "First plugin should be deleted");
        assert!(manager.get_plugin_at_position(pos2, TEST_ZOOM).is_some(), "Second plugin should still exist");
    }
}
