use std::collections::HashMap;

use sdl2::{rect::{Point, Rect}, render::Texture};

use crate::{asset_management::{asset_holders::{EntityAssets, EntityData}, common_assets::CommonAssets, vfx::particle::Particle}, collision::{collider_manager::ColliderManager, collision_attack_resolution::{detect_hit, did_sucessfully_block, hit_opponent, hit_particles, opponent_blocked}}, engine_types::animator::Animator, game_logic::{characters::{Attack, Character, player::{Player, EntityState}}, effects::{Effect, events_pub_sub::{CharacterEventUpdate, EventsPubSub}}, movement_controller::MovementController}, rendering::camera::Camera};

use super::{enemy_components::{Health, Position, Renderable}, enemy_manager::EnemyManager};


pub fn attack(controller: &mut MovementController, animator: &mut Animator, collision_manager: &mut ColliderManager, attack_animation: String) {
    if controller.can_attack() {
        controller.is_attacking = true;
        controller.combo_counter += 1;

        collision_manager.collisions_detected.clear();
        controller.has_hit = false;

       
        if let Some(attack_anim) =  controller.animations.animations.get(&attack_animation) { 
            animator.play_animation(attack_anim.clone(),1.0, false, true, true);
        }

        if let Some(_) = animator.current_animation.as_ref().unwrap().collider_animation {
            collision_manager.init_colliders(&animator);
        }
    }
}

pub fn take_damage(hp: &mut Health, damage: i32, mov: &mut MovementController, animator: &mut Animator) {
    if hp.0 > 0 {
        hp.0 -= damage;
        mov.set_entity_state(EntityState::Hurt, animator);
    }

    if hp.0 <= 0 {
        mov.set_entity_state(EntityState::Dead, animator);
    }
}

pub fn take_damage_light(hp: &mut Health, damage: i32, mov: &mut MovementController) {
    if hp.0 > 0 {
        hp.0 = std::cmp::max(hp.0 - damage, 1);
    }
}

pub fn heal(hp: &mut Health, heal_amount: i32, char: &Character) {
    hp.0 = std::cmp::min(hp.0 + heal_amount, char.hp);
}

pub fn update_events(enemy_manager: &mut EnemyManager, player: &mut Player, dt: f64) {
    
    let zip = enemy_manager.
        events_components.iter_mut().enumerate()
        .zip(enemy_manager.health_components.iter());

    let mut enemy_events = zip
    .filter_map(|((i, events), health) : ((usize, &mut Option<EventsPubSub>), &Option<Health>)| {
        if let (Some(hp), Some(events)) = (health, events) {
            if hp.0 > 0 {
                return Some((i, events.on_update.clone()))
            }
        }
        None
    }).collect::<Vec<(usize, Vec<(CharacterEventUpdate, Effect)>)>> ();
    
    enemy_events.iter_mut().for_each(|(i, events): &mut (usize,  Vec<(CharacterEventUpdate, Effect)>)| {
        for event in events.iter_mut() {
            event.0(player, enemy_manager, *i as i32, &mut event.1, dt);
        }
    });

    let replace_events = enemy_manager.
        events_components.iter_mut().enumerate()
        .zip(enemy_events);

    replace_events
    .for_each(|((i, events), (to_replace_i, to_replace_events)) : ((usize, &mut Option<EventsPubSub>), (usize,  Vec<(CharacterEventUpdate, Effect)>))| {
        if i == to_replace_i {
            if let Some(events) = events {
                events.on_update = to_replace_events;
            }
        }
    });
}

pub fn update_animations_enemies(enemy_manager: &mut EnemyManager) {
    let zip = enemy_manager.animator_components.iter_mut();
    zip.for_each(|animator| {
        if let Some(animator) = animator {
            animator.update();
        }
    });
}

pub fn update_colliders_enemies(enemy_manager: &mut EnemyManager, enemy_assets: &HashMap<&str, EntityAssets>) {
    let zip = enemy_manager.
        collider_components.iter_mut()
        .zip(enemy_manager.positions_components.iter())
        .zip(enemy_manager.animator_components.iter())
        .zip(enemy_manager.renderable_components.iter())
        .zip(enemy_manager.character_components.iter());

        zip
        .filter_map(| ((((collider, pos), animator), renderable), character): 
            ((((&mut Option<ColliderManager>, &Option<Position>), &Option<Animator>), &Option<Renderable>), &Option<Character>)| {
                Some((collider.as_mut()?, pos.as_ref()?, animator.as_ref()?, renderable.as_ref()?, &character.as_ref()?.name))
        })
        .for_each(|(collider, pos, animator, renderable, name): (&mut ColliderManager, &Position, &Animator, &Renderable, &String) | {
            collider.update_colliders(renderable.flipped, pos.0, animator , &enemy_assets.get(name as &str).unwrap().texture_data);
        });
}

