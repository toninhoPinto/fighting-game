use std::fmt::{self, Display};

use crate::input::translated_inputs::TranslatedInput;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameAction {
    Right =   0b0000000001,     // 1
    Left =  0b0000000010,     // 2
    Up =        0b0000000100,     // 4
    Down =      0b0000001000,     // 8
    Punch =     0b0000010000,     // 16
    Kick =      0b0000100000,     // 32
    Jump =      0b0001000000,     // 64
    Block =     0b0010000000,     // 128
    Dash =      0b0100000000,     
    Slide =     0b1000000000,
}

impl Display for GameAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl GameAction {

    pub fn combinate_states(curr_state: &mut i32){
        if GameAction::check_if_pressed(*curr_state, GameAction::Dash as i32) && GameAction::check_if_pressed(*curr_state, GameAction::Down as i32) {
            *curr_state ^= GameAction::Slide as i32
        }
    }

    pub fn update_state(curr_state: i32, update: (GameAction, bool)) -> i32 {
        if update.1 {
            curr_state | update.0 as i32
        } else if curr_state & (update.0 as i32) > 0 {
            curr_state ^ (update.0 as i32)
        } else {
            curr_state
        }
    }

    pub fn check_if_pressed(curr_state: i32, check: i32) -> bool {
        curr_state & check > 0
    }

    pub fn check_if_pressed_direction(curr_state: i32) -> bool {
        curr_state & GameAction::Right as i32 > 0 || 
        curr_state & GameAction::Left as i32 > 0 || 
        curr_state & GameAction::Up as i32 > 0 ||
        curr_state & GameAction::Down as i32 > 0
    }

    pub fn from_translated_input(
        original_input: TranslatedInput,
        curr_state: i32,
        player_facing_dir: i8,
    ) -> Result<GameAction, String> {
        match original_input {
            TranslatedInput::Punch => Ok(GameAction::Punch),
            TranslatedInput::Kick => Ok(GameAction::Kick),
            TranslatedInput::Jump => Ok(GameAction::Jump),
            TranslatedInput::Block => Ok(GameAction::Block),
            TranslatedInput::Horizontal(h) if h != 0 => {
                let right_dir = if h > 0 {
                    GameAction::Right
                } else {
                    GameAction::Left
                };
                Ok(right_dir)
            },
            TranslatedInput::Horizontal(h) if h == 0 => {
                    //Specifically for joysticks that do not inform what was once pressed and then released for the axis
                    //so whatever was once pressed is the direction that was released (this works because joystick only lets you have 1 direction at a time)
                    if GameAction::check_if_pressed(curr_state, GameAction::Right as i32){
                        Ok(GameAction::Right)
                    } else {
                        Ok(GameAction::Left)
                    }
            }
            TranslatedInput::Vertical(v) if v > 0 => Ok(GameAction::Up),
            TranslatedInput::Vertical(v) if v < 0 => Ok(GameAction::Down),

            TranslatedInput::Vertical(v) if v == 0 => {
                if GameAction::check_if_pressed(curr_state, GameAction::Up as i32) {
                    Ok(GameAction::Up)
                } else {
                    Ok(GameAction::Down)
                }
            }
            _ => Err("cannot identify this input".to_string()),
        }
    }
}
