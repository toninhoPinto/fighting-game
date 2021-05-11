use super::{characters::{Ability, Attack, AttackType, player::Player}};
use sdl2::rect::Point;
use parry2d::na::Vector2;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use super::characters::Character;
use super::inputs::game_inputs::GameAction;
use crate::asset_management::{animation::Animation, asset_loader::asset_loader::load_textures_for_character, sprite_data::SpriteData};
use crate::asset_management::asset_loader::asset_loader;
use std::collections::HashMap;
use std::string::String;

pub struct CharacterAssets<'a> {
    pub textures: HashMap<String, Texture<'a>>,
    pub texture_data: HashMap<String, SpriteData>
}

pub struct CharacterAnimations {
    pub animations: HashMap<String, Animation>,
    pub projectile_animation: HashMap<String, Animation>,
}

pub struct CharacterData {
    pub input_combination_anims: Vec<(Vec<i32>, String)>,
    pub directional_variation_anims: Vec<((GameAction, GameAction), String)>,
    pub attack_effects: HashMap<String, (i32, Ability)>,
    pub attacks: HashMap<String, Attack>,
}

pub fn load_stage(texture_creator: &TextureCreator<WindowContext>) -> Texture {
    asset_loader::load_texture(&texture_creator, "assets/stages/Sf3si-hugo.png")
}

pub fn load_character(character_name: &str, spawn_pos: Point, id: i32) -> Player {
    let fighter = match character_name {
        "foxgirl" => Some(Character::new(
            character_name.to_string(),
            240,
            200,
            200,
            4,
            250.0,
            350.0,
            700.0,
            600.0,
        )),
        _ => None,
    }
    .unwrap();
    Player::new(id, fighter, spawn_pos)
}

pub fn load_character_anim_data<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    name: &str,
) -> (CharacterAssets<'a>, CharacterAnimations, CharacterData) {
    match name {
        "foxgirl" => Some((load_foxgirl_assets(texture_creator), load_foxgirl_animations(), load_foxgirl_data())),
        _ => None,
    }.unwrap()
}

//===========================================================


fn load_foxgirl_directional_inputs() ->   Vec<((GameAction, GameAction), String)>{
    let mut directional_inputs: Vec<((GameAction, GameAction), String)> = Vec::new();

    directional_inputs.push(((GameAction::Right, GameAction::Punch), "directional_light_punch".to_string()));
    directional_inputs.push(((GameAction::Left, GameAction::Punch), "directional_light_punch".to_string()));

    directional_inputs.push(( (GameAction::Right, GameAction::Kick), "directional_heavy_punch".to_string()));
    directional_inputs.push(( (GameAction::Left, GameAction::Kick), "directional_heavy_punch".to_string()));

    directional_inputs
}

fn load_foxgirl_special_inputs() -> Vec<(Vec<i32>, String)>{
    let mut specials_inputs: Vec<(Vec<i32>, String)> = Vec::new();
    let spam_light_punch_inputs = vec![
        GameAction::Punch as i32,
        GameAction::Punch as i32,
        GameAction::Punch as i32,
    ];
    //specials_inputs.push((spam_light_punch_inputs, "spam_light_punch".to_string()));

    specials_inputs
}

