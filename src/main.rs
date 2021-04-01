
use game_logic::inputs::apply_inputs::apply_game_input_state;
use sdl2::pixels::Color;
use sdl2::image::{self, InitFlag};
use sdl2::rect::Point;
use sdl2::render::BlendMode;

//simply for exiting program
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use parry2d::bounding_volume::AABB;
use parry2d::math::Point as aabbPoint;

use std::time::{Instant};
use std::collections::HashMap;
use std::collections::VecDeque;

#[macro_use]
extern crate serde_derive;
extern crate directories;
extern crate splines;

mod input;
mod rendering;
mod game_logic;
mod asset_management;

use crate::input::controller_handler::Controller;
use crate::asset_management::{controls, asset_loader};
use crate::game_logic::character_factory::{load_character, load_character_anim_data};
use crate::game_logic::inputs::game_inputs::GameInput;
use crate::game_logic::inputs::process_inputs::transform_input_state;
use crate::game_logic::inputs::apply_inputs::apply_game_inputs;

use input::translated_inputs::TranslatedInput;

//TODO list
//improve reset input timers
//Hold attacks
//2 inputs at the same time (grabs)
//attack animations that vary depending on distance
//dash attacks
//add movement to each attack
//add different animation speeds to each animation
//projectile with a specific target location
//specific projectile only live if keep holding button
//Improve dash smoothing <- add a small movement break after a dash
//character should only flip once it hits the ground and not on the air

const FRAME_WINDOW_BETWEEN_INPUTS: i32 = 20;
    
fn main() -> Result<(), String> {
    println!("Starting Game");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let joystick = sdl_context.joystick()?;
    let controller = sdl_context.game_controller()?;

    let window = video_subsystem.window("game tutorial", 1280, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    canvas.set_blend_mode(BlendMode::Blend); //blend mode was added specifically to see the colliders
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;
    let mut joys: HashMap<u32, Controller> = HashMap::new();

    let player1_character = "keetar";
    let player2_character = "foxgirl";

    let p1_assets = load_character_anim_data(&texture_creator, player1_character);
    let p2_assets = load_character_anim_data(&texture_creator, player2_character);

    let mut player1 = load_character(player1_character, Point::new(400, 0), false, 2);
    let mut player2 = load_character(player2_character, Point::new(900, -50), true, 1);
    player1.animator.play(p1_assets.animations.get("idle").unwrap(), false);
    player2.animator.play(p2_assets.animations.get("idle").unwrap(), false);

    //controllers
    let mut controls: HashMap<_, TranslatedInput> = controls::load_controls();

    //inputs
    let mut input_reset_timers: Vec<i32> = Vec::new();
    let mut last_inputs: VecDeque<GameInput> = VecDeque::new();
    let mut current_state_input: [(GameInput, bool); 10] = GameInput::init_input_state();
    let mut directional_state_input: [(TranslatedInput, bool); 4] = TranslatedInput::init_dir_input_state();
    let mut _input_buffer: Vec<i32> = Vec::new();

    let mut previous_time = Instant::now();
    let logic_timestep: f64 = 0.016;
    let mut logic_time_accumulated: f64 = 0.0;

    let rendering_timestep: f64 = 0.016; // 60fps
    let mut rendering_time_accumulated: f64 = 0.0;

    let mut projectiles: Vec<game_logic::projectile::Projectile> = Vec::new();
    let mut colliders: Vec<AABB> = Vec::new();
    let idle_hitboxes = asset_loader::load_hitboxes(format!("assets/{}/standing/idle/idle.json", "keetar").to_string());

    for i in 0..idle_hitboxes.0.len() {
        let mut aabb = idle_hitboxes.0[i];
        let offset_x = idle_hitboxes.1[0][i].x as f32 * 2.0;
        let offset_y = idle_hitboxes.1[0][i].y as f32 * 2.0;
        //aabb.mins = aabbPoint::new( aabb.mins.x * 2.0 + offset + player1.position.x as f32, aabb.mins.y * 2.0 + offset + player1.position.y as f32);
        //aabb.maxs = aabbPoint::new(offset + aabb.maxs.x * 2.0 + player1.position.x as f32, offset + aabb.maxs.y * 2.0 + player1.position.y as f32);
        aabb.mins = aabbPoint::new(aabb.mins.x * 2.0 + offset_x + player1.position.x as f32, aabb.mins.y * 2.0 + offset_y + player1.position.y as f32);
        aabb.maxs = aabbPoint::new(aabb.maxs.x * 2.0 + offset_x + player1.position.x as f32, aabb.maxs.y * 2.0+ offset_y + player1.position.y as f32);
        colliders.push(aabb);
    }

    'running: loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(previous_time);
        let delta_time_as_mili = delta_time.as_secs() as f64 + (delta_time.subsec_nanos() as f64 * 1e-9);

        previous_time = current_time;

        logic_time_accumulated += delta_time_as_mili;
        rendering_time_accumulated += delta_time_as_mili;

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                _ => {}
            };
            input::controller_handler::handle_new_controller(&controller, &joystick, &event, &mut joys);

            //needs also to return which controller/ which player
            let input = input::input_handler::rcv_input(&event,&mut controls);
            //this actually needs to be x2 for both players
            //&mut current_inputs_state
            match input {
                Some((input, is_pressed)) => {
                    //directional_state_input
                    //current_state_input
                    let game_input = transform_input_state(input , is_pressed, &mut current_state_input, &mut directional_state_input, &mut last_inputs, &player1);
                    match game_input {
                        Some(final_input) => {
                            apply_game_inputs(&p1_assets, &mut player1, final_input, is_pressed, &current_state_input, &mut last_inputs);
                            if is_pressed {
                                input_reset_timers.push(0);
                            }  
                        },
                        None => {}
                    }
                    
                },
                None => {}
            }
        }

        //Update
        while logic_time_accumulated >= logic_timestep {

            apply_game_input_state(&p1_assets, &mut player1, &mut input_reset_timers, &current_state_input, &mut last_inputs);

            //Number of frames to delete each input
            for i in 0..input_reset_timers.len() {
                input_reset_timers[i] += 1;
                if input_reset_timers[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                    last_inputs.pop_front();
                    println!("pop {:?}", last_inputs);
                }
            }
            input_reset_timers.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);

            player1.update(logic_timestep, player2.position.x);
            player2.update(logic_timestep, player1.position.x);

            //Handle projectile movement
            for i in 0..projectiles.len() {
               projectiles[i].update();
            }
            logic_time_accumulated -= logic_timestep;
        }


        // Render
        if rendering_time_accumulated >= rendering_timestep {
            let _dt = rendering_time_accumulated * 0.001;

            rendering::renderer::render(&mut canvas, Color::RGB(60, 64, 255 ),
                                        &mut player1, &p1_assets,
                                        &mut player2, &p2_assets,
                                        &projectiles, &colliders, true)?;

            rendering_time_accumulated = 0.0;
        }

    }

    Ok(())
}