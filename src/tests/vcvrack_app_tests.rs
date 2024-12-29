#[cfg(test)]
mod tests {
    use crate::app::VcvRackApp;
    use eframe::egui;

    // Mock the CreationContext for testing
    struct MockCreationContext {
        egui_ctx: egui::Context,
    }

    impl MockCreationContext {
        fn new() -> Self {
            Self {
                egui_ctx: egui::Context::default(),
            }
        }
    }

    impl AsRef<egui::Context> for MockCreationContext {
        fn as_ref(&self) -> &egui::Context {
            &self.egui_ctx
        }
    }

    #[test]
    fn test_new_app_default_values() {
        let cc = MockCreationContext::new();
        let app = VcvRackApp::new_test(&cc.egui_ctx);
        assert!(!app.is_fullscreen());
        assert_eq!(app.get_zoom_level(), 1.0);
    }

    #[test]
    fn test_fullscreen_toggle() {
        let cc = MockCreationContext::new();
        let mut app = VcvRackApp::new_test(&cc.egui_ctx);
        let ctx = egui::Context::default();
        assert!(!app.is_fullscreen());
        app.toggle_fullscreen(&ctx);
        assert!(app.is_fullscreen());
        app.toggle_fullscreen(&ctx);
        assert!(!app.is_fullscreen());
    }

    #[test]
    fn test_zoom_in_max() {
        let cc = MockCreationContext::new();
        let mut app = VcvRackApp::new_test(&cc.egui_ctx);
        
        // Test zooming in to max
        for _ in 0..20 {
            app.zoom_in();
        }
        assert_eq!(app.get_zoom_level(), 5.0);
    }

    #[test]
    fn test_zoom_out_min() {
        let cc = MockCreationContext::new();
        let mut app = VcvRackApp::new_test(&cc.egui_ctx);
        
        // Test zooming out to min
        for _ in 0..25 {
            app.zoom_out();
        }
        assert_eq!(app.get_zoom_level(), 0.1);
    }

    #[test]
    fn test_zoom_in_single() {
        let cc = MockCreationContext::new();
        let mut app = VcvRackApp::new_test(&cc.egui_ctx);
        let initial_zoom = app.get_zoom_level();
        app.zoom_in();
        assert!(app.get_zoom_level() > initial_zoom);
        assert_eq!(app.get_zoom_level(), 1.1);
    }

    #[test]
    fn test_zoom_out_single() {
        let cc = MockCreationContext::new();
        let mut app = VcvRackApp::new_test(&cc.egui_ctx);
        let initial_zoom = app.get_zoom_level();
        app.zoom_out();
        assert!(app.get_zoom_level() < initial_zoom);
        assert!((app.get_zoom_level() - (1.0 / 1.1)).abs() < 0.0001);
    }

    #[test]
    fn test_reset_zoom() {
        let cc = MockCreationContext::new();
        let mut app = VcvRackApp::new_test(&cc.egui_ctx);
        
        // Zoom in a few times
        for _ in 0..3 {
            app.zoom_in();
        }
        
        // Reset zoom
        app.reset_zoom();
        assert_eq!(app.get_zoom_level(), 1.0);
    }

    #[test]
    fn test_plugin_manager_methods() {
        let cc = MockCreationContext::new();
        let mut app = VcvRackApp::new_test(&cc.egui_ctx);
        
        // Test add_plugin and get_plugins
        let pos1 = egui::pos2(100.0, 100.0);
        app.add_plugin(pos1);
        assert_eq!(app.get_plugins().len(), 1);
        assert_eq!(app.get_plugins()[0], pos1);

        // Test delete_plugin
        app.delete_plugin(pos1);
        assert!(app.get_plugins().is_empty());
    }

    #[test]
    fn test_plugin_selection() {
        let cc = MockCreationContext::new();
        let mut app = VcvRackApp::new_test(&cc.egui_ctx);
        
        let pos = egui::pos2(100.0, 100.0);
        app.add_plugin(pos);
        
        // Get the plugin and check selection state
        let plugin = app.plugin_manager.get_plugin_at_position(pos, 0.1).unwrap();
        assert!(!plugin.is_selected());

        // Select the plugin
        app.plugin_manager.select_plugin(pos);
        let plugin = app.plugin_manager.get_plugin_at_position(pos, 0.1).unwrap();
        assert!(plugin.is_selected());
    }

    #[test]
    fn test_plugin_manager_state() {
        let cc = MockCreationContext::new();
        let mut app = VcvRackApp::new_test(&cc.egui_ctx);
        
        assert!(app.plugin_manager.is_empty());
        assert_eq!(app.plugin_manager.plugin_count(), 0);

        let pos = egui::pos2(100.0, 100.0);
        app.add_plugin(pos);
        
        assert!(!app.plugin_manager.is_empty());
        assert_eq!(app.plugin_manager.plugin_count(), 1);
    }
}
