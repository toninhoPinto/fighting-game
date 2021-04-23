use super::{characters::{Ability, Attack, AttackHeight, AttackType, keetar, player::Player}};
use sdl2::rect::Point;
use parry2d::na::Vector2;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use super::characters::Character;
use super::inputs::game_inputs::GameAction;
use super::projectile::Projectile;
use crate::asset_management::animation::Animation;
use crate::asset_management::{asset_loader, collider::ColliderAnimation};
use std::collections::HashMap;
use std::string::String;

pub struct CharacterAssets<'a> {
    pub animations: HashMap<String, Animation<'a>>,
    pub input_combination_anims: Vec<(Vec<i32>, String)>,
    pub directional_variation_anims: Vec<((GameAction, GameAction), String)>,
    pub projectiles: HashMap<String, Projectile>,
    pub projectile_animation: HashMap<String, Vec<(i32, Texture<'a>)>>,
    pub collider_animations: HashMap<String, ColliderAnimation>,
    pub attack_points: HashMap<String, Point>,
    pub attack_effects: HashMap<String, (i32, Ability)>,
    pub attacks: HashMap<String, Attack>,
}

pub fn load_stage(texture_creator: &TextureCreator<WindowContext>) -> Texture {
    asset_loader::load_texture(&texture_creator, "assets/stages/Sf3si-hugo.png")
}

pub fn load_character(character_name: &str, spawn_pos: Point, flipped: bool, id: i32) -> Player {
    let fighter = match character_name {
        "foxgirl" => Some(Character::new(
            character_name.to_string(),
            406 * 2,
            215 * 2,
            200,
            4,
            250.0,
            350.0,
            650.0,
            700.0,
            600.0,
        )),
        "keetar" => Some(Character::new(
            character_name.to_string(),
            580,
            356,
            100,
            3,
            350.0,
            570.0,
            600.0,
            600.0,
            500.0,
        )),
        _ => None,
    }
    .unwrap();
    Player::new(id, fighter, spawn_pos, flipped)
}

pub fn load_character_anim_data<'a, 'b>(
    texture_creator: &'a TextureCreator<WindowContext>,
    name: &'b str,
) -> CharacterAssets<'a> {
    match name {
        "foxgirl" => Some(load_foxgirl_assets(texture_creator)),
        "keetar" => Some(load_keetar_assets(texture_creator)),
        _ => None,
    }
    .unwrap()
}

//===========================================================

fn load_keetar_abilities() -> HashMap<String, (i32, Ability)> {
    let mut abilities = HashMap::new();

    abilities.insert("light_special_attack".to_string(),  (3, keetar::spawn_note as Ability));
    abilities.insert("med_special_attack".to_string(),  (3, keetar::spawn_note as Ability));
    abilities.insert("heavy_special_attack".to_string(),  (3, keetar::spawn_note as Ability));

    abilities
}

fn load_keetar_directional_inputs() -> Vec<((GameAction, GameAction), String)> {
    let mut directional_inputs: Vec<((GameAction, GameAction), String)> = Vec::new();
    let directional_string = (GameAction::Forward, GameAction::LightPunch);
    directional_inputs.push((directional_string, "directional_light_punch".to_string()));

    directional_inputs
}

fn load_keetar_special_inputs() -> Vec<(Vec<i32>, String)> {
    let mut specials_inputs: Vec<(Vec<i32>, String)> = Vec::new();
    let light_combo_string: Vec<i32> = vec![
        GameAction::Down as i32,
        GameAction::Down as i32 + GameAction::Forward as i32,
        GameAction::Forward as i32,
        GameAction::LightPunch as i32,
    ];
    specials_inputs.push((light_combo_string, "light_special_attack".to_string()));
    
    let med_combo_string = vec![
        GameAction::Down as i32,
        GameAction::Down as i32 + GameAction::Forward as i32,
        GameAction::Forward as i32,
        GameAction::MediumPunch as i32,
    ];
    specials_inputs.push((med_combo_string, "med_special_attack".to_string()));

    let heavy_combo_string: Vec<i32> = vec![
        GameAction::Down as i32,
        GameAction::Down as i32 + GameAction::Forward as i32,
        GameAction::Forward as i32,
        GameAction::HeavyPunch as i32,
    ];
    specials_inputs.push((heavy_combo_string, "heavy_special_attack".to_string()));

    specials_inputs
}

