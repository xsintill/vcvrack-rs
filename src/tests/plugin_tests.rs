#[cfg(test)]
mod tests {
    use crate::models::plugin::{Plugin, PluginManager};
    use eframe::egui;

    #[test]
    fn test_new_plugin() {
        let pos = egui::pos2(100.0, 100.0);
        let plugin = Plugin::new(pos, None);
        assert_eq!(plugin.position, pos);
        assert!(!plugin.selected);
        assert!(plugin.texture.is_none());
    }

    #[test]
    fn test_plugin_is_at_position() {
        let pos = egui::pos2(100.0, 100.0);
        let plugin = Plugin::new(pos, None);
        
        // Test exact position
        assert!(plugin.is_at_position(pos, 0.1));
        
        // Test position within tolerance
        assert!(plugin.is_at_position(egui::pos2(100.05, 100.05), 0.1));
        
        // Test position outside tolerance
        assert!(!plugin.is_at_position(egui::pos2(100.2, 100.2), 0.1));
    }

    #[test]
    fn test_plugin_selection() {
        let pos = egui::pos2(100.0, 100.0);
        let mut plugin = Plugin::new(pos, None);
        
        assert!(!plugin.selected);
        plugin.set_selected(true);
        assert!(plugin.selected);
        plugin.set_selected(false);
        assert!(!plugin.selected);
    }

    #[test]
    fn test_new_plugin_manager() {
        let manager = PluginManager::new();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_plugin_manager_add_plugin() {
        let mut manager = PluginManager::new();
        let pos = egui::pos2(100.0, 100.0);
        
        // Add first plugin
        manager.add_plugin(pos, None);
        assert_eq!(manager.plugin_count(), 1);
        assert_eq!(manager.get_plugins()[0], pos);
        
        // Try to add plugin at same position
        manager.add_plugin(pos, None);
        assert_eq!(manager.plugin_count(), 1, "Should not add plugin at same position");
        
        // Add plugin at different position
        manager.add_plugin(egui::pos2(200.0, 200.0), None);
        assert_eq!(manager.plugin_count(), 2);
    }

    #[test]
    fn test_plugin_manager_delete_plugin() {
        let mut manager = PluginManager::new();
        let pos1 = egui::pos2(100.0, 100.0);
        let pos2 = egui::pos2(200.0, 200.0);
        
        manager.add_plugin(pos1, None);
        manager.add_plugin(pos2, None);
        assert_eq!(manager.plugin_count(), 2);
        
        // Delete first plugin
        manager.delete_plugin(pos1);
        assert_eq!(manager.plugin_count(), 1);
        assert!(manager.get_plugin_at_position(pos1, 0.2).is_none());
        assert!(manager.get_plugin_at_position(pos2, 0.2).is_some());
        
        // Try to delete non-existent plugin
        manager.delete_plugin(egui::pos2(300.0, 300.0));
        assert_eq!(manager.plugin_count(), 1);
    }

    #[test]
    fn test_delete_plugin_with_tolerance() {
        let mut manager = PluginManager::new();
        let pos = egui::pos2(100.0, 100.0);
        manager.add_plugin(pos, None);

        // Try to delete with position slightly off but within tolerance
        manager.delete_plugin(egui::pos2(100.1, 100.1));
        assert!(manager.is_empty());
    }

    #[test]
    fn test_plugin_manager_selection() {
        let mut manager = PluginManager::new();
        let pos1 = egui::pos2(100.0, 100.0);
        let pos2 = egui::pos2(200.0, 200.0);
        
        manager.add_plugin(pos1, None);
        manager.add_plugin(pos2, None);
        
        // Select first plugin
        manager.select_plugin(pos1);
        assert!(manager.get_plugin_at_position(pos1, 0.2).unwrap().selected);
        assert!(!manager.get_plugin_at_position(pos2, 0.2).unwrap().selected);
        
        // Select second plugin
        manager.select_plugin(pos2);
        assert!(!manager.get_plugin_at_position(pos1, 0.2).unwrap().selected);
        assert!(manager.get_plugin_at_position(pos2, 0.2).unwrap().selected);
        
        // Deselect all
        manager.deselect_all();
        assert!(!manager.get_plugin_at_position(pos1, 0.2).unwrap().selected);
        assert!(!manager.get_plugin_at_position(pos2, 0.2).unwrap().selected);
    }

    #[test]
    fn test_multiple_plugins() {
        let mut manager = PluginManager::new();
        let pos1 = egui::pos2(100.0, 100.0);
        let pos2 = egui::pos2(200.0, 200.0);

        manager.add_plugin(pos1, None);
        manager.add_plugin(pos2, None);
        assert_eq!(manager.plugin_count(), 2);
        assert_eq!(manager.get_plugins()[0], pos1);
        assert_eq!(manager.get_plugins()[1], pos2);
    }

    #[test]
    fn test_get_plugin_at_position() {
        let mut manager = PluginManager::new();
        let pos = egui::pos2(100.0, 100.0);
        manager.add_plugin(pos, None);

        let plugin = manager.get_plugin_at_position(pos, 0.1);
        assert!(plugin.is_some());
        assert_eq!(plugin.unwrap().position, pos);

        let plugin = manager.get_plugin_at_position(egui::pos2(200.0, 200.0), 0.1);
        assert!(plugin.is_none());
    }
}
