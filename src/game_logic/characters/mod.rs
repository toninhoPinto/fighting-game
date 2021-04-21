use sdl2::rect::Rect;
use std::string::String;

use super::{character_factory::CharacterAssets, game::Game};

pub mod foxgirl;
pub mod keetar;
pub mod player;

pub(crate) type Ability = fn(&mut Game, i32, &CharacterAssets) -> ();

#[derive(Debug, Clone)]
pub struct Character {
    //visual
    pub sprite: Rect,

    //stats
    pub name: String,
    pub hp: i32,
    pub special_max: i32,
    pub special_curr: f32,
    pub speed: f64,
    pub dash_speed: f64,
    pub dash_back_speed: f64,
    pub jump_height: f64,
    pub jump_distance: f64,
}

pub enum AttackHeight {
    LOW,
    MIDDLE,
    HIGH,
}

pub enum AttackType {
    Normal,
    Special,
    Ultra,
}

pub struct Attack {
    pub damage: i32,
    pub stun_on_hit: i32,
    pub stun_on_block: i32,
    pub push_back: i32,
    pub attack_move: i32,
    pub attack_height: AttackHeight,
    pub attack_type: AttackType,
}

impl Character {
    pub fn new(
        name: String,
        height: u32,
        width: u32,
        hp: i32,
        special_max: i32,
        speed: f64,
        dash_speed: f64,
        dash_back_speed: f64,
        jump_height: f64,
        jump_distance: f64,
    ) -> Self {
        Self {
            name,
            sprite: Rect::new(0, 0, height, width),
            speed,
            dash_speed,
            dash_back_speed,
            hp,
            special_max,
            special_curr: special_max as f32,

            jump_height,
            jump_distance,
        }
    }
}
