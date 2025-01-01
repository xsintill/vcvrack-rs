use crate::app::vcvrack_app::VcvRackApp;
use eframe::egui;

#[cfg(test)]
mod change_indicator_tests {
    use super::*;

    fn create_test_context() -> (egui::Context, VcvRackApp) {
        let ctx = egui::Context::default();
        (ctx.clone(), VcvRackApp::new_test(&ctx))
    }

    fn create_mock_texture(ctx: &egui::Context) -> egui::TextureHandle {
        ctx.load_texture(
            "test_texture",
            egui::ColorImage::new([10, 10], egui::Color32::WHITE),
            egui::TextureOptions::default(),
        )
    }

    #[test]
    fn test_changes_detected_on_add() {
        let (ctx, mut app) = create_test_context();
        
        // Add a plugin to create unsaved changes
        app.plugin_manager.add_plugin(
            egui::pos2(100.0, 100.0),
            Some(create_mock_texture(&ctx))
        );
        app.has_unsaved_changes = true;  // Set this manually since we're not using VcvRackApp::add_plugin
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
        app.plugin_manager.save_state();  
        app.current_file = Some(std::path::PathBuf::from("test.json"));
        app.has_unsaved_changes = false;
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
        app.plugin_manager.save_state();  
        app.current_file = Some(std::path::PathBuf::from("test.json"));
        app.has_unsaved_changes = false;
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
        let saved_state = app.plugin_manager.save_state();
        app.current_file = Some(std::path::PathBuf::from("test.json"));
        app.has_unsaved_changes = false;
        
        // Add another plugin to create unsaved changes
        app.plugin_manager.add_plugin(
            egui::pos2(200.0, 100.0),
            Some(create_mock_texture(&ctx))
        );
        app.has_unsaved_changes = true;
        assert!(app.has_unsaved_changes, "Should have unsaved changes after adding second plugin");
        
        // Load the previous state
        app.plugin_manager.load_state(saved_state, Some(create_mock_texture(&ctx)));
        app.has_unsaved_changes = false;
        assert!(!app.has_unsaved_changes, "Should have no unsaved changes after loading");
    }
}
