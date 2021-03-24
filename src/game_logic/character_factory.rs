use super::player::{Player, PlayerState};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use std::collections::HashMap;
use std::string::String;
use crate::rendering::renderer;
use super::game_input::GameInputs;

pub struct CharacterAnimationData<'a> {
    pub animations: HashMap<std::string::String, Vec<Texture<'a>>>,
    pub input_combination_anims: Vec<(Vec<GameInputs>, String)>,
    pub directional_variation_anims: Vec<(Vec<GameInputs>, String)>,
}

pub fn load_character(character_name: std::string::String, spawnPos: Point, flipped: bool) -> Player {
    //if character_name == "ryu".to_string()
    let player = Player {
        position: spawnPos,
        sprite: Rect::new(0, 0, 580, 356),
        speed: 5,
        dash_speed: 10,
        dash_back_speed: 7,
        prev_direction: 0,
        direction: 0,
        dir_related_of_other: 0,
        state: PlayerState::Standing,
        isAttacking: false,
        animation_index: 0.0,
        current_animation: "idle".to_string(),
        flipped: flipped,
        last_directional_input: None,
        last_directional_input_v: None,
        last_directional_input_h: None
    };

    player
}

pub fn load_character_anim_data(texture_creator: &TextureCreator<WindowContext>, character_name: std::string::String) -> CharacterAnimationData {
    let anims = load_character_anims(texture_creator, character_name);

    //TODO should this be deserialized or kep as code in this factory?
    let mut specials_inputs: Vec<(Vec<GameInputs>, String)> = Vec::new();
    let mut combo_string: Vec<GameInputs> = Vec::new();
    combo_string.push(GameInputs::DOWN);
    combo_string.push(GameInputs::FwdDOWN);
    combo_string.push(GameInputs::FWD);
    combo_string.push(GameInputs::LightPunch);
    specials_inputs.push((combo_string, "special_attack".to_string()));

    let mut directional_inputs: Vec<(Vec<GameInputs>, String)> = Vec::new();
    let mut directional_string: Vec<GameInputs> = Vec::new();
    directional_string.push(GameInputs::FWD);
    directional_string.push(GameInputs::LightPunch);
    directional_inputs.push((directional_string, "directional_light_punch".to_string()));

    CharacterAnimationData{
        animations: anims,
        input_combination_anims: specials_inputs,
        directional_variation_anims: directional_inputs,
    }
}

fn load_character_anims(texture_creator: &TextureCreator<WindowContext>, character_name: std::string::String) -> HashMap<std::string::String, Vec<Texture>>{
    let mut character_anims = HashMap::new();

    //TODO iterate through folders and use folder name as key for hashmap
    let idle_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/idle_anim", character_name).to_string());
    let walk_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/walk_anim", character_name).to_string());
    let walk_back_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/walk_back_anim", character_name).to_string());
    let crouch_start_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/crouch/crouched", character_name).to_string());
    let crouch_idle_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/crouch/crouching", character_name).to_string());
    let light_punch_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/light_punch", character_name).to_string());
    let special1_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/directionals", character_name).to_string());
    let special2_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/combinations", character_name).to_string());
    let dash_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/dash", character_name).to_string());
    let dash_back_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/back_dash", character_name).to_string());

    character_anims.insert("idle".to_string(), idle_anim);
    character_anims.insert("walk".to_string(), walk_anim);
    character_anims.insert("walk_back".to_string(), walk_back_anim);
    character_anims.insert("light_punch".to_string(), light_punch_anim);
    character_anims.insert("crouch".to_string(), crouch_start_anim);
    character_anims.insert("crouching".to_string(), crouch_idle_anim);
    character_anims.insert("directional_light_punch".to_string(), special1_anim);
    character_anims.insert("special_attack".to_string(), special2_anim);
    character_anims.insert("dash".to_string(), dash_anim);
    character_anims.insert("dash_back".to_string(), dash_back_anim);

    character_anims
}