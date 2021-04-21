use std::collections::VecDeque;

use crate::input::translated_inputs::TranslatedInput;

const FRAME_WINDOW_BETWEEN_INPUTS: i32 = 60;

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
            input_processed_reset_timer: Vec::new(),

            action_history: VecDeque::new(),
            special_reset_timer: Vec::new(),

            directional_state_input: TranslatedInput::init_dir_input_state(),
        }
    }

    pub fn update_inputs_reset_timer(&mut self) {
        for i in 0..self.input_processed_reset_timer.len() {
            self.input_processed_reset_timer[i] += 1;
            if self.input_processed_reset_timer[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                self.input_processed.pop_front();
            }
        }
        self.input_processed_reset_timer
            .retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);
    }

    pub fn update_special_inputs_reset_timer(&mut self) {
        for i in 0..self.special_reset_timer.len() {
            self.special_reset_timer[i] += 1;
            if self.special_reset_timer[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                if self.action_history.len() > 1 {
                    self.action_history.pop_front();
                }
            }
        }
        self.special_reset_timer
            .retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);
    }
}
