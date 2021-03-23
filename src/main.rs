use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
use sdl2::image::{self, InitFlag};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture};

use std::time::{Instant};
use std::collections::HashMap;
use std::collections::VecDeque;

#[macro_use]
extern crate serde_derive;
extern crate preferences;

mod input;
mod rendering;
mod game_logic;
mod controls;

//TODO list
//Projectiles
//Dashes (2 fast directional inputs)
//Hold attacks
//2 inputs at the same time (grabs)
//attack animations that vary depending on distance

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

    let mut event_pump = sdl_context.event_pump()?;

    let joystick = sdl_context.joystick()?;
    let mut joys: HashMap<u32, sdl2::joystick::Joystick> = HashMap::new();

    let texture_creator = canvas.texture_creator();

    let anims = game_logic::player::load_character_anims(&texture_creator, "ryu".to_string());

    let mut texture_to_display_1: Option<&Texture> = None;
    let mut texture_to_display_2: Option<&Texture> = None;

    //TODO should vary per animation, some are faster others are slower because of less frames
    let anim_speed = 0.35;

    //TODO this should be deserialized from somewhere that has each information per character
    //or maybe from a specific factory like module
    let mut specials_inputs: Vec<(Vec<game_logic::game_input::GameInputs>, &str)> = Vec::new();
    let mut combo_string: Vec<game_logic::game_input::GameInputs> = Vec::new();
    combo_string.push(game_logic::game_input::GameInputs::DOWN);
    combo_string.push(game_logic::game_input::GameInputs::FwdDOWN);
    combo_string.push(game_logic::game_input::GameInputs::FWD);
    combo_string.push(game_logic::game_input::GameInputs::LightPunch);
    specials_inputs.push((combo_string, "special_attack"));

    let mut directional_inputs: Vec<(Vec<game_logic::game_input::GameInputs>, &str)> = Vec::new();
    let mut directional_string: Vec<game_logic::game_input::GameInputs> = Vec::new();
    directional_string.push(game_logic::game_input::GameInputs::FWD);
    directional_string.push(game_logic::game_input::GameInputs::LightPunch);
    directional_inputs.push((directional_string, "directional_light_punch"));

    //TODO move this to game_logic::player::
    let mut player1 = game_logic::player::Player {
        position: Point::new(-200, 100),
        sprite: Rect::new(0, 0, 580, 356),
        speed: 5,
        dash_speed: 10,
        prev_direction: 0,
        direction: 0,
        dir_related_of_other: 0,
        state: game_logic::player::PlayerState::Standing,
        isAttacking: false,
        animation_index: 0.0,
        current_animation: &anims.get(&"idle".to_string()).unwrap(),
        animations: &anims,
        flipped: true,
        input_combination_anims: &specials_inputs,
        directional_variation_anims: &directional_inputs,
        last_directional_input: None,
        last_directional_input_v: None,
        last_directional_input_h: None
    };

    let mut player2 = game_logic::player::Player {
        position: Point::new(200, 100),
        sprite: Rect::new(0, 0, 580, 356),
        speed: 5,
        dash_speed: 10,
        prev_direction: 0,
        direction: 0,
        dir_related_of_other: 0,
        state: game_logic::player::PlayerState::Standing,
        isAttacking: false,
        animation_index: 0.0,
        current_animation: &anims.get(&"idle".to_string()).unwrap(),
        animations: &anims,
        flipped: false,
        input_combination_anims: &specials_inputs,
        directional_variation_anims: &directional_inputs,
        last_directional_input: None,
        last_directional_input_v: None,
        last_directional_input_h: None
    };

    let mut controls: HashMap<_, game_logic::game_input::GameInputs> = controls::load_controls();
    let mut last_inputs: VecDeque<game_logic::game_input::GameInputs> = VecDeque::new();

    let mut input_reset_timers: Vec<i32> = Vec::new();

    let mut previous_time = Instant::now();
    let logic_timestep: f64 = 0.016;
    let mut logic_time_accumulated: f64 = 0.0;

    let rendering_timestep: f64 = 0.016; // 60fps
    let mut rendering_time_accumulated: f64 = 0.0;

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
                Event::JoyDeviceAdded { which, .. } => {
                    println!("added controller: {}", which);
                    let joy = joystick.open(which as u32).unwrap();
                    joys.insert(which, joy);
                },
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }

            let input = input::input_handler::rcv_input(event, &mut controls);
            match input {
                Some(input) => {
                    input_reset_timers.push(0);
                    game_logic::game_input::apply_game_inputs(&mut player1, input, &mut last_inputs);
                },
                None => {}
            }
        }




        while logic_time_accumulated >= logic_timestep {
            logic_time_accumulated -= logic_timestep;

            if !player1.isAttacking {
                //TODO: this is also game engine, try and move it away somewhere else
                if player1.state == game_logic::player::PlayerState::Standing {
                    player1.position = player1.position.offset(player1.direction * player1.speed, 0);
                }

                if player1.state == game_logic::player::PlayerState::DashingForward ||
                    player1.state == game_logic::player::PlayerState::DashingBackward  {
                    player1.position = player1.position.offset(player1.direction * player1.dash_speed, 0);
                }

            }

        }





        // Render
        //println!("{:?} {:?}", rendering_time_accumulated, rendering_timestep);
        if rendering_time_accumulated >= rendering_timestep {
            //TODO ????? what is this for
            let dt = rendering_time_accumulated * 0.001;

            //Number of frames to delete each input
            for i in 0..input_reset_timers.len() {
                input_reset_timers[i] += 1;
                if input_reset_timers[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                    last_inputs.pop_front();
                }
            }
            input_reset_timers.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);
            println!("{:?}", last_inputs);


            player1.animation_index = (player1.animation_index + anim_speed) % player1.current_animation.len() as f32;
            player2.animation_index = (player2.animation_index + anim_speed) % player2.current_animation.len() as f32;


            println!("{:?} {:?}", (player1.animation_index as f32 + anim_speed as f32) as usize, player1.current_animation.len());
            //TODO: trigger finished animation, instead make a function that can play an animation once and run callback at the end
            if (player1.animation_index as f32 + anim_speed as f32) as usize >= player1.current_animation.len() {
                if player1.isAttacking {
                    player1.isAttacking = false;
                }
                if player1.state == game_logic::player::PlayerState::DashingForward ||
                    player1.state == game_logic::player::PlayerState::DashingBackward {
                    player1.state = game_logic::player::PlayerState::Standing;
                }
                player1.animation_index = 0.0;
            }



            if !player1.isAttacking {
                println!("{:?}", player1.state);
                if player1.state == game_logic::player::PlayerState::Standing {
                    player1.dir_related_of_other = (player2.position.x - player1.position.x).signum();

                    //TODO flip has a small animation i believe, also, have to take into account mixups
                    //TODO needs to switch the FWD to BCK and vice versa
                    player1.flipped = player1.dir_related_of_other > 0;

                    if player1.direction * -player1.dir_related_of_other < 0 {
                        player1.current_animation = player1.animations.get("walk").unwrap();
                    } else if player1.direction * -player1.dir_related_of_other > 0 {
                        player1.current_animation = player1.animations.get("walk_back").unwrap();
                    } else {
                        player1.current_animation = player1.animations.get("idle").unwrap();
                    }
                } else if player1.state == game_logic::player::PlayerState::Crouching {
                    player1.current_animation = player1.animations.get("crouching").unwrap();
                } else if player1.state == game_logic::player::PlayerState::DashingForward {
                    player1.current_animation = player1.animations.get("dash").unwrap();
                } else if player1.state == game_logic::player::PlayerState::DashingBackward {
                    player1.current_animation = player1.animations.get("dash").unwrap();
                }

                if player1.prev_direction != player1.direction {
                    player1.animation_index = 0.0;
                }

                player1.prev_direction = player1.direction;
            }


            //TODO fix array out of bounds, possibly due to a change of animation without resetting the index
            texture_to_display_1 = Some(&player1.current_animation[player1.animation_index as usize]);

            player2.dir_related_of_other = (player1.position.x - player2.position.x).signum();
            player2.flipped = player2.dir_related_of_other > 0;

            texture_to_display_2 = Some(&player2.current_animation[player2.animation_index as usize]);

            rendering::renderer::render(&mut canvas, Color::RGB(60, 64, 255 ), texture_to_display_1, &player1, texture_to_display_2, &player2)?;
            rendering_time_accumulated = 0.0;
        }

    }

    Ok(())
}