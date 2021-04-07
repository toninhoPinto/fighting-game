use super::game_inputs::GameAction;
use crate::input::translated_inputs::TranslatedInput;
use std::collections::VecDeque;

pub fn filter_already_pressed_direction(
    input: TranslatedInput,
    directional_state_input: &mut [(TranslatedInput, bool); 4],
) -> Option<TranslatedInput> {
    match input {
        TranslatedInput::Horizontal(h) if h != 0 => {
            let index = TranslatedInput::get_button_index(directional_state_input, input).unwrap();
            if !directional_state_input[index].1 {
                Some(input)
            } else {
                None
            }
        }
        TranslatedInput::Vertical(v) if v != 0 => {
            let index = TranslatedInput::get_button_index(directional_state_input, input).unwrap();
            if !directional_state_input[index].1 {
                Some(input)
            } else {
                None
            }
        }
        _ => Some(input),
    }
}

pub fn released_joystick_reset_directional_state(
    input: TranslatedInput,
    directional_state_input: &mut [(TranslatedInput, bool); 4],
) {
    if input == TranslatedInput::Horizontal(0) {
        directional_state_input[0].1 = false;
        directional_state_input[1].1 = false;
    } else if input == TranslatedInput::Vertical(0) {
        directional_state_input[2].1 = false;
        directional_state_input[3].1 = false;
    }
}

pub fn update_directional_state(
    input: TranslatedInput,
    is_pressed: bool,
    directional_state_input: &mut [(TranslatedInput, bool); 4],
) {
    match input {
        TranslatedInput::Horizontal(h) if h > 0 => {
            directional_state_input[0].1 = is_pressed;
        }
        TranslatedInput::Horizontal(h) if h < 0 => {
            directional_state_input[1].1 = is_pressed;
        }
        TranslatedInput::Vertical(v) if v > 0 => {
            directional_state_input[2].1 = is_pressed;
        }
        TranslatedInput::Vertical(v) if v < 0 => {
            directional_state_input[3].1 = is_pressed;
        }
        _ => {}
    }
}

pub fn filter_already_pressed_button(
    input: GameAction,
    current_state_input: &mut [(GameAction, bool); 10],
) -> Option<GameAction> {
    let index = GameAction::get_button_index(current_state_input, input);

    if !current_state_input[index].1 {
        Some(input)
    } else {
        None
    }
}

pub fn transform_input_state(
    game_input: GameAction,
    is_pressed: bool,
    current_state_input: &mut [(GameAction, bool); 10],
    directional_state_input: &mut [(TranslatedInput, bool); 4],
    last_inputs: &mut VecDeque<GameAction>,
) -> Option<GameAction> {
    //update current state
    let id = match game_input {
        GameAction::Forward | GameAction::Backward | GameAction::Up | GameAction::Down => {
            GameAction::get_direction_index(game_input)
        }
        _ => GameAction::get_button_index(current_state_input, game_input),
    };
    current_state_input[id] = (game_input, is_pressed);

    //specifically for the case where with keyboard
    //you start walking right / forward
    //you pass the opponent and still move right but now its backwards
    //you release, it will change the flag for currently pressing right
    //but it wont change the flag for currently pressing forward since it think it is now backwards
    if !is_pressed && !TranslatedInput::is_currently_any_directional_input(directional_state_input)
    {
        current_state_input[6].1 = false;
        current_state_input[8].1 = false;
    }

    let consolidated_input =
        consolidate_directional_inputs(game_input, is_pressed, current_state_input);

    match consolidated_input {
        Some(consolidated) => {
            record_input(last_inputs, consolidated);
            Some(game_input)
        }
        None => Some(game_input),
    }
}

fn consolidate_directional_inputs(
    recent_input: GameAction,
    is_pressed: bool,
    current_input_state: &mut [(GameAction, bool); 10],
) -> Option<GameAction> {
    //check if 1 vertical and 1 horizontal are currently pressed
    //if so -> merge into diagonal
    //if has up is more important
    //if has forward it is more important
    let is_directional_input = recent_input == GameAction::Forward
        || recent_input == GameAction::Up
        || recent_input == GameAction::Down
        || recent_input == GameAction::Backward;

    if !is_directional_input && is_pressed {
        //if the current input was not a direction button and it was a press (not release), just return
        Some(recent_input)
    } else if !is_directional_input && !is_pressed {
        //if the current input was not a direction button and it was a release
        None
    } else if is_directional_input && is_pressed {
        //if the current input was a direnction button and it was a press
        if current_input_state[7] == (GameAction::Up, true) {
            if current_input_state[6] == (GameAction::Forward, true) {
                Some(
                    GameAction::merge_horizontal_vertical(GameAction::Up, GameAction::Forward)
                        .unwrap(),
                )
            } else if current_input_state[8] == (GameAction::Backward, true) {
                Some(
                    GameAction::merge_horizontal_vertical(GameAction::Up, GameAction::Backward)
                        .unwrap(),
                )
            } else {
                Some(GameAction::Up)
            }
        } else if current_input_state[9] == (GameAction::Down, true) {
            if current_input_state[6] == (GameAction::Forward, true) {
                Some(
                    GameAction::merge_horizontal_vertical(GameAction::Down, GameAction::Forward)
                        .unwrap(),
                )
            } else if current_input_state[8] == (GameAction::Backward, true) {
                Some(
                    GameAction::merge_horizontal_vertical(GameAction::Down, GameAction::Backward)
                        .unwrap(),
                )
            } else {
                Some(GameAction::Down)
            }
        } else {
            Some(recent_input) //nothing to merge with
        }
    } else {
        if current_input_state[7] == (GameAction::Up, true) {
            Some(GameAction::Up)
        } else if current_input_state[6] == (GameAction::Forward, true) {
            Some(GameAction::Forward)
        } else if current_input_state[9] == (GameAction::Down, true) {
            Some(GameAction::Down)
        } else if current_input_state[8] == (GameAction::Backward, true) {
            Some(GameAction::Backward)
        } else {
            None
        }
    }
}

pub fn record_input(last_inputs: &mut VecDeque<GameAction>, input: GameAction) {
    last_inputs.push_back(input);
    if last_inputs.len() > 5 {
        last_inputs.pop_front();
    }
}
