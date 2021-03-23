use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use std::fmt;
use std::collections::HashMap;
use super::game_input::GameInputs;

use crate::rendering::renderer;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlayerState {
    Standing,
    Crouch,
    Crouching,
    UnCrouch,
    Jumping
}
impl fmt::Display for PlayerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Player<'a>{
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub dash_speed: i32,
    pub prev_direction: i32,
    pub direction: i32,
    pub dir_related_of_other: i32,
    pub state: PlayerState,
    pub isAttacking: bool,
    pub animation_index: f32,
    pub current_animation: &'a Vec<Texture<'a>>,
    pub animations: &'a HashMap<std::string::String, Vec<Texture<'a>>>,
    pub flipped: bool,
    pub input_combination_anims: &'a Vec<([GameInputs; 5], &'a str)>
}

/*
pub fn load_character(texture_creator: &TextureCreator<WindowContext>, character_name: std::string::String) -> Player {
    let anims = load_character_anims(texture_creator, character_name);

    let mut player = Player {
        position: Point::new(-100, 0),
        sprite: Rect::new(0, 0, 290, 178),
        speed: 5,
        dash_speed: 10,
        prev_direction: 0,
        direction: 0,
        state: PlayerState::Standing,
        isAttacking: false,
        animation_index: 0.0,
        animations: anims,
        current_animation: anims.get(&"idle".to_string()).unwrap(),
    };

    player
}
*/

pub fn load_character_anims(texture_creator: &TextureCreator<WindowContext>, character_name: std::string::String) -> HashMap<std::string::String, Vec<Texture>>{
    let mut character_anims = HashMap::new();

    //TODO iterate through folders and use folder name as key for hashmap
    let idle_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/idle_anim", character_name).to_string());
    let walk_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/walk_anim", character_name).to_string());
    let walk_back_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/walk_back_anim", character_name).to_string());
    let crouch_start_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/crouch/crouched", character_name).to_string());
    let crouch_idle_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/crouch/crouching", character_name).to_string());
    let light_punch_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/light_punch", character_name).to_string());
    let special1_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/attacks/specials/directionals", character_name).to_string());
    let dash_anim: Vec<Texture> = renderer::load_anim_from_dir(&texture_creator, format!("assets/{}/standing/dash", character_name).to_string());

    character_anims.insert("idle".to_string(), idle_anim);
    character_anims.insert("walk".to_string(), walk_anim);
    character_anims.insert("walk_back".to_string(), walk_back_anim);
    character_anims.insert("light_punch".to_string(), light_punch_anim);
    character_anims.insert("crouch".to_string(), crouch_start_anim);
    character_anims.insert("crouching".to_string(), crouch_idle_anim);
    character_anims.insert("directional_light_punch".to_string(), special1_anim);
    character_anims.insert("dash".to_string(), dash_anim);

    character_anims
}

