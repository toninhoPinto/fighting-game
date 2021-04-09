use std::fmt::{self, Display};

use crate::input::translated_inputs::TranslatedInput;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameAction {
    Forward = 0b00001, // 1
    Backward = 0b00010, // 2
    Up = 0b00100, // 4
    Down = 0b01000, // 8 
    LightPunch = 0b10000, // 16 
    MediumPunch = 0b100000, // 32 
    HeavyPunch = 0b1000000, // 64 
    LightKick = 0b10000000, // 128 
    MediumKick = 0b100000000, 
    HeavyKick = 0b1000000000,
}

impl GameAction {
    pub fn update_state(curr_state: &mut i32, update: (GameAction, bool)) {
        if update.1 { 
            *curr_state |= update.0 as i32;
        } else if *curr_state & (update.0 as i32) > 0 {
            *curr_state ^= update.0 as i32;
        }
    }

    pub fn check_if_pressed(curr_state: &mut i32, check: i32) -> bool {
        *curr_state & check > 0
    }
}

impl Display for GameAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl GameAction {
    pub fn init_input_state() -> [(GameAction, bool); 10] {
        let mut current_inputs_state: [(GameAction, bool); 10] =
            [(GameAction::LightPunch, false); 10];
        current_inputs_state[0] = (GameAction::LightPunch, false);
        current_inputs_state[1] = (GameAction::MediumPunch, false);
        current_inputs_state[2] = (GameAction::HeavyPunch, false);
        current_inputs_state[3] = (GameAction::LightKick, false);
        current_inputs_state[4] = (GameAction::MediumKick, false);
        current_inputs_state[5] = (GameAction::HeavyKick, false);
        current_inputs_state[6] = (GameAction::Forward, false);
        current_inputs_state[7] = (GameAction::Up, false);
        current_inputs_state[8] = (GameAction::Backward, false);
        current_inputs_state[9] = (GameAction::Down, false);

        current_inputs_state
    }

    pub fn is_pressed(current_inputs_state: &[(GameAction, bool); 10], input: GameAction) -> bool {
        let mut return_bool = false;
        for i in 0..10 {
            if current_inputs_state[i].0 == input {
                return_bool = current_inputs_state[i].1;
                break;
            }
        }
        return_bool
    }

    //TODO maybe return Result on these to avoid 0 being default
    pub fn get_button_index(
        current_inputs_state: &mut [(GameAction, bool); 10],
        input: GameAction,
    ) -> usize {
        let mut return_index: usize = 0;
        for i in 0..6 {
            if current_inputs_state[i].0 == input {
                return_index = i;
                break;
            }
        }
        return_index
    }

    pub fn get_direction_index(input: GameAction) -> usize {
        match input {
            GameAction::Forward => 6,
            GameAction::Backward => 8,
            GameAction::Up => 7,
            GameAction::Down => 9,
            _ => 0,
        }
    }

    pub fn from_translated_input(
        original_input: TranslatedInput,
        current_input_state: &[(TranslatedInput, bool); 4],
        player_facing_dir: i32,
    ) -> Result<GameAction, String> {
        match original_input {
            TranslatedInput::LightPunch => Ok(GameAction::LightPunch),
            TranslatedInput::MediumPunch => Ok(GameAction::MediumPunch),
            TranslatedInput::HeavyPunch => Ok(GameAction::HeavyPunch),
            TranslatedInput::LightKick => Ok(GameAction::LightKick),
            TranslatedInput::MediumKick => Ok(GameAction::MediumKick),
            TranslatedInput::HeavyKick => Ok(GameAction::HeavyKick),
            TranslatedInput::Horizontal(h) => {
                if h != 0 {
                    let right_dir = if h * player_facing_dir > 0 {
                        GameAction::Forward
                    } else {
                        GameAction::Backward
                    };
                    Ok(right_dir)
                } else {
                    //Specifically for joysticks that do not inform what was once pressed and then released for the axis
                    //so whatever was once pressed is the direction that was released (this works because joystick only lets you have 1 direction at a time)
                    if current_input_state[1].1 {
                        Ok(GameAction::Backward)
                    } else {
                        Ok(GameAction::Forward)
                    }
                }
            }
            TranslatedInput::Vertical(v) if v > 0 => Ok(GameAction::Up),
            TranslatedInput::Vertical(v) if v < 0 => Ok(GameAction::Down),

            TranslatedInput::Vertical(v) if v == 0 => {
                if current_input_state[2].1 {
                    Ok(GameAction::Up)
                } else {
                    Ok(GameAction::Down)
                }
            }
            _ => Err("cannot identify this input".to_string()),
        }
    }


}
