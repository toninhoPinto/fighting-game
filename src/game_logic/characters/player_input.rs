use std::{cmp, collections::VecDeque};

use parry2d::na::Vector2;

use crate::{asset_management::asset_holders::EntityData, ecs_system::enemy_manager::EnemyManager, game_logic::inputs::{game_inputs::GameAction, input_cycle::AllInputManagement}};

use super::player::{EntityState, Player};


pub fn apply_input_state(player: &mut Player, inputs: &mut AllInputManagement, character_data: &EntityData, enemies: &mut EnemyManager) {
    if let Some(&last_action) = inputs.action_history.back() {
        if GameAction::is_pressed(last_action, GameAction::Right) { //1
            player.controller.set_velocity_x(1, &mut player.animator);
        }
        if GameAction::is_pressed(last_action, GameAction::Left) { //-1
            player.controller.set_velocity_x(-1, &mut player.animator);
        }
    }

    let mut occupied = (player.controller.is_attacking && !player.controller.has_hit) ||
        player.controller.state == EntityState::Hurt ||
        player.controller.state == EntityState::Landing || 
        player.controller.state == EntityState::Dashing;

    let action_history = inputs.action_history.clone();

    inputs.input_buffer.retain(|&buffered_input| {

        occupied = (player.controller.is_attacking && !player.controller.has_hit) ||
        player.controller.state == EntityState::Hurt ||
        player.controller.state == EntityState::Landing || 
        player.controller.state == EntityState::Dashing;

        if !occupied {
            apply_input(player, buffered_input, character_data, &action_history, enemies);
        }

        occupied
    });
        
}

pub fn process_input(player: &mut Player,   
    character_data: &EntityData,
    inputs: &mut AllInputManagement,
    enemies: &mut EnemyManager) {
        
    let mut inputs_for_current_frame = if let Some(&last_action) = inputs.action_history.back() {last_action} else {0};
    inputs_for_current_frame ^= inputs.input_new_frame;

    if inputs_for_current_frame & GameAction::Dash as i32 > 0 {
        inputs_for_current_frame ^= GameAction::Dash as i32;
    }

    if check_for_dash_inputs(inputs_for_current_frame, &inputs.action_history) {
        inputs_for_current_frame |= GameAction::Dash as i32;
    }

    let occupied = (player.controller.is_attacking && !player.controller.has_hit) ||
    player.controller.state == EntityState::Hurt ||
    player.controller.state == EntityState::Landing;
    
    if inputs_for_current_frame != 0 && occupied {
        if inputs.input_buffer.len() < 1 {
            let mut input_to_buffer = inputs_for_current_frame;
            let dashing = inputs_for_current_frame & GameAction::Dash as i32 > 0;
            if !dashing {
                if input_to_buffer & GameAction::Right as i32 > 0 {
                    input_to_buffer ^= GameAction::Right as i32;
                }
                if input_to_buffer & GameAction::Left as i32 > 0 {
                    input_to_buffer ^= GameAction::Left as i32;
                }
                if input_to_buffer & GameAction::Up as i32 > 0 {
                    input_to_buffer ^= GameAction::Up as i32;
                }
                if input_to_buffer & GameAction::Down as i32 > 0 {
                    input_to_buffer ^= GameAction::Down as i32;
                }
            }
            inputs.input_buffer.push_front(input_to_buffer);
            inputs.input_buffer_reset_time.push(0);
        }
        
        inputs.action_history.push_back(inputs_for_current_frame);
        inputs.input_reset_timer.push(0);
        inputs.input_new_frame = 0;
        return;
    }

    apply_input(player, inputs_for_current_frame, character_data, &inputs.action_history, enemies);

    inputs.action_history.push_back(inputs_for_current_frame);
    inputs.input_reset_timer.push(0);
    inputs.input_new_frame = 0;
}

