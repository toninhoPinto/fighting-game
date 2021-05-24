use crate::{ecs_system::{enemy_manager::EnemyManager, enemy_systems::{heal, take_damage_light}}, game_logic::{characters::player::Player, effects::{Effect, events_pub_sub::{CharacterEvent, CharacterEventUpdate}}}};

pub fn add_attack(player: &mut Player, effect: &mut Effect) {
    match &effect.add_attack.as_ref().unwrap() as &str {
        "punch" => {player.character.punch_string_curr+=1;},
        "kick" => {player.character.kick_string_curr+=1;},
        "airborne punch" => {player.character.airborne_punch_string_curr+=1;},
        "airborne kick" => {player.character.airborne_kick_string_curr+=1;},
        "launcher" => {player.character.directional_attacks_mask =  0b0001u32;},
        "dropper" => {player.character.directional_attacks_mask =   0b0010u32;}, 
        "dashing" => {player.character.directional_attacks_mask =   0b0100u32;},
        "crash" => {player.character.directional_attacks_mask =     0b1000u32;},
        _ => {},
    }
}

pub fn apply_lifesteal(player: &mut Player, effect: &mut Effect){
    player.events.on_hit.push((lifesteal, effect.clone()));
}

pub fn lifesteal(player: &mut Player, enemies: &mut EnemyManager, enemy_id: i32, effect: &mut Effect){
    heal(&mut player.hp, effect.change.unwrap(), &player.character);
}

pub fn apply_poison_to_enemies(player: &mut Player, effect: &mut Effect){
    player.events.on_hit.push((apply_poison as CharacterEvent, effect.clone()));
}

pub fn apply_poison(player: &mut Player, enemies: &mut EnemyManager, enemy_id: i32, effect: &mut Effect){
    if let Some(enemy_events) = &mut enemies.events_components[enemy_id as usize] {
        enemy_events.on_update.push((poison as CharacterEventUpdate, effect.clone()));
    }
} 

pub fn poison(player: &mut Player, enemies: &mut EnemyManager, enemy_id: i32, effect: &mut Effect, dt: f64){
    if let (Some(hp), Some(mov), Some(animator)) = 
        (&mut enemies.health_components[enemy_id as usize],
        &mut enemies.movement_controller_components[enemy_id as usize],
        &mut enemies.animator_components[enemy_id as usize])
            {
                effect.time_elapsed += (dt * 1000f64) as i32;
                if let Some(time_threshold) = effect.apply_at_every {
                    if effect.time_elapsed % time_threshold == 0 {
                        take_damage_light(hp, effect.change.unwrap(), mov);
                    }
                }
                
    }
}