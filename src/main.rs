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

mod input;
mod rendering;
mod game_logic;
mod asset_management;

use crate::input::controller_handler::Controller;
use crate::asset_management::{controls, asset_loader};
use crate::game_logic::character_factory::{load_character, load_character_anim_data};
use crate::game_logic::inputs::game_inputs::GameInputs;
use crate::game_logic::inputs::process_inputs::apply_game_inputs;


//TODO list
//Hold attacks
//2 inputs at the same time (grabs)
//attack animations that vary depending on distance
//dash attacks
//add movement to each attack
//add different animation speeds to each animation
//projectile with a specific target location
//specific projectile only live if keep holding button
//Improve dash smoothing <- add a small movement break after a dash

const FRAME_WINDOW_BETWEEN_INPUTS: i32 = 20;

fn main() -> Result<(), String> {
    println!("Starting Game");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("game tutorial", 1280, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    //blend mode was added specifically to see the colliders
    canvas.set_blend_mode(BlendMode::Blend);

    let mut event_pump = sdl_context.event_pump()?;

    let joystick = sdl_context.joystick()?;
    let controller = sdl_context.game_controller()?;
    let mut joys: HashMap<u32, Controller> = HashMap::new();

    let texture_creator = canvas.texture_creator();

    let player1_character = "keetar";
    let player2_character = "foxgirl";

    let p1_assets = load_character_anim_data(&texture_creator, player1_character);
    let p2_assets = load_character_anim_data(&texture_creator, player2_character);

    let mut player1 = load_character(player1_character, Point::new(0, 20), true, 1);
    let mut player2 = load_character(player2_character, Point::new(800, -40), false, 2);
    player1.animator.play(p1_assets.animations.get("idle").unwrap(), false);
    player2.animator.play(p2_assets.animations.get("idle").unwrap(), false);

    let mut controls: HashMap<_, GameInputs> = controls::load_controls();
    let mut last_inputs: VecDeque<GameInputs> = VecDeque::new();

    let mut input_reset_timers: Vec<i32> = Vec::new();

    //TODO input buffer, finish
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
        println!("{:?} {:?}", aabb.mins, aabb.maxs);
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

            let input = input::input_handler::rcv_input(&event, &mut controls);
            match input {
                Some(input) => {
                    input_reset_timers.push(0);
                    apply_game_inputs(&p1_assets, &mut player1, input, &mut last_inputs);
                },
                None => {}
            }
        }

        //Update
        while logic_time_accumulated >= logic_timestep {
            logic_time_accumulated -= logic_timestep;

            player1.update(player2.position.x);
            player2.update(player1.position.x);

            //Handle projectile movement
            for i in 0..projectiles.len() {
               projectiles[i].update();
            }
        }

        // Render
        if rendering_time_accumulated >= rendering_timestep {
            //TODO ????? what is this for
            let _dt = rendering_time_accumulated * 0.001;


            for i in 0..colliders.len() {
                let mut aabb = colliders[i];
                let _offset = idle_hitboxes.1[0][i].x as f32;
                aabb.mins = aabbPoint::new(aabb.mins.x * 2.0 + player1.position.x as f32, aabb.mins.y * 2.0 + player1.position.y as f32);
                aabb.maxs = aabbPoint::new(aabb.maxs.x * 2.0 + player1.position.x as f32, aabb.maxs.y * 2.0 + player1.position.y as f32);
                //aabb.mins = aabbPoint::new((aabb.mins.x + offset + player1.position.x as f32) * 2.0, (aabb.mins.y + offset + player1.position.y as f32 -player1.sprite.height()as f32 /2.0 ) * 2.0);
                //aabb.maxs = aabbPoint::new((aabb.maxs.x + offset + player1.position.x as f32) * 2.0, (aabb.maxs.y  + offset + player1.position.y as f32 -player1.sprite.height()as f32 /2.0) * 2.0);
            }

            //Number of frames to delete each input
            for i in 0..input_reset_timers.len() {
                input_reset_timers[i] += 1;
                if input_reset_timers[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                    last_inputs.pop_front();
                }
            }
            input_reset_timers.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);


            //TODO: trigger finished animation, instead make a function that can play an animation once and run callback at the end
            let player_current_animation = player1.animator.current_animation.unwrap();
            let p1_curr_anim = player_current_animation.length;
            if (player1.animator.animation_index as f32 + 0.35 as f32) as usize >= p1_curr_anim as usize {
                //TODO temp location, currently it adds the projectile once at the end, but should add at specific key frames
                if player1.is_attacking && p1_assets.effects.contains_key(&player_current_animation.name) {
                    let mut projectile = (*p1_assets.effects.get(&player_current_animation.name).unwrap()).clone();
                    projectile.position = projectile.position.offset(player1.position.x(), 0);
                    projectile.direction.x = (player2.position.x - player1.position.x).signum();
                    projectile.flipped = player1.dir_related_of_other > 0;
                    projectile.player_owner = player1.id;

                    let target_pos = Point::new(player2.position.x + (projectile.direction.x * 100), projectile.position.y);
                    projectile.target_position = Some(target_pos);
                    projectiles.push(projectile);
                }
            }


            rendering::renderer::render(&mut canvas, Color::RGB(60, 64, 255 ),
                                        &mut player1, &p1_assets,
                                        &mut player2, &p2_assets,
                                        &projectiles, &colliders, true)?;

            rendering_time_accumulated = 0.0;
        }

    }

    Ok(())
}