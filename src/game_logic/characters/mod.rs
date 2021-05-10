use sdl2::rect::Rect;
use std::string::String;

use super::{character_factory::CharacterAnimations, game::Game};

pub mod foxgirl;
pub mod player;

pub(crate) type Ability = fn(&mut Game, i32, &CharacterAnimations) -> ();

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
    pub jump_height: f64,
    pub jump_distance: f64,
}


#[derive(Debug, PartialEq)]
pub enum AttackType {
    Normal,
    Special,
    Ultra,
}
#[derive(Debug)]
pub struct Attack {
    pub damage: i32,
    pub stun_on_hit: i32,
    pub stun_on_block: i32,
    pub push_back: f64,
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
        jump_height: f64,
        jump_distance: f64,
    ) -> Self {
        Self {
            name,
            sprite: Rect::new(0, 0, height, width),
            speed,
            dash_speed,
            hp,
            special_max,
            special_curr: special_max as f32,

            jump_height,
            jump_distance,
        }
    }
}
