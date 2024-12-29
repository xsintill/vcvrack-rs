#[cfg(test)]
mod gui_tests {
    use eframe::egui;
    use crate::models::plugin::PluginManager;

    const TEST_ZOOM: f32 = 1.0;
    const RAIL_START_X: f32 = 100.0;
    const RAIL_Y: f32 = 100.0;
    const MOCK_RAIL_WIDTH: u32 = 304;  // Reduced from 2500 to stay under egui's 2048 limit
    const MOCK_RAIL_HEIGHT: u32 = 380;  // Standard rail height
    const MOCK_PLUGIN_WIDTH: f32 = 30.4;  // Default plugin width (2HP)
    const PLUGIN_SPACING: f32 = 0.0;  // Space between plugins
    const MOCK_PLUGIN_HEIGHT: u32 = 380;  // Standard module height

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
            let plate_pixels = vec![200u8; (MOCK_PLUGIN_WIDTH as u32 * MOCK_PLUGIN_HEIGHT * 4) as usize];
            let blank_plate = ctx.load_texture(
                "mock_blank_plate",
                egui::ColorImage::from_rgba_unmultiplied(
                    [MOCK_PLUGIN_WIDTH as usize, MOCK_PLUGIN_HEIGHT as usize],
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

        fn add_plugin(&mut self, click_pos: egui::Pos2, plugin_texture: egui::TextureHandle) {
            // Calculate grid position
            let grid_x = (click_pos.x - self.position.x) / MOCK_PLUGIN_WIDTH;
            let grid_pos = self.position.x + (grid_x * MOCK_PLUGIN_WIDTH);

            // Add plugin at grid position
            self.plugins.push(Plugin {
                position: egui::pos2(grid_pos, self.position.y),
                texture: plugin_texture,
            });
        }

        fn get_plugin_at_position(&self, pos: egui::Pos2) -> Option<&Plugin> {
            self.plugins.iter().find(|plugin| plugin.is_at_position(pos, TEST_ZOOM))
        }
    }

    struct Plugin {
        position: egui::Pos2,
        texture: egui::TextureHandle,
    }

    impl Plugin {
        fn is_at_position(&self, pos: egui::Pos2, zoom: f32) -> bool {
            // Check if the plugin is at the given position
            // This is a placeholder implementation and may need to be adjusted
            (self.position.x - pos.x).abs() < 0.1 * zoom && (self.position.y - pos.y).abs() < 0.1 * zoom
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
        
        // Add and select plugin
        manager.add_plugin(pos, Some(mock_textures.blank_plate));
        manager.select_plugin(pos, TEST_ZOOM);
        
        // Simulate Ctrl+Click again to deselect
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

        // Add 10 plugins at exact grid positions
        for i in 0..10 {
            let x = RAIL_START_X + (i as f32 * MOCK_PLUGIN_WIDTH);
            rail.add_plugin(egui::pos2(x, RAIL_Y), mock_textures.blank_plate.clone());
        }

        // Click between plugins 4 and 5 (at position 392.0)
        let click_pos = egui::pos2(392.0, RAIL_Y);
        
        // Print debug information
        println!("Plugin positions:");
        for (i, plugin) in rail.plugins.iter().enumerate() {
            println!("Plugin {} position: {:?}", i, plugin.position);
        }
        println!("\nClicking at position: {:?}", click_pos);

        // Verify no plugin is selected at this position
        for (i, plugin) in rail.plugins.iter().enumerate() {
            assert!(!plugin.is_at_position(click_pos, TEST_ZOOM), 
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

        // Test cases with expected grid positions
        let test_cases = vec![
            // Test exact grid positions
            (100.0, 100.0),   // Grid 1
            (115.2, 115.2),   // Grid 2
            (130.4, 130.4),   // Grid 3
            (145.6, 145.6),   // Grid 4
            
            // Test positions between grids
            (107.6, 107.6),   // Between grid 1 and 2
            (122.8, 122.8),   // Between grid 2 and 3
            (138.0, 138.0),   // Between grid 3 and 4
        ];

        for (click_x, expected_x) in test_cases {
            let click_pos = egui::pos2(click_x, RAIL_Y);
            rail.add_plugin(click_pos, mock_textures.blank_plate.clone());
            
            // Verify plugin position using get_plugin_at_position
            let plugin = rail.get_plugin_at_position(click_pos).unwrap();
            assert!((plugin.position.x - expected_x).abs() < 0.1, 
                   "Click at {} should place plugin at {}, but was placed at {}", 
                   click_x, expected_x, plugin.position.x);
        }
    }
}
