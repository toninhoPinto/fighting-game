use crate::game_logic::characters::player::{Player, PlayerState};
use crate::game_logic::character_factory::CharacterAssets;
use super::game_inputs::GameInputs;
use std::collections::VecDeque;
use std::string::String; // 1.0.104

pub fn apply_game_inputs<'a, 'b>(character_anims: &'a CharacterAssets, player: &'b mut Player<'a>, input: GameInputs, last_inputs: &mut VecDeque<GameInputs>){
    match input {
        GameInputs::Vertical(v) => {
            if v > 0 {
                player.player_state_change(PlayerState::Jump);
                player.last_directional_input_v = Some(GameInputs::UP);
                player.animator.animation_index = 0.0;
            } else if v < 0 {
                if player.state == PlayerState::Standing {
                    player.player_state_change(PlayerState::Crouch);
                }
                player.last_directional_input_v = Some(GameInputs::DOWN);
            } else {
                if player.state == PlayerState::Crouch ||  player.state == PlayerState::Crouching {
                    player.player_state_change(PlayerState::UnCrouch);
                } else {
                    player.player_state_change(PlayerState::Standing);
                }
                player.last_directional_input_v = None;
            }
            merge_last_horizontal_and_vertical_inputs(player, last_inputs);
        },
        GameInputs::Horizontal(h) => {
            player.direction = h;
            if h != 0 {
                if player.direction * player.dir_related_of_other > 0 {
                    player.last_directional_input_h = Some(GameInputs::FWD);
                } else {
                    player.last_directional_input_h = Some(GameInputs::BACK);
                }
                println!("Moved to side");
            } else {
                player.last_directional_input_h = None;
            }
            merge_last_horizontal_and_vertical_inputs(player, last_inputs);
            check_for_dash_inputs(player, last_inputs);
        },
        GameInputs::LightPunch => {

            record_input(last_inputs, GameInputs::LightPunch);

            //TODO add input buffering both on button down and button up
            //but only buffer on button up IF already attacking, dont normal attack
            handle_attack_input_for_possible_combos(character_anims,
                                                    player,
                                                    GameInputs::LightPunch,
                                                    last_inputs,
                                                    "light_punch".to_string());
        },
        GameInputs::MediumPunch => {
            record_input(last_inputs, GameInputs::MediumPunch);

            //TODO add input buffering both on button down and button up
            //but only buffer on button up IF already attacking, dont normal attack
            handle_attack_input_for_possible_combos(character_anims,
                                                    player,
                                                    GameInputs::MediumPunch,
                                                    last_inputs,
                                                    "med_punch".to_string());
        },
        GameInputs::HeavyPunch => {
            record_input(last_inputs, GameInputs::HeavyPunch);

            //TODO add input buffering both on button down and button up
            //but only buffer on button up IF already attacking, dont normal attack
            handle_attack_input_for_possible_combos(character_anims,
                                                    player,
                                                    GameInputs::HeavyPunch,
                                                    last_inputs,
                                                    "heavy_punch".to_string());
        },
        GameInputs::LightKick => {
            println!("Light Kick");
            record_input(last_inputs, GameInputs::LightKick);

            //TODO add input buffering both on button down and button up
            //but only buffer on button up IF already attacking, dont normal attack
            handle_attack_input_for_possible_combos(character_anims,
                                                    player,
                                                    GameInputs::LightKick,
                                                    last_inputs,
                                                    "light_kick".to_string());
        },
        GameInputs::MediumKick => { () },
        GameInputs::HeavyKick => { () },
        _ => { () }
    }
}

fn handle_attack_input_for_possible_combos<'a, 'b>(character_anims: &'a CharacterAssets, player: &'b mut Player<'a>, input: GameInputs, last_inputs: &mut VecDeque<GameInputs>, animation_name: String) {
    let special_attack = check_for_history_string_inputs(character_anims, last_inputs);
    if special_attack != "" {
        player_attack(character_anims, player, special_attack);
    } else { //check for directional inputs and if nothing then normal light punch
            let directional_attack = check_for_last_directional_inputs_directional_attacks(character_anims,input, &player);
            if directional_attack != "" {
                player_attack(character_anims, player, directional_attack);
            } else {
                player_attack(character_anims, player, animation_name);
            }
    }
}

fn check_for_dash_inputs(player: &mut Player, last_inputs: &mut VecDeque<GameInputs>) {
    let len = last_inputs.len();
    if len >= 2 && last_inputs[len - 2] == last_inputs[len - 1]{
        if last_inputs[len - 1] == GameInputs::BACK {
            player.player_state_change(PlayerState::DashingBackward);
            player.animator.animation_index = 0.0;
            last_inputs.clear();
        } else if last_inputs[len - 1] == GameInputs::FWD {
            player.player_state_change(PlayerState::DashingForward);
            player.animator.animation_index = 0.0;
            last_inputs.clear();
        }
    }
}

fn merge_last_horizontal_and_vertical_inputs(player: &mut Player, last_inputs: &mut VecDeque<GameInputs>){
    match (player.last_directional_input_h, player.last_directional_input_v)  {
        (Some(GameInputs::FWD), Some(GameInputs::DOWN)) => {
            player.last_directional_input = Some(GameInputs::FwdDOWN);
            record_input(last_inputs, GameInputs::FwdDOWN);
        },
        (Some(GameInputs::BACK), Some(GameInputs::DOWN)) => {
            player.last_directional_input = Some(GameInputs::BackDOWN);
            record_input(last_inputs, GameInputs::BackDOWN);
        },
        (Some(GameInputs::FWD), Some(GameInputs::UP)) => {
            player.last_directional_input = Some(GameInputs::FwdUP);
            record_input(last_inputs, GameInputs::FwdUP);
        },
        (Some(GameInputs::BACK), Some(GameInputs::UP)) => {
            player.last_directional_input = Some(GameInputs::BackUP);
            record_input(last_inputs, GameInputs::BackUP);
        },
        (None, a) if a.is_none()=> {
            player.last_directional_input = None;
        },
        (a, None) => {
            player.last_directional_input = a;
            record_input(last_inputs, a.unwrap());
        },
        (None, a) => {
            player.last_directional_input = a;
            record_input(last_inputs, a.unwrap());
        },
        _ => {}
    }
}

fn check_for_last_directional_inputs_directional_attacks(character_anims: &CharacterAssets, current_input: GameInputs, player: &Player) -> String {
    let mut ability_name: &str = "";

    'search_directionals: for possible_combo in character_anims.directional_variation_anims.iter() {
        let (moves, name) = possible_combo;

        match player.last_directional_input {
            Some(_v) => {  if moves[0] == player.last_directional_input.unwrap() && moves[1] == current_input {
                                ability_name = name;
                                break 'search_directionals;
                            }
                        },
            None => {},
        }
    }

    ability_name.to_string()
}

fn check_for_history_string_inputs(character_anims: &CharacterAssets, last_inputs: &mut VecDeque<GameInputs>) -> String {
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

fn record_input(last_inputs: &mut VecDeque<GameInputs>, input: GameInputs){
    last_inputs.push_back(input);
    if last_inputs.len() > 5 {
        last_inputs.pop_front();
    }
}

fn player_attack<'a, 'b>(character_anims: &'a CharacterAssets, player: &'b mut Player<'a>, attack_animation: String) {
    if !player.is_attacking {
        player.is_attacking = true;
        player.animator.play_once(character_anims.animations.get(&attack_animation).unwrap(), false);
    }
}
