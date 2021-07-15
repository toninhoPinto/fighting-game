use std::collections::HashMap;
use std::path::Path;

use crate::asset_management::asset_loader::load_tiled_map::load_level;
use crate::asset_management::rng_tables::load_item_table;

use super::asset_loader::asset_loader;
use super::rng_tables::LootTable;
use super::{sound::audio_player};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::{
    mixer::Chunk,
    render::TextureCreator,
    video::WindowContext,
};


const SFX_VOLUME: i32 = 10;

pub struct CommonAssets<'a> {
    //sounds
    pub sound_effects: HashMap<String, Chunk>,
    pub fonts : HashMap<String, Font<'a, 'a>>,

    pub loot_tables: HashMap<String, LootTable>,
}

impl<'a> CommonAssets<'a> {
    pub fn load(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Self {
        let mut hit_sound =
            audio_player::load_from_file(Path::new("assets/sounds/104183__ekokubza123__punch.wav"))
                .map_err(|e| format!("Cannot load sound file: {:?}", e))
                .unwrap();
        let mut miss_sound =
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
 
        let mut dash_sound = audio_player::load_from_file(Path::new("assets/sounds/521999__kastenfrosch__whoosh-dash.wav"))
            .map_err(|e| format!("Cannot load sound file: {:?}", e))
            .unwrap();
            
            
        hit_sound.set_volume(SFX_VOLUME);
        miss_sound.set_volume(SFX_VOLUME * 2);
        block_sound.set_volume(SFX_VOLUME);
        select_level_sound.set_volume(SFX_VOLUME * 2);
        scroll_levels_sound.set_volume(SFX_VOLUME);
        jump_sound.set_volume(20);
        land_sound.set_volume(10);
        dropped_sound.set_volume(100);
        dash_sound.set_volume(10);
        
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

        let basic_font = ttf_context.load_font("assets/fonts/No_Virus.ttf", 16).unwrap();
        let main_menu_font = ttf_context.load_font("assets/fonts/No_Virus.ttf", 32).unwrap();
        let event_font = ttf_context.load_font("assets/fonts/No_Virus.ttf", 32).unwrap();
        let combo_font = ttf_context.load_font("assets/fonts/ApeMount-WyPM9.ttf", 100).unwrap();

        let mut fonts = HashMap::new();
        fonts.insert("main_menu_font".to_string(), main_menu_font);
        fonts.insert("basic_font".to_string(), basic_font);
        fonts.insert("event_font".to_string(), event_font);
        fonts.insert("combo_font".to_string(), combo_font);

        CommonAssets {
            sound_effects: sounds,
            loot_tables,
            fonts,
        }
    }
}
