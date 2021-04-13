use std::collections::HashMap;
use std::path::Path;

use sdl2::{mixer::Chunk, render::{Texture, TextureCreator}, video::WindowContext};
use super::{animation::Animation, sound::audio_player};
use super::asset_loader;

pub struct CommonAssets<'a>{
    //sounds
    pub sound_effects: HashMap<String, Chunk>,

    //hit effects
    pub hit_effect_animations: HashMap<String, Animation<'a>>,
}


impl<'a> CommonAssets<'a> {

    pub fn load(texture_creator: &'a TextureCreator<WindowContext>) -> Self{

        let mut sound_chunk = audio_player::load_from_file(Path::new("assets/sounds/104183__ekokubza123__punch.wav"))
        .map_err(|e| format!("Cannot load sound file: {:?}", e)).unwrap();
        let mut sounds = HashMap::new();
        sounds.insert("hit".to_string(), sound_chunk);

        let hit_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
            &texture_creator,
            "assets/vfx/normal_hit",
        );
        
        let mut vfx = HashMap::new();
        vfx.insert("normal_hit".to_string(), Animation::new(hit_anim, "hit".to_string(), 0.35));

        
        CommonAssets{
            sound_effects: sounds,
            hit_effect_animations: vfx,
        }
    }

}