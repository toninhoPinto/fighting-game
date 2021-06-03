use rand::{Rng, SeedableRng, prelude::SmallRng};

use crate::{ecs_system::{enemy_components::{AIType, Position}, enemy_manager::EnemyManager, enemy_systems::{heal, take_damage_light}}, engine_types::animator::Animator, game_logic::{characters::{Attack, AttackType, player::Player}, effects::{Effect, events_pub_sub::{CharacterEvent, CharacterEventAttack, CharacterEventMap, CharacterEventUpdate}}, movement_controller::MovementController}, scenes::overworld_scene::OverworldScene};

pub fn apply_add_attack_at_level_start(player: &mut Player, effect: &mut Effect){
    player.events.on_start_level.push((add_attack_wrap, effect.clone()));
}

pub fn add_attack_wrap(player: &mut Player, _: &mut EnemyManager, _: i32, effect: &mut Effect){
    add_attack(player, effect);
}

pub fn add_attack(player: &mut Player, effect: &mut Effect) {
    match &effect.add_attack.as_ref().unwrap() as &str {
        "punch" => {player.character.punch_string_curr+=1;},
        "kick" => {player.character.kick_string_curr+=1;},
        "airborne punch" => {player.character.airborne_punch_string_curr+=1;},
        "airborne kick" => {player.character.airborne_kick_string_curr+=1;},
        "launcher" => {player.character.directional_attacks_mask_curr   |=  0b0001u32;},
        "dropper" => {player.character.directional_attacks_mask_curr    |=  0b0010u32;}, 
        "dashing" => {player.character.directional_attacks_mask_curr    |=  0b0100u32;},
        "crash" => {player.character.directional_attacks_mask_curr      |=  0b1000u32;},
        "doublejump" => {player.controller.can_double_jump = true;}
        "airdash" => {player.controller.can_air_dash = true;}
        _ => {},
    }
}

pub fn remove_attack(player: &mut Player, effect: &mut Effect) {
    match &effect.add_attack.as_ref().unwrap() as &str {
        "punch" => {player.character.punch_string_curr-=1;},
        "kick" => {player.character.kick_string_curr-=1;},
        "airborne punch" => {player.character.airborne_punch_string_curr-=1;},
        "airborne kick" => {player.character.airborne_kick_string_curr-=1;},
        "launcher" => {player.character.directional_attacks_mask_curr   ^=  0b0001u32;},
        "dropper" => {player.character.directional_attacks_mask_curr    ^=  0b0010u32;}, 
        "dashing" => {player.character.directional_attacks_mask_curr    ^=  0b0100u32;},
        "crash" => {player.character.directional_attacks_mask_curr      ^=  0b1000u32;},
        "doublejump" => {player.controller.can_double_jump = false;}
        "airdash" => {player.controller.can_air_dash = false;}
        _ => {},
    }
}

pub fn change_stats(player: &mut Player, effect: &mut Effect){
    for stat in effect.stat.as_ref().unwrap().iter() {
        match &stat as &str {
            "max_hp" => {
                player.character.hp += effect.change.unwrap();
                player.hp.0 += effect.change.unwrap();
            },
            "mov_speed" => {},
            "atck_speed" => {},
            "atck_dmg" => {},
            _ => {},
        }
    }
}

pub fn apply_remove_all_extra_attacks_on_hurt(player: &mut Player, effect: &mut Effect){
    player.events.on_hurt.push((remove_all_extra_attacks_wrap, effect.clone()));
}

pub fn remove_all_extra_attacks_wrap(player: &mut Player, _: &mut EnemyManager, _: i32, effect: &mut Effect){
    remove_all_extra_attacks(player, effect);
}

pub fn remove_all_extra_attacks(player: &mut Player, _: &mut Effect) {
    player.character.punch_string_curr = player.character.punch_string;
    player.character.kick_string_curr = player.character.kick_string;
    player.character.airborne_punch_string_curr = player.character.airborne_punch_string;
    player.character.airborne_kick_string_curr = player.character.airborne_kick_string;
    player.character.directional_attacks_mask_curr =  0b0000u32;
}

pub fn remove_all_extra_punches(player: &mut Player, effect: &mut Effect) {
    let multiplier = (player.character.punch_string_curr - 1) as i32 * effect.change.unwrap();
    effect.change = Some(multiplier);
    player.character.punch_string_curr = 1;
    //buff first punch damage, need access to EntityData, maybe put it inside player
    player.events.on_hit.push((buff_punch_damage, effect.clone()));
}

pub fn buff_punch_damage(player: &mut Player, enemies: &mut EnemyManager, enemy_id: i32, effect: &mut Effect, attack: &mut Attack){
    if attack.attack_type == AttackType::Punch {
        attack.damage *= effect.change.unwrap();
    }
}

pub fn heal_on_active(player: &mut Player, effect: &mut Effect){
    player.active_item = Some((heal_player, effect.clone()));
}

pub fn heal_player(player: &mut Player, enemies: &mut EnemyManager, effect: &mut Effect){
    heal(&mut player.hp, effect.change.unwrap(), &player.character);
}

pub fn charm_on_active(player: &mut Player, effect: &mut Effect){
    player.active_item = Some((charm_enemies, effect.clone()));
}

