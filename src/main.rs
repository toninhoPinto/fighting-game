use engine_traits::scene::Scene;
use game_logic::match_scene::Match;
use sdl2::image::{self, InitFlag};
use sdl2::render::BlendMode;

use std::collections::HashMap;
use std::path::Path;

#[macro_use]
extern crate serde_derive;
extern crate directories;
extern crate splines;

mod asset_management;
mod game_logic;
mod input;
mod rendering;
mod ui;
mod engine_traits;

use asset_management::sound::{init_sound, music_player};

use crate::asset_management::controls;
use crate::input::controller_handler::Controller;

use input::translated_inputs::TranslatedInput;

//TODO list
//REFACTOR MAIN.rs AND ADD MENU 
//refactor controller and input to be able to distinguish between local p1 and local p2 input sources
//make characters push correctly when jumped on top
//add vfx to attacks on hit
//change color of vfx using sdl2 texture tint
//apply attacks struct values (knockback, hitstun, etc)
//calculate frame advantage on the fly
//display different vfx colors and sizes depending on the frame advantage

//Hold attacks
//attack animations that vary depending on distance
//check how to pitch shift attacks depending on frame advantage
//dash attacks
//add movement to each attack
//projectile with a specific target location
//specific projectile only live if keep holding button
//Add startup, active and recovery per animation

fn main() -> Result<(), String> {
    println!("Starting Game");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let joystick = sdl_context.joystick()?;
    let controller = sdl_context.game_controller()?;

    let _mixer_context = init_sound();    
    
    let music = music_player::load_from_file(Path::new("assets/musics/RetroFuture_Dirty.mp3")).unwrap();
    music_player::play_music(&music);

    let window = video_subsystem
        .window("game tutorial", 1280, 720)
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
    let mut joys: HashMap<u32, Controller> = HashMap::new();

    //controllers
    let mut controls: HashMap<_, TranslatedInput> = controls::load_controls();

    let mut scene = Match::new(
        false, true, 
        "keetar".to_string(), "foxgirl".to_string());

    scene.run(&texture_creator, &mut event_pump, &joystick, &controller, &controls, &mut joys, &mut canvas);

    Ok(())
}
