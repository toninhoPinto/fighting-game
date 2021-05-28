use parry2d::{bounding_volume::BoundingVolume, math::Point, math::Real, na::{Isometry2, Point2, Vector2}, query::{self, Contact}, shape::Cuboid};
use sdl2::{pixels::Color, rect::Rect, render::TextureQuery};

use crate::{asset_management::{asset_holders::{EntityAnimations}, common_assets::CommonAssets, sound::audio_player, vfx::particle::Particle}, ecs_system::enemy_components::{Health, Position}, engine_types::{animator::Animator, collider::{Collider, ColliderType}}, game_logic::{characters::{Attack, player::Player}, game::Game, movement_controller::MovementController}, utils::math_sign::Sign};

use crate::ecs_system::enemy_systems::take_damage;


//TODO, this cant be right, instead of iterating like this, perhaps use a quadtree? i think Parry2d has SimdQuadTree
//TODO probably smartest is to record the hits, and then have a separate function to handle if there is a trade between characters??

pub fn detect_hit(player_hit_colliders: &Vec<Collider>, enemy_hurt_colliders: &Vec<Collider>) -> Option<(Point<Real>, String)>{
    for collider in player_hit_colliders
        .iter()
        .filter(|&c| c.collider_type == ColliderType::Hitbox && c.enabled)
    {
        for collider_to_take_dmg in enemy_hurt_colliders
            .iter()
            .filter(|&c| c.collider_type == ColliderType::Hurtbox && c.enabled)
        {
            if collider.aabb.intersects(&collider_to_take_dmg.aabb) {
                let contact = contact(collider, collider_to_take_dmg);
                return if let Some(contact) = contact {
                    Some((contact.point2, collider.name.clone()))
                } else {
                    None
                };
            }
        }
    }
    None
}
 
fn contact(p1_collider: &Collider, p2_collider: &Collider) -> Option<Contact> {
    let cuboid1 = Cuboid::new(p1_collider.aabb.half_extents());
    let cuboid2 = Cuboid::new(p2_collider.aabb.half_extents());
    let prediction = 1.0;

    let cuboid1_pos = Isometry2::translation(
        p1_collider.aabb.center().x,
        p1_collider.aabb.center().y,
    );
    let cuboid2_pos = Isometry2::translation(
        p2_collider.aabb.center().x,
        p2_collider.aabb.center().y,
    );

    query::contact(&cuboid1_pos, &cuboid1, &cuboid2_pos, &cuboid2, prediction)
        .unwrap()
}

pub fn hit_opponent(
    attack: &Attack, 
    time: f64, 
    general_assets: &CommonAssets, 
    attacker: &MovementController, 
    receiver: (&mut Health, &mut Position, &mut Animator, &mut MovementController)){
    
    audio_player::play_sound(general_assets.sound_effects.get("hit").unwrap());
    take_damage(receiver.0, attack.damage, receiver.3, receiver.2);                                               
    receiver.3.state_update(receiver.2, false);     
    
    let dir_to_push = if attacker.is_airborne {                                            
        attacker.direction_at_jump_time
    } else {
        attacker.facing_dir
    };
    receiver.3.knock_back(receiver.1, attack.push_back * dir_to_push.sign() as f64, time);
}

pub fn opponent_blocked(attack: &Attack, 
    time: f64, 
    general_assets: &CommonAssets, 
    attacker: &MovementController, 
    receiver: (&mut Position, &mut MovementController)){
    
    audio_player::play_sound(general_assets.sound_effects.get("block").unwrap());
    let dir_to_push = if attacker.is_airborne {                          
        attacker.direction_at_jump_time
    } else {
        attacker.facing_dir
    };
    receiver.1.knock_back(receiver.0, attack.push_back * dir_to_push.sign() as f64, time); 
}

pub fn hit_particles(particles: &mut Vec<Particle>, point: Point2<f32>, hit_particle: &str, general_assets: &CommonAssets) {
    let texture_id = &general_assets.hit_effect_animations.get(hit_particle).unwrap().sprites[0].1;
    let TextureQuery { width, height, .. } = general_assets
                            .hit_effect_textures
                            .get(texture_id)
                            .unwrap()
                            .query();

    let texture_width = width * 2;
    let texture_height = height * 2;
    //^ * 2 above is to make the sprite bigger, and the hardcoded - 80 and -100 is because the sprite is not centered
    //this will have issues with other vfx
    Game::spawn_vfx(
        particles,
        Rect::new(
            point.x as i32,
            point.y as i32 - texture_height as i32 / 2,
            texture_width,
            texture_height,
        ),
        false,
        hit_particle.to_string(),
        Some(Color::GREEN),
    );
}


pub fn did_sucessfully_block(point: Point2<f32>, blocking_pos: Vector2<f64>, blocking_controller: &MovementController) -> bool {                   //MovementController
    
    let facing_correct_dir = (point.x > blocking_pos.x as f32 && blocking_controller.facing_dir > 0) || 
    (point.x < blocking_pos.x as f32 && !blocking_controller.facing_dir > 0);

    blocking_controller.is_blocking && facing_correct_dir
}