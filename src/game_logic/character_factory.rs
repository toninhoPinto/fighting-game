use super::characters::{Attack, AttackType, player::Player};
use sdl2::rect::Point;
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
    pub effects: HashMap<String, Projectile>,
    pub projectile_animation: HashMap<String, Vec<Texture<'a>>>,
    pub collider_animations: HashMap<String, ColliderAnimation>,
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
            450.0,
            650.0,
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

fn load_keetar_colliders() -> HashMap<String, ColliderAnimation> {
    let mut collider_animations = HashMap::new();

    collider_animations.insert(
        "idle".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/idle/idle.json", "keetar"),
        ),
    );
    collider_animations.insert(
        "light_punch".to_string(),
        asset_loader::load_hitboxes(
            format!(
                "assets/{}/standing/attacks/light_punch/light_punch.json",
                "keetar"
            )
            .to_string(),
        ),
    );
    collider_animations.insert(
        "walk".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/walk/walk.json", "keetar"),
        ),
    );
    collider_animations.insert(
        "walk_back".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/walk_back/walk_back.json", "keetar"),
        ),
    );
    collider_animations.insert(
        "dash".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/dash/dash.json", "keetar"),
        ),
    );

    collider_animations.insert(
        "neutral_jump".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/neutral_jump/jump.json", "keetar"),
        ),
    );

    collider_animations
}

fn load_keetar_assets(texture_creator: &TextureCreator<WindowContext>) -> CharacterAssets {
    let anims = load_keetar_anims(texture_creator);

    let mut directional_inputs: Vec<((GameAction, GameAction), String)> = Vec::new();
    let directional_string= (GameAction::Forward,GameAction::LightPunch);
    directional_inputs.push((directional_string, "directional_light_punch".to_string()));

    let mut specials_inputs: Vec<(Vec<i32>, String)> = Vec::new();
    let light_combo_string:Vec<i32>= vec![
        GameAction::Down as i32,
        GameAction::Down as i32 +  GameAction::Forward as i32,
        GameAction::Forward as i32,
        GameAction::LightPunch as i32
    ];
    specials_inputs.push((light_combo_string, "light_special_attack".to_string()));
    let light_projectile = Projectile::new(0, Point::new(120, 5));

    let med_combo_string= vec![
        GameAction::Down as i32,
        GameAction::Down as i32 +  GameAction::Forward as i32,
        GameAction::Forward as i32,
        GameAction::MediumPunch as i32
    ];
    specials_inputs.push((med_combo_string, "med_special_attack".to_string()));
    let med_projectile = Projectile::new(0, Point::new(120, 105));

    let heavy_combo_string: Vec<i32> = vec![
        GameAction::Down as i32,
        GameAction::Down as i32 +  GameAction::Forward as i32,
        GameAction::Forward as i32,
        GameAction::HeavyPunch as i32
    ];
    specials_inputs.push((heavy_combo_string, "heavy_special_attack".to_string()));
    let heavy_projectile = Projectile::new(0, Point::new(120, 205));

    let mut effects_of_abilities = HashMap::new();
    effects_of_abilities.insert("light_special_attack".to_string(), light_projectile);
    effects_of_abilities.insert("med_special_attack".to_string(), med_projectile);
    effects_of_abilities.insert("heavy_special_attack".to_string(), heavy_projectile);

    let mut projectile_anims = HashMap::new();
    let projectile_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/projectiles",
    );
    projectile_anims.insert("note".to_string(), projectile_anim);

    CharacterAssets {
        animations: anims,
        input_combination_anims: specials_inputs,
        directional_variation_anims: directional_inputs,
        effects: effects_of_abilities,
        projectile_animation: projectile_anims,
        collider_animations: load_keetar_colliders(),
        attacks: load_keetar_attacks(),
    }
}

