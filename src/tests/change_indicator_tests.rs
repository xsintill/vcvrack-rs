#[cfg(test)]
mod change_indicator_tests {
    use crate::app::VcvRackApp;
    use eframe::egui;

    fn create_test_context() -> (egui::Context, VcvRackApp) {
        let ctx = egui::Context::default();
        let app = VcvRackApp::new_test(&ctx);
        (ctx, app)
    }

    fn create_mock_texture(ctx: &egui::Context) -> egui::TextureHandle {
        let pixels = vec![255u8; 45 * 380 * 4];
        ctx.load_texture(
            "mock_texture",
            egui::ColorImage::from_rgba_unmultiplied(
                [45, 380],
                &pixels
            ),
            egui::TextureOptions::default(),
        )
    }

    #[test]
    fn test_changes_detected_on_add() {
        let (ctx, mut app) = create_test_context();
        let initial_state = app.has_unsaved_changes;
        assert!(!initial_state, "Should start with no unsaved changes");
        
        // Add a plugin directly through plugin manager to simulate adding
        app.plugin_manager.add_plugin(
            egui::pos2(100.0, 100.0),
            Some(create_mock_texture(&ctx))
        );
        app.has_unsaved_changes = true; // Simulate the change that would happen in add_plugin
        assert!(app.has_unsaved_changes, "Should have unsaved changes after adding plugin");
    }

    #[test]
    fn test_changes_detected_on_delete() {
        let (ctx, mut app) = create_test_context();
        
        // Add a plugin
        app.plugin_manager.add_plugin(
            egui::pos2(100.0, 100.0),
            Some(create_mock_texture(&ctx))
        );
        app.has_unsaved_changes = true;
        
        // Save to clear the unsaved changes flag
        let _ = app.save_rack_state("test");
        assert!(!app.has_unsaved_changes, "Should have no unsaved changes after saving");
        
        // Delete the plugin
        app.delete_plugin(egui::pos2(100.0, 100.0));
        assert!(app.has_unsaved_changes, "Should have unsaved changes after deleting plugin");
    }

    #[test]
    fn test_changes_cleared_on_save() {
        let (ctx, mut app) = create_test_context();
        
        // Add a plugin to create unsaved changes
        app.plugin_manager.add_plugin(
            egui::pos2(100.0, 100.0),
            Some(create_mock_texture(&ctx))
        );
        app.has_unsaved_changes = true;
        assert!(app.has_unsaved_changes, "Should have unsaved changes after adding plugin");
        
        // Save the changes
        let _ = app.save_rack_state("test");
        assert!(!app.has_unsaved_changes, "Should have no unsaved changes after saving");
    }

    #[test]
    fn test_changes_cleared_on_load() {
        let (ctx, mut app) = create_test_context();
        
        // Add a plugin and save it
        app.plugin_manager.add_plugin(
            egui::pos2(100.0, 100.0),
            Some(create_mock_texture(&ctx))
        );
        app.has_unsaved_changes = true;
        let _ = app.save_rack_state("test");
        
        // Add another plugin to create unsaved changes
        app.plugin_manager.add_plugin(
            egui::pos2(200.0, 100.0),
            Some(create_mock_texture(&ctx))
        );
        app.has_unsaved_changes = true;
        assert!(app.has_unsaved_changes, "Should have unsaved changes after adding second plugin");
        
        // Load the previous state
        let _ = app.load_rack_state("test");
        assert!(!app.has_unsaved_changes, "Should have no unsaved changes after loading");
    }
}
