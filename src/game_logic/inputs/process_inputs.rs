use crate::input::translated_inputs::TranslatedInput;

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

pub fn update_button_state(
    input: TranslatedInput,
    is_pressed: bool,
    button_state_input: &mut [(TranslatedInput, bool); 6],
) {
    match input {
        TranslatedInput::LightPunch => {
            button_state_input[0].1 = is_pressed;
        }
        TranslatedInput::MediumPunch => {
            button_state_input[1].1 = is_pressed;
        }
        TranslatedInput::HeavyPunch => {
            button_state_input[2].1 = is_pressed;
        }
        TranslatedInput::LightKick => {
            button_state_input[3].1 = is_pressed;
        }
        TranslatedInput::MediumKick => {
            button_state_input[4].1 = is_pressed;
        }
        TranslatedInput::HeavyKick => {
            button_state_input[5].1 = is_pressed;
        }
        _ => {}
    }
}