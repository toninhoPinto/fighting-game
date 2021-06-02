use std::collections::HashMap;
use std::path::Path;

use crate::engine_types::animation::Animation;

use super::asset_loader::asset_loader;
use super::{sound::audio_player};
use sdl2::{
    mixer::Chunk,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

const SFX_VOLUME: i32 = 10;

pub struct CommonAssets<'a> {
    //sounds
    pub sound_effects: HashMap<String, Chunk>,

    //hit effects
    pub hit_effect_textures: HashMap<String, Texture<'a>>,
    pub hit_effect_animations: HashMap<String, Animation>,
    pub shadow: Texture<'a>
}

impl<'a> CommonAssets<'a> {
    pub fn load(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
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
 
        hit_sound.set_volume(SFX_VOLUME);
        miss_sound.set_volume(SFX_VOLUME * 2);
        block_sound.set_volume(SFX_VOLUME);
        select_level_sound.set_volume(SFX_VOLUME * 10);
        scroll_levels_sound.set_volume(SFX_VOLUME);
        jump_sound.set_volume(100);
        land_sound.set_volume(50);
        dropped_sound.set_volume(100);
        
        let mut sounds = HashMap::new();
        sounds.insert("hit".to_string(), hit_sound);
        sounds.insert("miss".to_string(), miss_sound);
        sounds.insert("block".to_string(), block_sound);
        sounds.insert("select_level".to_string(), select_level_sound);
        sounds.insert("scroll_level".to_string(), scroll_levels_sound);
        sounds.insert("jump".to_string(), jump_sound);
        sounds.insert("land".to_string(), land_sound);
        sounds.insert("dropped".to_string(), dropped_sound);

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

        CommonAssets {
            sound_effects: sounds,
            hit_effect_textures: textures,
            hit_effect_animations: vfx,
            shadow: asset_loader::load_texture(&texture_creator, "assets/vfx/shadow/29492.png")
        }
    }
}
