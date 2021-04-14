extern crate sdl2;

use sdl2::event::Event;
use std::collections::HashMap;

use super::translated_inputs::TranslatedInput;

pub fn rcv_input(
    event: &Event,
    game_controls: &HashMap<std::string::String, TranslatedInput>,
) -> Option<(u32, TranslatedInput, bool)> {
    return match *event {
        Event::JoyAxisMotion {
            which,
            axis_idx,
            value,
            ..
        } => {
            println!("joy#{} axis#{} value:{}", which, axis_idx, value);
            let sign = i32::from(value).signum();
            if axis_idx == 0 {
                Some((which, TranslatedInput::Horizontal(sign), sign != 0))
            } else {
                Some((which, TranslatedInput::Vertical(-sign), sign != 0))
            }
        }
        Event::ControllerAxisMotion {
            which, axis, value, ..
        } => {
            println!("ctrl#{}, axis {:?} value:{}", which, axis, value);
            None
        }
        Event::JoyButtonDown {
            which, button_idx, ..
        } => {
            println!("joy#{} button#{} down", which, button_idx);
            if game_controls.contains_key(&button_idx.to_string()) {
                let input = *game_controls.get(&button_idx.to_string()).unwrap();
                Some((which, input, true))
            } else {
                None
            }
        }
        Event::JoyButtonUp {
            which, button_idx, ..
        } => {
            println!("joy#{} button#{} up", which, button_idx);
            if game_controls.contains_key(&button_idx.to_string()) {
                let input = *game_controls.get(&button_idx.to_string()).unwrap();
                Some((which, input, false))
            } else {
                None
            }
        }
        Event::KeyDown { keycode, repeat,  ..} => {
            let key_down = keycode.unwrap();
            if game_controls.contains_key(&key_down.to_string()) && !repeat {
                let input = *game_controls.get(&key_down.to_string()).unwrap();
                Some((666, input, true))
            } else {
                None
            }
        }
        Event::KeyUp { keycode, .. } => {
            let key_up = keycode.unwrap();
            if game_controls.contains_key(&key_up.to_string()) {
                let input = *game_controls.get(&key_up.to_string()).unwrap();
                Some((666, input, false))
            } else {
                None
            }
        }
        _ => None,
    };
}
