#[cfg(test)]
mod tests {
    use eframe::egui;
    use crate::app::vcvrack_app::VcvRackApp;

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

    fn create_test_context() -> (egui::Context, VcvRackApp) {
        let ctx = egui::Context::default();
        let app = VcvRackApp::new_test(&ctx);
        (ctx, app)
    }

    #[test]
    fn test_new_app_default_values() {
        let (_ctx, app) = create_test_context();
        assert_eq!(app.get_zoom_level(), 1.0);
    }

    #[test]
    fn test_fullscreen_toggle() {
        let (ctx, mut app) = create_test_context();
        let initial_state = app.is_fullscreen();
        app.toggle_fullscreen(&ctx);
        assert_ne!(app.is_fullscreen(), initial_state);
    }

    #[test]
    fn test_zoom_in_single() {
        let (_ctx, mut app) = create_test_context();
        let initial_zoom = app.get_zoom_level();
        app.zoom_in();
        assert!(app.get_zoom_level() > initial_zoom);
    }

    #[test]
    fn test_zoom_out_single() {
        let (_ctx, mut app) = create_test_context();
        let initial_zoom = app.get_zoom_level();
        app.zoom_out();
        assert!(app.get_zoom_level() < initial_zoom);
    }

    #[test]
    fn test_zoom_in_max() {
        let (_ctx, mut app) = create_test_context();
        // Zoom in multiple times
        for _ in 0..10 {
            app.zoom_in();
        }
        let max_zoom = app.get_zoom_level();
        app.zoom_in();
        assert_eq!(app.get_zoom_level(), max_zoom);
    }

    #[test]
    fn test_zoom_out_min() {
        let (_ctx, mut app) = create_test_context();
        // Zoom out multiple times
        for _ in 0..10 {
            app.zoom_out();
        }
        let min_zoom = app.get_zoom_level();
        app.zoom_out();
        assert_eq!(app.get_zoom_level(), min_zoom);
    }

    #[test]
    fn test_reset_zoom() {
        let (_ctx, mut app) = create_test_context();
        app.zoom_in();
        app.zoom_in();
        app.reset_zoom();
        assert_eq!(app.get_zoom_level(), 1.0);
    }

    #[test]
    fn test_plugin_manager_methods() {
        let (ctx, mut app) = create_test_context();
        let texture = create_mock_texture(&ctx);
        assert!(app.get_plugins().is_empty());

        let pos = egui::pos2(100.0, 100.0);
        app.plugin_manager.add_plugin(pos, Some(texture));
        assert_eq!(app.get_plugins().len(), 1);
        
        let plugin = app.plugin_manager.get_plugin_at_position(pos, TEST_ZOOM);
        assert!(plugin.is_some());
    }

    #[test]
    fn test_plugin_selection() {
        let (ctx, mut app) = create_test_context();
        let texture = create_mock_texture(&ctx);
        
        let pos = egui::pos2(100.0, 100.0);
        app.plugin_manager.add_plugin(pos, Some(texture));
        app.plugin_manager.select_plugin(pos, TEST_ZOOM);
        
        // Get the plugin and check selection state
        let plugin = app.plugin_manager.get_plugin_at_position(pos, TEST_ZOOM).unwrap();
        assert!(plugin.is_selected());
    }

    #[test]
    fn test_plugin_manager_state() {
        let (ctx, mut app) = create_test_context();
        let texture = create_mock_texture(&ctx);
        
        // Add a plugin
        let pos = egui::pos2(100.0, 100.0);
        app.plugin_manager.add_plugin(pos, Some(texture));
        assert_eq!(app.get_plugins().len(), 1);
        
        // Delete the plugin
        app.plugin_manager.delete_plugin(pos, TEST_ZOOM);
        assert!(app.get_plugins().is_empty());
    }
}
