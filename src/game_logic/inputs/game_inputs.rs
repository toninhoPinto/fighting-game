use std::fmt::{self, Display};

use crate::input::translated_inputs::TranslatedInput;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameInput {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
    Forward,
    ForwardDown,
    ForwardUp,
    Backward,
    BackwardDown,
    BackwardUp,
    Up,
    Down,
    DashForward,
    DashBackward,
    Grab
}

impl Display for GameInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl GameInput {
    pub fn init_input_state() -> [(GameInput, bool); 10]{
        let mut current_inputs_state: [(GameInput, bool); 10] = [(GameInput::LightPunch, false); 10];
        current_inputs_state[0] = (GameInput::LightPunch, false);
        current_inputs_state[1] = (GameInput::MediumPunch, false);
        current_inputs_state[2] = (GameInput::HeavyPunch, false);
        current_inputs_state[3] = (GameInput::LightKick, false);
        current_inputs_state[4] = (GameInput::MediumKick, false);
        current_inputs_state[5] = (GameInput::HeavyKick, false);
        current_inputs_state[6] = (GameInput::Forward, false);
        current_inputs_state[7] = (GameInput::Up, false);
        current_inputs_state[8] = (GameInput::Backward, false);
        current_inputs_state[9] = (GameInput::Down, false);
    
        current_inputs_state
    }

    pub fn is_pressed(current_inputs_state: &[(GameInput, bool); 10], input: GameInput) -> bool {
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
    pub fn get_button_index(current_inputs_state: &mut [(GameInput, bool); 10], input: GameInput) -> usize {
        let mut return_index: usize = 0;
        for i in 0..6 {
            if current_inputs_state[i].0 == input{
                return_index = i;
                break;
            }
        }
        return_index
    }

    pub fn get_direction_index(input: GameInput) -> usize {
        match input {
            GameInput::Forward => { 6 },
            GameInput::Backward => { 8 },
            GameInput::Up => { 7 },
            GameInput::Down => { 9 }
            _ => { 0 }
        }
    }

    pub fn from_translated_input(original_input: TranslatedInput, current_input_state: &[(GameInput, bool); 10], player_facing_dir: i32) -> Result<GameInput, String> {
        
        match original_input {
            TranslatedInput::LightPunch => { Ok(GameInput::LightPunch) }
            TranslatedInput::MediumPunch => { Ok(GameInput:: MediumPunch) }
            TranslatedInput::HeavyPunch => { Ok(GameInput::HeavyPunch) }
            TranslatedInput::LightKick => { Ok(GameInput::LightKick) }
            TranslatedInput::MediumKick => { Ok(GameInput::MediumKick) }
            TranslatedInput::HeavyKick => { Ok(GameInput::HeavyKick) }
            TranslatedInput::Horizontal(h) => { 
                if h != 0 {
                    let right_dir = if h * player_facing_dir > 0 { 
                        GameInput::Forward 
                    } else { 
                        GameInput::Backward 
                    };
                    Ok(right_dir) 
                } else { 
                    //Specifically for joysticks that do not inform what was once pressed and then released for the axis
                    //so whatever was once pressed is the direction that was released (this works because joystick only lets you have 1 direction at a time)
                    if current_input_state[6].1 { 
                        Ok(current_input_state[6].0) 
                    } else {
                        Ok(current_input_state[8].0) 
                    }
                }
            }
            TranslatedInput::Vertical(v) if v > 0 => { Ok(GameInput::Up) }
            TranslatedInput::Vertical(v) if v < 0 => { Ok(GameInput::Down) }

            TranslatedInput::Vertical(v) if v == 0 => { 
                if current_input_state[7].1 {
                    Ok(GameInput::Up) 
                } else {
                    Ok(GameInput::Down) 
                }
            }
            _ => { Err("cannot identify this input".to_string()) }
        }
    }

    pub fn merge_horizontal_vertical(input_1: GameInput, input_2: GameInput) -> Result<GameInput, &'static str>  {
        if (input_1 == GameInput::Forward && input_2 == GameInput::Up ) || (input_2 == GameInput::Forward && input_1 == GameInput::Up ) {
            Ok(GameInput::ForwardUp)
        } else if (input_1 == GameInput::Backward && input_2 == GameInput::Up) || (input_2 == GameInput::Backward && input_1 == GameInput::Up) {
            Ok(GameInput::BackwardUp)
        } else if (input_1 == GameInput::Forward && input_2 == GameInput::Down ) || (input_2 == GameInput::Forward && input_1 == GameInput::Down ) {
            Ok(GameInput::ForwardDown)
        } else if (input_1 == GameInput::Backward && input_2 == GameInput::Down) || (input_2 == GameInput::Backward && input_1 == GameInput::Down) {
            Ok(GameInput::BackwardDown)
        } else {
            Err("trying to merge two incorrect inputs")
        }
    } 
}