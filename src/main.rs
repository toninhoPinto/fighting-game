use engine_traits::scene::Scene;
use game_logic::{characters::player::Player, effects::ItemEffects, events::Event, items::Item};
use rand::prelude::SmallRng;
use scenes::menu_scene::MenuScene;
use sdl2::{image::{self, InitFlag}, pixels::Color, rect::{Point, Rect}, render::Texture, ttf::Font};
use sdl2::render::BlendMode;
use ui::ingame::{segmented_bar_ui::SegmentedBar, wrapping_list_ui::WrappingList};

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

use asset_management::{asset_holders::{EntityAnimations, ItemAssets, LevelAssets, UIAssets}, common_assets::CommonAssets, sound::{init_sound, music_player}};

use crate::{asset_management::{asset_loader::events_loader::load_events, controls}, game_logic::{effects::hash_effects, factories::item_factory::{load_item_assets, load_items}}, input::input_devices::InputDevices};
use crate::input::controller_handler::Controller;


//TODO features tomorrow

//Buttons (failed to do this, its hard)
// change items to buttons in the store 
// change the back into a button 
// change the main menu into a bunch of proper buttons 

// add a font cache hashmap to GameStateData ???

// improve store UI on the selected item - make better sprite and center it better
// add sounds in store for - moving cursor between items, purchasing item

//OpenGL rendering??

//Level generation
    //make enemy tables
    //tweak placement of objects (both the debug cubes and enemy spawning seem to be slightly off)
    //create collider from wall

//add sound_effects
    //everytime punch happens (very light sound)
    //add variation to sounds through pitch changing

//implement more item effects
    
//make overworld proc gen map
    // replace lines with rotated squares with textures
    // decision making on the type of node (level, event, store)
    // add small up/down animation on the "character" icon

//separate json for animation offsets and animation states (startup, active, recovery)

//Improve AI

//TODO FEATURES
//play block animation while standing 
//add hit combos and block combos, these should be displayed while they are happening and not at the end to give faster feedback
//charge special attacks like makoto where you can hold punch for a stronger attack
//dash attack
//save sound volume settings on config
//add menu to change the controllers for each player
//add menu to change the controller mapping of keys/buttons
//make ui loop only 60fps to avoid the computer doing too many wasted computations

//TODO TECH DEBT AND BUGS

//changing the connection level shouldnt re-generate everything-> level start_x and map to position tags, but maybe pre-compute a new array and ignore/remove the Map inside Level
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

    player: Option<Player>,
    hp_bar: Option<SegmentedBar<'a>>,
    energy_bar: Option<SegmentedBar<'a>>,

    items: HashMap<i32, Item>,
    effects: HashMap<i32, ItemEffects>,
    events: HashMap<u32, Event>,

    enemy_animations: HashMap<String, Rc<EntityAnimations>>,
    
    general_assets: CommonAssets<'a>,
    item_assets: ItemAssets<'a>,
    level_assets: LevelAssets<'a>, 
    ui_assets: UIAssets<'a>,


    curr_level: i32,

    //rng
    seed: Option<u64>,
    map_rng: Option<SmallRng>,
}

pub enum Transition {
    Continue,
    Change(Box<dyn Scene>),
    Push(Box<dyn Scene>),
    Pop,
    Quit,
}


pub fn hp_bar_init<'a>(screen_res: (u32, u32), max_hp: i32, curr_hp: i32) -> SegmentedBar<'a> {
    SegmentedBar::new(
        80,
        20,
        screen_res.0 / 3 - 50,
        25,
        max_hp,
        curr_hp,
        20,
        Some(Color::RGB(255, 100, 100)),
        None,
    )
}

pub fn energy_bar_init<'a>(screen_res: (u32, u32), max_energy: i32, curr_energy: i32) -> SegmentedBar<'a> {
    SegmentedBar::new(
        80,
        60,
        screen_res.0 / 3 - 50,
        10,
        max_energy,
        curr_energy,
        1,
        Some(Color::RGB(100, 100, 255)),
        None,
    )
}

pub fn item_list_init(game_state_data: &GameStateData) -> WrappingList {
    WrappingList::new(
        Point::new(10, 70),
        200,
        game_state_data.player.as_ref().unwrap().items.iter()
            .map(|_item| {Rect::new(0,0,32,32)})
            .collect::<Vec<Rect>>(), 
        10
    )
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

    let menu = MenuScene::new_main_menu(&font);

    let general_assets = CommonAssets::load(&texture_creator, &ttf_context);
    let mut game_state_data = GameStateData {
        player: None,
        hp_bar: None,
        energy_bar: None,


        events: load_events("assets/events/events.json".to_string()),
        items: load_items("assets/items/items.json".to_string()),
        effects: hash_effects(),
        enemy_animations: HashMap::new(),
        
        seed: None,
        map_rng: None,

        curr_level: -1,

        ui_assets: UIAssets::load(&texture_creator, &general_assets.fonts),
        general_assets,
        item_assets: load_item_assets(&texture_creator),
        level_assets: LevelAssets::load(&texture_creator, &ttf_context),
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
