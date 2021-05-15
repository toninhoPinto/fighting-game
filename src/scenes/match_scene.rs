use sdl2::{rect::Rect};
use std::{
    collections::HashMap,
    time::Instant,
};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Point,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump, GameControllerSubsystem, JoystickSubsystem,
};

use crate::{ecs_system::enemy_systems::{get_enemy_colliders, update_animations_enemies, update_behaviour_enemies, update_colliders_enemies, update_movement_enemies}, engine_types::collider::ColliderType, game_logic::{characters::{player::{EntityState}}, factories::{character_factory::{load_character, load_character_anim_data, load_stage}, enemy_factory::{load_enemy_ryu_animations, load_enemy_ryu_assets}}, game::Game, inputs::{input_cycle::AllInputManagement}}};
use crate::{
    asset_management::common_assets::CommonAssets,
    collision::collision_detector::detect_hit,
    engine_traits::scene::Scene,
    input::{self, controller_handler::Controller, translated_inputs::TranslatedInput},
    rendering::{self, camera::Camera},
    ui::ingame::{bar_ui::Bar, segmented_bar_ui::SegmentedBar},
    GameStateData,
};

const MAX_UPDATES_AVOID_SPIRAL_OF_DEATH: i32 = 4;

const LEVEL_WIDTH: i32 = 2560;
const LEVEL_HEIGHT: i32 = 720;

//Screen dimension constants
const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

pub struct Match {
    pub character: String,
    p1_inputs: AllInputManagement,
}

impl Match {
    pub fn new(

        character: String,
    ) -> Self {
        Self {
            character,
            p1_inputs: AllInputManagement::new(),
        }
    }

    fn hp_bars_init<'a>(screen_res: (u32, u32), p1_hp: i32) -> Bar<'a> {
        Bar::new(
            10,
            20,
            screen_res.0 / 2 - 20,
            50,
            p1_hp,
            Some(Color::RGB(255, 100, 100)),
            None,
        )
    }

    fn special_bars_init<'a>(
        screen_res: (u32, u32),
        p1_special: i32,
    ) -> SegmentedBar<'a> {
        SegmentedBar::new(
            10,
            screen_res.1 as i32 - 30,
            150,
            10,
            p1_special,
            Some(Color::RGB(20, 250, 250)),
            None,
        )
    }
}

impl Scene for Match {
    fn run(
        &mut self,
        game_state_stack: &mut Vec<Box<dyn Scene>>,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,

        event_pump: &mut EventPump,
        joystick: &JoystickSubsystem,
        controller: &GameControllerSubsystem,
        controls: &HashMap<String, TranslatedInput>,
        connected_controllers: &mut Controller,
        canvas: &mut Canvas<Window>,
    ) {
        let mut general_assets = CommonAssets::load(&texture_creator);

        let (p1_assets, p1_anims, p1_data) = load_character_anim_data(texture_creator, &self.character);

        let mut enemy_assets = HashMap::new();
        enemy_assets.insert("ryu", load_enemy_ryu_assets(texture_creator));
        let mut enemy_animations = HashMap::new();
        enemy_animations.insert("ryu", load_enemy_ryu_animations());

        let stage = load_stage(texture_creator);
        let stage_rect = Rect::new(0, 0, LEVEL_WIDTH as u32, LEVEL_HEIGHT as u32);

        let camera: Camera = Camera::new(
            //LEVEL_WIDTH as i32 / 2 - SCREEN_WIDTH as i32 / 2,
            0,
            0,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        );

        let player = load_character(
            &self.character,
            Point::new(200, 50),
            1,
        );

        let mut game = Game::new(player.clone(),camera);

        game.player
            .animator
            .play(p1_anims.animations.get("idle").unwrap().clone(), 1.0,false);

        game.player.collision_manager.init_colliders(&game.player.animator);

        let screen_res = canvas.output_size().unwrap();
        let mut hp_bars = Match::hp_bars_init(
            screen_res,
            game.player.character.hp,
        );
        let mut special_bars = Match::special_bars_init(
            screen_res,
            game.player.character.special_max
        );

        let mut hit_stop = 0;

        let mut previous_time = Instant::now();
        let logic_timestep: f64 = 0.016;
        let mut logic_time_accumulated: f64 = 0.0;
        let mut update_counter = 0;

        let mut debug_pause = false;

        //let end_game_match = EndMatch::new(Rect::new(0, 0, 600, 600), Point::new(0, 0), font);

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
                    Event::KeyDown {
                        keycode: Some(input),
                        ..
                    } => {
                        if input == Keycode::L {
                            game.enemies.add_enemy(game.player.position, enemy_animations.get("ryu").unwrap().animations.get("idle").unwrap().clone());
                        }
                        if input == Keycode::P {
                            debug_pause ^= true;
                            logic_time_accumulated = 0.0;
                            update_counter = 0;
                        }
                        if input == Keycode::Right && debug_pause {
                            logic_time_accumulated += logic_timestep;
                        }
                    }
                    _ => {}
                };
                input::controller_handler::handle_new_controller(
                    controller,
                    joystick,
                    &event,
                    connected_controllers,
                );

