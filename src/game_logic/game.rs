use sdl2::rect::Point;

use crate::asset_management::collider::Collider;

use super::{character_factory::CharacterAssets, characters::{Character, player::{Player, PlayerState}}, projectile::Projectile};

#[derive(Serialize, Deserialize)]
pub struct Game<'a>{
    pub current_frame: i32,
    #[serde(borrow)]
    pub player1: Player<'a>,
    #[serde(borrow)]
    pub player2: Player<'a>,

    pub projectiles: Vec<Projectile>,

    pub p1_colliders: Vec<Collider>,
    pub p2_colliders: Vec<Collider>,
}

impl<'a> Game<'a>{
    pub fn new(player1: Player<'a>, player2: Player<'a>) -> Self{
        Self{
            current_frame: 0,

            player1,
            player2,

            projectiles: Vec::new(),

            p1_colliders: Vec::new(),
            p2_colliders: Vec::new(),
        }
    }

    //it is maybe better if serialize into some binary format for compression 
    pub fn save(&self) -> Vec<u8>{
        bincode::serialize(self).unwrap()
    }

    pub fn load(saved_game: &'a Vec<u8>, p1_assets: &'a CharacterAssets, p2_assets: &'a CharacterAssets) -> Game<'a> {
        let mut decoded: Game = bincode::deserialize(&saved_game[..]).unwrap();
        decoded.player1.animator.current_animation = p1_assets.animations.get(&decoded.player1.animator.current_animation_name);
        decoded.player2.animator.current_animation = p2_assets.animations.get(&decoded.player2.animator.current_animation_name);
        decoded
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
}