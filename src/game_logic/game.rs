use sdl2::{pixels::Color, rect::{Point, Rect}};

use crate::asset_management::{collider::Collider, common_assets::CommonAssets, vfx::particle::Particle};

use super::{character_factory::CharacterAssets, characters::{Character, player::{Player, PlayerState}}, projectile::Projectile};

const LIMIT_NUMBER_OF_VFX: usize = 5;
pub struct Game<'a>{
    pub current_frame: i32,
    pub player1: &'a mut Player<'a>,
    pub player2: &'a mut Player<'a>,

    pub projectiles: Vec<Projectile>,

    pub p1_colliders: Vec<Collider>,
    pub p2_colliders: Vec<Collider>,

    //TODO probably smart to make this a separate struct instead of a weird tuple
    pub hit_vfx: Vec<Particle>,
}

impl<'a> Game<'a>{
    pub fn new(player1: &'a mut Player<'a>, player2: &'a mut Player<'a>) -> Self{
        Self{
            current_frame: 0,

            player1,
            player2,

            projectiles: Vec::new(),

            p1_colliders: Vec::new(),
            p2_colliders: Vec::new(),

            hit_vfx: Vec::new(),
        }
    }

    pub fn spawn_vfx(&mut self, rect: Rect, type_of_animation: String, tint: Option<Color>){
        if self.hit_vfx.len() < LIMIT_NUMBER_OF_VFX {
            //push with bool as true
            self.hit_vfx.push( Particle {
                active: true,
                sprite: rect,
                name: type_of_animation,
                animation_index: 0,
                tint,
             } );
        } else {
            let mut disabled_index = None;
            for i in 0..self.hit_vfx.len() {
                if !self.hit_vfx[i].active {
                    disabled_index = Some(i);
                    break;
                }
            }
            if disabled_index.is_some() {
                self.hit_vfx[disabled_index.unwrap()].active = true;
                self.hit_vfx[disabled_index.unwrap()].sprite = rect;
                self.hit_vfx[disabled_index.unwrap()].name = type_of_animation;
                self.hit_vfx[disabled_index.unwrap()].animation_index = 0;
                self.hit_vfx[disabled_index.unwrap()].tint = tint;
            }
            
        }
    }

    pub fn update_vfx(&mut self, assets: &CommonAssets){
        for i in 0..self.hit_vfx.len() {
            if self.hit_vfx[i].active {
                self.hit_vfx[i].animation_index += 1; // multiply by dt and by animation speed i think, check animator code
                if self.hit_vfx[i].animation_index >= assets.hit_effect_animations.get(&self.hit_vfx[i].name).unwrap().length {
                    self.hit_vfx[i].active = false;
                    self.hit_vfx[i].animation_index = 0;
                }
            }
        }
    }

    //it is maybe better if serialize into some binary format for compression 
    pub fn save(&self) -> SavedGame{
        SavedGame{
            p1_id: self.player1.id,
            p1_position: self.player1.position,
            p1_ground_height: self.player1.ground_height,
            p1_velocity_y: self.player1.velocity_y,
            p1_direction_at_jump_time: self.player1.direction_at_jump_time,
            p1_jump_initial_velocity: self.player1.jump_initial_velocity,
            p1_extra_gravity: self.player1.extra_gravity,
            p1_prev_velocity_x: self.player1.prev_velocity_x,
            p1_velocity_x: self.player1.velocity_x,
            p1_dir_related_of_other: self.player1.dir_related_of_other,
            p1_state: self.player1.state,
            p1_is_attacking: self.player1.is_attacking,
            p1_is_airborne: self.player1.is_airborne,
            p1_flipped: self.player1.flipped,
            p1_has_hit: self.player1.has_hit,
            p1_mid_jump_pos: self.player1.mid_jump_pos,
            p1_animation_index: self.player1.animator.animation_index,
            p1_is_playing: self.player1.animator.is_playing,
            p1_is_finished: self.player1.animator.is_finished,
            p1_play_once: self.player1.animator.play_once,
            p1_rewind: self.player1.animator.rewind,
            p1_name: self.player1.animator.current_animation.unwrap().name.clone(),
            p1_speed: self.player1.animator.current_animation.unwrap().speed,
            p1_length: self.player1.animator.current_animation.unwrap().length,
            p1_character: self.player1.character.clone(),
            
            p2_id: self.player2.id,
            p2_position: self.player2.position,
            p2_ground_height: self.player2.ground_height,
            p2_velocity_y: self.player2.velocity_y,
            p2_direction_at_jump_time: self.player2.direction_at_jump_time,
            p2_jump_initial_velocity: self.player2.jump_initial_velocity,
            p2_extra_gravity: self.player2.extra_gravity,
            p2_prev_velocity_x: self.player2.prev_velocity_x,
            p2_velocity_x: self.player2.velocity_x,
            p2_dir_related_of_other: self.player2.dir_related_of_other,
            p2_state: self.player2.state,
            p2_is_attacking: self.player2.is_attacking,
            p2_is_airborne: self.player2.is_airborne,
            p2_flipped: self.player2.flipped,
            p2_has_hit: self.player2.has_hit,
            p2_mid_jump_pos: self.player2.mid_jump_pos,
            p2_animation_index: self.player2.animator.animation_index,
            p2_is_playing: self.player2.animator.is_playing,
            p2_is_finished: self.player2.animator.is_finished,
            p2_play_once: self.player2.animator.play_once,
            p2_rewind: self.player2.animator.rewind,
            p2_name: self.player2.animator.current_animation.unwrap().name.clone(),
            p2_speed: self.player2.animator.current_animation.unwrap().speed,
            p2_length: self.player2.animator.current_animation.unwrap().length,
            p2_character: self.player2.character.clone(),

            projectiles: self.projectiles.clone(),
            p1_colliders: self.p1_colliders.clone(),
            p2_colliders: self.p2_colliders.clone(),

            hit_vfx: self.hit_vfx.clone(),
        }
    }