pub fn charm_enemies(player: &mut Player, enemies: &mut EnemyManager, effect: &mut Effect){
    let actual_enemies = enemies.ai_type_components.iter().enumerate().filter(|(usize, ai)| { 
        if let Some(AIType::Enemy) = ai {
            true
        } else {
            false
        }
    }).collect::<Vec<_>>();
    let n_enemies = actual_enemies.len();

    if n_enemies > 0 {
        let mut rng = rand::thread_rng();
        let enemy_to_charm = (rng.gen::<f64>() * n_enemies as f64) as usize;
    
        let actual_id = actual_enemies[enemy_to_charm].0;
        enemies.ai_type_components[actual_id] = Some(AIType::Allied);
    }
}

pub fn apply_lifesteal(player: &mut Player, effect: &mut Effect){
    player.events.on_hit.push((lifesteal, effect.clone()));
}

pub fn lifesteal(player: &mut Player, enemies: &mut EnemyManager, enemy_id: i32, effect: &mut Effect, attack: &mut Attack){
    heal(&mut player.hp, effect.change.unwrap(), &player.character);
}

pub fn apply_life_on_kill(player: &mut Player, effect: &mut Effect){
    player.events.on_kill.push((lifesteal_kill, effect.clone()));
}

pub fn lifesteal_kill(player: &mut Player, enemies: &mut EnemyManager, enemy_id: i32, effect: &mut Effect){
    heal(&mut player.hp, effect.change.unwrap(), &player.character);
}


pub fn apply_anti_grav(player: &mut Player, effect: &mut Effect){
    player.events.on_jump.push((anti_grav as CharacterEvent, effect.clone()));
}

pub fn anti_grav(player: &mut Player, enemies: &mut EnemyManager, _: i32, _: &mut Effect) {
    let player_position = player.position;
    enemies.positions_components.iter()
    .zip(enemies.movement_controller_components.iter_mut())
    .zip(enemies.animator_components.iter_mut())
    .filter_map(|((pos, mov), animator): ((&Option<Position>, &mut Option<MovementController>), &mut Option<Animator>)| {
        Some((pos.as_ref()?, mov.as_mut()?, animator.as_mut()?))
    }).for_each(|(pos, mov, animator): (&Position, &mut MovementController, &mut Animator)| {
        if (player_position - pos.0).magnitude() < 100f64 {
            mov.launch(animator);
             //launch enemies up ? or force them to jump but skip the crouch animation
        }
    });
}

pub fn apply_poison_to_enemies(player: &mut Player, effect: &mut Effect){
    player.events.on_hit.push((apply_poison as CharacterEventAttack, effect.clone()));
}

pub fn apply_poison(_: &mut Player, enemies: &mut EnemyManager, enemy_id: i32, effect: &mut Effect, attack: &mut Attack){
    if let Some(enemy_events) = &mut enemies.events_components[enemy_id as usize] {
        enemy_events.on_update.push((poison as CharacterEventUpdate, effect.clone()));
    }
} 

pub fn poison(_: &mut Player, enemies: &mut EnemyManager, enemy_id: i32, effect: &mut Effect, dt: f64){
    if let (Some(hp), Some(mov)) = 
        (&mut enemies.health_components[enemy_id as usize],
        &mut enemies.movement_controller_components[enemy_id as usize])
    {
        effect.time_elapsed += (dt * 1000f64) as i32;
        if let Some(time_threshold) = effect.apply_at_every {
            if effect.time_elapsed > time_threshold {
                take_damage_light(hp, effect.change.unwrap(), mov);
                effect.time_elapsed = 0;
            }
        }
        
    }
}

pub fn apply_once_in_awhile_forget_or_remenber_attacks(player: &mut Player, effect: &mut Effect) {
    player.events.on_update.push((once_in_awhile_forget_or_remenber_attacks as CharacterEventUpdate, effect.clone()));
}

pub fn once_in_awhile_forget_or_remenber_attacks(player: &mut Player, _: &mut EnemyManager, _: i32, effect: &mut Effect, dt: f64){
    effect.time_elapsed += (dt * 1000f64) as i32;
    if let Some(time_threshold) = effect.apply_at_every {
        if effect.time_elapsed > time_threshold {
            let mut rng = SmallRng::seed_from_u64(96873523456);
            let random_n = rng.gen::<f64>();
            let random_attack = rng.gen::<f64>() * 9f64;
            let possible_attacks= ["punch", "kick", "airborne punch", "airborne kick", "launcher", "dropper", "dashing", "crash", "doublejump", "airdash"];
            
            let mut effect = Effect {
                //handler for function
                effect_id: -1,
            
                //for overtime effects
                duration: None,
                time_elapsed: 0,
                apply_at_every: None,
                change: None,
                stat:  None,
            
                //for adding new animations
                add_attack: Some(possible_attacks[random_attack as usize].to_string()),
            };
            println!("attack {}", random_attack);
            if random_n > 0.5 {
                add_attack(player, &mut effect);
            } else {
                remove_attack(player, &mut effect);
            }
            effect.time_elapsed = 0;
        }
    }
}


pub fn apply_map_exploration(player: &mut Player, effect: &mut Effect){
    player.events.on_overworld_map.push((increase_map_exploration as CharacterEventMap, effect.clone()));
}

pub fn increase_map_exploration(player: &mut Player, map: &mut OverworldScene, effect: &mut Effect)  {
    map.change_exploration_level(true);
    player.events.on_overworld_map.pop(); //TODO, should actually find itself somehow and pop itself only
}

