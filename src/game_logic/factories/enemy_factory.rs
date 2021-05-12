use std::collections::HashMap;

use parry2d::na::Vector2;
use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{asset_management::{asset_holders::{EntityAnimations, EntityAssets}, asset_loader::asset_loader::{load_anim_and_data_from_dir, load_anim_from_dir, load_textures_for_character}}, engine_types::{animation::Animation, sprite_data::SpriteData}, game_logic::characters::{Attack, Character}};

pub fn load_enemy_ryu_assets(texture_creator: &TextureCreator<WindowContext>) -> EntityAssets {
    let (textures, data) = load_textures_for_character(texture_creator, "assets/keetar");
    EntityAssets {
        textures,
        texture_data: data,
    }
}

pub fn load_enemy_ryu_animations() -> EntityAnimations {
    let anims= load_enemy_ryu_anims();
    EntityAnimations {
        animations: anims,
        projectile_animation: HashMap::new(),
    }
}

fn load_enemy_ryu_anims() -> HashMap<String, Animation> {

    let idle_anim  =
        load_anim_and_data_from_dir("assets/keetar/standing/idle", "idle");
    
    let walk_anim  =
        load_anim_and_data_from_dir("assets/keetar/standing/walk", "walk");

    let crouch_start_anim=
        load_anim_and_data_from_dir("assets/keetar/crouch/crouched", "crouch");

    let crouch_idle_anim=
       load_anim_and_data_from_dir("assets/keetar/crouch/crouching", "crouching");

    let light_punch_anim = 
        load_anim_and_data_from_dir("assets/keetar/standing/attacks/light_punch", "light_punch");

    let medium_punch_anim= 
        load_anim_and_data_from_dir("assets/keetar/standing/attacks/medium_punch", "medium_punch");

    let heavy_punch_anim = 
        load_anim_and_data_from_dir("assets/keetar/standing/attacks/heavy_punch", "heavy_punch");
    
    let light_kick_anim = 
        load_anim_and_data_from_dir("assets/keetar/standing/attacks/light_kick", "light_kick");

    let special1_anim= 
        load_anim_and_data_from_dir("assets/keetar/standing/attacks/specials/directionals/directional_light_punch", "directional_light_punch");

    let mut dash_anim =
        load_anim_and_data_from_dir("assets/keetar/standing/dash", "dash");

    let neutral_jump_anim=
        load_anim_and_data_from_dir("assets/keetar/standing/neutral_jump","neutral_jump");

    let directional_jump_anim = 
        load_anim_from_dir("assets/keetar/standing/directional_jump", "directional_jump");

    let grab_anim =
        load_anim_from_dir("assets/keetar/standing/attacks/grab", "grab");

    let dead_anim =
        load_anim_from_dir("assets/keetar/dead", "dead");

    let take_damage_anim =
        load_anim_and_data_from_dir("assets/keetar/standing/take_damage", "take_damage");

    let mut character_anims = HashMap::new();
    
    character_anims.insert(idle_anim.name.clone(), idle_anim);


    dash_anim.offsets = Some(vec![Vector2::new(1900.0, 0.0), Vector2::new(1700.0, 0.0)
    , Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)]);
    character_anims.insert(dash_anim.name.clone(), dash_anim);

    character_anims.insert(walk_anim.name.clone(), walk_anim);
    
    character_anims.insert(light_punch_anim.name.clone(), light_punch_anim);
    character_anims.insert(medium_punch_anim.name.clone(), medium_punch_anim);
    character_anims.insert(heavy_punch_anim.name.clone(), heavy_punch_anim);
    
    character_anims.insert(light_kick_anim.name.clone(), light_kick_anim);
    
    character_anims.insert(crouch_start_anim.name.clone(), crouch_start_anim);
    character_anims.insert(crouch_idle_anim.name.clone(), crouch_idle_anim);

    character_anims.insert(neutral_jump_anim.name.clone(), neutral_jump_anim);
    character_anims.insert(directional_jump_anim.name.clone(), directional_jump_anim);

    character_anims.insert(special1_anim.name.clone(), special1_anim);

    character_anims.insert(grab_anim.name.clone(), grab_anim);

    character_anims.insert(dead_anim.name.clone(), dead_anim);
    character_anims.insert(take_damage_anim.name.clone(), take_damage_anim);

    let special_anim = 
    load_anim_and_data_from_dir("assets/keetar/standing/attacks/specials/combinations", "special_attack");

    let special = special_anim.clone();
    character_anims.insert("light_special_attack".to_string(), special_anim);
    character_anims.insert("med_special_attack".to_string(), special.clone());
    character_anims.insert("heavy_special_attack".to_string(), special.clone());

    character_anims
}

pub fn load_enemy(character_name: &str) -> Character {
    match character_name {
        "ryu" => Some(Character::new(
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
    .unwrap()
}

