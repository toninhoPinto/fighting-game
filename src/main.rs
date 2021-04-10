use asset_management::collider::ColliderType;
use game_logic::{game::{Game, SavedGame}, inputs::{apply_inputs::{apply_input_state, apply_input}, input_cycle::AllInputManagement, process_inputs::{update_button_state, update_directional_state}}};
use sdl2::image::{self, InitFlag};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::BlendMode;

//simply for exiting program
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use parry2d::bounding_volume::BoundingVolume;
use ui::{bar_ui::Bar, segmented_bar_ui::SegmentedBar};

use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Instant;

#[macro_use]
extern crate serde_derive;
extern crate directories;
extern crate splines;

mod asset_management;
mod game_logic;
mod input;
mod rendering;
mod ui;

use crate::asset_management::controls;
use crate::game_logic::character_factory::{load_character, load_character_anim_data};
use crate::game_logic::inputs::game_inputs::GameAction;
use crate::game_logic::inputs::process_inputs::{released_joystick_reset_directional_state};
use crate::input::controller_handler::Controller;

use input::translated_inputs::TranslatedInput;

//TODO list
//add pushboxes
//make characters pushable

//FIX GRAB, if you press light kick, and then halfway through the animation you press light punch, you can cancel the kick halfway and then grab
//improve reset input timers
//Hold attacks
//attack animations that vary depending on distance
//dash attacks
//add movement to each attack
//projectile with a specific target location
//specific projectile only live if keep holding button
//Add startup, active and recovery per animation
const FRAME_WINDOW_BETWEEN_INPUTS: i32 = 20;
const MAX_UPDATES_AVOID_SPIRAL_OF_DEATH: i32 = 4;
const FRAME_AMOUNT_CAN_ROLLBACK: i16 = 7;

