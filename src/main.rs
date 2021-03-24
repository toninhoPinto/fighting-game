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

use game_logic::projectile::Projectile;
use game_logic::character_factory::CharacterAnimationData;

//TODO list
//Projectiles
//Hold attacks
//2 inputs at the same time (grabs)
//attack animations that vary depending on distance
//dash attacks
//add movement to each attack
//add different animation speeds to each animation
//Improve dash smoothing

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

    let p1_anims = game_logic::character_factory::load_character_anim_data(&texture_creator, "ryu".to_string());

    let p2_anims = &p1_anims;

    let mut texture_to_display_1: Option<&Texture> = None;
    let mut texture_to_display_2: Option<&Texture> = None;

    //TODO should vary per animation, some are faster others are slower because of less frames
    let anim_speed = 0.35;

    //TODO move this to game_logic::player::
    let mut player1 = game_logic::character_factory::load_character("ryu".to_string(), Point::new(-200, 100), true, 1);
    let mut player2 = game_logic::character_factory::load_character("ryu".to_string(), Point::new(200, 100), false, 2);

    let mut controls: HashMap<_, game_logic::game_input::GameInputs> = controls::load_controls();
    let mut last_inputs: VecDeque<game_logic::game_input::GameInputs> = VecDeque::new();

    let mut input_reset_timers: Vec<i32> = Vec::new();

    let mut previous_time = Instant::now();
    let logic_timestep: f64 = 0.016;
    let mut logic_time_accumulated: f64 = 0.0;

    let rendering_timestep: f64 = 0.016; // 60fps
    let mut rendering_time_accumulated: f64 = 0.0;

    let mut p1_curr_anim: &Vec<Texture> = p1_anims.animations.get(&player1.current_animation).unwrap();
    let mut p2_curr_anim: &Vec<Texture> = p2_anims.animations.get(&player2.current_animation).unwrap();

    let mut projectiles: Vec<game_logic::projectile::Projectile> = Vec::new();

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
                    game_logic::game_input::apply_game_inputs(&p1_anims ,&mut player1, input, &mut last_inputs);
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
                    if player1.state == game_logic::player::PlayerState::DashingForward {
                        player1.position = player1.position.offset(player1.dir_related_of_other.signum() * player1.dash_speed, 0);
                    } else {
                        player1.position = player1.position.offset(-player1.dir_related_of_other.signum() * player1.dash_speed, 0);
                    }
                }

            }

            //Handle projectile movement
            for i in 0..projectiles.len() {
                //handle
                projectiles[i].position = projectiles[i].position.offset(projectiles[i].speed, 0);
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


            player1.animation_index = (player1.animation_index + anim_speed) % p1_curr_anim.len() as f32;
            player2.animation_index = (player2.animation_index + anim_speed) % p2_curr_anim.len() as f32;

            //println!("{:?}", player1.state);
            //println!("{:?} {:?}", (player1.animation_index as f32 + anim_speed as f32) as usize, player1.current_animation.len());
            //TODO: trigger finished animation, instead make a function that can play an animation once and run callback at the end
            if (player1.animation_index as f32 + anim_speed as f32) as usize >= p1_curr_anim.len() {
                //TODO temp location
                if player1.isAttacking && p1_anims.effects.contains_key(&player1.current_animation) {
                    let mut projectile = (*p1_anims.effects.get(&player1.current_animation).unwrap()).clone();
                    projectile.position = projectile.position.offset(player1.position.x(), 0);
                    projectile.direction = (player2.position.x - player1.position.x).signum();
                    projectile.flipped = player1.dir_related_of_other > 0;
                    projectile.player_owner = player1.id;
                    projectiles.push(projectile);
                }

                if player1.isAttacking {
                    player1.isAttacking = false;
                }


                if player1.state == game_logic::player::PlayerState::DashingForward ||
                    player1.state == game_logic::player::PlayerState::DashingBackward {
                    player1.state = game_logic::player::PlayerState::Standing;
                }
                println!("reset");
                player1.animation_index = 0.0;
            }

            if !player1.isAttacking {

                if player1.state == game_logic::player::PlayerState::Standing {
                    player1.dir_related_of_other = (player2.position.x - player1.position.x).signum();

                    //TODO flip has a small animation i believe, also, have to take into account mixups
                    //TODO needs to switch the FWD to BCK and vice versa
                    player1.flipped = player1.dir_related_of_other > 0;

                    if player1.direction * -player1.dir_related_of_other < 0 {
                        player1.current_animation = "walk".to_string();
                    } else if player1.direction * -player1.dir_related_of_other > 0 {
                        player1.current_animation = "walk_back".to_string();
                    } else {
                        player1.current_animation = "idle".to_string();
                    }
                } else if player1.state == game_logic::player::PlayerState::Crouching {
                    player1.current_animation = "crouching".to_string();
                } else if player1.state == game_logic::player::PlayerState::DashingForward {
                    player1.current_animation = "dash".to_string();
                } else if player1.state == game_logic::player::PlayerState::DashingBackward {
                    player1.current_animation = "dash_back".to_string();
                }

                if player1.state != game_logic::player::PlayerState::DashingForward &&
                    player1.state != game_logic::player::PlayerState::DashingBackward {
                    if player1.prev_direction != player1.direction {
                        player1.animation_index = 0.0;
                    }
                }


                player1.prev_direction = player1.direction;
            }



            p1_curr_anim = p1_anims.animations.get(&player1.current_animation).unwrap();
            p2_curr_anim = p2_anims.animations.get(&player2.current_animation).unwrap();

            //println!("{:?} {:?}", player1.current_animation, player1.animation_index);
            //TODO fix array out of bounds, possibly due to a change of animation without resetting the index
            texture_to_display_1 = Some(&p1_curr_anim[player1.animation_index as usize]);

            player2.dir_related_of_other = (player1.position.x - player2.position.x).signum();
            player2.flipped = player2.dir_related_of_other > 0;

            texture_to_display_2 = Some(&p2_curr_anim[player2.animation_index as usize]);

            rendering::renderer::render(&mut canvas, Color::RGB(60, 64, 255 ),
                                        texture_to_display_1, &player1, &p1_anims,
                                        texture_to_display_2, &player2, &p2_anims,
                                        &projectiles)?;

            rendering_time_accumulated = 0.0;
        }

    }

    Ok(())
}