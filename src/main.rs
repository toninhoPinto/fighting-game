use engine_traits::scene::Scene;
use scenes::menu_scene::MenuScene;
use sdl2::image::{self, InitFlag};
use sdl2::render::BlendMode;

use std::collections::HashMap;
use std::path::Path;

#[macro_use]
extern crate serde_derive;
extern crate directories;

mod asset_management;
mod collision;
mod engine_traits;
mod game_logic;
mod input;
mod rendering;
mod ui;
mod scenes;

use asset_management::sound::{init_sound, music_player};

use crate::asset_management::controls;
use crate::input::controller_handler::Controller;

use input::translated_inputs::TranslatedInput;

//TODO features tomorrow
//add particles on the ground when dashing (dust cloud)
//add particles on the ground when jumping and landing -> needs to be able to import POINTs from spriter pro
//add particles on feet ground when doing some attacks or receiving damage
//add particles to projectile destruction on hit (or other reasons)
//make end game UI work

//TODO FEATURES
//decide which attacks can cancel into which they can cancel into types of attacks or specific attacks 
//add input buffer- decide if multiple buffers for different things or not
//if there is a trade make the hitstun work during attack animation
//make an enum for with startup|active|recovery and use code to detect a hitbox and switch to active, and switch back to recovery
//this is important to be able to cancel the recovery of attacks
//calculate frame advantage on the fly
//display different vfx colors and sizes depending on the frame advantage //change color of vfx using sdl2 texture tint OR shader, which one?
//add hit combos and block combos, these should be displayed while they are happening and not at the end to give faster feedback
//charge special attacks like makoto where you can hold punch for a stronger attack
//charge special attacks like guile where you have to hold a direction for 40 frames
//rekka  special attacks where you can chain special attacks but as one
//makoto has a knee that cancels into a kick -> attacks that only cancel for another specific attack
//check how to pitch shift attacks sound chunks depending on frame advantage
//dash attacks
//projectile with a specific target location
//specific projectile only live if keep holding button
//save sound settings on config
//add menu to change the controllers for each player
//add menu to change the controller mapping of keys/buttons
//rollback should not happen during enemy stunned/hitstun if there is no way to escape
//same during uncancellable animations
//should it still re-simulate? probably not, but need to handle inputs if we buffer inputs during hitstun
//lobby system -> needs server
//instant rematch (avoid going back to lobby or going back to selection)
//show ping and show wifi/ethernet
//make ui loop only 60fps to avoid the computer doing too many wasted computations
//check if game is deterministic <- THIS IS VITAL FOR ROLLBACK MULTIPLAYER

//TODO TECH DEBT AND BUGS
//Projectiles offset is not correct when taking into account if the sprite is flipped, may need refactor of collider to make it more generic
//change texture keys to integers instead of strings
// fix duplicated code -> game::update_player_colliders_position_only and game::update_projectile_colliders_position_only change player and projectile to &Vec<Collider> and fuse both functions
//refactor menu and maybe remove menu having a separate loop?
//VFX sprites are not centered, hard to place
//fix init colliders, its a mess
//depenetration needs cleanup
//not a big fan on how character width is calculated
//define a ground height and a offset for each character to be at the correct ground height
//hit stun should only freeze two characters not everything else
//Since Animator now holds Option<Animation> and Animation has a Vec, it doesnt implement Copy trait, so there are lots of clone()
//^^^^^possibly bad, and its a bit ugly, maybe have Animator only hold a handle to the animation just like the Textures?

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

    let music =
        music_player::load_from_file(Path::new("assets/musics/RetroFuture_Dirty.mp3")).unwrap();
    music_player::play_music(&music);

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
    let mut controller_data = Controller::new();
    controller_data.add_keyboard(); //should not use 0

    let font = ttf_context.load_font("assets/fonts/No_Virus.ttf", 128)?;

    //controllers
    let controls: HashMap<_, TranslatedInput> = controls::load_controls();

    let scene2 = MenuScene::new_main_menu(&font);

    let mut state_stack: Vec<Box<dyn Scene>> = Vec::new();
    state_stack.push(Box::new(scene2)); //menu state

    let mut game_state_data = GameStateData {
        p1_character: "",
        p2_character: "",
    };

    while !state_stack.is_empty() {
        state_stack.pop().unwrap().run(
            &mut state_stack,
            &mut game_state_data,
            &texture_creator,
            &mut event_pump,
            &joystick,
            &controller,
            &controls,
            &mut controller_data,
            &mut canvas,
        );
    }

    Ok(())
}
