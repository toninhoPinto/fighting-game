use parry2d::na::Vector2;
use sdl2::{pixels::Color, rect::Rect, render::Texture};
use std::{collections::HashMap, rc::Rc, time::Instant};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};

use crate::{Transition, collision::collision_detection::{calculate_hits}, debug_console::console::Console, ecs_system::enemy_systems::{update_animations_enemies, update_colliders_enemies, update_events, update_movement_enemies}, enemy_behaviour::update_behaviour_enemies, engine_types::{collider::ColliderType, simple_animator::init_combo_animation}, game_logic::{characters::{player::{EntityState}, player_input::{apply_input_state, process_input}}, combo_string::{ComboCounter, manage_combo_resources::{Combo, update_and_manage}}, effects::hash_effects, factories::{character_factory::load_character_anim_data, enemy_factory::load_enemy_ryu_assets, item_factory::load_items}, game::Game, inputs::{game_inputs::GameAction, input_cycle::AllInputManagement}}, input::input_devices::InputDevices, level_generation::generate::generate_levels, rendering::renderer_ui::{render_combo, render_ui, text_gen}, ui::ingame::popup_ui::{PopUp, new_item_popup, popup_fade}};
use crate::{
    collision::collision_attack_resolution::detect_hit,
    engine_traits::scene::Scene,
    input::{self},
    rendering::{self, camera::Camera},
    GameStateData,
};