    pub fn load(&mut self, saved_game: &SavedGame, p1_assets: &'a CharacterAssets, p2_assets: &'a CharacterAssets) {
        self.player1.id = saved_game.p1_id;
        self.player1.position = saved_game.p1_position;
        self.player1.ground_height = saved_game.p1_ground_height;
        self.player1.velocity_y = saved_game.p1_velocity_y;
        self.player1.direction_at_jump_time = saved_game.p1_direction_at_jump_time;
        self.player1.jump_initial_velocity = saved_game.p1_jump_initial_velocity;
        self.player1.extra_gravity = saved_game.p1_extra_gravity;
        self.player1.prev_velocity_x = saved_game.p1_prev_velocity_x;
        self.player1.velocity_x = saved_game.p1_velocity_x;
        self.player1.dir_related_of_other = saved_game.p1_dir_related_of_other;
        self.player1.state = saved_game.p1_state;
        self.player1.is_attacking = saved_game.p1_is_attacking;
        self.player1.is_airborne = saved_game.p1_is_airborne;
        self.player1.flipped = saved_game.p1_flipped; 
        self.player1.has_hit = saved_game.p1_has_hit; 
        self.player1.mid_jump_pos = saved_game.p1_mid_jump_pos; 
        self.player1.animator.animation_index = saved_game.p1_animation_index; 
        self.player1.animator.is_playing = saved_game.p1_is_playing; 
        self.player1.animator.is_finished = saved_game.p1_is_finished; 
        self.player1.animator.play_once = saved_game.p1_play_once; 
        self.player1.animator.rewind = saved_game.p1_rewind; 
        self.player1.animator.current_animation = p1_assets.animations.get(&saved_game.p1_name);
        self.player1.character = saved_game.p1_character.clone();
        
        self.player2.id = saved_game.p2_id; 
        self.player2.position = saved_game.p2_position; 
        self.player2.ground_height = saved_game.p2_ground_height; 
        self.player2.velocity_y = saved_game.p2_velocity_y; 
        self.player2.direction_at_jump_time = saved_game.p2_direction_at_jump_time; 
        self.player2.jump_initial_velocity = saved_game.p2_jump_initial_velocity; 
        self.player2.extra_gravity = saved_game.p2_extra_gravity; 
        self.player2.prev_velocity_x = saved_game.p2_prev_velocity_x; 
        self.player2.velocity_x = saved_game.p2_velocity_x;
        self.player2.dir_related_of_other = saved_game.p2_dir_related_of_other; 
        self.player2.state = saved_game.p2_state; 
        self.player2.is_attacking = saved_game.p2_is_attacking; 
        self.player2.is_airborne = saved_game.p2_is_airborne; 
        self.player2.flipped = saved_game.p2_flipped; 
        self.player2.has_hit = saved_game.p2_has_hit; 
        self.player2.mid_jump_pos = saved_game.p2_mid_jump_pos; 
        self.player2.animator.animation_index = saved_game.p2_animation_index; 
        self.player2.animator.is_playing = saved_game.p2_is_playing; 
        self.player2.animator.is_finished = saved_game.p2_is_finished; 
        self.player2.animator.play_once = saved_game.p2_play_once; 
        self.player2.animator.rewind = saved_game.p2_rewind; 
        self.player2.animator.current_animation = p2_assets.animations.get(&saved_game.p2_name);
        self.player2.character = saved_game.p2_character.clone(); 

        self.projectiles = saved_game.projectiles.clone(); 
        self.p1_colliders = saved_game.p1_colliders.clone(); 
        self.p2_colliders = saved_game.p2_colliders.clone(); 

        self.hit_vfx = saved_game.hit_vfx.clone();
    }
}


pub struct SavedGame{
    pub p1_id: i32,
    pub p1_position: Point,
    pub p1_ground_height: i32,
    pub p1_velocity_y: f64,
    pub p1_direction_at_jump_time: i32,
    pub p1_jump_initial_velocity: f64,
    pub p1_extra_gravity: Option<f64>,
    pub p1_prev_velocity_x: i32,
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
    pub p2_position: Point,
    pub p2_ground_height: i32,
    pub p2_velocity_y: f64,
    pub p2_direction_at_jump_time: i32,
    pub p2_jump_initial_velocity: f64,
    pub p2_extra_gravity: Option<f64>,
    pub p2_prev_velocity_x: i32,
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