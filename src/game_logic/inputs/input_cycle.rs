use std::collections::VecDeque;

use crate::input::translated_inputs::TranslatedInput;

#[derive(Clone, PartialEq, Debug)]
pub struct AllInputManagement {
    pub input_new_frame: VecDeque<(TranslatedInput, bool)>,
    pub input_processed: VecDeque<TranslatedInput>,
    pub input_processed_reset_timer: Vec<i32>,
    pub action_history: VecDeque<i32>,
    pub special_reset_timer: Vec<i32>,
    
    pub directional_state_input: [(TranslatedInput, bool); 4],
}

impl AllInputManagement {
    pub fn new() -> Self {
        Self {
            input_new_frame: VecDeque::new(),
            input_processed: VecDeque::new(),
            input_processed_reset_timer:  Vec::new(),
            
            action_history: VecDeque::new(),
            special_reset_timer: Vec::new(),
            
            directional_state_input: TranslatedInput::init_dir_input_state(),
        }
    }
}