extern crate sdl2;

use sdl2::event::Event;
use std::collections::HashMap;

use crate::game_logic::game_input::GameInputs;

pub fn rcv_input(event: Event, game_controls: &mut HashMap<std::string::String, GameInputs>) -> Option<GameInputs> {

    let mut m: Vec<u8>  = Vec::new();
    return match event {
        Event::JoyAxisMotion {which, axis_idx, value, ..} => {
            println!("joy#{} axis#{} value:{}", which, axis_idx, value);
            let sign = i32::from(value).signum();
            if axis_idx == 0 {
                Some(GameInputs::Horizontal(sign))
            } else {
                Some(GameInputs::Vertical(sign))
            }
        },
        Event::ControllerAxisMotion {which, axis, value, ..} => {
            println!("ctrl#{}, axis {:?} value:{}", which, axis, value);
            None
        },
        Event::JoyButtonDown {which, button_idx, ..} => {
            println!("joy#{} button#{} down", which, button_idx);
            m.push(button_idx);
            if game_controls.contains_key(&button_idx.to_string()) {
                Some(*game_controls.get(&button_idx.to_string()).unwrap()) //am i going crazy?
            } else {
                None
            }
        },
        Event::JoyButtonUp {which, button_idx, ..} => {
            //println!("joy#{} button#{} up", which, button_idx);
            None
        },
        _ => { None }
    }

}