fn load_keetar_anims(
    texture_creator: &TextureCreator<WindowContext>,
) -> HashMap<String, Animation> {
    let mut character_anims = HashMap::new();

    //TODO iterate through folders and use folder name as key for hashmap
    let idle_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/standing/idle");
    let walk_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/standing/walk");
    let walk_back_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/standing/walk_back");
    let crouch_start_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/crouch/crouched");
    let crouch_idle_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/crouch/crouching");
    let light_punch_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/light_punch",
    );
    let medium_punch_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/medium_punch",
    );
    let heavy_punch_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/heavy_punch",
    );
    let light_kick_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/light_kick",
    );
    let special1_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/specials/directionals/directional_light_punch",
    );
    let special2_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/specials/combinations",
    );
    let dash_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/standing/dash");
    let dash_back_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/standing/back_dash");
    let neutral_jump_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/standing/neutral_jump");
    let directional_jump_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/directional_jump",
    );
    let grab_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/standing/attacks/grab");
    let dead_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/dead");
    let take_damage_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/keetar/standing/take_damage");

    character_anims.insert(
        "idle".to_string(),
        Animation::new(idle_anim, "idle".to_string(), 0.35),
    );
    character_anims.insert(
        "dash".to_string(),
        Animation::new(dash_anim, "dash".to_string(), 0.35),
    );
    character_anims.insert(
        "dash_back".to_string(),
        Animation::new(dash_back_anim, "dash_back".to_string(), 0.5),
    );
    character_anims.insert(
        "walk".to_string(),
        Animation::new(walk_anim, "walk".to_string(), 0.35),
    );
    character_anims.insert(
        "walk_back".to_string(),
        Animation::new(walk_back_anim, "walk_back".to_string(), 0.35),
    );
    character_anims.insert(
        "light_punch".to_string(),
        Animation::new(light_punch_anim, "light_punch".to_string(), 0.35),
    );
    character_anims.insert(
        "medium_punch".to_string(),
        Animation::new(medium_punch_anim, "medium_punch".to_string(), 0.35),
    );
    character_anims.insert(
        "heavy_punch".to_string(),
        Animation::new(heavy_punch_anim, "heavy_punch".to_string(), 0.35),
    );
    character_anims.insert(
        "light_kick".to_string(),
        Animation::new(light_kick_anim, "light_kick".to_string(), 0.35),
    );
    character_anims.insert(
        "crouch".to_string(),
        Animation::new(crouch_start_anim, "crouch".to_string(), 0.35),
    );
    character_anims.insert(
        "crouching".to_string(),
        Animation::new(crouch_idle_anim, "crouching".to_string(), 0.35),
    );
    character_anims.insert(
        "neutral_jump".to_string(),
        Animation::new(neutral_jump_anim, "neutral_jump".to_string(), 0.35),
    );
    character_anims.insert(
        "directional_jump".to_string(),
        Animation::new(directional_jump_anim, "directional_jump".to_string(), 0.35),
    );
    character_anims.insert(
        "directional_light_punch".to_string(),
        Animation::new(special1_anim, "directional_light_punch".to_string(), 0.35),
    );
    character_anims.insert(
        "grab".to_string(),
        Animation::new(grab_anim, "grab".to_string(), 0.15),
    );
    character_anims.insert(
        "dead".to_string(),
        Animation::new(dead_anim, "dead".to_string(), 0.15),
    );
    character_anims.insert(
        "take_damage".to_string(),
        Animation::new(take_damage_anim, "take_damage".to_string(), 0.35),
    );

    character_anims.insert(
        "light_special_attack".to_string(),
        Animation::new(special2_anim, "light_special_attack".to_string(), 0.35),
    );

    //TODO DUPLICATED DATA, i think the only solution is to have a separate texture manager and character anims becomes a hashmap<string, id on texturemanager>
    let special3_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/specials/combinations",
    );
    let special4_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/keetar/standing/attacks/specials/combinations",
    );
    character_anims.insert(
        "med_special_attack".to_string(),
        Animation::new(special3_anim, "med_special_attack".to_string(), 0.35),
    );
    character_anims.insert(
        "heavy_special_attack".to_string(),
        Animation::new(special4_anim, "heavy_special_attack".to_string(), 0.35),
    );

    character_anims
}

