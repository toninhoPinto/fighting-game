use std::{fmt::{self, Display}};

use crate::input::translated_inputs::TranslatedInput;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GameAction {
    Right =     0b0000000001,     // 1
    Left =      0b0000000010,     // 2
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

    pub fn debug_i32(mut n: i32) -> Vec<Self>{
        let mut result = Vec::new();
        while n > 0 {
            if n & GameAction::Right as i32 > 0 {
                result.push(GameAction::Right);
                n ^= GameAction::Right as i32;
            }
            if n & GameAction::Left as i32 > 0 { 
                result.push(GameAction::Left);
                n ^= GameAction::Left as i32;
            }
            if n & GameAction::Up as i32 > 0 { 
                result.push(GameAction::Up);
                n ^= GameAction::Up as i32;
            }
            if n & GameAction::Down as i32 > 0 { 
                result.push(GameAction::Down);
                n ^= GameAction::Down as i32;
            }
            if n & GameAction::Punch as i32 > 0 { 
                result.push(GameAction::Punch);
                n ^= GameAction::Punch as i32;
            }
            if n & GameAction::Kick as i32 > 0 { 
                result.push(GameAction::Kick);
                n ^= GameAction::Kick as i32;
            }
            if n & GameAction::Jump as i32 > 0 { 
                result.push(GameAction::Jump);
                n ^= GameAction::Jump as i32;
            }
            if n & GameAction::Block as i32 > 0 { 
                result.push(GameAction::Block);
                n ^= GameAction::Block as i32;
            }
            if n & GameAction::Dash as i32 > 0 { 
                result.push(GameAction::Dash);
                n ^= GameAction::Dash as i32;
            }
            if n & GameAction::Slide as i32 > 0 { 
                result.push(GameAction::Slide);
                n ^= GameAction::Slide as i32;
            }
        }
        result
    }

    pub fn combinate_states(curr_state: &mut i32){
        if GameAction::is_pressed(*curr_state, GameAction::Dash) && GameAction::is_pressed(*curr_state, GameAction::Down) {
            *curr_state ^= GameAction::Slide as i32
        }
    }

    pub fn is_pressed(curr_state: i32, check: GameAction) -> bool {
        curr_state & check as i32 > 0
    }

    pub fn is_pressed_direction(curr_state: i32) -> bool {
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
                    if GameAction::is_pressed(curr_state, GameAction::Right){
                        Ok(GameAction::Right)
                    } else {
                        Ok(GameAction::Left)
                    }
            }
            TranslatedInput::Vertical(v) if v > 0 => Ok(GameAction::Up),
            TranslatedInput::Vertical(v) if v < 0 => Ok(GameAction::Down),

            TranslatedInput::Vertical(v) if v == 0 => {
                if GameAction::is_pressed(curr_state, GameAction::Up) {
                    Ok(GameAction::Up)
                } else {
                    Ok(GameAction::Down)
                }
            }
            _ => Err("cannot identify this input".to_string()),
        }
    }
}
