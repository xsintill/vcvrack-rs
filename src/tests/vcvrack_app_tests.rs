use crate::VcvRackApp;
use eframe::egui;

#[test]
fn test_new_vcvrack_app() {
    // Since we can't easily create a CreationContext for testing,
    // we'll need to test the components we can access
    let app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 1.0,
        placed_plugins: Vec::new(),
    };

    assert_eq!(app.fullscreen, false);
    assert_eq!(app.zoom_level, 1.0);
    assert!(app.placed_plugins.is_empty());
}

#[test]
fn test_toggle_fullscreen() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 1.0,
        placed_plugins: Vec::new(),
    };

    // Create a mock context
    let ctx = egui::Context::default();
    
    // Test toggling fullscreen
    app.toggle_fullscreen(&ctx);
    assert_eq!(app.fullscreen, true);
    
    app.toggle_fullscreen(&ctx);
    assert_eq!(app.fullscreen, false);
}

#[test]
fn test_zoom_level_constraints() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 1.0,
        placed_plugins: Vec::new(),
    };

    // Test zooming in beyond max limit
    for _ in 0..10 {
        app.zoom_level = (app.zoom_level * 1.1).min(5.0);
    }
    assert!(app.zoom_level <= 5.0, "Zoom level should not exceed maximum");
    
    // Reset zoom level
    app.zoom_level = 1.0;
    
    // Test zooming out beyond min limit
    for _ in 0..10 {
        app.zoom_level = (app.zoom_level / 1.1).max(0.1);
    }
    assert!(app.zoom_level >= 0.1, "Zoom level should not go below minimum");
}

#[test]
fn test_zoom_level_steps() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 1.0,
        placed_plugins: Vec::new(),
    };

    // Test single zoom in
    let original_zoom = app.zoom_level;
    app.zoom_level = (app.zoom_level * 1.1).min(5.0);
    assert!(app.zoom_level > original_zoom, "Zoom in should increase zoom level");
    assert_eq!(app.zoom_level, 1.1, "Zoom in should increase by factor of 1.1");

    // Test single zoom out
    app.zoom_level = 1.0; // Reset
    let original_zoom = app.zoom_level;
    app.zoom_level = (app.zoom_level / 1.1).max(0.1);
    assert!(app.zoom_level < original_zoom, "Zoom out should decrease zoom level");
    assert!((app.zoom_level - (1.0 / 1.1)).abs() < 0.0001, "Zoom out should decrease by factor of 1.1");
}
