use super::player::Player;
use sdl2::rect::Point;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use std::collections::HashMap;
use std::string::String;
use super::game_input::GameInputs;
use super::projectile::Projectile;
use crate::asset_management::asset_loader;
use crate::asset_management::animation::Animation;

pub struct CharacterAssets<'a> {
    pub animations: HashMap<std::string::String, Animation<'a>>,
    pub input_combination_anims: Vec<(Vec<GameInputs>, String)>,
    pub directional_variation_anims: Vec<(Vec<GameInputs>, String)>,
    pub effects: HashMap<String, Projectile>,
    pub projectile_animation: HashMap<String, Vec<Texture<'a>>>
}

pub fn load_character<'a>(_character_name: std::string::String, spawn_pos: Point, flipped: bool, id: i32) -> Player<'a> {
    //if character_name == "ryu".to_string()
    let player = Player::new(id, spawn_pos, flipped);
    player
}

pub fn load_character_anim_data(texture_creator: &TextureCreator<WindowContext>, character_name: std::string::String) -> CharacterAssets {
    let anims = load_character_anims(texture_creator, character_name);

    //TODO should this be deserialized or kep as code in this factory?
    let mut specials_inputs: Vec<(Vec<GameInputs>, String)> = Vec::new();
    let mut light_combo_string: Vec<GameInputs> = Vec::new();
    light_combo_string.push(GameInputs::DOWN);
    light_combo_string.push(GameInputs::FwdDOWN);
    light_combo_string.push(GameInputs::FWD);
    light_combo_string.push(GameInputs::LightPunch);
    specials_inputs.push((light_combo_string, "light_special_attack".to_string()));

    let mut med_combo_string: Vec<GameInputs> = Vec::new();
    med_combo_string.push(GameInputs::DOWN);
    med_combo_string.push(GameInputs::FwdDOWN);
    med_combo_string.push(GameInputs::FWD);
    med_combo_string.push(GameInputs::MediumPunch);
    specials_inputs.push((med_combo_string, "med_special_attack".to_string()));

    let mut heav_combo_string: Vec<GameInputs> = Vec::new();
    heav_combo_string.push(GameInputs::DOWN);
    heav_combo_string.push(GameInputs::FwdDOWN);
    heav_combo_string.push(GameInputs::FWD);
    heav_combo_string.push(GameInputs::HeavyPunch);
    specials_inputs.push((heav_combo_string, "heavy_special_attack".to_string()));

    let mut directional_inputs: Vec<(Vec<GameInputs>, String)> = Vec::new();
    let mut directional_string: Vec<GameInputs> = Vec::new();
    directional_string.push(GameInputs::FWD);
    directional_string.push(GameInputs::LightPunch);
    directional_inputs.push((directional_string, "directional_light_punch".to_string()));


    let projectile_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, "assets/ryu/standing/attacks/projectiles".to_string());

    let light_projectile = Projectile::new(0, Point::new(120, 5));
    let med_projectile = Projectile::new(0, Point::new(120, 105));
    let heavy_projectile = Projectile::new(0, Point::new(120,205));

    let mut effects_of_abilities = HashMap::new();
    effects_of_abilities.insert("light_special_attack".to_string(), light_projectile);
    effects_of_abilities.insert("med_special_attack".to_string(), med_projectile);
    effects_of_abilities.insert("heavy_special_attack".to_string(), heavy_projectile);

    let mut projectile_anims = HashMap::new();
    projectile_anims.insert("note".to_string(), projectile_anim);

    CharacterAssets {
        animations: anims,
        input_combination_anims: specials_inputs,
        directional_variation_anims: directional_inputs,
        effects: effects_of_abilities,
        projectile_animation: projectile_anims
    }
}

fn load_character_anims(texture_creator: &TextureCreator<WindowContext>, character_name: std::string::String) -> HashMap<std::string::String, Animation>{
    let mut character_anims = HashMap::new();

    //TODO iterate through folders and use folder name as key for hashmap
    let idle_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/idle", character_name).to_string());
    let walk_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/walk", character_name).to_string());
    let walk_back_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/walk_back", character_name).to_string());
    let crouch_start_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/crouch/crouched", character_name).to_string());
    let crouch_idle_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/crouch/crouching", character_name).to_string());
    let light_punch_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/light_punch", character_name).to_string());
    let medium_punch_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/medium_punch", character_name).to_string());
    let heavy_punch_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/heavy_punch", character_name).to_string());
    let light_kick_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/light_kick", character_name).to_string());
    let special1_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/directionals", character_name).to_string());
    let special2_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/combinations", character_name).to_string());
    let dash_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/dash", character_name).to_string());
    let dash_back_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/back_dash", character_name).to_string());
    let neutral_jump_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/neutral_jump", character_name).to_string());
    let directional_jump_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/directional_jump", character_name).to_string());

    character_anims.insert("idle".to_string(), Animation::new(idle_anim, "idle".to_string(), 0.35));
    character_anims.insert("dash".to_string(), Animation::new(dash_anim, "dash".to_string(), 0.35));
    character_anims.insert("dash_back".to_string(), Animation::new(dash_back_anim, "dash_back".to_string(), 0.35));
    character_anims.insert("walk".to_string(), Animation::new(walk_anim, "walk".to_string(), 0.35));
    character_anims.insert("walk_back".to_string(), Animation::new(walk_back_anim, "walk_back".to_string(), 0.35));
    character_anims.insert("light_punch".to_string(), Animation::new(light_punch_anim, "light_punch".to_string(), 0.35));
    character_anims.insert("med_punch".to_string(), Animation::new(medium_punch_anim, "med_punch".to_string(), 0.35));
    character_anims.insert("heavy_punch".to_string(), Animation::new(heavy_punch_anim, "heavy_punch".to_string(), 0.35));
    character_anims.insert("light_kick".to_string(), Animation::new(light_kick_anim, "light_kick".to_string(), 0.35));
    character_anims.insert("crouch".to_string(), Animation::new(crouch_start_anim, "crouch".to_string(), 0.35));
    character_anims.insert("crouching".to_string(), Animation::new(crouch_idle_anim, "crouching".to_string(), 0.35));
    character_anims.insert("neutral_jump".to_string(), Animation::new(neutral_jump_anim, "neutral_jump".to_string(), 0.35));
    character_anims.insert("directional_jump".to_string(), Animation::new(directional_jump_anim, "directional_jump".to_string(), 0.35));
    character_anims.insert("directional_light_punch".to_string(), Animation::new(special1_anim, "directional_light_punch".to_string(), 0.35));


    character_anims.insert("light_special_attack".to_string(), Animation::new(special2_anim, "light_special_attack".to_string(), 0.35));

    //TODO DUPLICATED DATA, i think the only solution is to have a separate texture manager and character anims becomes a hashmap<string, id on texturemanager>
    let special3_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/combinations", character_name).to_string());
    let special4_anim: Vec<Texture> = asset_loader::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/combinations", character_name).to_string());
    character_anims.insert("med_special_attack".to_string(), Animation::new(special3_anim, "med_special_attack".to_string(), 0.35));
    character_anims.insert("heavy_special_attack".to_string(), Animation::new(special4_anim, "heavy_special_attack".to_string(), 0.35));

    character_anims
}