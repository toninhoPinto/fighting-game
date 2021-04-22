use parry2d::na::Vector2;
use sdl2::rect::Point;

use crate::asset_management::{collider::Collider, vfx::particle::Particle};

use super::{
    character_factory::CharacterAssets,
    characters::{player::PlayerState, Character},
    game::Game,
    projectile::Projectile,
};

pub struct SavedGame {
    pub p1_id: i32,
    pub p1_position: Vector2<f64>,
    pub p1_ground_height: i32,
    pub p1_velocity_y: f64,
    pub p1_direction_at_jump_time: i32,
    pub p1_jump_initial_velocity: f64,
    pub p1_extra_gravity: Option<f64>,
    pub p1_velocity_x: i32,
    pub p1_dir_related_of_other: i32,
    pub p1_state: PlayerState,
    pub p1_is_attacking: bool,
    pub p1_is_airborne: bool,
    pub p1_flipped: bool,
    pub p1_has_hit: bool,
    pub p1_mid_jump_pos: f64,

    pub p1_animation_index: f64,
    pub p1_is_playing: bool,
    pub p1_is_finished: bool,
    p1_play_once: bool,
    p1_rewind: bool,

    pub p1_name: String,
    pub p1_speed: f64,
    pub p1_length: i32,

    pub p1_character: Character,

    pub p2_id: i32,
    pub p2_position: Vector2<f64>,
    pub p2_ground_height: i32,
    pub p2_velocity_y: f64,
    pub p2_direction_at_jump_time: i32,
    pub p2_jump_initial_velocity: f64,
    pub p2_extra_gravity: Option<f64>,
    pub p2_velocity_x: i32,
    pub p2_dir_related_of_other: i32,
    pub p2_state: PlayerState,
    pub p2_is_attacking: bool,
    pub p2_is_airborne: bool,
    pub p2_flipped: bool,
    pub p2_has_hit: bool,
    pub p2_mid_jump_pos: f64,

    pub p2_animation_index: f64,
    pub p2_is_playing: bool,
    pub p2_is_finished: bool,
    p2_play_once: bool,
    p2_rewind: bool,

    pub p2_name: String,
    pub p2_speed: f64,
    pub p2_length: i32,

    pub p2_character: Character,

    pub projectiles: Vec<Projectile>,

    pub p1_colliders: Vec<Collider>,
    pub p2_colliders: Vec<Collider>,

    pub hit_vfx: Vec<Particle>,
}

impl SavedGame {
    //it is maybe better if serialize into some binary format for compression
    pub fn save(game: &Game) -> Self {
        Self {
            p1_id: game.player1.id,
            p1_position: game.player1.position,
            p1_ground_height: game.player1.ground_height,
            p1_velocity_y: game.player1.velocity_y,
            p1_direction_at_jump_time: game.player1.direction_at_jump_time,
            p1_jump_initial_velocity: game.player1.jump_initial_velocity,
            p1_extra_gravity: game.player1.extra_gravity,
            p1_velocity_x: game.player1.velocity_x,
            p1_dir_related_of_other: game.player1.dir_related_of_other,
            p1_state: game.player1.state,
            p1_is_attacking: game.player1.is_attacking,
            p1_is_airborne: game.player1.is_airborne,
            p1_flipped: game.player1.flipped,
            p1_has_hit: game.player1.has_hit,
            p1_mid_jump_pos: game.player1.mid_jump_pos,
            p1_animation_index: game.player1.animator.animation_index,
            p1_is_playing: game.player1.animator.is_playing,
            p1_is_finished: game.player1.animator.is_finished,
            p1_play_once: game.player1.animator.play_once,
            p1_rewind: game.player1.animator.rewind,
            p1_name: game
                .player1
                .animator
                .current_animation
                .unwrap()
                .name
                .clone(),
            p1_speed: game.player1.animator.speed,
            p1_length: game.player1.animator.current_animation.unwrap().length,
            p1_character: game.player1.character.clone(),

            p2_id: game.player2.id,
            p2_position: game.player2.position,
            p2_ground_height: game.player2.ground_height,
            p2_velocity_y: game.player2.velocity_y,
            p2_direction_at_jump_time: game.player2.direction_at_jump_time,
            p2_jump_initial_velocity: game.player2.jump_initial_velocity,
            p2_extra_gravity: game.player2.extra_gravity,
            p2_velocity_x: game.player2.velocity_x,
            p2_dir_related_of_other: game.player2.dir_related_of_other,
            p2_state: game.player2.state,
            p2_is_attacking: game.player2.is_attacking,
            p2_is_airborne: game.player2.is_airborne,
            p2_flipped: game.player2.flipped,
            p2_has_hit: game.player2.has_hit,
            p2_mid_jump_pos: game.player2.mid_jump_pos,
            p2_animation_index: game.player2.animator.animation_index,
            p2_is_playing: game.player2.animator.is_playing,
            p2_is_finished: game.player2.animator.is_finished,
            p2_play_once: game.player2.animator.play_once,
            p2_rewind: game.player2.animator.rewind,
            p2_name: game
                .player2
                .animator
                .current_animation
                .unwrap()
                .name
                .clone(),
            p2_speed: game.player2.animator.speed,
            p2_length: game.player2.animator.current_animation.unwrap().length,
            p2_character: game.player2.character.clone(),

            projectiles: game.projectiles.clone(),
            p1_colliders: game.player1.colliders.clone(),
            p2_colliders: game.player2.colliders.clone(),

            hit_vfx: game.hit_vfx.clone(),
        }
    }

