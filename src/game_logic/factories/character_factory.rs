
use sdl2::rect::Point;
use parry2d::na::Vector2;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::{asset_management::{asset_holders::{EntityAnimations, EntityAssets, EntityData}, asset_loader::asset_loader::{self, load_textures_for_character}}, engine_types::{animation::Animation, sprite_data::SpriteData}, game_logic::{characters::{Attack, AttackType, Character, OnHitSpecificAttack, player::Player}, inputs::game_inputs::GameAction, on_hit::basic_on_hits::launch}};
use std::collections::HashMap;
use std::string::String;


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
            250.0,
            350.0,
            700.0,
            600.0,
            2,
            1,
            2,
            1,
            0b0000u32,
        )),
        _ => None,
    }
    .unwrap();
    Player::new(id, fighter, spawn_pos)
}

pub fn load_character_anim_data<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    name: &str,
) -> (EntityAssets<'a>, EntityAnimations, EntityData) {
    match name {
        "foxgirl" => Some((load_foxgirl_assets(texture_creator), load_foxgirl_animations(), load_foxgirl_data())),
        _ => None,
    }.unwrap()
}

//===========================================================


fn load_foxgirl_directional_inputs() ->   Vec<(u32, (GameAction, GameAction), String)>{
    let mut directional_inputs: Vec<(u32, (GameAction, GameAction), String)> = Vec::new();

    //directional_inputs.push(( (GameAction::Right, GameAction::Punch), "directional_light_punch".to_string()) );
    //directional_inputs.push(( (GameAction::Left, GameAction::Punch), "directional_light_punch".to_string()) );

    directional_inputs.push((0b0001u32, (GameAction::Up, GameAction::Punch), "launcher".to_string()) );

    directional_inputs
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

    let airborne_punch = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/airborne/attacks/punch", "airborne_punch");

    let airborne_poke = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/airborne/attacks/poke", "airborne_poke");

    let airborne_slash = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/airborne/attacks/slash", "airborne_slash");

    let launcher_anim = 
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/attacks/launcher", "launcher");

    let mut dash_anim=
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/dash", "dash");
    let neutral_jump_anim =
        asset_loader::load_anim_and_data_from_dir("assets/foxgirl/standing/neutral_jump", "neutral_jump");

    let mut character_anims = HashMap::new();

    character_anims.insert(airborne_punch.name.clone(),airborne_punch);
    character_anims.insert(airborne_poke.name.clone(),airborne_poke);
    character_anims.insert(airborne_slash.name.clone(),airborne_slash);

    character_anims.insert(idle_anim.name.clone(),idle_anim);
    character_anims.insert(take_damage_anim.name.clone(),take_damage_anim);
    character_anims.insert(dead_anim.name.clone(),dead_anim);

    dash_anim.offsets = Some(vec![Vector2::new(0.0, 0.0), Vector2::new(8000.0, 0.0), Vector2::new(5000.0, 0.0), 
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
    Vector2::new(300.0, 0.0), Vector2::new(300.0, 0.0),  Vector2::new(300.0, 0.0)]);
    character_anims.insert(light_kick_anim.name.clone(),light_kick_anim);

    character_anims.insert(airborne_light_kick_anim.name.clone(),airborne_light_kick_anim);
    
    character_anims.insert(crouch_start_anim.name.clone(),crouch_start_anim);
    character_anims.insert(crouch_idle_anim.name.clone(),crouch_idle_anim);
    character_anims.insert(neutral_jump_anim.name.clone(),neutral_jump_anim);
    
    character_anims.insert(launcher_anim.name.clone(),launcher_anim);

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
            attack_type: AttackType::Normal,
            on_hit: None,
        },
    );

    attacks.insert(
        "j.lk".to_string(),
        Attack {
            damage: 15,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 300.0,
            attack_type: AttackType::Normal,
            on_hit: None,
        },
    );

    attacks.insert(
        "j.p".to_string(),
        Attack {
            damage: 15,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 300.0,
            attack_type: AttackType::Normal,
            on_hit: None,
        },
    );

    attacks.insert(
        "poke".to_string(),
        Attack {
            damage: 15,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 300.0,
            attack_type: AttackType::Normal,
            on_hit: None,
        },
    );

    attacks.insert(
        "slash".to_string(),
        Attack {
            damage: 15,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 300.0,
            attack_type: AttackType::Normal,
            on_hit: None,
        },
    );

    attacks.insert(
        "lp".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 50.0,
            attack_type: AttackType::Normal,
            on_hit: None,
        },
    );

    attacks.insert(
        "mp".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 1250.0,
            attack_type: AttackType::Normal,
            on_hit: None,
        },
    );

    attacks.insert(
        "hp".to_string(),
        Attack {
            damage: 10,
            stun_on_hit: 20,
            stun_on_block: 14,
            push_back: 550.0,
            attack_type: AttackType::Normal,
            on_hit: None,
        },
    );

    attacks.insert(
        "launcher".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 20,
            stun_on_block: 14,
            push_back: 0.0,
            attack_type: AttackType::Special,
            on_hit: Some(launch as OnHitSpecificAttack),
        },
    );

    attacks
}

fn load_foxgirl_animations() -> EntityAnimations {
    let anims= load_foxgirl_anims();
    EntityAnimations {
        animations: anims,
        projectile_animation: HashMap::new(),
    }
}

fn load_foxgirl_assets(texture_creator: &TextureCreator<WindowContext>) -> EntityAssets {
    let (textures, data) = load_textures_for_character(texture_creator, "assets/foxgirl");
    EntityAssets {
        textures,
        texture_data: data,
    }
}

fn load_foxgirl_auto_combos() -> HashMap<i32, Vec<&'static str>> {
    let mut auto_combos = HashMap::new();

    auto_combos.insert(0, vec!["light_punch", "heavy_punch", "medium_punch"]);
    auto_combos.insert(1, vec!["light_kick"]);

    auto_combos.insert(2, vec!["airborne_poke", "airborne_slash", "airborne_punch"]);
    auto_combos.insert(3, vec!["airborne_light_kick"]);

    auto_combos
}

fn load_foxgirl_data() -> EntityData {
    EntityData {
        auto_combo_strings: load_foxgirl_auto_combos(),
        directional_variation_anims: load_foxgirl_directional_inputs(),
        attacks: load_foxgirl_attacks(),
    }
}
