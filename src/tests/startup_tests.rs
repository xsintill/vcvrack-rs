#[cfg(test)]
mod startup_tests {
    use crate::app::VcvRackApp;
    use crate::models::plugin::RackState;
    use eframe::egui;
    use serde_json;
    use std::fs;

    #[test]
    fn test_load_default_on_startup() {
        let ctx = egui::Context::default();
        
        // Ensure save directory exists and create a default.json
        if let Some(save_dir) = VcvRackApp::get_save_directory() {
            fs::create_dir_all(&save_dir).unwrap();
            
            // Create a default.json with some test data
            let default_file = save_dir.join("default.json");
            let test_state = RackState { plugins: vec![] };
            let json = serde_json::to_string_pretty(&test_state).unwrap();
            fs::write(&default_file, json).unwrap();
            
            // Create new app instance
            let app = VcvRackApp::new_test(&ctx);
            
            // Verify the default file is loaded
            assert!(app.current_file.is_some());
            if let Some(current_file) = &app.current_file {
                assert_eq!(
                    current_file.file_name().unwrap().to_string_lossy(),
                    "default.json"
                );
            }
            
            // Clean up test file
            fs::remove_file(default_file).ok();
        }
    }
}
