use super::player::{Player, PlayerState};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use std::collections::HashMap;
use std::string::String;
use crate::rendering::renderer;
use super::game_input::GameInputs;
use super::projectile::Projectile;

pub struct CharacterAnimationData<'a> {
    pub animations: HashMap<std::string::String, Vec<Texture<'a>>>,
    pub input_combination_anims: Vec<(Vec<GameInputs>, String)>,
    pub directional_variation_anims: Vec<(Vec<GameInputs>, String)>,
    pub effects: HashMap<String, Projectile>,
    pub projectile_animation: HashMap<String, Vec<Texture<'a>>>
}

pub fn load_character(character_name: std::string::String, spawnPos: Point, flipped: bool, id: i32) -> Player {
    //if character_name == "ryu".to_string()
    let player = Player {
        id,
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
        flipped,
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


    let projectile_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, "assets/ryu/standing/attacks/projectiles".to_string());

    let light_projectile = Projectile {
        position: Point::new(120, 205),
        sprite: Rect::new(0, 0, 100, 110),
        speed: 10,
        direction: Point::new(0, 0),
        target_position: None,
        damage: 10,
        flipped: false,
        animation_index: 0.0,
        animation_name: "note".to_string(),
        player_owner: 0,
    };
    let med_projectile = Projectile {
        position: Point::new(120, 105),
        sprite: Rect::new(0, 0, 100, 110),
        speed: 10,
        direction: Point::new(0, 0),
        target_position: None,
        damage: 15,
        flipped: false,
        animation_index: 0.0,
        animation_name: "note".to_string(),
        player_owner: 0,
    };
    let heavy_projectile = Projectile {
        position: Point::new(120, 5),
        sprite: Rect::new(0, 0, 100, 110),
        speed: 10,
        direction: Point::new(0, 0),
        target_position: None,
        damage: 20,
        flipped: false,
        animation_index: 0.0,
        animation_name: "note".to_string(),
        player_owner: 0,
    };

    let mut effects_of_abilities = HashMap::new();
    effects_of_abilities.insert("light_special_attack".to_string(), light_projectile);
    effects_of_abilities.insert("med_special_attack".to_string(), med_projectile);
    effects_of_abilities.insert("heavy_special_attack".to_string(), heavy_projectile);

    let mut projectile_anims = HashMap::new();
    projectile_anims.insert("note".to_string(), projectile_anim);

    CharacterAnimationData {
        animations: anims,
        input_combination_anims: specials_inputs,
        directional_variation_anims: directional_inputs,
        effects: effects_of_abilities,
        projectile_animation: projectile_anims
    }
}

fn load_character_anims(texture_creator: &TextureCreator<WindowContext>, character_name: std::string::String) -> HashMap<std::string::String, Vec<Texture>>{
    let mut character_anims = HashMap::new();

    //TODO iterate through folders and use folder name as key for hashmap
    let idle_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/idle", character_name).to_string());
    let walk_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/walk", character_name).to_string());
    let walk_back_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/walk_back", character_name).to_string());
    let crouch_start_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/crouch/crouched", character_name).to_string());
    let crouch_idle_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/crouch/crouching", character_name).to_string());
    let light_punch_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/light_punch", character_name).to_string());
    let medium_punch_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/medium_punch", character_name).to_string());
    let heavy_punch_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/heavy_punch", character_name).to_string());
    let light_kick_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/light_kick", character_name).to_string());
    let special1_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/directionals", character_name).to_string());
    let special2_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/combinations", character_name).to_string());
    let dash_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/dash", character_name).to_string());
    let dash_back_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/back_dash", character_name).to_string());
    let neutral_jump_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/neutral_jump", character_name).to_string());
    let directional_jump_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/directional_jump", character_name).to_string());

    character_anims.insert("idle".to_string(), idle_anim);
    character_anims.insert("dash".to_string(), dash_anim);
    character_anims.insert("dash_back".to_string(), dash_back_anim);
    character_anims.insert("walk".to_string(), walk_anim);
    character_anims.insert("walk_back".to_string(), walk_back_anim);
    character_anims.insert("light_punch".to_string(), light_punch_anim);
    character_anims.insert("med_punch".to_string(), medium_punch_anim);
    character_anims.insert("heavy_punch".to_string(), heavy_punch_anim);
    character_anims.insert("light_kick".to_string(), light_kick_anim);
    character_anims.insert("crouch".to_string(), crouch_start_anim);
    character_anims.insert("crouching".to_string(), crouch_idle_anim);
    character_anims.insert("neutral_jump".to_string(), neutral_jump_anim);
    character_anims.insert("directional_jump".to_string(), directional_jump_anim);
    character_anims.insert("directional_light_punch".to_string(), special1_anim);


    character_anims.insert("light_special_attack".to_string(), special2_anim);

    //TODO DUPLICATED DATA, i think the only solution is to have a separate texture manager and character anims becomes a hashmap<string, id on texturemanager>
    let special3_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/combinations", character_name).to_string());
    let special4_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/combinations", character_name).to_string());
    character_anims.insert("med_special_attack".to_string(), special3_anim);
    character_anims.insert("heavy_special_attack".to_string(), special4_anim);

    character_anims
}