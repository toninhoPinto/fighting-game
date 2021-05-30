use sdl2::rect::Rect;
use std::string::String;

use crate::{asset_management::asset_holders::EntityAnimations, collision::collider_manager::ColliderManager, engine_types::animator::Animator};

use super::{movement_controller::MovementController};

pub mod foxgirl;
pub mod player;

pub(crate) type OnHitSpecificAttack = fn(&Attack, &mut ColliderManager, &mut MovementController, &mut Animator)  -> ();

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

    pub can_double_jump: bool,

    pub punch_string: i8,
    pub kick_string: i8,
    pub airborne_punch_string: i8,
    pub airborne_kick_string: i8,
    pub directional_attacks_mask: u32,

    pub punch_string_curr: i8,
    pub kick_string_curr: i8,
    pub airborne_punch_string_curr: i8,
    pub airborne_kick_string_curr: i8,
    pub directional_attacks_mask_curr: u32,
}

/*
    launcher    0b0001u32
    dropper     0b0010u32
    dashing     0b0100u32
    crash       0b1000u32
*/

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
    pub on_hit: Option<OnHitSpecificAttack>,
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
        can_double_jump: bool,
        punch_string: i8,
        kick_string: i8,
        airborne_punch_string: i8,
        airborne_kick_string: i8,
        directional_attacks_mask: u32,
    ) -> Self {
        Self {
            name,
            sprite: Rect::new(0, 0, height, width),
            speed,
            dash_speed,
            hp,

            jump_height,
            jump_distance,
            can_double_jump,

            punch_string,
            kick_string,
            airborne_punch_string,
            airborne_kick_string,

            directional_attacks_mask,

            punch_string_curr: punch_string,
            kick_string_curr: kick_string,
            airborne_punch_string_curr: airborne_punch_string,
            airborne_kick_string_curr: airborne_kick_string,
            directional_attacks_mask_curr: directional_attacks_mask,
        }
    }
}
