use std::collections::HashMap;

use sdl2::{rect::Rect, render::Texture};

use crate::{engine_types::{animation::Animation, sprite_data::SpriteData}, game_logic::{characters::Attack, inputs::game_inputs::GameAction}};

pub struct EntityAssets<'a> {
    pub textures: HashMap<String, Texture<'a>>,
    pub texture_data: HashMap<String, SpriteData>
}
pub struct EntityAnimations {
    pub animations: HashMap<String, Animation>,
    pub projectile_animation: HashMap<String, Animation>,
}

pub struct DirectionalAttack {
    pub mask: u32,
    pub is_airborne: bool,
    pub is_dashing: bool,
    pub inputs: (GameAction, GameAction),
    pub key: String
}

impl DirectionalAttack {
    pub fn new(mask: u32, is_airborne: bool, is_dashing: bool, inputs: (GameAction, GameAction),key: String ) -> Self {
        Self {
            mask,
            is_airborne,
            is_dashing,
            inputs,
            key,
        }
    }
}

pub struct EntityData {
    pub auto_combo_strings: HashMap<i32, Vec<&'static str>>,
    pub directional_variation_anims: Vec<DirectionalAttack>,    //mask, is_airborne, inputs, name_of_attack 
    pub attacks: HashMap<String, Attack>,
}

pub struct OverworldAssets<'a>{
    pub spritesheet: Texture<'a>,
    pub src_rects: HashMap<String, Rect>,
    pub portraits: HashMap<String, Texture<'a>>,
    pub backgrounds: Vec<Texture<'a>>,
}

pub struct ItemAssets<'a>{
    pub spritesheet: Texture<'a>,
    pub src_rects: HashMap<String, Rect>,
}

pub struct UIAssets<'a>{
    pub store_ui_sheet: Texture<'a>,
    pub store_ui_src_rects: HashMap<String, Rect>,
    pub level_ui_sheet: Texture<'a>,
    pub level_ui_src_rects: HashMap<String, Rect>,
}