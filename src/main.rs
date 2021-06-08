use engine_traits::scene::Scene;
use game_logic::characters::player::Player;
use scenes::menu_scene::MenuScene;
use sdl2::{image::{self, InitFlag}, rect::Rect, ttf::Font};
use sdl2::render::BlendMode;

use std::{collections::HashMap, path::Path, rc::Rc};

extern crate serde_derive;
extern crate directories;

mod utils;
mod engine_types;
mod asset_management;
mod collision;
mod engine_traits;
mod game_logic;
mod input;
mod rendering;
mod ui;
mod scenes;
mod ecs_system;
mod enemy_behaviour;
mod overworld;
mod level_generation;

mod debug_console;

use asset_management::{asset_holders::{EntityAnimations, ItemAssets}, common_assets::CommonAssets, sound::{init_sound, music_player}};

use crate::{asset_management::controls, game_logic::factories::item_factory::load_item_assets, input::input_devices::InputDevices};
use crate::input::controller_handler::Controller;


//TODO features tomorrow
//add sound_effects
    //when player misses punches (very light sound)
    //add variation to sounds through pitch changing

//Add to game state data the seed AND the rng object_type
//make multiple rng objects, one for map generation
//one for effects
//improve overworld map generation ->  changing the connection level shouldnt re-generate everything
//implement more item effects

//add to game_state_data
    //add seed to game_state_data
    //make overworld display stuff like: currency ?
    
//make overworld proc gen map
    // replace lines with rotated squares with textures
    // decision making on the type of node (level, event, store)
    // add sounds when moving the arrow through the possible next levels
    // add confimation sound when selecting a level
    // add small up/down animation on the "character" icon

    // make a store Scene
    // make an event UI window appear on top

//separate json for animation offsets and animation states (startup, active, recovery)
//Improve AI

//TODO FEATURES
//play block animation while standing 
 //add hit combos and block combos, these should be displayed while they are happening and not at the end to give faster feedback
//charge special attacks like makoto where you can hold punch for a stronger attack
//dash attacks
//save sound volume settings on config
//add menu to change the controllers for each player
//add menu to change the controller mapping of keys/buttons
//make ui loop only 60fps to avoid the computer doing too many wasted computations

//TODO TECH DEBT AND BUGS~

//16 Match_scene no inicio faz clone de player para dentro do game, e vice versa no fim, secalhar pode usar directamente apartir do game_state_data.player
//15 so much duplicated code inside match_scene
//13 the placement of the particles spawned at the moment of a projectile hit are a bit weird
//12 For the animation import the texture names from the scon file instead of iterating through the dir 
//11 Projectiles offset is not correct when taking into account if the sprite is flipped, may need refactor of collider to make it more generic
//10 change texture keys to integers instead of strings
//9 fix duplicated code -> game::update_player_colliders_position_only and game::update_projectile_colliders_position_only change player and projectile to &Vec<Collider> and fuse both functions
//8 refactor menu and maybe remove menu having a separate loop?
//7 VFX sprites are not centered, hard to place
//6 fix init colliders, its a mess
//1 Since Animator now holds Option<Animation> and Animation has a Vec, it doesnt implement Copy trait, so there are lots of clone()
//^^^^^possibly bad, and its a bit ugly, maybe have Animator only hold a handle to the animation just like the Textures?



pub struct GameStateData<'a> {
    item_sprites: ItemAssets<'a>,
    player: Option<Player>,
    font: Font<'a, 'a>,
    enemy_animations: HashMap<String, Rc<EntityAnimations>>,
    //loot_tables; HashMap<>
    general_assets: CommonAssets<'a>,
}

pub enum Transition {
    Continue,
    Change(Box<dyn Scene>),
    Push(Box<dyn Scene>),
    Pop,
    Quit,
}

fn main() -> Result<(), String> {
    println!("Starting Game");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let _mixer_context = init_sound();

    let music =
        music_player::load_from_file(Path::new("assets/musics/RetroFuture_Dirty.mp3")).unwrap();
   // music_player::play_music(&music);

    let window = video_subsystem
        .window("fighting game", 1280, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    canvas.set_blend_mode(BlendMode::Blend); //blend mode was added specifically to see the colliders
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;


    let font = ttf_context.load_font("assets/fonts/No_Virus.ttf", 128)?;
    let console_font = ttf_context.load_font("assets/fonts/No_Virus.ttf", 32)?;

    let mut controller_data = Controller::new();
    controller_data.add_keyboard();
    let mut device_management = InputDevices {
        joystick: sdl_context.joystick()?,
        controller: sdl_context.game_controller()?,
        controls: controls::load_controls(),
        joys: controller_data,
    };

    let menu = MenuScene::new_main_menu(&font);

    let mut game_state_data = GameStateData {  
        item_sprites: load_item_assets(&texture_creator),
        player: None,
        enemy_animations: HashMap::new(),
        font: console_font,
        general_assets: CommonAssets::load(&texture_creator),
    };
    
    let mut state_stack: Vec<Box<dyn Scene>> = Vec::new();
    state_stack.push(Box::new(menu));

    while !state_stack.is_empty() {
        let scene =  state_stack.last_mut().unwrap();

        match scene.run(
            &mut game_state_data,
            &texture_creator,
            &mut event_pump,
            &mut device_management,
            &mut canvas,
        ) {
            Transition::Continue => {}
            Transition::Push(next_state) => {state_stack.push(next_state);}
            Transition::Pop => {state_stack.pop();}
            Transition::Quit => {state_stack.clear();}
            Transition::Change(next_state) => {state_stack.pop(); state_stack.push(next_state);}
        }

    }

    Ok(())
}
