use crate::{asset_management::{asset_holders::EntityData, common_assets::CommonAssets, vfx::particle::Particle}, ecs_system::{enemy_components::{AIType, Health, Position}, enemy_manager::EnemyManager}, engine_types::animator::Animator, game_logic::{characters::{Attack, player::Player}, movement_controller::MovementController}, rendering::camera::Camera};

use super::{collider_manager::ColliderManager, collision_attack_resolution::{detect_hit, did_sucessfully_block, hit_opponent, hit_particles, opponent_blocked}};

pub(crate) enum HitResult {
    Hit(usize, Attack),
    Block
}

// zip enumerate + hp + colliders + AIType -> check if hp > 0
// for collider A if AIType == Enemy
// for collider B if AIType == Allied + Player
// match detect_hit(&colliders.colliders, player_colliders)
// collect vector of -> ids of entities that collided, collider name to get attack damage

pub fn calculate_hits(player: &mut Player,
    enemy_manager: &mut EnemyManager,
    particles: &mut Vec<Particle>,
    hit_stop: &mut i32, 
    logic_timestep: f64, 
    general_assets: &CommonAssets, 
    player_data: &EntityData,
    camera: &mut Camera) {

    let n_entities = enemy_manager.collider_components.len();
    let zip = enemy_manager.health_components.iter().enumerate()
    .zip(enemy_manager.collider_components.iter_mut())
    .zip(enemy_manager.ai_type_components.iter());

    let mut entities = zip.filter_map(| (((i, hp), collider), ai) : 
        (((usize, &Option<Health>), &mut Option<ColliderManager>), &Option<AIType>)| {
        if let Some(hp) = hp {
            if hp.0 > 0 {
                return Some((i, collider.as_mut()?, ai.as_ref()?));
            }
        }
        None
    }).collect::<Vec<(usize, &mut ColliderManager, &AIType)>>();

    entities.push((n_entities, &mut player.collision_manager, &AIType::Allied));

    let mut collisions = Vec::new();
    for entity_hitting in entities.iter() {
        for entity_being_hit in entities.iter() {
            let same_entity = entity_hitting.0 == entity_being_hit.0;
            let same_team = entity_hitting.2 == entity_being_hit.2;
            if same_team || same_entity {
                continue;
            }
            match detect_hit(&entity_hitting.1.colliders, &entity_being_hit.1.colliders) {
                Some((point, name)) => {
                    collisions.push((entity_hitting.0, entity_being_hit.0 , point, name));
                }
                _ => {}
            }
        }
    }

    let mut enemies_hit = Vec::new();
    for collision in collisions.iter() {
        let is_player_hitting = collision.0 == enemy_manager.collider_components.len();
        let is_player_hurting = collision.1 == enemy_manager.collider_components.len();

        let mut hitting_colliders = if !is_player_hitting {
            enemy_manager.collider_components[collision.0].clone().unwrap()
        } else {
            player.collision_manager.clone()
        };

        let mut hitting_mov = if !is_player_hitting {
            enemy_manager.movement_controller_components[collision.0].clone().unwrap()
        } else {
            player.controller.clone()
        };

        let mut hurting_mov = if !is_player_hurting {
            enemy_manager.movement_controller_components[collision.1].clone().unwrap()
        } else {
            player.controller.clone()
        };

        let mut hitting_animator = if !is_player_hitting {
            enemy_manager.animator_components[collision.0].clone().unwrap()
        } else {
            player.animator.clone()
        };

        let mut hurting_animator = if !is_player_hurting {
            enemy_manager.animator_components[collision.1].clone().unwrap()
        } else {
            player.animator.clone()
        };

        let mut hurt_pos = if !is_player_hurting {
            enemy_manager.positions_components[collision.1].as_mut().unwrap().0
        } else {
            player.position
        };

        let mut hurt_hp = if !is_player_hurting {
            enemy_manager.health_components[collision.1].clone().unwrap()
        } else {
            player.hp.clone()
        };

        if !hitting_colliders.collisions_detected.contains(&(collision.1 as i32)) { 
            hitting_colliders.collisions_detected.insert(collision.1 as i32);
            hitting_mov.has_hit = true;

            let mut attack = player_data
                .attacks
                .get(&collision.3.replace("?", ""))
                .unwrap().clone();
            if !did_sucessfully_block(collision.2, hurt_pos, &mut hurting_mov){
                
                enemies_hit.push(collision.1 as i32);

                if is_player_hitting {
                    let mut p_on_hits = player.events.on_hit.clone();
                    for onhit in p_on_hits.iter_mut() {
                        onhit.0(player, enemy_manager, collision.1 as i32, &mut onhit.1, &mut attack);
                    }
                }

                if let Some(on_hit) = attack.on_hit {
                    on_hit(&attack, &mut hitting_colliders, &mut hitting_mov, &mut hitting_animator);
                }
                hit_opponent(
                    &attack,
                    logic_timestep,
                    &general_assets, 
                    &hitting_mov, (&mut hurt_hp, &mut hurt_pos, &mut hurting_animator, &mut hurting_mov));

                if is_player_hitting {
                    camera.shake();
                }
                hit_particles(particles, collision.2, "special_hit", &general_assets);
                *hit_stop = 10;
            } else {
                opponent_blocked(
                    &attack,
                    logic_timestep,
                    &general_assets, 
                    &hitting_mov, (&mut hurt_pos, &mut hurting_mov));
                hit_particles(particles, collision.2, "block", &general_assets);
                *hit_stop = 5;
            }

            //re-save clonned components
            {
                if !is_player_hitting {
                    enemy_manager.collider_components[collision.0] = Some(hitting_colliders)
                } else {
                    player.collision_manager = hitting_colliders;
                };

                if !is_player_hitting {
                    enemy_manager.movement_controller_components[collision.0] = Some(hitting_mov);
                } else {
                    player.controller = hitting_mov;
                }
        
                if !is_player_hurting {
                    enemy_manager.movement_controller_components[collision.1] = Some(hurting_mov)
                } else {
                    player.controller = hurting_mov;
                };

                if !is_player_hitting {
                    enemy_manager.animator_components[collision.0] = Some(hitting_animator);
                } else {
                    player.animator = hitting_animator;
                }
        
                if !is_player_hurting {
                    enemy_manager.animator_components[collision.1] = Some(hurting_animator)
                } else {
                    player.animator = hurting_animator;
                }

                if !is_player_hurting {
                    enemy_manager.positions_components[collision.1].as_mut().unwrap().0 = hurt_pos;
                } else {
                    player.position = hurt_pos;
                };
        
                if !is_player_hurting {
                    enemy_manager.health_components[collision.1] = Some(hurt_hp);
                } else {
                    player.hp = hurt_hp;
                };
            }

        }
    }

    let mut p_on_hurts = player.events.on_hurt.clone();
    for &i in enemies_hit.iter() {
        for onhurt in p_on_hurts.iter_mut() {
            onhurt.0(player, enemy_manager, i, &mut onhurt.1);
        }
    }


}