pub fn update_movement_enemies(enemy_manager: &mut EnemyManager, camera: &Camera, dt: f64, general_assets: &CommonAssets) {
    let zip = enemy_manager
    .positions_components.iter_mut()
    .zip(enemy_manager.animator_components.iter_mut())
    .zip(enemy_manager.health_components.iter())
    .zip(enemy_manager.movement_controller_components.iter_mut())
    .zip(enemy_manager.character_components.iter())
    .zip(enemy_manager.renderable_components.iter_mut());

    zip
    .filter_map(| (((((pos, animator), hp), mov), character), renderable): 
        (((((&mut Option<Position>, &mut Option<Animator>), &Option<Health>), &mut Option<MovementController>), &Option<Character>), &mut Option<Renderable>)| {
        return Some((pos.as_mut()?, mov.as_mut()?, animator.as_mut()?, character.as_ref()?, renderable.as_mut()?))
    })
    .for_each(|(pos, mov, animator, character, renderable): (&mut Position, &mut MovementController, &mut Animator, &Character, &mut Renderable)| {
        mov.state_update(animator, false);
        mov.update(
            &mut pos.0,
            character,
            animator,
            camera,
            dt,
            100, //TODO fix this,
            general_assets
        );

        renderable.flipped = mov.facing_dir > 0;
    });
}

pub fn get_ground_pos_enemies(enemy_manager: &EnemyManager) -> Vec<Point> {
    let zip = enemy_manager
        .positions_components
        .iter()
        .zip(enemy_manager.movement_controller_components.iter());

        let ground_pos =
        zip
        .filter_map(|(pos, mov): (&Option<Position>, &Option<MovementController>)| {
            Some((pos.as_ref()?, mov.as_ref()?))
        })
        .map(|(pos, mov): (&Position, &MovementController)| {
            Point::new(pos.0.x as i32, mov.ground_height)
        });

        ground_pos.collect::<Vec<Point>>()
}

pub fn render_enemies<'a>(enemy_manager: &EnemyManager, assets: &'a HashMap<&str, EntityAssets>) -> Vec<(&'a Texture<'a>, Rect, Point, bool, i32)> {
    let zip = enemy_manager
        .animator_components
        .iter()
        .zip(enemy_manager.renderable_components.iter())
        .zip(enemy_manager.positions_components.iter())
        .zip(enemy_manager.character_components.iter())
        .zip(enemy_manager.movement_controller_components.iter());

    let living =
        zip
        .filter_map(|((((animator, renderable), pos), character), mov): ((((&Option<Animator>, &Option<Renderable>), &Option<Position>), &Option<Character>), &Option<MovementController>)| {
            Some((animator.as_ref()?, renderable.as_ref()?, pos.as_ref()?, character.as_ref()?, mov.as_ref()?.ground_height))
        })
        .map(|(animator, renderable, pos, character, render_order): (&Animator, &Renderable, &Position, &Character, i32)| {
            let (tex, rect, offsets) = render_entity(animator.render(), animator, renderable, assets.get(&character.name as &str).unwrap());
            let pos = Point::new((pos.0.x - offsets.0) as i32, (pos.0.y - offsets.1 )as i32);
            (tex, rect, pos, renderable.flipped, render_order)
        });

    living.collect::<Vec<(&'a Texture<'a>, Rect, Point, bool, i32)>>()
}

fn render_entity<'a>(texture_handle: String, animator: &Animator, renderable: &Renderable, assets: &'a EntityAssets<'a>) -> (&'a Texture<'a>, Rect, (f64, f64))  {
    let sprite_data = assets.texture_data.get(&texture_handle);
    
    let mut rect = renderable.rect.clone();
    let mut offset = (0f64, 0f64);

    if let Some(sprite_data) = sprite_data {
        rect.resize(sprite_data.width * 2 , sprite_data.height * 2 );

        let pivot_x_offset = if renderable.flipped {(1f64 - sprite_data.pivot_x)* 2.0 * sprite_data.width as f64} else {sprite_data.pivot_x * 2.0 * sprite_data.width as f64};
        let pivot_y_offset = sprite_data.pivot_y * 2.0 * sprite_data.height as f64;

        offset = if let Some(sprite_alignment) = animator.current_animation.as_ref().unwrap().sprite_alignments.get(&animator.sprite_shown) {
            let facing_dir = if renderable.flipped {1} else {-1};
            (pivot_x_offset + facing_dir as f64 * sprite_alignment.pos.x * 2.0, pivot_y_offset + sprite_alignment.pos.y * 2.0)
        } else {
            (pivot_x_offset, pivot_y_offset)
        };

    }
    (assets.textures.get(&texture_handle).unwrap(), rect, offset)
}