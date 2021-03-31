use crate::game_logic::characters::player::Player;
use super::game_inputs::GameInput;
use crate::input::translated_inputs::TranslatedInput;
use std::collections::VecDeque;


pub fn transform_input_state(input: TranslatedInput, is_pressed: bool, 
    current_state_input : &mut [(GameInput, bool); 10],
    directional_state_input : &mut [(TranslatedInput, bool); 4],
    last_inputs: &mut VecDeque<GameInput>,
     player: &Player) -> Option<GameInput>{
    if TranslatedInput::is_directional_input(input) {
        
        if input == TranslatedInput::Horizontal(0) {
            directional_state_input[0].1 = false;
            directional_state_input[1].1 = false;
            update_current_state_and_consolidate_input(input, is_pressed, current_state_input,directional_state_input,last_inputs, player)
        } else if input == TranslatedInput::Vertical(0) {
            directional_state_input[2].1 = false;
            directional_state_input[3].1 = false;
            update_current_state_and_consolidate_input(input, is_pressed, current_state_input,directional_state_input,last_inputs, player)
        } else {
            let index = TranslatedInput::get_button_index(directional_state_input, input);
            if directional_state_input[index.unwrap()].1 != is_pressed {
                match input {
                    TranslatedInput::Horizontal(h) if h > 0 => {
                        directional_state_input[0].1 = is_pressed;
                    },
                    TranslatedInput::Horizontal(h) if h < 0 => {
                        directional_state_input[1].1 = is_pressed;
                    },
                    TranslatedInput::Vertical(v) if v > 0 => {
                        directional_state_input[2].1 = is_pressed;
                    },
                    TranslatedInput::Vertical(v) if v < 0 => {
                        directional_state_input[3].1 = is_pressed;
                    },
                    _ => {}
                }
                update_current_state_and_consolidate_input(input, is_pressed, current_state_input,directional_state_input,last_inputs, player)
            } else {
                None
            }    
        }
    } else {
        update_current_state_and_consolidate_input(input, is_pressed, current_state_input,directional_state_input,last_inputs, player)
    }
}

fn update_current_state_and_consolidate_input(input: TranslatedInput, is_pressed: bool, 
    current_state_input : &mut [(GameInput, bool); 10],
    directional_state_input : &mut [(TranslatedInput, bool); 4],
    last_inputs: &mut VecDeque<GameInput>,
     player: &Player) -> Option<GameInput>{

    let game_input = GameInput::from_translated_input(input, current_state_input,  player.dir_related_of_other).unwrap();
    let id = match game_input {
        GameInput::Forward | GameInput::Backward 
        | GameInput::Up | GameInput::Down => { GameInput::get_direction_index(game_input) }
        _ => {
            GameInput::get_button_index(current_state_input, game_input)
        }
    };

    current_state_input[id] = (game_input, is_pressed);
    
    //specifically for the case where with keyboard
    //you start walking left / forward
    //you pass the opponent and still move left but now its backwards
    //you release, it will change the flag for currently pressing left
    //but it wont change the flag for currently pressing forward since it think it is now backwards
    if !TranslatedInput::is_currently_any_directional_input(directional_state_input) {
        current_state_input[6].1 = false;
        current_state_input[8].1 = false;
    }

    let consolidated_input = consolidate_directional_inputs(game_input, is_pressed, current_state_input);
    match consolidated_input {
        Some(consolidated) => { 
            record_input(last_inputs, consolidated);
            Some(game_input)
        }
        None => {
            Some(game_input)
        }
    }
}

fn consolidate_directional_inputs(recent_input: GameInput, is_pressed: bool, current_input_state: &mut [(GameInput, bool); 10]) -> Option<GameInput> {
    //check if 1 vertical and 1 horizontal are currently pressed
    //if so -> merge into diagonal
    //if has up is more important
    //if has forward it is more important
    let is_directional_input = recent_input == GameInput::Forward || 
    recent_input == GameInput::Up ||
    recent_input == GameInput::Down ||
    recent_input == GameInput::Backward;

    if !is_directional_input && is_pressed { //if the current input was not a direction button and it was a press (not release), just return 
        Some(recent_input)
    } else if !is_directional_input && !is_pressed { //if the current input was not a direction button and it was a release
        None
    } else if is_directional_input && is_pressed { //if the current input was a direnction button and it was a press
        if current_input_state[7] == (GameInput::Up, true) {
            if current_input_state[6] == (GameInput::Forward, true) {
                Some(GameInput::merge_horizontal_vertical(GameInput::Up, GameInput::Forward).unwrap())
            } else if current_input_state[8] == (GameInput::Backward, true) {
                Some(GameInput::merge_horizontal_vertical(GameInput::Up, GameInput::Backward).unwrap())
            } else {
                Some(GameInput::Up)
            }
        } else if current_input_state[9] == (GameInput::Down, true) {
            if current_input_state[6] == (GameInput::Forward, true) {
                Some(GameInput::merge_horizontal_vertical(GameInput::Down, GameInput::Forward).unwrap())
            } else if current_input_state[8] == (GameInput::Backward, true) {
                Some(GameInput::merge_horizontal_vertical(GameInput::Down, GameInput::Backward).unwrap())
            } else {
                Some(GameInput::Down)
            }
        } else {
            Some(recent_input) //nothing to merge with
        }
    } else {
        if current_input_state[7] == (GameInput::Up, true) { 
            Some(GameInput::Up) 
        } else if current_input_state[6] == (GameInput::Forward, true) {
            Some(GameInput::Forward)
        } else if current_input_state[9] == (GameInput::Down, true) {
            Some(GameInput::Down)
        } else if current_input_state[8] == (GameInput::Backward, true) {
            Some(GameInput::Backward)
        } else {
            None
        }
    }
}

//TODO maybe change this to somewhere else
pub fn record_input(last_inputs: &mut VecDeque<GameInput>, input: GameInput){
    last_inputs.push_back(input);
    if last_inputs.len() > 5 {
        last_inputs.pop_front();
    }
}