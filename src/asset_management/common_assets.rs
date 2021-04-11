use std::collections::HashMap;

use sdl2::{mixer::Chunk, render::Texture};

pub struct CommonAssets<'a>{
    //sounds
    pub sound_effects: HashMap<String, Chunk>,

    //hit effects
    pub hit_effect_animations: HashMap<String, Vec<Texture<'a>>>,
}