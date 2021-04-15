use engine_traits::scene::Scene;
use game_logic::match_scene::Match;
use sdl2::image::{self, InitFlag};
use sdl2::render::BlendMode;
use ui::menus::menu_scene::MenuScene;

use std::collections::HashMap;
use std::path::Path;

#[macro_use]
extern crate serde_derive;
extern crate directories;

mod asset_management;
mod game_logic;
mod input;
mod rendering;
mod ui;
mod engine_traits;
mod collision;

use asset_management::{sound::{init_sound, music_player}};

use crate::asset_management::controls;
use crate::input::controller_handler::Controller;

use input::translated_inputs::TranslatedInput;

//TODO list
//make dash have movcement only during some frames and not during others + lock in position
//apply attacks struct values (knockback, hitstun, etc)
//calculate frame advantage on the fly
//display different vfx colors and sizes depending on the frame advantage
//make an enum for with startup|active|recovery and use code to detect a hitbox and switch to active, and switch back to recovery 
//this is important to be able to cancel the recovery of attacks

//define a ground height and a offset for each character to be at the correct ground height
//add hit combos and block combos
//Hold attacks
//attack animations that vary depending on distance
//check how to pitch shift attacks depending on frame advantage
//dash attacks
//add movement to each attack
//projectile with a specific target location
//specific projectile only live if keep holding button
//VFX sprites are not centered, hard to place
//change color of vfx using sdl2 texture tint OR shader, which one?

//add menu to be apply to change the controllers for each player
//refactor menu and maybe remove menu having a separate loop?
//rollback should not happen during enemy stunned/hitstun if there is no way to escape
//same during uncancellable animations
//it stills re-simulate but doesnt change anything 
//lobby system -> needs server
//instant rematch (avoid going back to lobby or going back to selection)
//show ping and show wifi/ethernet

pub struct GameStateData<'a> {
    p1_character: &'a str,
    p2_character: &'a str,
}

fn main() -> Result<(), String> {
    println!("Starting Game");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let joystick = sdl_context.joystick()?;
    let controller = sdl_context.game_controller()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

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
    let mut controller_data = Controller::new();
    controller_data.add_keyboard( ); //should not use 0

    let mut font = ttf_context.load_font("assets/fonts/No_Virus.ttf", 128)?;

    //controllers
    let mut controls: HashMap<_, TranslatedInput> = controls::load_controls();

    let scene2 = MenuScene::new_main_menu(&font);

    let mut state_stack: Vec<Box<dyn Scene>> = Vec::new();
    state_stack.push(Box::new(scene2)); //menu state

    let mut game_state_data = GameStateData {
        p1_character: "",
        p2_character: "",
    };

    while !state_stack.is_empty() {
        state_stack.pop().unwrap().run(&mut state_stack, &mut game_state_data, &texture_creator,
             &mut event_pump, &joystick, &controller, &controls, &mut controller_data, &mut canvas);
    }
    
    Ok(())
}
