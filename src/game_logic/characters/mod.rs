use sdl2::rect::Rect;
use std::string::String;

use crate::{asset_management::asset_holders::EntityAnimations, collision::collider_manager::ColliderManager, engine_types::animator::Animator};

use super::{game::Game, movement_controller::MovementController};

pub mod foxgirl;
pub mod player;

pub(crate) type Ability = fn(&mut Game, i32, &EntityAnimations) -> ();
pub(crate) type OnHit = fn(&Attack, &mut ColliderManager, &mut MovementController, &mut Animator, &EntityAnimations)  -> ();

#[derive(Debug, Clone)]
pub struct Character {
    //visual
    pub sprite: Rect,

    //stats
    pub name: String,
    pub hp: i32,
    pub speed: f64,
    pub dash_speed: f64,
    pub jump_height: f64,
    pub jump_distance: f64,
    pub punch_string_curr: i8,
    pub kick_string_curr: i8,
    pub airborne_punch_string_curr: i8,
    pub airborne_kick_string_curr: i8,
    pub launcher: bool,
    pub dropper: bool,
    pub dash_attack: bool,
    pub crash: bool,
}

#[derive(Debug, PartialEq)]
pub enum AttackType {
    Normal,
    Special,
    Ultra,
}
pub struct Attack {
    pub damage: i32,
    pub stun_on_hit: i32,
    pub stun_on_block: i32,
    pub push_back: f64,
    pub attack_type: AttackType,
    pub on_hit: Option<OnHit>,
}

impl Character {
    pub fn new(
        name: String,
        height: u32,
        width: u32,
        hp: i32,
        speed: f64,
        dash_speed: f64,
        jump_height: f64,
        jump_distance: f64,
        punch_string_curr: i8,
        kick_string_curr: i8,
        airborne_punch_string_curr: i8,
        airborne_kick_string_curr: i8,
        launcher: bool,
        dropper: bool,
        dash_attack: bool,
        crash: bool,
    ) -> Self {
        Self {
            name,
            sprite: Rect::new(0, 0, height, width),
            speed,
            dash_speed,
            hp,

            jump_height,
            jump_distance,

            punch_string_curr,
            kick_string_curr,
            airborne_punch_string_curr,
            airborne_kick_string_curr,

            launcher,
            dropper,
            dash_attack,
            crash,
        }
    }
}
