use std::fmt::{self, Display};

use crate::input::translated_inputs::TranslatedInput;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameAction {
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
    Grab,
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

    pub fn merge_horizontal_vertical(
        input_1: GameAction,
        input_2: GameAction,
    ) -> Result<GameAction, &'static str> {
        if (input_1 == GameAction::Forward && input_2 == GameAction::Up)
            || (input_2 == GameAction::Forward && input_1 == GameAction::Up)
        {
            Ok(GameAction::ForwardUp)
        } else if (input_1 == GameAction::Backward && input_2 == GameAction::Up)
            || (input_2 == GameAction::Backward && input_1 == GameAction::Up)
        {
            Ok(GameAction::BackwardUp)
        } else if (input_1 == GameAction::Forward && input_2 == GameAction::Down)
            || (input_2 == GameAction::Forward && input_1 == GameAction::Down)
        {
            Ok(GameAction::ForwardDown)
        } else if (input_1 == GameAction::Backward && input_2 == GameAction::Down)
            || (input_2 == GameAction::Backward && input_1 == GameAction::Down)
        {
            Ok(GameAction::BackwardDown)
        } else {
            Err("trying to merge two incorrect inputs")
        }
    }
}
