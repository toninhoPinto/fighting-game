use super::game_inputs::GameAction;
use crate::game_logic::character_factory::CharacterAssets;
use crate::{
    game_logic::characters::player::{Player, PlayerState},
    input::translated_inputs::TranslatedInput,
};
use std::collections::VecDeque;
use std::string::String;

pub fn record_input(last_inputs: &mut VecDeque<GameAction>, input: GameAction) {
    last_inputs.push_back(input);
    if last_inputs.len() > 5 {
        last_inputs.pop_front();
    }
}

pub fn apply_input_state(player: &mut Player, directional_state: &[(TranslatedInput, bool); 4], button_state: &[(TranslatedInput, bool); 6]) {
    
    //in case you press forward, then press backwards, and then release backwards
    //since forward should still be applied
    if directional_state[0].1 {
        player.velocity_x = 1;
    } else {
        if directional_state[1].1 {
            player.velocity_x = -1;
        }
    }
    
    if directional_state[2].1 {
        player.jump();
    } else if directional_state[3].1 {
        player.player_state_change(PlayerState::Crouch);
    }
}


pub fn apply_input<'a, 'b>(
player: &'b mut Player<'a>,
character_anims: &'a CharacterAssets,
directional_state: &[(TranslatedInput, bool); 4],
button_state: &[(TranslatedInput, bool); 6],
to_process: &mut VecDeque<(TranslatedInput, bool)>,
inputs_processed: &mut VecDeque<TranslatedInput>,
action_history: &mut VecDeque<GameAction>,
special_reset_timer: &mut Vec<i32>){

    for &(recent_input, is_pressed) in to_process.iter(){

        if is_pressed {
            inputs_processed.push_back(recent_input);
        }

        let recent_input_as_game_action = GameAction::from_translated_input(
            recent_input ,
            directional_state, 
            player.dir_related_of_other).unwrap();

        if is_pressed && !action_history.is_empty() {
            match GameAction::merge_horizontal_vertical(recent_input_as_game_action, action_history[action_history.len() - 1]) {
                Ok(game_action) => {
                    action_history.push_back(game_action);
                    action_history.push_back(recent_input_as_game_action);
                    special_reset_timer.push(0);
                },
                _ => action_history.push_back(recent_input_as_game_action)
            }
        } else if action_history.is_empty() {
            action_history.push_back(recent_input_as_game_action)
        }
        special_reset_timer.push(0);

        match recent_input {
            TranslatedInput::Horizontal(h) => {
                if is_pressed {
                    check_for_dash_inputs(player, recent_input_as_game_action, inputs_processed);
                }
                player.velocity_x = if is_pressed {h} else {0};
            }
            TranslatedInput::Vertical(v) => {
                if is_pressed && v > 0 {
                    player.jump();
                }
                if is_pressed && v < 0{
                    player.player_state_change(PlayerState::Crouch);
                } else if player.state == PlayerState::Crouching || player.state == PlayerState::Crouch {
                    player.player_state_change(PlayerState::UnCrouch);
                }
            }
            TranslatedInput::LightPunch => {
                if is_pressed {
                    check_attack_inputs(
                        player,
                        character_anims,
                        GameAction::LightPunch,
                        "light_punch".to_string(),
                        directional_state,
                        button_state,
                        action_history,
                        special_reset_timer,
                    );
                }
            }
            TranslatedInput::MediumPunch => {
                if is_pressed {
                    check_attack_inputs(
                        player,
                        character_anims,
                        GameAction::MediumPunch,
                        "medium_punch".to_string(),
                        directional_state,
                        button_state,
                        action_history,
                        special_reset_timer,
                    );
                }
            }
            TranslatedInput::HeavyPunch => {
                if is_pressed {
                    check_attack_inputs(
                        player,
                        character_anims,
                        GameAction::HeavyPunch,
                        "heavy_punch".to_string(),
                        directional_state,
                        button_state,
                        action_history,
                        special_reset_timer,
                    );
                }
            }
            TranslatedInput::LightKick => {
                if is_pressed {
                    check_attack_inputs(
                        player,
                        character_anims,
                        GameAction::LightKick,
                        "light_kick".to_string(),
                        directional_state,
                        button_state,
                        action_history,
                        special_reset_timer,
                    );
                }
            }
            TranslatedInput::MediumKick => {}
            TranslatedInput::HeavyKick => {}
        }



    }
    to_process.clear();
}