fn apply_input(player: &mut Player, 
    inputs_for_current_frame: i32, 
    character_data: &EntityData, 
    action_history: &VecDeque<i32>,
    enemies: &mut EnemyManager) {
    //println!("run inputs {:?}", GameAction::debug_i32(inputs_for_current_frame));
    let x = if inputs_for_current_frame & GameAction::Right as i32 > 0 {
        1i8
    } else if inputs_for_current_frame & GameAction::Left as i32 > 0 {
        -1i8
    } else {
        0i8
    };

    let y = if inputs_for_current_frame & GameAction::Up as i32 > 0 {
        1i8
    } else if inputs_for_current_frame & GameAction::Down as i32 > 0 {
        -1i8
    } else {
        0i8
    };

    player.controller.set_velocity(Vector2::new(x, y), &mut player.animator);

    if inputs_for_current_frame & GameAction::Jump as i32 > 0 {
        player.jump();
    }

    let n_prev_actions = action_history.len();
    let punch_kick_not_pressed = n_prev_actions == 0 || (n_prev_actions > 0 && action_history[n_prev_actions-1] & GameAction::Punch as i32 == 0 && action_history[n_prev_actions-1] & GameAction::Kick as i32 == 0);
    let punch_kick_simultaneously = inputs_for_current_frame & GameAction::Punch as i32 > 0 && inputs_for_current_frame & GameAction::Kick as i32 > 0;
    let has_currency_to_activate = player.currency >= player.active_item_cost as u32;

    if punch_kick_simultaneously && punch_kick_not_pressed && has_currency_to_activate {
        if let Some(active_item) = &mut player.active_item {
            
            let mut item = active_item.clone();
            
            if item.0(player, enemies, &mut item.1) {
                player.currency = cmp::max(0, player.currency - player.active_item_cost as u32);
            }

            player.active_item = Some(item);
        }
    } else {
        if inputs_for_current_frame & GameAction::Punch as i32 > 0 {
            check_attack_inputs(
                player,
                character_data,
                action_history,
                GameAction::Punch,
                "light_punch".to_string(),
            );
        }
        if inputs_for_current_frame & GameAction::Kick as i32 > 0 {
            check_attack_inputs(
                player,
                character_data,
                action_history,
                GameAction::Kick,
                "light_kick".to_string(),
            );
        }
    }
    
    if inputs_for_current_frame & GameAction::Block as i32 > 0 { player.controller.is_blocking = true } else { player.controller.is_blocking = false }

    if inputs_for_current_frame & GameAction::Dash as i32 > 0 {
        player.controller.set_entity_state(EntityState::Dashing, &mut player.animator);
    }

    if inputs_for_current_frame & GameAction::Slide as i32 > 0 {}
}

fn check_for_dash_inputs(current_actions: i32, last_inputs: &VecDeque<i32>) -> bool {
    let len = last_inputs.len();
    if len >= 2 {

        let repeated_actions = current_actions & last_inputs[len-2];
        let gap_frame_actions = repeated_actions & last_inputs[len-1];
        if repeated_actions > 0 {

            let avoid_dash_combo = !GameAction::is_pressed(last_inputs[len-2], GameAction::Dash);

            let dir_not_pressed = !(GameAction::is_pressed(gap_frame_actions, GameAction::Right) || 
                GameAction::is_pressed(gap_frame_actions, GameAction::Left));

            let dir_pressed = GameAction::is_pressed(repeated_actions, GameAction::Right) || 
            GameAction::is_pressed(repeated_actions, GameAction::Left);
            
            return dir_not_pressed && dir_pressed && avoid_dash_combo;
        }
    }
    return false;
}

fn check_attack_inputs(
    player: &mut Player,
    character_data: &EntityData,
    action_history: &VecDeque<i32>,
    recent_input_as_game_action: GameAction,
    animation_name: String,
) {
    if action_history.len() > 0  {
        if let Some(directional_input) = check_directional_inputs(player, character_data,action_history.back().unwrap() | recent_input_as_game_action as i32) {
            player.attack( character_data, directional_input);
        } else if player.controller.can_attack() {
            let mut combo_id = 0;
            let mut current_combo_length = 0;
            
            if recent_input_as_game_action == GameAction::Punch {
                combo_id = 0;
                current_combo_length = player.character.punch_string_curr;
            }
            if recent_input_as_game_action == GameAction::Kick {
                combo_id = 1;
                current_combo_length = player.character.kick_string_curr;
            }
            if player.controller.is_airborne {
                combo_id += 2;
                if recent_input_as_game_action == GameAction::Punch {
                    current_combo_length = player.character.airborne_punch_string_curr;
                }
                if recent_input_as_game_action == GameAction::Kick {
                    current_combo_length = player.character.airborne_kick_string_curr;
                }
            } 

            if let Some(combo) = character_data.auto_combo_strings.get(&(combo_id)) {
                let curr_combo_length = std::cmp::min(combo.len(), current_combo_length as usize);
                let combo_number = player.controller.combo_counter as usize % curr_combo_length;
                player.attack(character_data, combo[combo_number].to_string());
            }
        }
    } else {
        if !player.controller.is_airborne {
            player.attack(character_data, animation_name);
        } else {
            player.attack(
                character_data, 
                format!("{}_{}", "airborne", animation_name),
            );
        }
    }
}

fn check_directional_inputs(
    player: &mut Player,
    character_data: &EntityData,
    recent_inputs: i32
) -> Option<String> {
    for possible_combo in character_data.directional_variation_anims.iter() {
        let moves = possible_combo.inputs;

        let can_dash_attack = player.controller.can_dash_attack();

        let attack_unlocked = possible_combo.mask & player.character.directional_attacks_mask_curr != 0;
        let is_dashing = !((player.controller.state == EntityState::Dashing) ^ possible_combo.is_dashing);
        let is_airborne = !(player.controller.is_airborne ^ possible_combo.is_airborne);


        if can_dash_attack && attack_unlocked && (is_dashing && is_airborne){
            if  GameAction::is_pressed(recent_inputs,moves.0) &&
                GameAction::is_pressed(recent_inputs,moves.1) 
            {
                println!("directional input {:?}", possible_combo.key);
                return Some(possible_combo.key.to_string());
            }
        }
        
    }
    None
}