pub const MAX_UPDATES_AVOID_SPIRAL_OF_DEATH: i32 = 4;

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

        let mut console = Console {
            up: false,
            command: "".to_string(),
        };

        let (p1_assets, p1_data) = load_character_anim_data(texture_creator, &self.character);

        let mut enemy_assets = HashMap::new();
        enemy_assets.insert("ryu", load_enemy_ryu_assets(texture_creator));

        let levels = generate_levels(&game_state_data.level_assets.level_rooms, &mut game_state_data.map_rng.as_mut().unwrap());

        let camera: Camera = Camera::new(
            //LEVEL_WIDTH as i32 / 2 - SCREEN_WIDTH as i32 / 2,
            0,
            0,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        );

        let mut game = Game::new(game_state_data.player.as_ref().unwrap().clone(), camera, levels);

        game.player.position = Vector2::new(150.0f64, 150f64);

        game.player
            .animator
            .play(game.player.controller.animations.animations.get("idle").unwrap().clone(), 1.0,false);

        game.player.collision_manager.init_colliders(&game.player.animator);

        let player = &mut game.player;
        let mut start_level_events = player.events.on_start_level.clone();
        for event_on_lvl_start in start_level_events.iter_mut() {
            (event_on_lvl_start.0)(player, &mut game.enemies, -1, &mut event_on_lvl_start.1);
        }
        player.events.on_start_level = start_level_events;

        let mut combo = Combo::new();

        let screen_res = canvas.output_size().unwrap();

        let mut popup_item = new_item_popup(screen_res);
        let mut popup_content: Option<Vec<Texture>> = None;

        let mut item_list = crate::item_list_init(&game_state_data);
        
        let mut hit_stop = 0;

        let mut previous_time = Instant::now();
        let logic_timestep: f64 = 0.016;
        let mut logic_time_accumulated: f64 = 0.0;
        let mut update_counter = 0;

        let mut debug_pause = false;

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
            'kb_events: for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Transition::Quit,
                    Event::KeyDown {
                        keycode: Some(input),
                        ..
                    } => {
                        if input == Keycode::Escape {
                            game_state_data.player = Some(game.player.clone());
                            return Transition::Pop;
                        }
                        

                        if input == Keycode::L {
                            game.enemies.add_enemy(game.player.position + Vector2::new(500f64, 0f64), Rc::clone(game_state_data.enemy_animations.get("ryu").unwrap()));
                        }
                        if input == Keycode::P {
                            debug_pause ^= true;
                            logic_time_accumulated = 0.0;
                            update_counter = 0;
                        }
                        if input == Keycode::M {
                            game.player.currency += 1;
                        }
                        if input == Keycode::Right && debug_pause {
                            logic_time_accumulated += logic_timestep;
                        }
                        if input == Keycode::C { //hurt self
                            game.player.hp.0 -= 10;
                        }

                        if input == Keycode::Backslash {
                            console.toggle();
                        } else if input == Keycode::Return{
                            console.run(&mut game, &game_state_data.items, game_state_data)
                        } else {
                            console.add(input);
                        }
                        
                    }
                    _ => {}
                };

                if console.up {
                    break 'kb_events;
                }
                input::controller_handler::handle_new_controller(
                    &input_devices.controller,
                    &input_devices. joystick,
                    &event,
                    &mut input_devices.joys,
                );

                let raw_input = input::input_handler::rcv_input(&event, &input_devices.controls);

                if let Some((_controller_id, translated_input, _is_pressed)) = raw_input {

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
                        process_input(&mut game.player, &p1_data, &mut self.p1_inputs, &mut game.enemies);
                    }

                    apply_input_state(&mut game.player, &mut self.p1_inputs, &p1_data, &mut game.enemies);
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

                let mut player_update_events = game.player.events.on_update.clone();
                for event in player_update_events.iter_mut() {
                    event.0(&mut game.player, &mut game.enemies, -1, &mut event.1, logic_timestep);
                }
                game.player.events.on_update = player_update_events;

                game.player.animator.update();
                game.player.state_update(&p1_assets.texture_data);
                game.player.update(
                    &mut game.camera,
                    logic_timestep,
                    game.player.character_width as i32,
                    &game_state_data.general_assets
                );
                game.player.state_update(&p1_assets.texture_data);
               
                let player_position = if !game.player.controller.is_airborne {
                    game.player.position
                } else {
                    Vector2::new(game.player.position.x, game.player.controller.ground_height as f64)
                };
                
                let mut items_spawned = game.items_on_ground.clone();
                items_spawned.iter_mut().for_each(|item_ground| {

                    if (player_position - item_ground.position).magnitude() <= 50.0 {
                        game.player.equip_item(&mut item_ground.item, &game_state_data.effects, &mut game_state_data.energy_bar.as_mut().unwrap());
                        
                        popup_content = Some(crate::ui::ingame::popup_ui::render_popup(texture_creator, 
                            &item_ground.item.name, 
                            &item_ground.item.description, 
                            &game_state_data.general_assets.fonts.get("basic_font").unwrap(), 
                            &mut popup_item));
                        
                        if let Some(chance_mod) = &item_ground.item.chance_mod {
                            (chance_mod.modifier)(chance_mod.item_ids.clone(), chance_mod.chance_mod, &game.player.character, &mut game_state_data.general_assets.loot_tables);
                        } else {
                            for (_key, val) in game_state_data.general_assets.loot_tables.iter_mut() {
                                val.items.retain(|x| x.item_id as i32 != item_ground.item.id);
                                val.acc = val.items.iter().map(|i|{i.rarity}).sum();
                            }
                        }
                    }

                });

                game.items_on_ground.retain(|item_ground| {
                    return (player_position - item_ground.position).magnitude() >= 50.0;
                });
                   
                update_animations_enemies(&mut game.enemies);
                update_behaviour_enemies(&mut game.enemies, &mut game.player, logic_timestep);
                update_movement_enemies(&mut game.enemies, &mut game.camera, logic_timestep, &game_state_data.general_assets);
                update_events(&mut game.enemies, &mut game.player, logic_timestep);
                update_colliders_enemies(&mut game.enemies, &enemy_assets);

                let start_p1_pos = game.player.position.clone();

                //TODO probably doesnt need to run unless there is a collision
                game.projectiles.retain(|p| p.is_alive);
                
                for i in 0..game.projectiles.len(){
                    if game.projectiles[i].player_owner == 2 {
                        match detect_hit(&game.projectiles[i].colliders, &game.player.collision_manager.colliders) {
                            Some((_point, _name)) => {
                                break;
                            }
                            None => {}
                        }
                    }
                }
                
                if game.player.position != start_p1_pos {
                    Game::update_player_colliders_position_only(&mut game.player, start_p1_pos);
                }

                calculate_hits( &mut game.player, 
                    &mut game.enemies, 
                    &mut game.hit_vfx, 
                    &mut hit_stop, 
                    logic_timestep, 
                    &game_state_data.general_assets, 
                    &game_state_data.level_assets, 
                    &p1_data,
                    &mut combo.combo_counter,
                    &mut game.camera);

                game.fx(&game_state_data.level_assets);
                game.update_vfx(&game_state_data.level_assets);

                game.camera.update(game.max_level_width(), &game.player, logic_timestep);
                game.check_level_tags_and_apply(game_state_data);

                game_state_data.hp_bar.as_mut().unwrap().update(game.player.character.hp, game.player.hp.0);
                if game.player.items.len() != item_list.rects.len() {
                    item_list.update(game.player.items.iter()
                        .map(|_| {Rect::new(0,0,32,32)})
                        .collect::<Vec<Rect>>()
                    );
                }

                game_state_data.energy_bar.as_mut().unwrap().update_width(game.player.active_item_cost as i32, game.player.currency as i32);
                popup_fade(&mut popup_item, &mut popup_content, logic_timestep);


                update_and_manage(logic_timestep, &mut combo, &texture_creator,&game_state_data);

                logic_time_accumulated -= logic_timestep;
            }

            // Render
            if update_counter > 0 {
                canvas.clear();
                
                rendering::renderer::render(
                    canvas,
                    &mut game,
                    &p1_assets, 
                    &enemy_assets,
                    &mut game_state_data.level_assets,
                    &game_state_data.item_assets,
                   // &end_game_match,
                    false,
                )
                .unwrap();

                render_combo(canvas, &combo);

                render_ui(canvas, 
                    &game.player,
                    &game_state_data.hp_bar.as_ref().unwrap(),
                    &game_state_data.energy_bar.as_ref().unwrap(),
                    &item_list,
                    &game_state_data.item_assets,
                    Some(&popup_item),
                    &popup_content
                    );
                
                console.render(texture_creator, canvas, &game_state_data.general_assets.fonts.get("basic_font").unwrap());
                
                canvas.present(); 

                update_counter = 0;
            }
        
        }
    }
}