fn check_for_dash_inputs(player: &mut Player, recent_input_as_game_action: GameAction, last_inputs: &mut VecDeque<TranslatedInput>) {
    let len = last_inputs.len();
    if len >= 2 && last_inputs[len - 2] == last_inputs[len - 1] {
        if last_inputs[len - 1] == TranslatedInput::Horizontal(-1)  || last_inputs[len - 1] == TranslatedInput::Horizontal(1) {
            if recent_input_as_game_action == GameAction::Forward {
                player.player_state_change(PlayerState::DashingForward);
            } else {
                player.player_state_change(PlayerState::DashingBackward);
            }
            last_inputs.clear();
        }
    }
}

fn check_attack_inputs<'a, 'b>(player: &'b mut Player<'a>, 
    character_anims: &'a CharacterAssets, 
    recent_input_as_game_action: GameAction, 
    animation_name: String, 
    directional_state: &[(TranslatedInput, bool); 4],
    button_state: &[(TranslatedInput, bool); 6],
    action_history: &mut VecDeque<GameAction>,
    special_reset_timer: &mut Vec<i32>){

    if let Some(special_input) = check_special_inputs(character_anims, action_history, special_reset_timer) {
        player.change_special_meter(-1.0);
        player.attack(character_anims, special_input);
    } else if let Some(directional_input) = check_directional_inputs(player, character_anims, directional_state, recent_input_as_game_action) {
        player.attack(character_anims, directional_input);
    } else if check_grab_input(button_state) {
        player.player_state_change(PlayerState::Grab);
        player.is_attacking = false;
    } else {
        player.change_special_meter(0.1);
        player.attack(character_anims, animation_name);
    }
}

fn check_special_inputs(character_anims: & CharacterAssets, action_history: &mut VecDeque<GameAction>, special_reset_timer: &mut Vec<i32>) -> Option<String> {
     //iterate over last inputs starting from the end
    //check of matches against each of the player.input_combination_anims
    //if no match
    // iterate over last inputs starting from the end -1
    //etc
    //if find match, play animation and remove that input from array
    let mut l;
    for possible_combo in character_anims.input_combination_anims.iter() {
        for n in 0..action_history.len() {
            l = 0;
            for d in n..action_history.len() {
                let (moves, name) = possible_combo;
                if action_history[d] == moves[l] {
                    l += 1;
                } else {
                    break;
                }

                if l == moves.len() {
                    action_history.clear();
                    special_reset_timer.clear();
                    return Some(name.to_string())
                }
            }
        }
    }

    None
}

fn check_directional_inputs(player: &mut Player, 
    character_anims: &CharacterAssets, 
    directional_state: &[(TranslatedInput, bool); 4],
    recent_input_as_game_action: GameAction
) -> Option<String>{
    
    for possible_combo in character_anims.directional_variation_anims.iter() {
        let (moves, name) = possible_combo;

        if TranslatedInput::is_currently_any_directional_input(directional_state)
         && recent_input_as_game_action == moves.1 {
            for i in 0..directional_state.len() {
                if directional_state[i].1 {
                    let direction_as_game_action = GameAction::from_translated_input(
                        directional_state[i].0,
                        directional_state, 
                        player.dir_related_of_other).unwrap();
        
                    if direction_as_game_action == moves.0 {
                        return Some(name.to_string())
                    }
                } 

            }
        }
    }
    None
}

fn check_grab_input(button_state: &[(TranslatedInput, bool); 6]) -> bool {
    button_state[0].1 && button_state[3].1
}

