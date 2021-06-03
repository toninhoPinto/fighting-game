use crate::{asset_management::{asset_holders::EntityData, common_assets::CommonAssets, vfx::particle::Particle}, ecs_system::{enemy_components::{Health, Position}, enemy_manager::EnemyManager}, engine_types::animator::Animator, game_logic::{characters::{Attack, player::Player}, movement_controller::MovementController}, rendering::camera::Camera};

use super::{collider_manager::ColliderManager, collision_attack_resolution::{detect_hit, did_sucessfully_block, hit_opponent, hit_particles, opponent_blocked}};

pub(crate) enum HitResult {
    Hit(usize, Attack),
    Block
}
pub fn get_enemy_colliders(player: &mut Player,
    enemy_manager: &mut EnemyManager,
    particles: &mut Vec<Particle>,
    hit_stop: &mut i32, 
    logic_timestep: f64, 
    general_assets: &CommonAssets, 
    player_data: &EntityData,
    camera: &mut Camera) {
    
    let player_collisions = &mut player.collision_manager.collisions_detected; 
    let player_colliders = &player.collision_manager.colliders;
    let player_controller_clone = player.controller.clone();
    let player_controller = &mut player.controller;
    let player_pos = player.position;
    let enemies_names = enemy_manager.character_components.iter()
        .map(|char| {if let Some(char) = char { Some(char.name.clone()) } else {None}  })
        .collect::<Vec<Option<String>>>();
    
    let zip = enemy_manager.
        collider_components.iter_mut().enumerate()
        .zip(enemy_manager.health_components.iter_mut())
        .zip(enemy_manager.positions_components.iter_mut())
        .zip(enemy_manager.movement_controller_components.iter_mut())
        .zip(enemy_manager.animator_components.iter_mut());
    
    
    let mut calculate_hits = 
        zip.filter_map(| (((((i, collider), hp), pos), mov), anim) : 
            (((((usize, &mut Option<ColliderManager>), &mut Option<Health>), &mut Option<Position>), &mut Option<MovementController>), &mut Option<Animator>)| {
            if let Some(hp) = hp {
                if hp.0 > 0 {
                    return Some((i, collider.as_mut()?, hp, pos.as_mut()?, mov.as_mut()?, anim.as_mut()?))
                }
            }
            None
        })
        .filter_map(|(i, colliders, hp, pos, mov, animator)| {
            match detect_hit(player_colliders, &colliders.colliders) {
                Some((point, name)) => {
                    if !player_collisions.contains(&(i as i32)) { 
                        player_collisions.insert(i as i32);
                        player_controller.has_hit = true;
                        return Some((i, point, name, hp, pos, colliders, mov, animator)) 
                    } else {
                        return None
                    } 

                }
                None => {None}
            }
        })
        .map(|(i, point, collider_name, hp, pos, colliders, mov, animator)| {
            let attack = player_data
                        .attacks
                        .get(&collider_name.replace("?", ""))
                        .unwrap().clone();

            if !did_sucessfully_block(point, player_pos, &mov){

                hit_particles(particles, point, "special_hit", &general_assets);

                return HitResult::Hit(i, attack);
            } else {
                opponent_blocked(
                    &attack,
                    logic_timestep,
                    &general_assets, 
                    &player_controller_clone, (&mut pos.0, mov));
                
                hit_particles(particles, point, "block", &general_assets);

                return HitResult::Block;
            }
        }).collect::<Vec<HitResult>>();

    
    
    
    calculate_hits.iter_mut().for_each(|hit_result: &mut HitResult| {
        match hit_result {
            HitResult::Hit(i, attack) => {
                let mut p_on_hits = player.events.on_hit.clone();
                for onhit in p_on_hits.iter_mut() {
                    onhit.0(player, enemy_manager, *i as i32, &mut onhit.1, attack);
                }

                let enemy = (
                    enemy_manager.health_components[*i].as_mut().unwrap(), 
                    &mut enemy_manager.positions_components[*i].as_mut().unwrap().0,
                    enemy_manager.animator_components[*i].as_mut().unwrap(),
                    enemy_manager.movement_controller_components[*i].as_mut().unwrap()
                );

                let colliders = enemy_manager.collider_components[*i].as_mut().unwrap();

                hit_opponent(
                    &attack,
                    logic_timestep,
                    &general_assets, 
                    &player_controller_clone, enemy);

                let enemy = (
                    enemy_manager.health_components[*i].as_mut().unwrap(), 
                    &mut enemy_manager.positions_components[*i].as_mut().unwrap().0,
                    enemy_manager.animator_components[*i].as_mut().unwrap(),
                    enemy_manager.movement_controller_components[*i].as_mut().unwrap()
                );

                if let Some(on_hit) = attack.on_hit {
                    on_hit(&attack, colliders, enemy.3, enemy.2);
                }

                camera.shake();
                *hit_stop = 10;
            }
            HitResult::Block => {

                *hit_stop = 5;
            }
        }
        
    });

}

