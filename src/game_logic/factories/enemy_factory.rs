use std::collections::HashMap;

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

    let light_punch_anim = 
        load_anim_and_data_from_dir("assets/keetar/standing/attacks/attack", "attack");

    let neutral_jump_anim=
        load_anim_and_data_from_dir("assets/keetar/standing/neutral_jump","neutral_jump");

    let dead_anim =
        load_anim_from_dir("assets/keetar/dead", "dead");

    let take_damage_anim =
        load_anim_and_data_from_dir("assets/keetar/standing/take_damage", "take_damage");

    let launched_anim =
        load_anim_and_data_from_dir("assets/keetar/launched", "launched");
    let knocked_landing_anim =
        load_anim_and_data_from_dir("assets/keetar/knock_land", "knock_land");

    let mut character_anims = HashMap::new();
    
    character_anims.insert(idle_anim.name.clone(), idle_anim);

    character_anims.insert(walk_anim.name.clone(), walk_anim);
    
    character_anims.insert(light_punch_anim.name.clone(), light_punch_anim);
        
    character_anims.insert(crouch_start_anim.name.clone(), crouch_start_anim);

    character_anims.insert(neutral_jump_anim.name.clone(), neutral_jump_anim);

    character_anims.insert(dead_anim.name.clone(), dead_anim);
    character_anims.insert(take_damage_anim.name.clone(), take_damage_anim);

    character_anims.insert(launched_anim.name.clone(), launched_anim);
    character_anims.insert(knocked_landing_anim.name.clone(), knocked_landing_anim);

    character_anims
}

pub fn load_enemy(character_name: &str) -> Character {
    match character_name {
        "ryu" => Some(Character::new(
            character_name.to_string(),
            240,
            200,
            50,
            250.0,
            350.0,
            500.0,
            600.0,
            false,
            false,
            2,
            2,
            0,
            0,
            0b0000u32,
            2,
            2,
            0,
            0,
        )),

        _ => None,
    }
    .unwrap()
}

