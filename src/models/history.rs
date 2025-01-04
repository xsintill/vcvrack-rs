use crate::models::plugin::RackState;
use log::debug;

#[derive(Debug)]
pub struct History {
    states: Vec<RackState>,
    current_index: usize,
}

impl History {
    pub fn new() -> Self {
        debug!("Creating new History");
        Self {
            states: Vec::new(),
            current_index: 0,
        }
    }

    pub fn push_state(&mut self, state: RackState) {
        #[cfg(not(test))]
        println!("History::push_state - current_index: {}, states.len: {}", self.current_index, self.states.len());

        // Truncate any redo history after current index
        if self.current_index < self.states.len() {
            self.states.truncate(self.current_index + 1);
        }
        
        self.states.push(state);
        self.current_index = self.states.len() - 1; // Point to the last state
        
        #[cfg(not(test))]
        println!("History::push_state - after push, current_index: {}, states.len: {}", self.current_index, self.states.len());
    }

    pub fn undo(&mut self) -> Option<RackState> {
        #[cfg(not(test))]
        println!("History::undo - current_index: {}, states.len: {}", self.current_index, self.states.len());
        
        if self.current_index > 0 {
            self.current_index -= 1;
            let state = self.states.get(self.current_index).cloned();
            #[cfg(not(test))]
            println!("History::undo - after undo, current_index: {}, state found: {}", self.current_index, state.is_some());
            state
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<RackState> {
        #[cfg(not(test))]
        println!("History::redo - current_index: {}, states.len: {}", self.current_index, self.states.len());
        
        if self.current_index < self.states.len() - 1 {
            self.current_index += 1;
            let state = self.states.get(self.current_index).cloned();
            #[cfg(not(test))]
            println!("History::redo - after redo, current_index: {}, state found: {}", self.current_index, state.is_some());
            state
        } else {
            None
        }
    }

    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_redo(&self) -> bool {
        self.current_index < self.states.len() - 1
    }

    pub fn clear(&mut self) {
        debug!("Clearing history");
        self.states.clear();
        self.current_index = 0;
    }
}