pub fn enemy_attack_player(player: &mut Player,
    enemy_manager: &mut EnemyManager,
    particles: &mut Vec<Particle>,
    hit_stop: &mut i32, 
    logic_timestep: f64, 
    general_assets: &CommonAssets, 
    player_data: &EntityData) {
    
    let player_colliders = &player.collision_manager.colliders.clone();
    let player_pos = player.position;
    let enemies_names = enemy_manager.character_components.iter()
        .map(|char| {if let Some(char) = char { Some(char.name.clone()) } else {None}  })
        .collect::<Vec<Option<String>>>();
    
    let mut enemies_hit = Vec::new();
    let zip = enemy_manager.
        collider_components.iter_mut().enumerate()
        .zip(enemy_manager.health_components.iter_mut())
        .zip(enemy_manager.positions_components.iter_mut())
        .zip(enemy_manager.movement_controller_components.iter_mut())
        .zip(enemy_manager.animator_components.iter_mut());
    
    zip.
    filter_map(| (((((i, collider), hp), pos), mov), anim) : 
        (((((usize, &mut Option<ColliderManager>), &mut Option<Health>), &mut Option<Position>), &mut Option<MovementController>), &mut Option<Animator>)| {
        if let Some(hp) = hp {
            if hp.0 > 0 {
                return Some((i, collider.as_mut()?, hp, pos.as_mut()?, mov.as_mut()?, anim.as_mut()?))
            }
        }
        None
    })
    .filter_map(|(i, colliders, hp, pos, mov, animator)| {
        match detect_hit(&colliders.colliders, player_colliders) {
            Some((point, name)) => {
                if !colliders.collisions_detected.contains(&(i as i32)) { 
                    colliders.collisions_detected.insert(i as i32);
                    mov.has_hit = true;
                    return Some((i, point, name, hp, pos, colliders, mov, animator)) 
                } else {
                    return None
                } 

            }
            None => {None}
        }
    })
    .for_each(|(i, point, collider_name, hp, pos, colliders, mov, animator)| {
        let attack = player_data
                    .attacks
                    .get(&collider_name.replace("?", ""))
                    .unwrap();
        if !did_sucessfully_block(point, player_pos, &player.controller){
            enemies_hit.push(i as i32);

            let enemy_name = &enemies_names[i].as_ref().unwrap();

            if let Some(on_hit) = attack.on_hit {
                on_hit(attack, colliders, mov, animator);
             }
            hit_opponent(
                attack,
                logic_timestep,
                &general_assets, 
                &mov, (&mut player.hp, &mut player.position, &mut player.animator, &mut player.controller));


            hit_particles(particles, point, "special_hit", &general_assets);
            *hit_stop = 10;
        } else {
            opponent_blocked(
                attack,
                logic_timestep,
                &general_assets, 
                &mov, (&mut player.position, &mut player.controller));
            hit_particles(particles, point, "block", &general_assets);
            *hit_stop = 5;
        }
    });

    let mut p_on_hurts = player.events.on_hurt.clone();
    for &i in enemies_hit.iter() {
        for onhurt in p_on_hurts.iter_mut() {
            onhurt.0(player, enemy_manager, i, &mut onhurt.1);
        }
    }
}
