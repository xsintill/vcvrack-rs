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

    // Test zooming in until max limit
    let mut prev_zoom = app.zoom_level;
    for _ in 0..50 {
        app.zoom_in();
        if app.zoom_level == prev_zoom {
            assert_eq!(app.zoom_level, 5.0, "Should stop at exactly 5.0");
            break;
        }
        prev_zoom = app.zoom_level;
    }
    assert_eq!(app.zoom_level, 5.0, "Should reach max zoom of 5.0");
    app.zoom_in();
    assert_eq!(app.zoom_level, 5.0, "Should not exceed max zoom of 5.0");

    // Reset zoom for out test
    app.reset_zoom();

    // Test zooming out until min limit
    prev_zoom = app.zoom_level;
    for _ in 0..50 {
        app.zoom_out();
        if app.zoom_level == prev_zoom {
            assert_eq!(app.zoom_level, 0.1, "Should stop at exactly 0.1");
            break;
        }
        prev_zoom = app.zoom_level;
    }
    assert_eq!(app.zoom_level, 0.1, "Should reach min zoom of 0.1");
    app.zoom_out();
    assert_eq!(app.zoom_level, 0.1, "Should not go below min zoom of 0.1");
}

#[test]
fn test_zoom_in() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 1.0,
        placed_plugins: Vec::new(),
    };

    // Test normal zoom in
    let initial_zoom = app.zoom_level;
    app.zoom_in();
    assert!(app.zoom_level > initial_zoom);
    assert_eq!(app.zoom_level, 1.1);

    // Test zoom in at max limit
    app.zoom_level = 5.0;
    app.zoom_in();
    assert_eq!(app.zoom_level, 5.0, "Should not zoom in beyond max limit");
}

#[test]
fn test_zoom_out() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 1.0,
        placed_plugins: Vec::new(),
    };

    // Test normal zoom out
    let initial_zoom = app.zoom_level;
    app.zoom_out();
    assert!(app.zoom_level < initial_zoom);
    assert!((app.zoom_level - (1.0 / 1.1)).abs() < 0.0001);

    // Test zoom out at min limit
    app.zoom_level = 0.1;
    app.zoom_out();
    assert_eq!(app.zoom_level, 0.1, "Should not zoom out beyond min limit");
}

#[test]
fn test_reset_zoom() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 2.5,
        placed_plugins: Vec::new(),
    };

    app.reset_zoom();
    assert_eq!(app.zoom_level, 1.0);
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
    let _ = ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            app.draw_rack(ui);
        });
    });
}

#[test]
fn test_update_menu() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 1.0,
        placed_plugins: Vec::new(),
    };

    let ctx = egui::Context::default();
    let _ = ctx.run(Default::default(), |ctx| {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            app.update_menu(ctx, ui);
        });
    });
}

#[test]
fn test_app_update() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 1.0,
        placed_plugins: Vec::new(),
    };

    let ctx = egui::Context::default();
    let _ = ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            app.draw_rack(ui);
        });
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            app.update_menu(ctx, ui);
        });
    });
}

#[test]
fn test_plugin_placement() {
    let mut app = VcvRackApp {
        fullscreen: false,
        rack_texture: None,
        blank_plate_plugin_texture: None,
        zoom_level: 1.0,
        placed_plugins: Vec::new(),
    };

    // Add a plugin
    app.placed_plugins.push(egui::pos2(100.0, 100.0));
    assert_eq!(app.placed_plugins.len(), 1);
    assert_eq!(app.placed_plugins[0], egui::pos2(100.0, 100.0));

    // Test multiple plugins
    app.placed_plugins.push(egui::pos2(200.0, 200.0));
    assert_eq!(app.placed_plugins.len(), 2);
    assert_eq!(app.placed_plugins[1], egui::pos2(200.0, 200.0));
}
