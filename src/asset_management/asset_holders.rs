use std::collections::HashMap;

use sdl2::{rect::Rect, render::Texture};

use crate::{engine_types::{animation::Animation, sprite_data::SpriteData}, game_logic::{characters::{Ability, Attack}, inputs::game_inputs::GameAction}};

pub struct EntityAssets<'a> {
    pub textures: HashMap<String, Texture<'a>>,
    pub texture_data: HashMap<String, SpriteData>
}
pub struct EntityAnimations {
    pub animations: HashMap<String, Animation>,
    pub projectile_animation: HashMap<String, Animation>,
}

pub struct EntityData {
    pub auto_combo_strings: HashMap<i32, Vec<&'static str>>,
    pub directional_variation_anims: Vec<((GameAction, GameAction), String)>,
    pub attack_effects: HashMap<String, (i32, Ability)>,
    pub attacks: HashMap<String, Attack>,
}

pub struct OverworldAssets<'a>{
    pub spritesheet: Texture<'a>,
    pub src_rects: HashMap<String, Rect>,
    pub portraits: HashMap<String, Texture<'a>>,
}
