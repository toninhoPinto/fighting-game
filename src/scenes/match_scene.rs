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
    EventPump,
};

use crate::{Transition, ecs_system::enemy_systems::{get_enemy_colliders, update_animations_enemies, update_behaviour_enemies, update_colliders_enemies, update_events, update_movement_enemies}, engine_types::collider::ColliderType, game_logic::{characters::{player::{EntityState}}, effects::hash_effects, factories::{character_factory::{load_character, load_character_anim_data, load_stage}, enemy_factory::{load_enemy_ryu_animations, load_enemy_ryu_assets}, item_factory::load_items}, game::Game, inputs::{game_inputs::GameAction, input_cycle::AllInputManagement}}, input::input_devices::InputDevices};
use crate::{
    asset_management::common_assets::CommonAssets,
    collision::collision_detector::detect_hit,
    engine_traits::scene::Scene,
    input::{self},
    rendering::{self, camera::Camera},
    ui::ingame::{bar_ui::Bar, segmented_bar_ui::SegmentedBar},
    GameStateData,
};

use super::overworld_scene::{hp_bar_init, item_list_init};

const MAX_UPDATES_AVOID_SPIRAL_OF_DEATH: i32 = 4;

const LEVEL_WIDTH: i32 = 2560;
const LEVEL_HEIGHT: i32 = 720;

//Screen dimension constants
const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

pub struct MatchScene {
    pub character: String,
    p1_inputs: AllInputManagement,
}

impl MatchScene {
    pub fn new(

        character: String,
    ) -> Self {
        Self {
            character,
            p1_inputs: AllInputManagement::new(),
        }
    }
}

impl Scene for MatchScene {
    fn run(
        &mut self,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,

        event_pump: &mut EventPump,
        input_devices: &mut InputDevices,
        canvas: &mut Canvas<Window>,
    ) -> Transition {
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

        let mut game = Game::new(game_state_data.player.as_ref().unwrap().clone(),camera);
        let mut items = load_items("assets/items/items.json".to_string());
        let effects = hash_effects();

        game.player
            .animator
            .play(p1_anims.animations.get("idle").unwrap().clone(), 1.0,false);

        game.player.collision_manager.init_colliders(&game.player.animator);

        let screen_res = canvas.output_size().unwrap();
        let mut hp_bars = hp_bar_init(
            screen_res,
            game.player.character.hp,
            game.player.hp.0,
        );

        let mut item_list = item_list_init(&game_state_data);
        
        let mut hit_stop = 0;

        let mut previous_time = Instant::now();
        let logic_timestep: f64 = 0.016;
        let mut logic_time_accumulated: f64 = 0.0;
        let mut update_counter = 0;

        let mut debug_pause = false;

        //let end_game_match = EndMatch::new(Rect::new(0, 0, 600, 600), Point::new(0, 0), font);

        loop {
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
                    Event::Quit { .. } => return Transition::Quit,
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
                        if input == Keycode::Escape {
                            game_state_data.player = Some(game.player.clone());
                            return Transition::Pop;
                        }
                        if input == Keycode::Num4 { //punch
                            game.player.equip_item(items.get_mut(&4).unwrap(), &effects);
                        }
                        if input == Keycode::Num8 { //launcher
                            game.player.equip_item(items.get_mut(&8).unwrap(), &effects);
                        }
                        if input == Keycode::Num6 { //poison
                            game.player.equip_item(items.get_mut(&20).unwrap(), &effects);
                        }
                        if input == Keycode::Num5 { //lifesteal
                            game.player.equip_item(items.get_mut(&19).unwrap(), &effects);
                        }
                        if input == Keycode::V { //hurt self
                            game.player.hp.0 -= 10;
                        }
                    }
                    _ => {}
                };
                input::controller_handler::handle_new_controller(
                    &input_devices.controller,
                    &input_devices. joystick,
                    &event,
                    &mut input_devices.joys,
                );

                let raw_input = input::input_handler::rcv_input(&event, &input_devices.controls);

                if let Some((controller_id, translated_input, is_pressed)) = raw_input {

                    let inputs_for_current_frame = if let Some(&last_action) = self.p1_inputs.action_history.back() {last_action} else {0};
                    let recent_input_as_game_action = GameAction::from_translated_input(
                        translated_input,
                        inputs_for_current_frame,
                        game.player.controller.facing_dir,
                    );
                    self.p1_inputs.input_new_frame ^= recent_input_as_game_action.unwrap() as i32;
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
                    if self.p1_inputs.input_new_frame != 0 {
                        game.player.apply_input(&p1_anims, &p1_data, &mut self.p1_inputs);
                    }

                    game.player.apply_input_state(&mut self.p1_inputs, &p1_anims, &p1_data);
                }

                self.p1_inputs.update_inputs_reset_timer();
                self.p1_inputs.update_input_buffer_reset_time();

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
                update_behaviour_enemies(&mut game.enemies, &mut game.player, &enemy_animations);
                update_movement_enemies(&mut game.enemies, &enemy_animations, &game.camera, logic_timestep);
                update_events(&mut game.enemies, &mut game.player, logic_timestep);
                update_colliders_enemies(&mut game.enemies, &enemy_assets);
                
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

                hp_bars.update(game.player.hp.0);
                if game.player.items.len() != item_list.rects.len() {
                    item_list.update(game.player.items.iter()
                        .map(|_| {Rect::new(0,0,32,32)})
                        .collect::<Vec<Rect>>()
                    );
                }

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
                    &game_state_data.item_sprites,
                    &item_list,
                    &hp_bars,
                   // &end_game_match,
                    false,
                )
                .unwrap();

                update_counter = 0;
            }
        }
    }
}
