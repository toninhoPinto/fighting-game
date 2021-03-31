use crate::game_logic::characters::player::{Player, PlayerState};
use crate::game_logic::character_factory::CharacterAssets;
use super::game_inputs::GameInput;
use super::process_inputs::record_input;
use std::collections::VecDeque;
use std::string::String;

pub fn apply_game_input_state<'a, 'b>(character_anims: &'a CharacterAssets, player: &'b mut Player<'a>, 
        current_input_state: &[(GameInput, bool); 10], last_inputs: &mut VecDeque<GameInput>) {
        let len = last_inputs.len();
        //if forward and backwards at the same time, keep priorizing forward
        //this is needed for when character jumps and the "forward" changes
        if current_input_state[6].1 && current_input_state[8].1 {
            if current_input_state[6].1 { 
                player.velocity_x = 1;
            } 
            player.velocity_x = player.velocity_x * player.dir_related_of_other;
        }

        if current_input_state[6].1 && (len == 0 || (last_inputs[len - 1] != GameInput::Forward && last_inputs[len - 1] != GameInput::ForwardUp && last_inputs[len - 1] != GameInput::ForwardDown )) {
            record_input(last_inputs, current_input_state[6].0);
        }
        if current_input_state[8].1 && (len == 0 || (last_inputs[len - 1] != GameInput::Backward && last_inputs[len - 1] != GameInput::BackwardDown && last_inputs[len - 1] != GameInput::BackwardUp )) {
            record_input(last_inputs, current_input_state[8].0);
        }
        //up
        if current_input_state[7].1 {
            if len == 0 || (last_inputs[len - 1] != GameInput::Up && last_inputs[len - 1] != GameInput::ForwardUp && last_inputs[len - 1] != GameInput::BackwardUp ) {
                record_input(last_inputs, current_input_state[7].0);
            }
            player.player_state_change(PlayerState::Jump); 
        } else if current_input_state[9].1 { //down
            if len == 0 || (last_inputs[len - 1] != GameInput::Down && last_inputs[len - 1] != GameInput::ForwardDown && last_inputs[len - 1] != GameInput::BackwardDown ){
                record_input(last_inputs, current_input_state[9].0);
            }
            player.player_state_change(PlayerState::Crouch);
        }
         
}

//TODO clean up this
pub fn apply_game_inputs<'a, 'b>(character_anims: &'a CharacterAssets, player: &'b mut Player<'a>, 
    recent_input: GameInput, is_pressed: bool, 
    current_input_state: &[(GameInput, bool); 10], last_inputs: &mut VecDeque<GameInput>) {

    match recent_input {
        GameInput::Forward => {
            player.velocity_x = 1;
            if is_pressed {
                check_for_dash_inputs(player, last_inputs);
            } else {
                if current_input_state[GameInput::get_direction_index(GameInput::Backward)].1 {
                    player.velocity_x = -1;
                } else {
                    player.velocity_x = 0;
                }
            }
            player.velocity_x = player.velocity_x * player.dir_related_of_other;
        }
        GameInput::Backward => {   
            if is_pressed {
                if !current_input_state[GameInput::get_direction_index(GameInput::Forward)].1 {
                    player.velocity_x = -1;
                    check_for_dash_inputs(player, last_inputs);
                    player.velocity_x = player.velocity_x * player.dir_related_of_other;
                }
            } else {
                if current_input_state[GameInput::get_direction_index(GameInput::Forward)].1 {
                    player.velocity_x = 1;
                } else {
                    player.velocity_x = 0;
                }
                player.velocity_x = player.velocity_x * player.dir_related_of_other;
            }
            
        }
        GameInput::Up => { 
            if is_pressed {
                player.player_state_change(PlayerState::Jump); 
            }
        }
        GameInput::Down => {
            if is_pressed {
                player.player_state_change(PlayerState::Crouch);
            } else {
                player.player_state_change(PlayerState::UnCrouch);
            }
        }
        GameInput::LightPunch => {
            if is_pressed {
                handle_attack_input_for_possible_combos(character_anims,
                    player,
                    GameInput::LightPunch,
                    last_inputs,
                    "light_punch".to_string());
            }
        }
        GameInput::MediumPunch => {
            if is_pressed {
                handle_attack_input_for_possible_combos(character_anims,
                    player,
                    GameInput::MediumPunch,
                    last_inputs,
                    "med_punch".to_string());
            }
        }
        GameInput::HeavyPunch => {
            if is_pressed {
                handle_attack_input_for_possible_combos(character_anims,
                    player,
                    GameInput::HeavyPunch,
                    last_inputs,
                    "heavy_punch".to_string());
            }
        }
        GameInput::LightKick => {
            //TODO add input buffering both on button down and button up
            if is_pressed {
                handle_attack_input_for_possible_combos(character_anims,
                    player,
                    GameInput::LightKick,
                    last_inputs,
                    "light_kick".to_string());
            }
        }
        GameInput::MediumKick => {
            ()
        }
        GameInput::HeavyKick => {
            ()
        }
        _ => { () }
    }
}

