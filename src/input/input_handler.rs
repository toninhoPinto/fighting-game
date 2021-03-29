extern crate sdl2;

use sdl2::event::Event;
use std::collections::HashMap;

use crate::game_logic::inputs::game_inputs::GameInputs;

pub fn rcv_input(event: &Event, current_inputs_state: &mut [(GameInputs, bool); 8], game_controls: &mut HashMap<std::string::String, GameInputs>) -> Option<GameInputs> {

    return match *event {
        Event::JoyAxisMotion {which, axis_idx, value, ..} => {
            println!("joy#{} axis#{} value:{}", which, axis_idx, value);
            let sign = i32::from(value).signum();
            handle_joystick(current_inputs_state, axis_idx, sign);
            if axis_idx == 0 {
                Some(GameInputs::Horizontal(sign))
            } else {
                Some(GameInputs::Vertical(-sign))
            }
        },
        Event::ControllerAxisMotion {which, axis, value, ..} => {
            println!("ctrl#{}, axis {:?} value:{}", which, axis, value);
            None
        },
        Event::JoyButtonDown {which, button_idx, ..} => {
            println!("joy#{} button#{} down", which, button_idx);
            if game_controls.contains_key(&button_idx.to_string()) {
                let input = *game_controls.get(&button_idx.to_string()).unwrap();
                handle_buttons(current_inputs_state, input, true);
                Some(input)
            } else {
                None
            }
        },
        Event::JoyButtonUp {which, button_idx, ..} => {
            println!("joy#{} button#{} up", which, button_idx);
            if game_controls.contains_key(&button_idx.to_string()) {
                let input = *game_controls.get(&button_idx.to_string()).unwrap();
                handle_buttons(current_inputs_state, input, false);
            }
            None
        },
        Event::KeyDown { keycode, ..} => {
            let key_down = keycode.unwrap();
            if game_controls.contains_key(&key_down.to_string()) {
                let input = *game_controls.get(&key_down.to_string()).unwrap();
                handle_buttons(current_inputs_state, input, true);
                Some(input)
            } else {
                None
            }
        } 
        Event::KeyUp { keycode, ..} => {
            let key_up = keycode.unwrap();
            if game_controls.contains_key(&key_up.to_string()) {
                let input = *game_controls.get(&key_up.to_string()).unwrap();
                handle_buttons(current_inputs_state, input, false);
            }
            None
        }
        _ => { None }
    }

}

fn handle_buttons(current_inputs_state: &mut [(GameInputs, bool); 8], input: GameInputs, is_pressed: bool) {
    for i in 0..8 {
        if current_inputs_state[i].0 == input {
            current_inputs_state[i] = (current_inputs_state[i].0, is_pressed);
            break;
        }
    }
}

fn handle_joystick(current_inputs_state: &mut [(GameInputs, bool); 8], axis_idx: u8, input: i32) {
    let is_pressed;
    if input == 0 {
        is_pressed = false;
    } else {
        is_pressed = true;
    }
    if axis_idx == 0 {
        current_inputs_state[6] = (GameInputs::Horizontal(1), is_pressed);
    } else {
        current_inputs_state[7] = (GameInputs::Vertical(1), is_pressed);
    }
}
