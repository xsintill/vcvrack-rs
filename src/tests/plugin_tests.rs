#[cfg(test)]
mod tests {
    use crate::models::plugin::{Plugin, PluginManager};
    use eframe::egui;

    const TEST_ZOOM: f32 = 1.0;
    const MOCK_PLUGIN_WIDTH: u32 = 45;  // 3HP width
    const MOCK_PLUGIN_HEIGHT: u32 = 380;  // Standard module height

    fn create_mock_texture(ctx: &egui::Context) -> egui::TextureHandle {
        // Create a mock blank plate texture (white rectangle)
        let blank_pixels = vec![255u8; MOCK_PLUGIN_WIDTH as usize * MOCK_PLUGIN_HEIGHT as usize * 4];
        ctx.load_texture(
            "mock_blank_plate",
            egui::ColorImage::from_rgba_unmultiplied(
                [MOCK_PLUGIN_WIDTH as usize, MOCK_PLUGIN_HEIGHT as usize],
                &blank_pixels,
            ),
            egui::TextureOptions::default(),
        )
    }

    fn create_test_context() -> (egui::Context, PluginManager, MockTextures) {
        let ctx = egui::Context::default();
        let texture = create_mock_texture(&ctx);
        let manager = PluginManager::new();
        let mock_textures = MockTextures {
            blank_plate: texture,
        };
        (ctx, manager, mock_textures)
    }

    struct MockTextures {
        blank_plate: egui::TextureHandle,
    }

    #[test]
    fn test_new_plugin() {
        let ctx = egui::Context::default();
        let texture = create_mock_texture(&ctx);
        let pos = egui::pos2(100.0, 100.0);
        let plugin = Plugin::new(pos, Some(texture), 0);
        assert_eq!(plugin.position, pos);
        assert!(!plugin.selected);
        assert!(plugin.texture.is_some());
        assert_eq!(plugin.id, 0);
    }

    #[test]
    fn test_plugin_is_at_position() {
        let ctx = egui::Context::default();
        let texture = create_mock_texture(&ctx);
        let pos = egui::pos2(100.0, 100.0);
        let plugin = Plugin::new(pos, Some(texture), 0);
        
        // Test exact position
        assert!(plugin.is_at_position(pos, TEST_ZOOM));
        
        // Test position within tolerance
        assert!(plugin.is_at_position(egui::pos2(100.05, 100.05), TEST_ZOOM));
        
        // Test position outside tolerance
        assert!(!plugin.is_at_position(egui::pos2(200.0, 200.0), TEST_ZOOM));
    }

    #[test]
    fn test_plugin_selection() {
        let ctx = egui::Context::default();
        let texture = create_mock_texture(&ctx);
        let pos = egui::pos2(100.0, 100.0);
        let mut plugin = Plugin::new(pos, Some(texture), 0);
        
        assert!(!plugin.selected);
        plugin.set_selected(true);
        assert!(plugin.selected);
        plugin.set_selected(false);
        assert!(!plugin.selected);
    }