fn load_keetar_attacks() -> HashMap<String,Attack> {
    let mut attacks = HashMap::new();
    
    attacks.insert("light_punch".to_string(), Attack {
        damage: 5,
        stun_on_hit: 10,
        stun_on_block: 4,
        push_back: 5,
        attack_move: 10,
        attack_type: AttackType::MIDDLE,
    });

    attacks
}

//===========================================================

fn load_foxgirl_colliders() -> HashMap<String, ColliderAnimation> {
    let mut collider_animations = HashMap::new();

    collider_animations.insert(
        "idle".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/idle/idle.json", "foxgirl"),
        ),
    );

    collider_animations.insert(
        "walk".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/walk/walk.json", "foxgirl"),
        ),
    );

    collider_animations.insert(
        "dash".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/dash/dash.json", "foxgirl"),
        ),
    );

    collider_animations.insert(
        "light_punch".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/attacks/light_punch/light_punch.json", "foxgirl"),
        ),
    );

    collider_animations.insert(
        "light_kick".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/attacks/light_kick/light_kick.json", "foxgirl"),
        ),
    );

    collider_animations.insert(
        "airborne_light_kick".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/airborne/attacks/light_kick/airborne_light_kick.json", "foxgirl"),
        ),
    );


    collider_animations.insert(
        "medium_punch".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/attacks/medium_punch/medium_punch.json", "foxgirl"),
        ),
    );

    collider_animations.insert(
        "heavy_punch".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/attacks/heavy_punch/heavy_punch.json", "foxgirl"),
        ),
    );

    collider_animations.insert(
        "neutral_jump".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/standing/neutral_jump/neutral_jump.json", "foxgirl"),
        ),
    );

    collider_animations.insert(
        "crouch".to_string(),
        asset_loader::load_hitboxes(
            format!("assets/{}/crouch/crouched/crouched.json", "foxgirl"),
        ),
    );

    collider_animations
}

fn load_foxgirl_assets(texture_creator: &TextureCreator<WindowContext>) -> CharacterAssets {
    let anims = load_foxgirl_anims(texture_creator);

    let mut directional_inputs: Vec<((GameAction, GameAction), String)> = Vec::new();

    let directional_string= (GameAction::Forward,GameAction::LightPunch);
    directional_inputs.push((directional_string, "directional_light_punch".to_string()));

    let directional_string_2 = (GameAction::Forward, GameAction::HeavyPunch);
    directional_inputs.push((directional_string_2, "directional_heavy_punch".to_string()));

    let effects_of_abilities = HashMap::new();
    let specials_inputs: Vec<(Vec<i32>, String)> = Vec::new();
    let projectile_anims = HashMap::new();

    CharacterAssets {
        animations: anims,
        input_combination_anims: specials_inputs,
        directional_variation_anims: directional_inputs,
        effects: effects_of_abilities,
        projectile_animation: projectile_anims,
        collider_animations: load_foxgirl_colliders(),
        attacks: load_foxgirl_attacks(),
    }
}