fn load_foxgirl_anims() -> HashMap<String, Animation> {

    let idle_anim =
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/idle", "idle");

    let take_damage_anim =
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/take_damage/1", "take_damage_anim");

    let dead_anim=
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/dead", "dead");

    let walk_anim=
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/walk", "walk");

    let crouch_start_anim =
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/crouch/crouched", "crouch");

    let crouch_idle_anim=
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/crouch/crouching", "crouching");

    let light_punch_anim = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/attacks/light_punch", "light_punch");

    let mut medium_punch_anim = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/attacks/medium_punch", "medium_punch");

    let heavy_punch_anim = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/attacks/heavy_punch", "heavy_punch");

    let mut light_kick_anim = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/attacks/light_kick", "light_kick");

    let airborne_light_kick_anim = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/airborne/attacks/light_kick", "airborne_light_kick");

    let special1_anim = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/attacks/specials/directionals/forward_light_punch", "forward_light_punch");

    let special2_anim = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/attacks/specials/directionals/forward_heavy_punch", "forward_heavy_punch");

    let spam_light_punch_anim = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/attacks/specials/spam/spam_light_punch", "spam_light_punch");

    let mut dash_anim=
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/dash", "dash");
    let neutral_jump_anim =
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/neutral_jump", "neutral_jump");

    let mut character_anims = HashMap::new();

    character_anims.insert(idle_anim.name.clone(),idle_anim);
    character_anims.insert(take_damage_anim.name.clone(),take_damage_anim);
    character_anims.insert(dead_anim.name.clone(),dead_anim);

    dash_anim.offsets = Some(vec![Vector2::new(0.0, 0.0), Vector2::new(3000.0, 0.0), Vector2::new(2000.0, 0.0), 
    Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0) , Vector2::new(0.0, 0.0) , Vector2::new(0.0, 0.0) , Vector2::new(0.0, 0.0) , Vector2::new(0.0, 0.0)]);
    character_anims.insert(dash_anim.name.clone(),dash_anim);
    
    character_anims.insert(walk_anim.name.clone(),walk_anim);
    character_anims.insert(light_punch_anim.name.clone(),light_punch_anim);


    medium_punch_anim.offsets = Some(vec![Vector2::new(100.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(50.0, 0.0), 
    Vector2::new(1000.0, 0.0), Vector2::new(400.0, 0.0),Vector2::new(50.0, 0.0), Vector2::new(50.0, 0.0), Vector2::new(50.0, 0.0), Vector2::new(50.0, 0.0), 
    Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0),  Vector2::new(0.0, 0.0),  Vector2::new(0.0, 0.0),  Vector2::new(0.0, 0.0)]);
    character_anims.insert(medium_punch_anim.name.clone(),medium_punch_anim);

    character_anims.insert(heavy_punch_anim.name.clone(),heavy_punch_anim);


    light_kick_anim.offsets =Some(vec![Vector2::new(400.0, 0.0), Vector2::new(100.0, 0.0), Vector2::new(0.0, 0.0), 
    Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(300.0, 0.0), Vector2::new(300.0, 0.0), 
    Vector2::new(300.0, 0.0), Vector2::new(300.0, 0.0),  Vector2::new(0.0, 0.0)]);
    character_anims.insert(light_kick_anim.name.clone(),light_kick_anim);

    character_anims.insert(airborne_light_kick_anim.name.clone(),airborne_light_kick_anim);
    
    character_anims.insert(crouch_start_anim.name.clone(),crouch_start_anim);
    character_anims.insert(crouch_idle_anim.name.clone(),crouch_idle_anim);
    character_anims.insert(neutral_jump_anim.name.clone(),neutral_jump_anim);
    
    character_anims.insert(special1_anim.name.clone(),special1_anim);
    character_anims.insert(special2_anim.name.clone(),special2_anim);
    character_anims.insert(spam_light_punch_anim.name.clone(),spam_light_punch_anim);

    character_anims
}

fn load_foxgirl_attacks() -> HashMap<String, Attack> {
    let mut attacks = HashMap::new();

    attacks.insert(
        "lk".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 400.0,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "c.lk".to_string(),
        Attack {
            damage: 15,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 300.0,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "j.lk".to_string(),
        Attack {
            damage: 15,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 300.0,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "lp".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 50.0,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "mp".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 50.0,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "hp".to_string(),
        Attack {
            damage: 10,
            stun_on_hit: 20,
            stun_on_block: 14,
            push_back: 50.0,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "first-spam-punch".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 20,
            stun_on_block: 14,
            push_back: 70.0,
            attack_type: AttackType::Special
        },
    );

    attacks.insert(
        "fast-spam-punch".to_string(),
        Attack {
            damage: 2,
            stun_on_hit: 20,
            stun_on_block: 14,
            push_back: 70.0,
            attack_type: AttackType::Special
        },
    );

    attacks.insert(
        "last-spam-punch".to_string(),
        Attack {
            damage: 20,
            stun_on_hit: 20,
            stun_on_block: 14,
            push_back: 70.0,
            attack_type: AttackType::Special
        },
    );


    attacks
}

fn load_foxgirl_animations() -> CharacterAnimations {
    let anims= load_foxgirl_anims();
    CharacterAnimations {
        animations: anims,
        projectile_animation: HashMap::new(),
    }
}

fn load_foxgirl_assets(texture_creator: &TextureCreator<WindowContext>) -> CharacterAssets {
    let (textures, data) = load_textures_for_character(texture_creator, "assets/foxgirl");
    CharacterAssets {
        textures,
        texture_data: data,
    }
}

fn load_foxgirl_data() -> CharacterData {
    CharacterData {
        input_combination_anims: load_foxgirl_special_inputs(),
        directional_variation_anims: load_foxgirl_directional_inputs(),
        attacks: load_foxgirl_attacks(),
        attack_effects: HashMap::new(),
    }
}