    pub fn load<'a>(
        &self,
        game: &mut Game<'a>,
        p1_assets: &'a CharacterAssets,
        p2_assets: &'a CharacterAssets,
    ) {
        game.player1.id = self.p1_id;
        game.player1.position = self.p1_position;
        game.player1.ground_height = self.p1_ground_height;
        game.player1.velocity_y = self.p1_velocity_y;
        game.player1.direction_at_jump_time = self.p1_direction_at_jump_time;
        game.player1.jump_initial_velocity = self.p1_jump_initial_velocity;
        game.player1.extra_gravity = self.p1_extra_gravity;
        game.player1.velocity_x = self.p1_velocity_x;
        game.player1.dir_related_of_other = self.p1_dir_related_of_other;
        game.player1.state = self.p1_state;
        game.player1.is_attacking = self.p1_is_attacking;
        game.player1.is_airborne = self.p1_is_airborne;
        game.player1.flipped = self.p1_flipped;
        game.player1.has_hit = self.p1_has_hit;
        game.player1.mid_jump_pos = self.p1_mid_jump_pos;
        game.player1.animator.animation_index = self.p1_animation_index;
        game.player1.animator.is_playing = self.p1_is_playing;
        game.player1.animator.is_finished = self.p1_is_finished;
        game.player1.animator.play_once = self.p1_play_once;
        game.player1.animator.rewind = self.p1_rewind;
        game.player1.animator.current_animation = p1_assets.animations.get(&self.p1_name);
        game.player1.character = self.p1_character.clone();

        game.player2.id = self.p2_id;
        game.player2.position = self.p2_position;
        game.player2.ground_height = self.p2_ground_height;
        game.player2.velocity_y = self.p2_velocity_y;
        game.player2.direction_at_jump_time = self.p2_direction_at_jump_time;
        game.player2.jump_initial_velocity = self.p2_jump_initial_velocity;
        game.player2.extra_gravity = self.p2_extra_gravity;
        game.player2.velocity_x = self.p2_velocity_x;
        game.player2.dir_related_of_other = self.p2_dir_related_of_other;
        game.player2.state = self.p2_state;
        game.player2.is_attacking = self.p2_is_attacking;
        game.player2.is_airborne = self.p2_is_airborne;
        game.player2.flipped = self.p2_flipped;
        game.player2.has_hit = self.p2_has_hit;
        game.player2.mid_jump_pos = self.p2_mid_jump_pos;
        game.player2.animator.animation_index = self.p2_animation_index;
        game.player2.animator.is_playing = self.p2_is_playing;
        game.player2.animator.is_finished = self.p2_is_finished;
        game.player2.animator.play_once = self.p2_play_once;
        game.player2.animator.rewind = self.p2_rewind;
        game.player2.animator.current_animation = p2_assets.animations.get(&self.p2_name);
        game.player2.character = self.p2_character.clone();


        game.player1.colliders = self.p1_colliders.clone();
        game.player2.colliders = self.p2_colliders.clone();

        game.projectiles = self.projectiles.clone();


        game.hit_vfx = self.hit_vfx.clone();
    }
}
