use std::collections::HashMap;
use std::path::Path;

use super::asset_loader;
use super::{animation::Animation, sound::audio_player};
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
    pub hit_effect_animations: HashMap<String, Animation<'a>>,
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
        hit_sound.set_volume(SFX_VOLUME);
        miss_sound.set_volume(SFX_VOLUME);
        let mut sounds = HashMap::new();
        sounds.insert("hit".to_string(), hit_sound);
        sounds.insert("miss".to_string(), miss_sound);

        let hit_anim: Vec<(i32, Texture)> =
            asset_loader::load_anim_from_dir(&texture_creator, "assets/vfx/normal_hit");
        let hit2_anim: Vec<(i32, Texture)> =
            asset_loader::load_anim_from_dir(&texture_creator, "assets/vfx/special_hit");

        let mut vfx = HashMap::new();
        vfx.insert(
            "normal_hit".to_string(),
            Animation::new(hit_anim, "normal_hit".to_string()),
        );
        vfx.insert(
            "special_hit".to_string(),
            Animation::new(hit2_anim, "special_hit".to_string()),
        );

        CommonAssets {
            sound_effects: sounds,
            hit_effect_animations: vfx,
        }
    }
}
