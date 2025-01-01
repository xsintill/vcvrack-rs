#[cfg(test)]
mod window_title_tests {
    use crate::app::VcvRackApp;
    use crate::models::plugin::RackState;
    use eframe::egui;
    use serde_json;
    use std::fs;

    #[test]
    fn test_window_title_updates() {
        let ctx = egui::Context::default();
        let mut app = VcvRackApp::new_test(&ctx);
        
        // Initially, no file should be loaded
        assert_eq!(app.current_file, None);
        
        // Ensure save directory exists
        if let Some(save_dir) = VcvRackApp::get_save_directory() {
            fs::create_dir_all(&save_dir).unwrap();
            
            // Save a rack state
            app.save_rack_state("test_rack").unwrap();
            
            // Check if current_file is set and points to the correct file
            assert!(app.current_file.is_some());
            let file_name = app.current_file.as_ref().unwrap().file_name().unwrap().to_string_lossy();
            assert_eq!(file_name, "test_rack.json");
            
            // Create another test file with valid RackState
            let other_file = save_dir.join("other_rack.json");
            let empty_state = RackState { plugins: vec![] };
            let json = serde_json::to_string_pretty(&empty_state).unwrap();
            fs::write(&other_file, json).unwrap();
            
            // Load a different rack state
            app.load_rack_state("other_rack").unwrap();
            
            // Check if current_file is updated
            let new_file_name = app.current_file.as_ref().unwrap().file_name().unwrap().to_string_lossy();
            assert_eq!(new_file_name, "other_rack.json");
            
            // Clean up test files
            fs::remove_file(save_dir.join("test_rack.json")).ok();
            fs::remove_file(save_dir.join("other_rack.json")).ok();
        }
    }
}
