use crate::VcvRackApp;
use eframe::egui;

#[test]
fn test_new_vcvrack_app() {
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

    let ctx = egui::Context::default();
    
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
    app.reset_zoom();
    assert_eq!(app.zoom_level, 1.0, "Zoom level should be reset to 1.0");
    
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
    app.reset_zoom(); // Reset to 1.0
    let original_zoom = app.zoom_level;
    app.zoom_level = (app.zoom_level / 1.1).max(0.1);
    assert!(app.zoom_level < original_zoom, "Zoom out should decrease zoom level");
    assert!((app.zoom_level - (1.0 / 1.1)).abs() < 0.0001, "Zoom out should decrease by factor of 1.1");
}

#[test]
fn test_reset_zoom() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 2.5, // Start with a different zoom level
        placed_plugins: Vec::new(),
    };

    // Test resetting zoom from a high value
    app.reset_zoom();
    assert_eq!(app.zoom_level, 1.0, "Zoom level should reset to 1.0 from high zoom");

    // Test resetting zoom from a low value
    app.zoom_level = 0.2;
    app.reset_zoom();
    assert_eq!(app.zoom_level, 1.0, "Zoom level should reset to 1.0 from low zoom");

    // Test resetting zoom when already at 1.0
    app.reset_zoom();
    assert_eq!(app.zoom_level, 1.0, "Zoom level should remain at 1.0 when already at default");
}

#[test]
fn test_draw_rack_with_input() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 1.0,
        placed_plugins: Vec::new(),
    };

    let ctx = egui::Context::default();
    let mut raw_input = egui::RawInput::default();
    
    // Test scroll zoom
    raw_input.events.push(egui::Event::PointerMoved(egui::pos2(0.0, 1.0)));
    raw_input.modifiers.ctrl = true;
    
    let _ = ctx.run(raw_input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            app.draw_rack(ui);
        });
    });
}
