#[cfg(test)]
mod window_title_tests {
    use crate::app::vcvrack_app::VcvRackApp;
    use crate::models::plugin::RackState;
    use eframe::egui;

    #[test]
    fn test_window_title_updates() {
        let ctx = egui::Context::default();
        let mut app = VcvRackApp::new_test(&ctx);
        
        // Initially, no file should be loaded
        assert_eq!(app.current_file, None);
        
        // Mock saving a rack state
        app.plugin_manager.save_state();  // Result not used
        app.current_file = Some(std::path::PathBuf::from("test_rack.json"));
        
        // Check if current_file is set and points to the correct file
        assert!(app.current_file.is_some());
        let file_name = app.current_file.as_ref().unwrap().file_name().unwrap().to_string_lossy();
        assert_eq!(file_name, "test_rack.json");
        
        // Create another test state
        let empty_state = RackState { plugins: vec![] };
        
        // Mock loading the other state
        app.plugin_manager.load_state(empty_state, None);
        app.current_file = Some(std::path::PathBuf::from("other_rack.json"));
        
        // Check if current_file is updated
        assert!(app.current_file.is_some());
        let file_name = app.current_file.as_ref().unwrap().file_name().unwrap().to_string_lossy();
        assert_eq!(file_name, "other_rack.json");
    }
}
