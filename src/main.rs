use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture};

use std::time::Duration;
use std::collections::HashMap;
use std::collections::VecDeque;

#[macro_use]
extern crate serde_derive;
extern crate preferences;

mod input;
mod rendering;
mod game_logic;
mod controls;

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

    let mut texture_to_display_1: &Texture;
    let mut texture_to_display_2: &Texture;

    //TODO should vary per animation, some are faster others are slower because of less frames
    let anim_speed = 0.35;


    //TODO this should be deserialized from somwhere that has each information per character
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
        last_directional_input: None
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
        last_directional_input: None
    };

    let mut controls: HashMap<_, game_logic::game_input::GameInputs> = controls::load_controls();
    let mut last_inputs: VecDeque<game_logic::game_input::GameInputs> = VecDeque::new();

    let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::JoyDeviceAdded {which, ..} => {
                    println!("added controller: {}", which);
                    let joy = joystick.open(which as u32).unwrap();
                    joys.insert(which, joy);
                },
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => { }
            }

            let input = input::input_handler::rcv_input(event, &mut controls);
            game_logic::game_input::apply_game_inputs(&mut player1, input, &mut last_inputs);
        }

        // Update
        i = (i + 1) % 255;

        // Render
        player1.animation_index = (player1.animation_index + (1.0 * anim_speed)) % player1.current_animation.len() as f32;
        player2.animation_index = (player2.animation_index + (1.0 * anim_speed)) % player2.current_animation.len() as f32;

        //TODO: trigger finished animation, instead make a function that can play an animation once and run callback at the end
        if (player1.animation_index as f32 + (1.0 * anim_speed)) as usize >= player1.current_animation.len() {
            player1.isAttacking = false;
            player1.animation_index = 0.0;
        }
        //TODO: animation logic, move somewhere else
        if !player1.isAttacking {
            //TODO: this is also game engine, try and move it away somewhere else
            if player1.state == game_logic::player::PlayerState::Standing {
                player1.position = player1.position.offset(player1.direction * player1.speed, 0);
            }

            //println!("{ } state anim", player1.state.to_string());

            if player1.state == game_logic::player::PlayerState::Standing {
                player1.dir_related_of_other = (player2.position.x - player1.position.x).signum();

                //TODO flip has a small animation i believe, also, have to take into account mixups
                //TODO needs to switch the FWD to BCK and vice versa
                player1.flipped = player1.dir_related_of_other > 0 ;

                if player1.direction * -player1.dir_related_of_other < 0 {
                    player1.current_animation = player1.animations.get("walk").unwrap();
                } else if player1.direction * -player1.dir_related_of_other > 0 {
                    player1.current_animation = player1.animations.get("walk_back").unwrap();
                } else {
                    player1.current_animation = player1.animations.get("idle").unwrap();
                }
            } else if player1.state == game_logic::player::PlayerState::Crouching {
                println!("crouch state anim");
                player1.current_animation = player1.animations.get("crouching").unwrap();
                player1.animation_index = 0.0;
            }

            if player1.prev_direction != player1.direction {
                player1.animation_index = 0.0;
            }

            player1.prev_direction = player1.direction;
        }

        //TODO fix array out of bounds, possibly due to a change of animation without resetting the index
        texture_to_display_1 = &player1.current_animation[player1.animation_index as usize];

        player2.dir_related_of_other = (player1.position.x - player2.position.x).signum();
        player2.flipped = player2.dir_related_of_other > 0 ;

        texture_to_display_2 = &player2.current_animation[player2.animation_index as usize];
        rendering::renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), texture_to_display_1, &player1, texture_to_display_2, &player2)?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}