fn main() -> Result<(), String> {
    println!("Starting Game");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let joystick = sdl_context.joystick()?;
    let controller = sdl_context.game_controller()?;

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

    let player1_character = "keetar";
    let player2_character = "foxgirl";

    let p1_assets = load_character_anim_data(&texture_creator, player1_character);
    let p2_assets = load_character_anim_data(&texture_creator, player2_character);

    let mut player1 = load_character(player1_character, Point::new(400, 0), false, 1);
    let mut player2 = load_character(player2_character, Point::new(700, -50), true, 2);
    player1
        .animator
        .play(p1_assets.animations.get("idle").unwrap(), false);
    player2
        .animator
        .play(p2_assets.animations.get("idle").unwrap(), false);

    let screen_res = canvas.output_size()?;
    let mut p1_health_bar = Bar::new(
        10,
        20,
        screen_res.0 / 2 - 20,
        50,
        player1.character.hp,
        Some(Color::RGB(255, 100, 100)),
        None,
    );
    let mut p2_health_bar = Bar::new(
        screen_res.0 as i32 / 2 + 10,
        20,
        screen_res.0 / 2 - 20,
        50,
        player2.character.hp,
        Some(Color::RGB(255, 100, 100)),
        None,
    );

    let special_bar_width = 150;
    let mut p1_special_bar = SegmentedBar::new(
        10,
        screen_res.1 as i32 - 30,
        special_bar_width,
        10,
        player1.character.special_max,
        Some(Color::RGB(20, 250, 250)),
        None,
    );
    let mut p2_special_bar = SegmentedBar::new(
        screen_res.0 as i32 - (special_bar_width as i32 + 10 * player2.character.special_max),
        screen_res.1 as i32 - 30,
        special_bar_width,
        10,
        player2.character.special_max,
        Some(Color::RGB(20, 250, 250)),
        None,
    );

    //controllers
    let mut controls: HashMap<_, TranslatedInput> = controls::load_controls();

    //p1 inputs
    let mut p1_input_history: VecDeque<AllInputManagement> = VecDeque::new();
    let mut p1_inputs: AllInputManagement = AllInputManagement::new();

    //p2 inputs 
    let mut p2_inputs: AllInputManagement = AllInputManagement::new();

    let mut game = Game::new(&mut player1, &mut player2);
    let mut game_rollback: VecDeque<SavedGame> = VecDeque::new();

    let mut previous_time = Instant::now();
    let logic_timestep: f64 = 0.016;
    let mut logic_time_accumulated: f64 = 0.0;
    let mut update_counter = 0;

    let mut is_single_player = true;
    let mut debug_pause = false;
    let mut debug_rollback = false;
    let mut rollback = 0;

    'running: loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(previous_time);
        let delta_time_as_nanos =
            delta_time.as_secs() as f64 + (delta_time.subsec_nanos() as f64 * 1e-9);

        previous_time = current_time;

        if !debug_pause {
            logic_time_accumulated += delta_time_as_nanos;
        }
        
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {keycode: Some(input),..} => {
                    if input == Keycode::L {
                        debug_rollback ^= true;
                    }
                    if input == Keycode::P {
                        debug_pause ^= true
                    }
                    if input == Keycode::Right {
                        logic_time_accumulated += logic_timestep;
                    }
                },
                _ => {}
            };
            input::controller_handler::handle_new_controller(
                &controller,
                &joystick,
                &event,
                &mut joys,
            );

            //needs also to return which controller/ which player
            let raw_input = input::input_handler::rcv_input(&event, &mut controls);

            if raw_input.is_some() {
                let (translated_input, is_pressed) = raw_input.unwrap();
                
                p1_inputs.input_new_frame.push_back((translated_input, is_pressed));

                let is_directional_input = TranslatedInput::is_directional_input(translated_input);
                if is_directional_input {
                    if !is_pressed {
                        released_joystick_reset_directional_state(
                            translated_input,
                            &mut p1_inputs.directional_state_input,
                        );
                    }
                    update_directional_state(
                        translated_input,
                        is_pressed,
                        &mut p1_inputs.directional_state_input,
                    );
                } 
            }
            //end of input management
        }

        //Update
        while logic_time_accumulated >= logic_timestep || rollback > 0 {
            update_counter +=1;
            if update_counter > MAX_UPDATES_AVOID_SPIRAL_OF_DEATH && rollback == 0 {
                logic_time_accumulated = 0.0;
            }

            if rollback > 0 {
                rollback -= 1;
                println!("ROLLBACK {:?} {:?} {:?}", p1_input_history.len(), FRAME_AMOUNT_CAN_ROLLBACK ,  rollback);
                if rollback > 0 {
                    p1_inputs = p1_input_history.get((FRAME_AMOUNT_CAN_ROLLBACK - rollback) as usize).unwrap().clone();
                }
            } 


            if debug_rollback {             
                if rollback == 0 {
                    rollback = FRAME_AMOUNT_CAN_ROLLBACK;
                    game.load(&game_rollback.get(0).unwrap(), &p1_assets, &p2_assets);
                    p1_inputs = p1_input_history.get(0).unwrap().clone();
                    debug_rollback = false;
                }
                println!("START ROLLBACK {:?} ", p1_input_history);
            }
            println!("INPUTS {:?}",p1_inputs);

            
            game.current_frame += 1;

            if rollback == 0 {
                game_rollback.push_back(game.save());
                p1_input_history.push_back(p1_inputs.clone());
                if p1_input_history.len() as i16 > FRAME_AMOUNT_CAN_ROLLBACK {
                    p1_input_history.pop_front();
                    game_rollback.pop_front();
                }
            }

            if !p1_inputs.input_new_frame.is_empty() {
                apply_input(&mut game.player1, &p1_assets, 
                    &p1_inputs.directional_state_input,
                    &mut p1_inputs.input_new_frame, 
                    &mut p1_inputs.input_processed, &mut p1_inputs.input_processed_reset_timer,
                    &mut p1_inputs.action_history, &mut p1_inputs.special_reset_timer);
            }
            apply_input_state(&mut game.player1, &p1_inputs.directional_state_input);

            for i in 0..p1_inputs.input_processed_reset_timer.len() {
                p1_inputs.input_processed_reset_timer[i] += 1;
                if p1_inputs.input_processed_reset_timer[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                    p1_inputs.input_processed.pop_front();
                }
            }
            p1_inputs.input_processed_reset_timer.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);


            for i in 0..p1_inputs.special_reset_timer.len() {
                p1_inputs.special_reset_timer[i] += 1;
                if p1_inputs.special_reset_timer[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                    if p1_inputs.action_history.len() > 1 {
                        p1_inputs.action_history.pop_front();
                    }
                }
            }
            p1_inputs.special_reset_timer.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);

            p1_health_bar.update(game.player1.character.hp);
            p2_health_bar.update(game.player2.character.hp);

            p1_special_bar.update(game.player1.character.special_curr);
            p2_special_bar.update(game.player2.character.special_curr);

            game.player1.update(logic_timestep, game.player2.position.x);
            game.player2.update(logic_timestep, game.player1.position.x);

            game.player1.state_update(&p1_assets);
            game.player2.state_update(&p2_assets);

            println!("inside while");
            if rollback == 0 {
                logic_time_accumulated -= logic_timestep;
            }
        }
        println!("left while");

        // Render
        if update_counter >= 0 {
            rendering::renderer::render(
                &mut canvas,
                Color::RGB(60, 64, 255),
                game.player1,
                &p1_assets,
                game.player2,
                &p2_assets,
                &game.projectiles,
                &mut game.p1_colliders,
                &mut game.p2_colliders,
                &p1_health_bar,
                &p2_health_bar,
                &p1_special_bar,
                &p2_special_bar,
                true,
            )?;

            update_counter = 0;
        }
    }

    Ok(())
}