    #[test]
    fn test_new_plugin_manager() {
        let manager = PluginManager::new();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_plugin_manager_add_plugin() {
        let ctx = egui::Context::default();
        let texture = create_mock_texture(&ctx);
        let mut manager = PluginManager::new();
        let pos = egui::pos2(100.0, 100.0);
        
        // Add first plugin
        manager.add_plugin(pos, Some(texture.clone()));
        assert_eq!(manager.plugin_count(), 1);
        assert_eq!(manager.get_plugins()[0], pos);
        
        // Try to add plugin at same position
        manager.add_plugin(pos, Some(texture.clone()));
        assert_eq!(manager.plugin_count(), 1, "Should not add plugin at same position");
        
        // Add plugin at different position
        manager.add_plugin(egui::pos2(200.0, 200.0), Some(texture));
        assert_eq!(manager.plugin_count(), 2);
    }

    #[test]
    fn test_plugin_manager_delete_plugin() {
        let (_ctx, mut manager, mock_textures) = create_test_context();

        // Add a plugin at a grid position
        let pos = egui::pos2(100.0, 100.0);
        manager.add_plugin(pos, Some(mock_textures.blank_plate.clone()));

        // Verify plugin exists at that position
        assert!(manager.get_plugin_at_position(pos, TEST_ZOOM).is_some());

        // Delete the plugin
        manager.delete_plugin(pos, TEST_ZOOM);

        // Verify plugin no longer exists at that position
        assert!(manager.get_plugin_at_position(pos, TEST_ZOOM).is_none());
    }

    #[test]
    fn test_delete_plugin_with_tolerance() {
        let ctx = egui::Context::default();
        let texture = create_mock_texture(&ctx);
        let mut manager = PluginManager::new();
        let pos = egui::pos2(100.0, 100.0);
        manager.add_plugin(pos, Some(texture));

        // Try to delete with position slightly off but within tolerance
        manager.delete_plugin(egui::pos2(100.1, 100.1), TEST_ZOOM);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_multiple_plugins() {
        let (_ctx, mut manager, mock_textures) = create_test_context();

        // Add plugins at specific positions
        let grid_positions = vec![
            100.0,   // Initial position
            130.4,   // Next position (+30.4)
            160.8,   // Next position (+30.4)
            191.2,   // Next position (+30.4)
        ];
        
        for grid_x in &grid_positions {
            let pos = egui::pos2(*grid_x, 100.0);
            manager.add_plugin(pos, Some(mock_textures.blank_plate.clone()));
        }

        // Get all plugin positions
        let positions = manager.get_plugins();
        
        // Verify we have the correct number of plugins
        assert_eq!(positions.len(), grid_positions.len());
        
        // Verify positions match grid positions
        for (i, &grid_x) in grid_positions.iter().enumerate() {
            assert_eq!(positions[i].x, grid_x);
        }
    }

    #[test]
    fn test_plugin_manager_selection() {
        let ctx = egui::Context::default();
        let texture = create_mock_texture(&ctx);
        let mut manager = PluginManager::new();
        let pos = egui::pos2(100.0, 100.0);
        
        manager.add_plugin(pos, Some(texture));
        manager.select_plugin(pos, TEST_ZOOM);
        
        let plugin = manager.get_plugin_at_position(pos, TEST_ZOOM).unwrap();
        assert!(plugin.is_selected());

        // Deselect by clicking outside
        manager.select_plugin(egui::pos2(200.0, 200.0), TEST_ZOOM);
        let plugin = manager.get_plugin_at_position(pos, TEST_ZOOM).unwrap();
        assert!(!plugin.is_selected());
    }

    #[test]
    fn test_get_plugin_at_position() {
        let ctx = egui::Context::default();
        let texture = create_mock_texture(&ctx);
        let mut manager = PluginManager::new();
        let pos = egui::pos2(100.0, 100.0);
        manager.add_plugin(pos, Some(texture));

        let plugin = manager.get_plugin_at_position(pos, TEST_ZOOM);
        assert!(plugin.is_some());
        assert_eq!(plugin.unwrap().position, pos);

        let plugin = manager.get_plugin_at_position(egui::pos2(200.0, 200.0), TEST_ZOOM);
        assert!(plugin.is_none());
    }

    #[test]
    fn test_save_load_state() {
        let (_ctx, mut manager, mock_textures) = create_test_context();
        let pos1 = egui::pos2(100.0, 100.0);
        let pos2 = egui::pos2(130.4, 100.0);
        
        // Add and select plugins
        manager.add_plugin(pos1, Some(mock_textures.blank_plate.clone()));
        manager.add_plugin(pos2, Some(mock_textures.blank_plate.clone()));
        manager.select_plugin(pos1, TEST_ZOOM);
        
        // Save state
        let state = manager.save_state();
        
        // Clear manager
        manager = PluginManager::new();
        assert_eq!(manager.get_plugins().len(), 0, "Manager should be empty after reset");
        
        // Load state
        manager.load_state(state, Some(mock_textures.blank_plate.clone()));
        
        // Verify plugins were restored correctly
        assert_eq!(manager.get_plugins().len(), 2, "Should have 2 plugins after loading");
        
        let plugin1 = manager.get_plugin_at_position(pos1, TEST_ZOOM).unwrap();
        let plugin2 = manager.get_plugin_at_position(pos2, TEST_ZOOM).unwrap();
        
        assert!(!plugin1.is_selected(), "First plugin should not be selected after loading");
        assert!(!plugin2.is_selected(), "Second plugin should not be selected after loading");
        assert_eq!(plugin1.position, pos1, "First plugin should be at original position");
        assert_eq!(plugin2.position, pos2, "Second plugin should be at original position");
    }
}