                let raw_input = input::input_handler::rcv_input(&event, &controls);

                if raw_input.is_some() {
                    let (controller_id, translated_input, is_pressed) = raw_input.unwrap();

                    self.p1_inputs
                        .input_new_frame
                        .push_back((translated_input, is_pressed));
                }
            }

            //Update
            while logic_time_accumulated >= logic_timestep {
                update_counter += 1;
                if update_counter >= MAX_UPDATES_AVOID_SPIRAL_OF_DEATH {
                    logic_time_accumulated = 0.0;
                }

                if hit_stop > 0 {
                    hit_stop -= 1;
                    logic_time_accumulated -= logic_timestep;
                    break;
                }

                game.current_frame += 1;

                if game.player.controller.state != EntityState::Dead
                {
                    if !self.p1_inputs.input_new_frame.is_empty() {
                        game.player.apply_input(&p1_anims, &p1_data, &mut self.p1_inputs);
                    }

                    game.player.apply_input_state(&self.p1_inputs.action_history, &p1_anims);
                }
                
                self.p1_inputs.update_inputs_reset_timer();

                game.player.character_width = match game
                    .player.collision_manager.colliders
                    .iter()
                    .filter(|&c| c.collider_type == ColliderType::Pushbox)
                    .last()
                {
                    Some(point) => point.aabb.half_extents().x as f64,
                    None => { game.player.character_width },
                };

                game.player.animator.update();
                game.player.update(
                    &game.camera,
                    &p1_anims,
                    logic_timestep,
                    game.player.character_width as i32,
                );
                game.player.state_update(&p1_anims, &p1_assets.texture_data);

                update_animations_enemies(&mut game.enemies);
                update_behaviour_enemies(&mut game.enemies, &game.player, &enemy_animations);
                update_movement_enemies(&mut game.enemies, &enemy_animations, &game.camera, logic_timestep);
                update_colliders_enemies(&mut game.enemies, &enemy_assets);

                if let Some(ability) = game.player.curr_special_effect {
                    if ability.0 == game.player.animator.sprite_shown {
                        ability.1(&mut game, 1, &p1_anims);
                        game.player.curr_special_effect = None;
                    }
                }
                
                let start_p1_pos = game.player.position.clone();

                //TODO probably doesnt need to run unless there is a collision
                game.projectiles.retain(|p| p.is_alive);
                
                for i in 0..game.projectiles.len(){
                    if game.projectiles[i].player_owner == 2 {
                        match detect_hit(&game.projectiles[i].colliders, &game.player.collision_manager.colliders) {
                            Some((point, name)) => {
                                break;
                            }
                            None => {}
                        }
                    }
                }
                
                if game.player.position != start_p1_pos {
                    Game::update_player_colliders_position_only(&mut game.player, start_p1_pos);
                }

                get_enemy_colliders( &mut game.player, 
                    &mut game.enemies, 
                    &mut game.hit_vfx, 
                    &mut hit_stop, 
                    logic_timestep, 
                    &general_assets, 
                    &p1_data, 
                    &enemy_animations);

                game.fx(&general_assets);
                game.update_vfx(&general_assets);

                game.update_projectiles(&self.p1_inputs, &p1_anims);

                game.camera.update(LEVEL_WIDTH, &game.player);

                hp_bars.update(game.player.character.hp);

                special_bars.update(game.player.character.special_curr);

                //crate::ecs_system::enemy_systems::update_animations_enemies(&mut enemy_manager);
                logic_time_accumulated -= logic_timestep;
            }

            // Render
            if update_counter > 0 {
                rendering::renderer::render(
                    canvas,
                    (&stage, stage_rect),
                    &mut game,
                    &p1_assets,
                    &enemy_assets,
                    &mut general_assets,
                    &hp_bars,
                    &special_bars,
                   // &end_game_match,
                    true,
                )
                .unwrap();

                update_counter = 0;
            }
        }
    }
}
