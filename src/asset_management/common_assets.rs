use std::collections::HashMap;
use std::path::Path;

use crate::asset_management::asset_loader::load_tiled_map::load_level;
use crate::asset_management::rng_tables::load_item_table;
use crate::engine_types::animation::Animation;

use super::asset_loader::asset_loader;
use super::rng_tables::LootTable;
use super::{sound::audio_player};
use rand::prelude::SmallRng;
use sdl2::{
    mixer::Chunk,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use tiled::Map;

const SFX_VOLUME: i32 = 10;

pub struct CommonAssets<'a> {
    //sounds
    pub sound_effects: HashMap<String, Chunk>,

    //rooms
    pub level_tiles: HashMap<String,Texture<'a>>,
    pub level_rooms: HashMap<i32, Map>,
    pub shadow: Texture<'a>,

    pub loot_tables: HashMap<String, LootTable>,

    //hit effects
    pub hit_effect_textures: HashMap<String, Texture<'a>>,
    pub hit_effect_animations: HashMap<String, Animation>,

    //rng
    pub map_rng: Option<SmallRng>,
    pub item_rng: Option<SmallRng>,
    pub enemy_rng: Option<SmallRng>,
}

impl<'a> CommonAssets<'a> {
    pub fn load(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut hit_sound =
            audio_player::load_from_file(Path::new("assets/sounds/104183__ekokubza123__punch.wav"))
                .map_err(|e| format!("Cannot load sound file: {:?}", e))
                .unwrap();
        let mut dash_sound =
            audio_player::load_from_file(Path::new("assets/sounds/60009__qubodup__swosh-22.wav"))
                .map_err(|e| format!("Cannot load sound file: {:?}", e))
                .unwrap();

        let mut block_sound =
            audio_player::load_from_file(Path::new("assets/sounds/131142__flameeagle__block.mp3"))
                .map_err(|e| format!("Cannot load sound file: {:?}", e))
                .unwrap();
                
        let mut select_level_sound = audio_player::load_from_file(Path::new("assets/sounds/506052__mellau__button-click-3.wav"))
            .map_err(|e| format!("Cannot load sound file: {:?}", e))
            .unwrap();

        let mut scroll_levels_sound = audio_player::load_from_file(Path::new("assets/sounds/540568__eminyildirim__ui-pop-up.wav"))
            .map_err(|e| format!("Cannot load sound file: {:?}", e))
            .unwrap();

        let mut jump_sound = audio_player::load_from_file(Path::new("assets/sounds/509410__jburunet__jumping-hop-sound.wav"))
            .map_err(|e| format!("Cannot load sound file: {:?}", e))
            .unwrap();

        let mut land_sound = audio_player::load_from_file(Path::new("assets/sounds/553520__newlocknew__pop-down-impact-1-3-select-4lrs-mltprcssng.wav"))
            .map_err(|e| format!("Cannot load sound file: {:?}", e))
            .unwrap();

        let mut dropped_sound = audio_player::load_from_file(Path::new("assets/sounds/377157__pfranzen__smashing-head-on-wall.wav"))
            .map_err(|e| format!("Cannot load sound file: {:?}", e))
            .unwrap();
 
        let mut miss_sound = audio_player::load_from_file(Path::new("assets/sounds/521999__kastenfrosch__whoosh-dash.wav"))
            .map_err(|e| format!("Cannot load sound file: {:?}", e))
            .unwrap();
            
        hit_sound.set_volume(SFX_VOLUME);
        dash_sound.set_volume(SFX_VOLUME * 2);
        block_sound.set_volume(SFX_VOLUME);
        select_level_sound.set_volume(SFX_VOLUME * 2);
        scroll_levels_sound.set_volume(SFX_VOLUME);
        jump_sound.set_volume(20);
        land_sound.set_volume(10);
        dropped_sound.set_volume(100);
        miss_sound.set_volume(10);
        
        let mut sounds = HashMap::new();
        sounds.insert("hit".to_string(), hit_sound);
        sounds.insert("dash".to_string(), dash_sound);
        sounds.insert("block".to_string(), block_sound);
        sounds.insert("select_level".to_string(), select_level_sound);
        sounds.insert("scroll_level".to_string(), scroll_levels_sound);
        sounds.insert("jump".to_string(), jump_sound);
        sounds.insert("land".to_string(), land_sound);
        sounds.insert("dropped".to_string(), dropped_sound);
        sounds.insert("miss".to_string(), miss_sound);

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

        CommonAssets {
            sound_effects: sounds,
            hit_effect_textures: textures,
            hit_effect_animations: vfx,
            level_tiles,
            level_rooms,
            loot_tables,
            shadow: asset_loader::load_texture(&texture_creator, "assets/vfx/shadow/29492.png"),
            map_rng: None,
            item_rng: None,
            enemy_rng: None,
        }
    }
}
