use std::collections::VecDeque;

use crate::input::translated_inputs::TranslatedInput;

const FRAME_WINDOW_BETWEEN_INPUTS: i32 = 10;

#[derive(Clone, PartialEq, Debug)]
pub struct AllInputManagement {
    pub input_new_frame: i32,
    pub action_history: VecDeque<i32>,
    pub input_reset_timer: Vec<i32>,

    pub input_buffer: VecDeque<(TranslatedInput, bool)>
}

impl AllInputManagement {
    pub fn new() -> Self {
        Self {
            input_new_frame: 0,
            action_history: VecDeque::new(),
            input_reset_timer: Vec::new(),

            input_buffer: VecDeque::new(),
        }
    }

    pub fn update_inputs_reset_timer(&mut self) {
        for i in 0..self.input_reset_timer.len() {
            self.input_reset_timer[i] += 1;
            if self.input_reset_timer[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                if self.action_history.len() > 1 {
                    self.action_history.pop_front();
                }
            }
        }
        self.input_reset_timer
            .retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);
    }
}
