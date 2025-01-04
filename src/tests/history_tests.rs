#[cfg(test)]
mod history_tests {
    use crate::models::history::History;
    use crate::models::plugin::RackState;

    #[test]
    fn test_new_history() {
        let history = History::new();
        assert!(!history.can_undo(), "New history should not have undo states");
        assert!(!history.can_redo(), "New history should not have redo states");
    }

    #[test]
    fn test_push_state() {
        let mut history = History::new();
        let state = RackState { plugins: vec![] };
        
        history.push_state(state);
        assert!(history.can_undo(), "Should be able to undo after pushing state");
        assert!(!history.can_redo(), "Should not be able to redo after pushing state");
    }

    #[test]
    fn test_undo_redo() {
        let mut history = History::new();
        
        // Create two states
        let state1 = RackState { plugins: vec![] };
        let state2 = RackState { plugins: vec![] };
        
        // Push states
        history.push_state(state1);
        history.push_state(state2);
        
        // Test undo
        assert!(history.can_undo(), "Should be able to undo");
        let undo_state = history.undo();
        assert!(undo_state.is_some(), "Should get state from undo");
        
        // Test redo
        assert!(history.can_redo(), "Should be able to redo");
        let redo_state = history.redo();
        assert!(redo_state.is_some(), "Should get state from redo");
    }

    #[test]
    fn test_undo_limit() {
        let mut history = History::new();
        let state = RackState { plugins: vec![] };
        
        // Push one state
        history.push_state(state);
        
        // Undo once
        assert!(history.undo().is_some(), "First undo should succeed");
        
        // Try to undo again
        assert!(history.undo().is_none(), "Second undo should fail");
    }

    #[test]
    fn test_redo_cleared_on_new_state() {
        let mut history = History::new();
        
        // Push initial states
        history.push_state(RackState { plugins: vec![] });
        history.push_state(RackState { plugins: vec![] });
        
        // Undo to enable redo
        history.undo();
        assert!(history.can_redo(), "Should be able to redo");
        
        // Push new state
        history.push_state(RackState { plugins: vec![] });
        assert!(!history.can_redo(), "Should not be able to redo after pushing new state");
    }

    #[test]
    fn test_clear() {
        let mut history = History::new();
        
        // Push some states
        let state1 = RackState { plugins: vec![] };
        history.push_state(state1);
        let state2 = RackState { plugins: vec![] };
        history.push_state(state2);
        
        // Clear history
        history.clear();
        
        // Verify history is reset
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }
}
