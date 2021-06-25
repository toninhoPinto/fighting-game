use std::collections::HashMap;

use sdl2::{rect::Rect, render::{Texture, TextureCreator}, ttf::Sdl2TtfContext, video::WindowContext};
use tiled::Map;

use crate::{engine_types::{animation::Animation, sprite_data::SpriteData}, game_logic::{characters::Attack, inputs::game_inputs::GameAction}};

use super::{asset_loader::{asset_loader, load_tiled_map::load_level}, rng_tables::load_item_table};

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
    
    pub ui_text: HashMap<String, Texture<'a>>,
}

impl<'a> UIAssets<'a> {
    pub fn load(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Self {
        Self {
            store_ui_sheet: asset_loader::load_texture(&texture_creator, "assets/vfx/shadow/29492.png"),
            store_ui_src_rects: HashMap::new(),
            ui_text: HashMap::new(),
        }
    }
}

pub struct LevelAssets<'a>{
    pub level_tiles: HashMap<String,Texture<'a>>,
    pub level_rooms: HashMap<i32, Map>,
    
    pub shadow: Texture<'a>,

    //hit effects
    pub hit_effect_textures: HashMap<String, Texture<'a>>,
    pub hit_effect_animations: HashMap<String, Animation>,
}

impl<'a> LevelAssets<'a> {
    pub fn load(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Self {

        let (textures, _) = asset_loader::load_textures_for_character(&texture_creator, "assets/vfx");

        let hit_anim = 
            asset_loader::load_anim_from_dir("assets/vfx/normal_hit", "normal_hit");
        let hit2_anim =
            asset_loader::load_anim_from_dir("assets/vfx/special_hit", "special_hit");
        let block_anim =
            asset_loader::load_anim_from_dir("assets/vfx/block", "block");
        let dash_ground_anim =
            asset_loader::load_anim_from_dir("assets/vfx/dash_ground", "dash");
        let jumping_ground_anim =
            asset_loader::load_anim_from_dir("assets/vfx/jumping_ground", "jumping");
        let feet_dust_ground_anim =
            asset_loader::load_anim_from_dir("assets/vfx/feet_dust_cloud", "feet_dust");

        let mut vfx = HashMap::new();
        vfx.insert(hit_anim.name.clone(),hit_anim);
        vfx.insert(hit2_anim.name.clone(),hit2_anim);
        vfx.insert(block_anim.name.clone(),block_anim);
        vfx.insert(dash_ground_anim.name.clone(),dash_ground_anim);
        vfx.insert(jumping_ground_anim.name.clone(),jumping_ground_anim);
        vfx.insert(feet_dust_ground_anim.name.clone(),feet_dust_ground_anim);

        let mut level_tiles = HashMap::new();

        level_tiles.insert("room_tileset".to_string(), asset_loader::load_texture(&texture_creator, "assets/level/hyptosis_tile-art-batch-1.png"));

        let mut level_rooms = HashMap::new();
        level_rooms.insert(0, load_level("assets/level/level1.tmx".to_string()));
        level_rooms.insert(1, load_level("assets/level/level2.tmx".to_string()));
        level_rooms.insert(2, load_level("assets/level/level3.tmx".to_string()));

        let loot_tables = load_item_table("assets/items/loot_tables.json".to_string());

        let font = ttf_context.load_font("assets/fonts/No_Virus.ttf", 16).unwrap();

        LevelAssets {
            hit_effect_textures: textures,
            hit_effect_animations: vfx,
            level_tiles,
            level_rooms,
            shadow: asset_loader::load_texture(&texture_creator, "assets/vfx/shadow/29492.png"),
        }
    }
}
