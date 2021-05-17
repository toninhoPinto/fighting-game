use std::collections::VecDeque;

const FRAME_WINDOW_BETWEEN_INPUTS: i32 = 10;
const FRAME_WINDOW_BUFFER: i32 = 10;

#[derive(Clone, PartialEq, Debug)]
pub struct AllInputManagement {
    pub input_new_frame: i32,
    pub action_history: VecDeque<i32>,
    pub input_reset_timer: Vec<i32>,

    pub input_buffer: VecDeque<i32>,
    pub input_buffer_reset_time: Vec<i32>,
}

impl AllInputManagement {
    pub fn new() -> Self {
        Self {
            input_new_frame: 0,
            action_history: VecDeque::new(),
            input_reset_timer: Vec::new(),

            input_buffer: VecDeque::new(),
            input_buffer_reset_time: Vec::new(),
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

    pub fn update_input_buffer_reset_time(&mut self) {
        for i in 0..self.input_buffer_reset_time.len() {
            self.input_buffer_reset_time[i] += 1;
            if self.input_buffer_reset_time[i] > FRAME_WINDOW_BUFFER {
                if self.input_buffer.len() > 0 {
                    self.input_buffer.pop_front();
                }
            }
        }
        self.input_buffer_reset_time
            .retain(|&i| i <= FRAME_WINDOW_BUFFER);
    }
}