fn load_keetar_special_abilities() -> HashMap<String, Projectile> {
    let mut effects_of_abilities = HashMap::new();
    let light_projectile = Projectile::new(0, Vector2::new(120.0, 5.0));
    let med_projectile = Projectile::new(0, Vector2::new(120.0, 105.0));
    let heavy_projectile = Projectile::new(0, Vector2::new(120.0, 205.0));

    effects_of_abilities.insert("light_special_attack".to_string(), light_projectile);
    effects_of_abilities.insert("med_special_attack".to_string(), med_projectile);
    effects_of_abilities.insert("heavy_special_attack".to_string(), heavy_projectile);
    effects_of_abilities
}

fn load_keetar_projectile_anims<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> HashMap<String, Vec<(i32, Texture<'a>)>> {
    let mut projectile_anims = HashMap::new();
    let projectile_anim: Vec<(i32, Texture)> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/projectiles",
    );
    projectile_anims.insert("note".to_string(), projectile_anim);

    projectile_anims
}

fn load_keetar_anims(
    texture_creator: &TextureCreator<WindowContext>,
) -> (HashMap<String, Animation>, HashMap<String, ColliderAnimation>) {

    //TODO iterate through folders and use folder name as key for hashmap
    let (idle_anim, idle_colliders)  =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/standing/idle");
    let (walk_anim, walk_colliders)  =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/standing/walk");
    let (walk_back_anim, walk_back_colliders)  =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/standing/walk_back");
    let (crouch_start_anim, crouch_start_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/crouch/crouched");
    let (crouch_idle_anim, crouch_idle_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/crouch/crouching");
    let (light_punch_anim, light_punch_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/light_punch",
    );
    let (medium_punch_anim, medium_punch_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/medium_punch",
    );
    let (heavy_punch_anim, heavy_punch_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/heavy_punch",
    );
    let (light_kick_anim, light_kick_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/light_kick",
    );
    let (special1_anim, special1_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/specials/directionals/directional_light_punch",
    );
    let (special2_anim, special2_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/specials/combinations",
    );
    let (dash_anim, dash_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/standing/dash");
    let (dash_back_anim, dash_back_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/standing/back_dash");
    let (neutral_jump_anim, neutral_jump_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/standing/neutral_jump");
    let (directional_jump_anim, directional_jump_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/keetar/standing/directional_jump",
    );
    let (grab_anim, grab_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/standing/attacks/grab");
    let (dead_anim, dead_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/dead");
    let (take_damage_anim, take_damage_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/keetar/standing/take_damage");


    let mut character_anims = HashMap::new();
    let mut collider_anims = HashMap::new();
    
    let idle = "idle".to_string();
    if let Some(idle_c) = idle_colliders {
        collider_anims.insert(
            idle.clone(),
            idle_c
        );
    }
    character_anims.insert(
        idle.clone(),
        Animation::new(idle_anim, idle, None),
    );

    let dash = "dash".to_string();
    if let Some(dash_c) = dash_colliders {
        collider_anims.insert(
            dash.clone(),
            dash_c
        );
    }
    character_anims.insert(
        dash.clone(),
        Animation::new(dash_anim, dash, Some(vec![Vector2::new(1900.0, 0.0), Vector2::new(1700.0, 0.0)
        , Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)])),
    );

    let dash_back = "dash_back".to_string();
    if let Some(dash_back_c) = dash_back_colliders {
        collider_anims.insert(
            dash_back.clone(),
            dash_back_c
        );
    }
    character_anims.insert(
        dash_back.clone(),
        Animation::new(dash_back_anim, dash_back, Some(vec![Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(-1700.0, 0.0)
        , Vector2::new(-1000.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)])),
    );

    let walk = "walk".to_string();
    if let Some(walk_c) = walk_colliders {
        collider_anims.insert(
            walk.clone(),
            walk_c
        );
    }
    character_anims.insert(
        walk.clone(),
        Animation::new(walk_anim, walk, None),
    );

    let walk_back = "walk_back".to_string();
    if let Some(walk_back_c) = walk_back_colliders {
        collider_anims.insert(
            walk_back.clone(),
            walk_back_c
        );
    }
    character_anims.insert(
        walk_back.clone(),
        Animation::new(walk_back_anim, walk_back, None),
    );

    let light_punch = "light_punch".to_string();
    if let Some(light_punch_c) = light_punch_colliders {
        collider_anims.insert(
            light_punch.clone(),
            light_punch_c
        );
    }
    character_anims.insert(
        light_punch.clone(),
        Animation::new(light_punch_anim, light_punch, None),
    );

    let medium_punch = "medium_punch".to_string();
    if let Some(medium_punch_c) = medium_punch_colliders {
        collider_anims.insert(
            medium_punch.clone(),
            medium_punch_c
        );
    }   
    character_anims.insert(
        medium_punch.clone(),
        Animation::new(medium_punch_anim, medium_punch, None),
    );

    let heavy_punch = "heavy_punch".to_string();
    if let Some(heavy_punch_c) = heavy_punch_colliders {
        collider_anims.insert(
            heavy_punch.clone(),
            heavy_punch_c
        );
    }   
    character_anims.insert(
        heavy_punch.clone(),
        Animation::new(heavy_punch_anim, heavy_punch, None),
    );

    let light_kick = "light_kick".to_string();
    if let Some(light_kick_c) = light_kick_colliders {
        collider_anims.insert(
            light_kick.clone(),
            light_kick_c
        );
    }
    character_anims.insert(
        light_kick.clone(),
        Animation::new(light_kick_anim, light_kick, None),
    );

    let crouch = "crouch".to_string();
    if let Some(crouch_start_c) = crouch_start_colliders {
        collider_anims.insert(
            crouch.clone(),
            crouch_start_c
        );
    }
    character_anims.insert(
        crouch.clone(),
        Animation::new(crouch_start_anim, crouch, None),
    );

    let crouching = "crouching".to_string();
    if let Some(crouch_idle_c) = crouch_idle_colliders {
        collider_anims.insert(
            crouching.clone(),
            crouch_idle_c
        );
    }
    character_anims.insert(
        crouching.clone(),
        Animation::new(crouch_idle_anim, crouching, None),
    );

    let neutral_jump = "neutral_jump".to_string();
    if let Some(neutral_jump_colliders) = neutral_jump_colliders {
        collider_anims.insert(
            neutral_jump.clone(),
            neutral_jump_colliders
        );
    }
    character_anims.insert(
        neutral_jump.clone(),
        Animation::new(neutral_jump_anim, neutral_jump, None),
    );

    let directional_jump = "directional_jump".to_string();
    if let Some(directional_jump_colliders) = directional_jump_colliders {
        collider_anims.insert(
            directional_jump.clone(),
            directional_jump_colliders
        );
    }
    character_anims.insert(
        directional_jump.clone(),
        Animation::new(directional_jump_anim, directional_jump, None),
    );

    let directional_light_punch = "directional_light_punch".to_string();
    if let Some(special1_colliders) = special1_colliders {
        collider_anims.insert(
            directional_light_punch.clone(),
            special1_colliders
        );
    }
    character_anims.insert(
        directional_light_punch.clone(),
        Animation::new(special1_anim, directional_light_punch, None),
    );

    let grab = "grab".to_string();
    if let Some(grab_colliders) = grab_colliders {
        collider_anims.insert(
            grab.clone(),
            grab_colliders
        );
    }
    character_anims.insert(
        grab.clone(),
        Animation::new(grab_anim, grab, None),
    );

    let dead = "dead".to_string();
    if let Some(dead_colliders) = dead_colliders {
        collider_anims.insert(
            dead.clone(),
            dead_colliders
        );
    }
    character_anims.insert(
        dead.clone(),
        Animation::new(dead_anim, dead, None),
    );

    let take_damage = "take_damage".to_string();
    if let Some(take_damage_colliders) = take_damage_colliders {
        collider_anims.insert(
            take_damage.clone(),
            take_damage_colliders
        );
    }
    character_anims.insert(
        take_damage.clone(),
        Animation::new(take_damage_anim, take_damage, None),
    );

    let light_special_attack = "light_special_attack".to_string();
    if let Some(special2_colliders) = special2_colliders {
        collider_anims.insert(
            light_special_attack.clone(),
            special2_colliders
        );
    }
    character_anims.insert(
        light_special_attack.clone(),
        Animation::new(special2_anim, light_special_attack, None),
    );

    //TODO DUPLICATED DATA, not very good because its duplicated textures which are not particularly light 
    let (special3_anim,special3_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/specials/combinations",
    );
    let (special4_anim, special4_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/specials/combinations",
    );

    let med_special_attack = "med_special_attack".to_string();
    if let Some(special3_colliders) = special3_colliders {
        collider_anims.insert(
            med_special_attack.clone(),
            special3_colliders
        );
    }
    character_anims.insert(
        med_special_attack.clone(),
        Animation::new(special3_anim, "med_special_attack".to_string(), None),
    );

    let heavy_special_attack = "heavy_special_attack".to_string();
    if let Some(special4_colliders) = special4_colliders {
        collider_anims.insert(
            heavy_special_attack.clone(),
            special4_colliders
        );
    }
    character_anims.insert(
        heavy_special_attack.clone(),
        Animation::new(special4_anim, heavy_special_attack, None),
    );

    (character_anims, collider_anims)
}

fn load_keetar_attacks() -> HashMap<String, Attack> {
    let mut attacks = HashMap::new();

    attacks.insert(
        "lp".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 5,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "light_special_attack".to_string(),
        Attack {
            damage: 0,
            stun_on_hit: 0,
            stun_on_block: 0,
            push_back: 0,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Special
        },
    );

    attacks.insert(
        "med_special_attack".to_string(),
        Attack {
            damage: 0,
            stun_on_hit: 0,
            stun_on_block: 0,
            push_back: 0,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Special
        },
    );

    attacks.insert(
        "heavy_special_attack".to_string(),
        Attack {
            damage: 0,
            stun_on_hit: 0,
            stun_on_block: 0,
            push_back: 0,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Special
        },
    );

    attacks
}

fn load_keetar_assets(texture_creator: &TextureCreator<WindowContext>) -> CharacterAssets {
    let animations_data = load_keetar_anims(texture_creator);
    CharacterAssets {
        animations: animations_data.0,
        input_combination_anims: load_keetar_special_inputs(),
        directional_variation_anims: load_keetar_directional_inputs(),
        projectiles: load_keetar_special_abilities(),
        projectile_animation: load_keetar_projectile_anims(texture_creator),
        collider_animations: animations_data.1,
        attacks: load_keetar_attacks(),
        attack_points: HashMap::new(),
        attack_effects: load_keetar_abilities(),
    }
}

//===========================================================


fn load_foxgirl_directional_inputs() ->   Vec<((GameAction, GameAction), String)>{
    let mut directional_inputs: Vec<((GameAction, GameAction), String)> = Vec::new();

    let directional_string = (GameAction::Forward, GameAction::LightPunch);
    directional_inputs.push((directional_string, "directional_light_punch".to_string()));

    let directional_string_2 = (GameAction::Forward, GameAction::HeavyPunch);
    directional_inputs.push((directional_string_2, "directional_heavy_punch".to_string()));

    directional_inputs
}

fn load_foxgirl_special_inputs() ->   Vec<(Vec<i32>, String)>{
    let mut specials_inputs: Vec<(Vec<i32>, String)> = Vec::new();
    let spam_light_punch_inputs = vec![
        GameAction::LightPunch as i32,
        GameAction::LightPunch as i32,
        GameAction::LightPunch as i32,
    ];
    specials_inputs.push((spam_light_punch_inputs, "spam_light_punch".to_string()));

    specials_inputs
}

fn load_foxgirl_anims(
    texture_creator: &TextureCreator<WindowContext>,
) -> (HashMap<String, Animation>, HashMap<String, ColliderAnimation>) {
    //TODO iterate through folders and use folder name as key for hashmap
    let (idle_anim, idle_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/foxgirl/standing/idle");
    let (take_damage_anim, take_damage_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/foxgirl/standing/take_damage/1");
    let (dead_anim, dead_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/foxgirl/dead");
    let (walk_anim, walk_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/foxgirl/standing/walk");
    let (crouch_start_anim, crouch_start_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/foxgirl/crouch/crouched");
    let (crouch_idle_anim, crouch_idle_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/foxgirl/crouch/crouching");
    let (light_punch_anim, light_punch_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/light_punch",
    );
    let (medium_punch_anim, medium_punch_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/medium_punch",
    );
    let (heavy_punch_anim, heavy_punch_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/heavy_punch",
    );
    let (light_kick_anim, light_kick_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/light_kick",
    );
    let (crouched_light_kick_anim, crouched_light_kick_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/foxgirl/crouch/attacks/light_kick",
    );
    let (airborne_light_kick_anim, airborne_light_kick_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/foxgirl/airborne/attacks/light_kick",
    );
    let (special1_anim, special1_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/specials/directionals/forward_light_punch",
    );
    let (special2_anim, special2_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/specials/directionals/forward_heavy_punch",
    );
    let (spam_light_punch_anim, spam_light_punch_colliders) = asset_loader::load_anim_and_data_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/specials/spam/spam_light_punch",
    );
    let (dash_anim, dash_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/foxgirl/standing/dash");
    let (dash_back_anim, dash_back_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/foxgirl/standing/back_dash");
    let (neutral_jump_anim, neutral_jump_colliders) =
        asset_loader::load_anim_and_data_from_dir(&texture_creator, "assets/foxgirl/standing/neutral_jump");


    let mut character_anims = HashMap::new();
    let mut collider_anims = HashMap::new();

    let idle = "idle".to_string();
    if let Some(idle_colliders) = idle_colliders {
        collider_anims.insert(
            idle.clone(),
            idle_colliders,
        );
    }
    character_anims.insert(
        idle.clone(),
        Animation::new(idle_anim, idle, None),
    );

    let take_damage = "take_damage".to_string();
    if let Some(take_damage_colliders) = take_damage_colliders {
        collider_anims.insert(
            take_damage.clone(),
            take_damage_colliders,
        );
    }
    character_anims.insert(
        take_damage.clone(),
        Animation::new(take_damage_anim, take_damage, None),
    );

    let dead = "dead".to_string();
    if let Some(dead_colliders) = dead_colliders {
        collider_anims.insert(
            dead.clone(),
            dead_colliders,
        );
    }
    character_anims.insert(
        dead.clone(),
        Animation::new(dead_anim, dead, None),
    );

    let dash = "dash".to_string();
    if let Some(dash_colliders) = dash_colliders {
        collider_anims.insert(
            dash.clone(),
            dash_colliders,
        );
    }
    character_anims.insert(
        dash.clone(),
        Animation::new(dash_anim, dash, 
        Some(vec![Vector2::new(0.0, 0.0), Vector2::new(3000.0, 0.0), Vector2::new(2000.0, 0.0), 
            Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0) , Vector2::new(0.0, 0.0) , Vector2::new(0.0, 0.0) , Vector2::new(0.0, 0.0) , Vector2::new(0.0, 0.0)]))
    );

    let dash_back = "dash_back".to_string();
    if let Some(dash_back_colliders) = dash_back_colliders {
        collider_anims.insert(
            dash_back.clone(),
            dash_back_colliders,
        );
    }
    character_anims.insert(
        dash_back.clone(),
        Animation::new(dash_back_anim, dash_back, 
        Some(vec![Vector2::new(0.0, 0.0), Vector2::new(-1000.0, 0.0), Vector2::new(-600.0, 0.0), Vector2::new(0.0, 0.0)])),
    );

    let walk = "walk".to_string();
    if let Some(walk_colliders) = walk_colliders {
        collider_anims.insert(
            walk.clone(),
            walk_colliders,
        );
    }
    character_anims.insert(
        walk.clone(),
        Animation::new(walk_anim, walk, None),
    );

    let light_punch = "light_punch".to_string();
    if let Some(light_punch_colliders) = light_punch_colliders {
        collider_anims.insert(
            light_punch.clone(),
            light_punch_colliders,
        );
    }
    character_anims.insert(
        light_punch.clone(),
        Animation::new(light_punch_anim, light_punch, None),
    );
    
    let medium_punch = "medium_punch".to_string();
    if let Some(medium_punch_colliders) = medium_punch_colliders {
        collider_anims.insert(
            medium_punch.clone(),
            medium_punch_colliders,
        );
    }
    character_anims.insert(
        medium_punch.clone(),
        Animation::new(medium_punch_anim, medium_punch, Some(vec![Vector2::new(100.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(50.0, 0.0), 
        Vector2::new(1000.0, 0.0), Vector2::new(400.0, 0.0),Vector2::new(50.0, 0.0), Vector2::new(50.0, 0.0), Vector2::new(50.0, 0.0), Vector2::new(50.0, 0.0), 
        Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0),  Vector2::new(0.0, 0.0),  Vector2::new(0.0, 0.0),  Vector2::new(0.0, 0.0)])),
    );

    let heavy_punch = "heavy_punch".to_string();
    if let Some(heavy_punch_colliders) = heavy_punch_colliders {
        collider_anims.insert(
            heavy_punch.clone(),
            heavy_punch_colliders,
        );
    }
    character_anims.insert(
        heavy_punch.clone(),
        Animation::new(heavy_punch_anim, heavy_punch.to_string(), None),
    );

    let light_kick = "light_kick".to_string();
    if let Some(light_kick_colliders) = light_kick_colliders {
        collider_anims.insert(
            light_kick.clone(),
        light_kick_colliders,
        );
    }
    character_anims.insert(
        light_kick.clone(),
        Animation::new(light_kick_anim, light_kick,Some(vec![Vector2::new(400.0, 0.0), Vector2::new(100.0, 0.0), Vector2::new(0.0, 0.0), 
        Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0),Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(300.0, 0.0), Vector2::new(300.0, 0.0), 
        Vector2::new(300.0, 0.0), Vector2::new(300.0, 0.0),  Vector2::new(0.0, 0.0)])),
    );

    let airborne_light_kick = "airborne_light_kick".to_string();
    if let Some(airborne_light_kick_colliders) = airborne_light_kick_colliders {
        collider_anims.insert(
            airborne_light_kick.clone(),
            airborne_light_kick_colliders,
        );
    }
    character_anims.insert(
        airborne_light_kick.clone(),
        Animation::new(airborne_light_kick_anim,airborne_light_kick, None),
    );

    let crouched_light_kick = "crouched_light_kick".to_string();
    if let Some(crouched_light_kick_colliders) = crouched_light_kick_colliders {
        collider_anims.insert(
            crouched_light_kick.clone(),
            crouched_light_kick_colliders,
        );
    }
    character_anims.insert(
        crouched_light_kick.clone(),
        Animation::new(crouched_light_kick_anim,crouched_light_kick, None),
    );

    let crouch = "crouch".to_string();
    if let Some(crouch_start_colliders) = crouch_start_colliders {
        collider_anims.insert(
            crouch.clone(),
            crouch_start_colliders,
        );
    }
    character_anims.insert(
        crouch.clone(),
        Animation::new(crouch_start_anim, crouch, None),
    );

    let crouching = "crouching".to_string();
    if let Some(crouch_idle_colliders) = crouch_idle_colliders {
        collider_anims.insert(
            crouching.clone(),
            crouch_idle_colliders,
        );
    }
    character_anims.insert(
        crouching.clone(),
        Animation::new(crouch_idle_anim, crouching, None),
    );

    let neutral_jump = "neutral_jump".to_string();
    if let Some(neutral_jump_colliders) = neutral_jump_colliders {
        collider_anims.insert(
            neutral_jump.clone(),
            neutral_jump_colliders,
        );
    }
    character_anims.insert(
        neutral_jump.clone(),
        Animation::new(neutral_jump_anim, neutral_jump, None),
    );

    let forward_light_punch = "forward_light_punch".to_string();
    if let Some(special1_colliders) = special1_colliders {
        collider_anims.insert(
            forward_light_punch.clone(),
            special1_colliders,
        );
    }
    character_anims.insert(
        forward_light_punch.clone(),
        Animation::new(special1_anim, forward_light_punch, None),
    );

    let forward_heavy_punch = "forward_heavy_punch".to_string();
    if let Some(special2_colliders) = special2_colliders {
        collider_anims.insert(
            forward_heavy_punch.clone(),
            special2_colliders,
        );
    }
    character_anims.insert(
        forward_heavy_punch.clone(),
        Animation::new(special2_anim, forward_heavy_punch, None),
    );

    let spam_light_punch = "spam_light_punch".to_string();
    if let Some(spam_light_punch_colliders) = spam_light_punch_colliders {
        collider_anims.insert(
            spam_light_punch.clone(),
            spam_light_punch_colliders,
        );
    }
    character_anims.insert(
        spam_light_punch.clone(),
        Animation::new(spam_light_punch_anim, spam_light_punch,None),
    );

    (character_anims, collider_anims)
}

fn load_foxgirl_attacks() -> HashMap<String, Attack> {
    let mut attacks = HashMap::new();

    attacks.insert(
        "lk".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 5,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "c.lk".to_string(),
        Attack {
            damage: 15,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 30,
            attack_height: AttackHeight::LOW,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "j.lk".to_string(),
        Attack {
            damage: 15,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 30,
            attack_height: AttackHeight::HIGH,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "lp".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 5,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "mp".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 10,
            stun_on_block: 4,
            push_back: 5,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "hp".to_string(),
        Attack {
            damage: 10,
            stun_on_hit: 20,
            stun_on_block: 14,
            push_back: 10,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Normal
        },
    );

    attacks.insert(
        "first-spam-punch".to_string(),
        Attack {
            damage: 5,
            stun_on_hit: 20,
            stun_on_block: 14,
            push_back: 10,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Special
        },
    );

    attacks.insert(
        "fast-spam-punch".to_string(),
        Attack {
            damage: 2,
            stun_on_hit: 20,
            stun_on_block: 14,
            push_back: 10,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Special
        },
    );

    attacks.insert(
        "last-spam-punch".to_string(),
        Attack {
            damage: 20,
            stun_on_hit: 20,
            stun_on_block: 14,
            push_back: 10,
            attack_height: AttackHeight::MIDDLE,
            attack_type: AttackType::Special
        },
    );


    attacks
}

fn load_foxgirl_assets(texture_creator: &TextureCreator<WindowContext>) -> CharacterAssets {
    let animations_data = load_foxgirl_anims(texture_creator);
    CharacterAssets {
        animations: animations_data.0,
        input_combination_anims: load_foxgirl_special_inputs(),
        directional_variation_anims: load_foxgirl_directional_inputs(),
        projectiles: HashMap::new(),
        projectile_animation: HashMap::new(),
        collider_animations: animations_data.1,
        attacks: load_foxgirl_attacks(),
        attack_points: HashMap::new(),
        attack_effects: HashMap::new(),
    }
}
