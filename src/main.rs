use engine_traits::scene::Scene;
use scenes::menu_scene::MenuScene;
use sdl2::image::{self, InitFlag};
use sdl2::render::BlendMode;

use std::collections::HashMap;
use std::path::Path;

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

use asset_management::sound::{init_sound, music_player};

use crate::{asset_management::controls, input::input_devices::InputDevices};
use crate::input::controller_handler::Controller;

use input::translated_inputs::TranslatedInput;

//TODO features tomorrow
//separate json for animation offsets and animation states (startup, active, recovery)
//Jumping fucks up rendering order, use grounded_y instead of actual y position
//make overworld proc gen man
//Improve AI

//TODO FEATURES
//play block animation while standing 
//add input buffer- decide if multiple buffers for different things or not
//add hit combos and block combos, these should be displayed while they are happening and not at the end to give faster feedback
//charge special attacks like makoto where you can hold punch for a stronger attack
//dash attacks
//save sound settings on config
//add menu to change the controllers for each player
//add menu to change the controller mapping of keys/buttons
//make ui loop only 60fps to avoid the computer doing too many wasted computations

//TODO TECH DEBT AND BUGS~

//16 
//15 so much duplicated code inside match_scene
//13 the placement of the particles spawned at the moment of a projectile hit are a bit weird
//12 For the animation import the texture names from the scon file instead of iterating through the dir 
//11 Projectiles offset is not correct when taking into account if the sprite is flipped, may need refactor of collider to make it more generic
//10 change texture keys to integers instead of strings
//9 fix duplicated code -> game::update_player_colliders_position_only and game::update_projectile_colliders_position_only change player and projectile to &Vec<Collider> and fuse both functions
//8 refactor menu and maybe remove menu having a separate loop?
//7 VFX sprites are not centered, hard to place
//6 fix init colliders, its a mess
//5 depenetration needs cleanup
//1 Since Animator now holds Option<Animation> and Animation has a Vec, it doesnt implement Copy trait, so there are lots of clone()
//^^^^^possibly bad, and its a bit ugly, maybe have Animator only hold a handle to the animation just like the Textures?

pub struct GameStateData<'a> {
    character: &'a str,
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

    let mut controller_data = Controller::new();
    controller_data.add_keyboard();
    let mut device_management = InputDevices {
        joystick: sdl_context.joystick()?,
        controller: sdl_context.game_controller()?,
        controls: controls::load_controls(),
        joys: controller_data,
    };

    let scene2 = MenuScene::new_main_menu(&font);

    let mut state_stack: Vec<Box<dyn Scene>> = Vec::new();
    state_stack.push(Box::new(scene2)); //menu state

    let mut game_state_data = GameStateData {
        character: "",
    };

    while !state_stack.is_empty() {
        state_stack.pop().unwrap().run(
            &mut state_stack,
            &mut game_state_data,
            &texture_creator,
            &mut event_pump,
            &mut device_management,
            &mut canvas,
        );
    }

    Ok(())
}