fn load_foxgirl_anims(
    texture_creator: &TextureCreator<WindowContext>,
) -> HashMap<std::string::String, Animation> {
    let mut character_anims = HashMap::new();

    //TODO iterate through folders and use folder name as key for hashmap
    let idle_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/foxgirl/standing/idle");
    let take_damage_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/foxgirl/standing/take_damage/1");
    let dead_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/foxgirl/dead");
    let walk_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/foxgirl/standing/walk");
    let crouch_start_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/foxgirl/crouch/crouched");
    let crouch_idle_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/foxgirl/crouch/crouching");
    let light_punch_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/light_punch",
    );
    let medium_punch_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/medium_punch",
    );
    let heavy_punch_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/heavy_punch",
    );
    let light_kick_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/light_kick",
    );
    let airborne_light_kick_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/foxgirl/airborne/attacks/light_kick",
    );
    let special1_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/specials/directionals/forward_light_punch",
    );
    let special2_anim: Vec<Texture> = asset_loader::load_anim_from_dir(
        &texture_creator,
        "assets/foxgirl/standing/attacks/specials/directionals/forward_heavy_punch",
    );
    let dash_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/foxgirl/standing/dash");
    let dash_back_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/foxgirl/standing/back_dash");
    let neutral_jump_anim: Vec<Texture> =
        asset_loader::load_anim_from_dir(&texture_creator, "assets/foxgirl/standing/neutral_jump");

    character_anims.insert(
        "idle".to_string(),
        Animation::new(idle_anim, "idle".to_string(), 0.35),
    );
    character_anims.insert(
        "take_damage".to_string(),
        Animation::new(take_damage_anim, "take_damage".to_string(), 0.35),
    );
    character_anims.insert(
        "dead".to_string(),
        Animation::new(dead_anim, "dead".to_string(), 0.35),
    );
    character_anims.insert(
        "dash".to_string(),
        Animation::new(dash_anim, "dash".to_string(), 0.45),
    );
    character_anims.insert(
        "dash_back".to_string(),
        Animation::new(dash_back_anim, "dash_back".to_string(), 0.35),
    );
    character_anims.insert(
        "walk".to_string(),
        Animation::new(walk_anim, "walk".to_string(), 0.35),
    );
    character_anims.insert(
        "light_punch".to_string(),
        Animation::new(light_punch_anim, "light_punch".to_string(), 0.35),
    );
    character_anims.insert(
        "airborne_light_kick".to_string(),
        Animation::new(airborne_light_kick_anim, "airborne_light_kick".to_string(), 0.35),
    );
    character_anims.insert(
        "medium_punch".to_string(),
        Animation::new(medium_punch_anim, "medium_punch".to_string(), 0.35),
    );
    character_anims.insert(
        "heavy_punch".to_string(),
        Animation::new(heavy_punch_anim, "heavy_punch".to_string(), 0.35),
    );
    character_anims.insert(
        "light_kick".to_string(),
        Animation::new(light_kick_anim, "light_kick".to_string(), 0.35),
    );
    character_anims.insert(
        "crouch".to_string(),
        Animation::new(crouch_start_anim, "crouch".to_string(), 0.8),
    );
    character_anims.insert(
        "crouching".to_string(),
        Animation::new(crouch_idle_anim, "crouching".to_string(), 0.35),
    );
    character_anims.insert(
        "neutral_jump".to_string(),
        Animation::new(neutral_jump_anim, "neutral_jump".to_string(), 0.35),
    );
    character_anims.insert(
        "directional_light_punch".to_string(),
        Animation::new(special1_anim, "forward_light_punch".to_string(), 0.35),
    );
    character_anims.insert(
        "directional_heavy_punch".to_string(),
        Animation::new(special2_anim, "forward_heavy_punch".to_string(), 0.35),
    );

    character_anims
}

fn load_foxgirl_attacks() -> HashMap<String,Attack> {
    let mut attacks = HashMap::new();

    attacks.insert("light_kick".to_string(), Attack {
        damage: 5,
        stun_on_hit: 10,
        stun_on_block: 4,
        push_back: 5,
        attack_move: 10,
        attack_type: AttackType::MIDDLE,
    });

    attacks.insert("airborne_light_kick".to_string(), Attack {
        damage: 15,
        stun_on_hit: 10,
        stun_on_block: 4,
        push_back: 30,
        attack_move: 10,
        attack_type: AttackType::HIGH,
    });

    attacks.insert("light_punch".to_string(), Attack {
        damage: 5,
        stun_on_hit: 10,
        stun_on_block: 4,
        push_back: 5,
        attack_move: 10,
        attack_type: AttackType::MIDDLE,
    });

    attacks.insert("medium_punch".to_string(), Attack {
        damage: 5,
        stun_on_hit: 10,
        stun_on_block: 4,
        push_back: 5,
        attack_move: 10,
        attack_type: AttackType::MIDDLE,
    });

    attacks.insert("heavy_punch".to_string(), Attack {
        damage: 10,
        stun_on_hit: 20,
        stun_on_block: 14,
        push_back: 10,
        attack_move: 10,
        attack_type: AttackType::MIDDLE,
    });

    attacks
}