fn handle_attack_input_for_possible_combos<'a, 'b>(character_anims: &'a CharacterAssets, player: &'b mut Player<'a>, input: GameInput, last_inputs: &mut VecDeque<GameInput>, animation_name: String) {
    let special_attack = check_for_history_string_inputs(character_anims, last_inputs);
    if special_attack != "" {
        player_attack(character_anims, player, special_attack);
    } else { //check for directional inputs and if nothing then normal light punch
            let directional_attack = check_for_last_directional_inputs_directional_attacks(character_anims, last_inputs, &player);
            if directional_attack != "" {
                player_attack(character_anims, player, directional_attack);
            } else {
                player_attack(character_anims, player, animation_name);
            }
    }
}

fn check_for_dash_inputs(player: &mut Player, last_inputs: &mut VecDeque<GameInput>) {
    let len = last_inputs.len();
    if len >= 2 && last_inputs[len - 2] == last_inputs[len - 1]{
        if last_inputs[len - 1] == GameInput::Backward {
            player.player_state_change(PlayerState::DashingBackward);
            player.animator.animation_index = 0.0;
            last_inputs.clear();
        } else if last_inputs[len - 1] == GameInput::Forward {
            player.player_state_change(PlayerState::DashingForward);
            player.animator.animation_index = 0.0;
            last_inputs.clear();
        }
    }
}

fn check_for_last_directional_inputs_directional_attacks(character_anims: &CharacterAssets, last_inputs: &mut VecDeque<GameInput>, player: &Player) -> String {
    let mut ability_name: &str = "";

    'search_directionals: for possible_combo in character_anims.directional_variation_anims.iter() {
        let (moves, name) = possible_combo;
        let mut match_moves = true;

        if last_inputs.len() >= moves.len() {
            let mut i = (last_inputs.len() - 1) as i32;
            let mut j = (moves.len() - 1) as i32;
            while i >= 0 && j >= 0 {

                match_moves &= moves[j as usize] == last_inputs[i as usize]; 
                j -= 1;
                i -= 1;
            }

            if match_moves {
                ability_name = name;
                break 'search_directionals;
            }
        }
    }

    ability_name.to_string()
}

fn check_for_history_string_inputs(character_anims: &CharacterAssets, last_inputs: &mut VecDeque<GameInput>) -> String {
    //iterate over last inputs starting from the end
    //check of matches against each of the player.input_combination_anims
    //if no match
    // iterate over last inputs starting from the end -1
    //etc
    //if find match, play animation and remove that input from array
    let mut l;
    let mut ability_name: &str = "";
    'search_combo: for possible_combo in character_anims.input_combination_anims.iter() {
        for n in 0..last_inputs.len() {
            l = 0;
            for d in n..last_inputs.len() {
                let (moves, name) = possible_combo;
                if last_inputs[d] == moves[l] {
                    l+= 1;
                } else {
                    break;
                }

                if l == moves.len() {
                    ability_name = name;
                    last_inputs.clear();
                    break 'search_combo;
                }
            }
        }

    }

    ability_name.to_string()
}

fn player_attack<'a, 'b>(character_anims: &'a CharacterAssets, player: &'b mut Player<'a>, attack_animation: String) {
    if player.player_can_attack() {
        player.is_attacking = true;
        player.animator.play_once(character_anims.animations.get(&attack_animation).unwrap(), false);
    }
}
