use asset_management::collider::{Collider, ColliderAnimation, ColliderType};
use game_logic::inputs::{
    apply_inputs::apply_game_input_state, process_inputs::update_directional_state,
};
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
use crate::game_logic::inputs::apply_inputs::apply_game_inputs;
use crate::game_logic::inputs::game_inputs::GameInput;
use crate::game_logic::inputs::process_inputs::{
    filter_already_pressed_button, filter_already_pressed_direction,
    released_joystick_reset_directional_state, transform_input_state,
};
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
    let mut player2 = load_character(player2_character, Point::new(900, -50), true, 2);
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

    let mut p1_colliders: Vec<Collider> = Vec::new();
    let mut p2_colliders: Vec<Collider> = Vec::new();

    //controllers
    let mut controls: HashMap<_, TranslatedInput> = controls::load_controls();

    //inputs
    let mut input_reset_timers: Vec<i32> = Vec::new();
    let mut last_inputs: VecDeque<GameInput> = VecDeque::new();
    let mut current_state_input: [(GameInput, bool); 10] = GameInput::init_input_state();
    let mut directional_state_input: [(TranslatedInput, bool); 4] =
        TranslatedInput::init_dir_input_state();
    let mut _input_buffer: Vec<i32> = Vec::new();

    let mut projectiles: Vec<game_logic::projectile::Projectile> = Vec::new();


    let mut previous_time = Instant::now();
    let logic_timestep: f64 = 0.016;
    let mut logic_time_accumulated: f64 = 0.0;
    let mut update_counter = 0;

    let mut debug_pause = false;

    'running: loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(previous_time);
        let delta_time_as_mili =
            delta_time.as_secs() as f64 + (delta_time.subsec_nanos() as f64 * 1e-9);

        previous_time = current_time;

        if !debug_pause {
            logic_time_accumulated += delta_time_as_mili;
        }
        
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {keycode: Some(input),..} => {
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
                //This is to make hitboxes and keyboards work more similarly to joystick events
                let filtered_input;
                if is_pressed && TranslatedInput::is_directional_input(translated_input) {
                    filtered_input = filter_already_pressed_direction(
                        translated_input,
                        &mut directional_state_input,
                        &player1,
                    );
                } else {
                    filtered_input = Some(translated_input);
                }

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
                    )
                }

                if filtered_input.is_some() {
                    let non_direction_repeated_input = filtered_input.unwrap();
                    let game_input = GameInput::from_translated_input(
                        non_direction_repeated_input,
                        &current_state_input,
                        player1.dir_related_of_other,
                    )
                    .unwrap();

                    let filtered_buttons_input;
                    if is_pressed && !is_directional_input {
                        filtered_buttons_input =
                            filter_already_pressed_button(game_input, &mut current_state_input);
                    } else {
                        filtered_buttons_input = Some(game_input);
                    }

                    if filtered_buttons_input.is_some() {
                        let game_input = transform_input_state(
                            filtered_buttons_input.unwrap(),
                            is_pressed,
                            &mut current_state_input,
                            &mut directional_state_input,
                            &mut last_inputs,
                            &player1,
                        );
                        if game_input.is_some() {
                            let final_input = game_input.unwrap();
                            apply_game_inputs(
                                &p1_assets,
                                &mut player1,
                                final_input,
                                is_pressed,
                                &current_state_input,
                                &mut last_inputs,
                            );
                            if is_pressed {
                                input_reset_timers.push(0);
                            }
                        }
                    }
                }
            }
            //end of input management
        }

        //Update
        while logic_time_accumulated >= logic_timestep {
            update_counter +=1;

            if update_counter > MAX_UPDATES_AVOID_SPIRAL_OF_DEATH {
                logic_time_accumulated = 0.0;
            }

            apply_game_input_state(
                &p1_assets,
                &mut player1,
                &mut input_reset_timers,
                &directional_state_input,
                &mut current_state_input,
                &mut last_inputs,
            );

            //Number of frames to delete each input
            for i in 0..input_reset_timers.len() {
                input_reset_timers[i] += 1;
                if input_reset_timers[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                    last_inputs.pop_front();
                }
            }
            input_reset_timers.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);

            p1_health_bar.update(player1.character.hp);
            p2_health_bar.update(player2.character.hp);

            p1_special_bar.update(player1.character.special_curr);
            p2_special_bar.update(player2.character.special_curr);

            player1.update(logic_timestep, player2.position.x);
            player2.update(logic_timestep, player1.position.x);

            player1.state_update(&p1_assets);
            player2.state_update(&p2_assets);

            let collider_animation1 = p1_assets.collider_animations.get(&player1.animator.current_animation.unwrap().name);
            println!("{:?}", &player1.animator.current_animation.unwrap().name);
            if collider_animation1.is_some() {
                if collider_animation1.unwrap().colliders.len() != p1_colliders.len() {
                    collider_animation1.unwrap().init(&mut p1_colliders);
                }
                collider_animation1.unwrap().update(&mut p1_colliders, &player1);
            }

            let collider_animation2 = p2_assets.collider_animations.get(&player2.animator.current_animation.unwrap().name);
            if collider_animation2.is_some() {
                if collider_animation2.unwrap().colliders.len() != p2_colliders.len() {
                    collider_animation2.unwrap().init(&mut p2_colliders);
                }
                collider_animation2.unwrap().update(&mut p2_colliders, &player2);
            }

            //TODO, this cant be right, instead of iterating like this, perhaps use a quadtree? i think Parry2d has SimdQuadTree
            //TODO probably smartest is to record the hits, and then have a separate function to handle if there is a trade between characters??
            for collider in p1_colliders
                .iter()
                .filter(|&c| c.collider_type == ColliderType::Hitbox)
            {
                for collider_to_take_dmg in p2_colliders
                    .iter()
                    .filter(|&c| c.collider_type == ColliderType::Hurtbox)
                {
                    if collider.aabb.intersects(&collider_to_take_dmg.aabb) {
                        println!("DEAL DMG");
                    }
                }
            }

            for collider in p2_colliders
                .iter()
                .filter(|&c| c.collider_type == ColliderType::Hitbox)
            {
                for collider_to_take_dmg in p1_colliders
                    .iter()
                    .filter(|&c| c.collider_type == ColliderType::Hurtbox)
                {
                    if collider.aabb.intersects(&collider_to_take_dmg.aabb) {
                        println!("TAKE DMG");
                    }
                }
            }

            //Handle projectile movement
            for i in 0..projectiles.len() {
                projectiles[i].update();
            }
       
            logic_time_accumulated -= logic_timestep;
        }

        // Render
        if update_counter >= 0 {
            rendering::renderer::render(
                &mut canvas,
                Color::RGB(60, 64, 255),
                &mut player1,
                &p1_assets,
                &mut player2,
                &p2_assets,
                &projectiles,
                &mut p1_colliders,
                &mut p2_colliders,
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
