use asset_management::collider::ColliderType;
use game_logic::{game::{Game, SavedGame}, inputs::{apply_inputs::{apply_input_state, apply_input}, process_inputs::{update_button_state, update_directional_state}}};
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
    let mut input_history: VecDeque<(TranslatedInput, bool)> = VecDeque::new();
    let mut input_processed: VecDeque<TranslatedInput> = VecDeque::new();
    let mut action_history: VecDeque<i32> = VecDeque::new();

    let mut special_reset_timer: Vec<i32> = Vec::new();
    
    let mut directional_state_input: [(TranslatedInput, bool); 4] = TranslatedInput::init_dir_input_state();
    let mut button_state_input: [(TranslatedInput, bool); 6] = TranslatedInput::init_button_input_state();
    
    //p2 inputs 

    let mut game = Game::new(&mut player1, &mut player2);
    let mut game_rollback: Option<SavedGame> = None;

    let mut previous_time = Instant::now();
    let logic_timestep: f64 = 0.016;
    let mut logic_time_accumulated: f64 = 0.0;
    let mut update_counter = 0;
    let mut frame_counter = 0;

    let mut is_single_player = true;
    let mut debug_pause = false;

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
                        match game_rollback {
                            Some(ref gr) => {game.load(&gr, &p1_assets, &p2_assets)},
                            None => {game_rollback = Some(game.save());}
                        }
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
                
                input_history.push_back((translated_input, is_pressed));

                let is_directional_input = TranslatedInput::is_directional_input(translated_input);
                if is_directional_input {
                    if !is_pressed {
                        released_joystick_reset_directional_state(
                            translated_input,
                            &mut directional_state_input,
                        );
                    }
                    update_directional_state(
                        translated_input,
                        is_pressed,
                        &mut directional_state_input,
                    );
                } else {
                    update_button_state(
                        translated_input,
                        is_pressed,
                        &mut button_state_input,
                    );
                }
            }
            //end of input management
        }

        //Update
        while logic_time_accumulated >= logic_timestep {
            update_counter +=1;
            frame_counter += 1;

            if update_counter > MAX_UPDATES_AVOID_SPIRAL_OF_DEATH {
                logic_time_accumulated = 0.0;
            }
         
            if !input_history.is_empty() {
                apply_input(&mut game.player1, &p1_assets, 
                    &directional_state_input, &button_state_input,
                    &mut input_history, &mut input_processed,
                    &mut action_history, &mut special_reset_timer);
            }
            apply_input_state(&mut game.player1, &directional_state_input, &button_state_input);

            for i in 0..special_reset_timer.len() {
                special_reset_timer[i] += 1;
                if special_reset_timer[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                    if action_history.len() > 1 {
                        action_history.pop_front();
                    }
                }
            }
            special_reset_timer.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);

            p1_health_bar.update(game.player1.character.hp);
            p2_health_bar.update(game.player2.character.hp);

            p1_special_bar.update(game.player1.character.special_curr);
            p2_special_bar.update(game.player2.character.special_curr);

            game.player1.update(logic_timestep, game.player2.position.x);
            game.player2.update(logic_timestep, game.player1.position.x);

            game.player1.state_update(&p1_assets);
            game.player2.state_update(&p2_assets);

            let collider_animation1 = p1_assets.collider_animations.get(&game.player1.animator.current_animation.unwrap().name);
            if collider_animation1.is_some() {
                if collider_animation1.unwrap().colliders.len() != game.p1_colliders.len() {
                    collider_animation1.unwrap().init(&mut game.p1_colliders);
                }
                collider_animation1.unwrap().update(&mut game.p1_colliders, &game.player1);
            }

            let collider_animation2 = p2_assets.collider_animations.get(&game.player2.animator.current_animation.unwrap().name);
            if collider_animation2.is_some() {
                if collider_animation2.unwrap().colliders.len() != game.p2_colliders.len() {
                    collider_animation2.unwrap().init(&mut game.p2_colliders);
                }
                collider_animation2.unwrap().update(&mut game.p2_colliders, &game.player2);
            }

            //TODO, this cant be right, instead of iterating like this, perhaps use a quadtree? i think Parry2d has SimdQuadTree
            //TODO probably smartest is to record the hits, and then have a separate function to handle if there is a trade between characters??
            if !game.player1.has_hit {
                for collider in game.p1_colliders
                    .iter()
                    .filter(|&c| c.collider_type == ColliderType::Hitbox)
                {
                    for collider_to_take_dmg in game.p2_colliders
                        .iter()
                        .filter(|&c| c.collider_type == ColliderType::Hurtbox)
                    {
                        if collider.aabb.intersects(&collider_to_take_dmg.aabb) {
                            println!("DEAL DMG");
                            game.player1.has_hit = true;
                            game.player2.take_damage(10);
                            game.player2.state_update(&p2_assets);
                        }
                    }
                }
            }
            
            if !game.player2.has_hit {
                for collider in game.p2_colliders
                    .iter()
                    .filter(|&c| c.collider_type == ColliderType::Hitbox)
                {
                    for collider_to_take_dmg in game.p1_colliders
                        .iter()
                        .filter(|&c| c.collider_type == ColliderType::Hurtbox)
                    {
                        if collider.aabb.intersects(&collider_to_take_dmg.aabb) {
                            game.player2.has_hit = true;
                            println!("TAKE DMG");
                        }
                    }
                }
            }

            //Handle projectile movement
            for i in 0..game.projectiles.len() {
                game.projectiles[i].update();
            }
       
            logic_time_accumulated -= logic_timestep;
        }

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
