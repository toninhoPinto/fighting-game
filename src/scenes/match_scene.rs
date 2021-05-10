use sdl2::{rect::Rect, render::TextureQuery};
use std::{
    collections::HashMap,
    time::Instant,
};

use parry2d::na::{Point as naPoint, U2};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Point,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump, GameControllerSubsystem, JoystickSubsystem,
};

use crate::{asset_management::{collider::ColliderType, sound::audio_player}, ecs_system::enemy_systems::update_animations_enemies, game_logic::{character_factory::{CharacterAnimations, CharacterAssets, load_character, load_character_anim_data, load_stage}, characters::{Attack, player::{Player, PlayerState}}, enemy_factory::{load_enemy_ryu_animations, load_enemy_ryu_assets}, game::Game, inputs::{input_cycle::AllInputManagement}}};
use crate::{
    asset_management::common_assets::CommonAssets,
    collision::collision_detector::{detect_hit, detect_push},
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

    pub fn reset(
        &mut self,
        is_single_player: bool,
        is_local_versus: bool,
        character: String,
    ) {
        self.character = character;
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


fn hit_opponent(attack: &Attack, time: f64, general_assets: &CommonAssets, 
    player_hitting: &mut Player, player_hit: &mut Player, player_hit_anims: &CharacterAnimations, player_assets: &CharacterAssets){
    
    audio_player::play_sound(general_assets.sound_effects.get("hit").unwrap());
    player_hit.take_damage(attack.damage);
    player_hit.state_update(&player_hit_anims, &player_assets.texture_data);
    let dir_to_push = if player_hitting.is_airborne {
        player_hitting.direction_at_jump_time
    } else {
        player_hitting.facing_dir
    };
    player_hit.knock_back(attack.push_back * dir_to_push.signum() as f64, time);
}

fn opponent_blocked(attack: &Attack, time: f64, general_assets: &CommonAssets, 
    player_hitting: &mut Player, player_hit: &mut Player, _player_hit_assets: &CharacterAnimations){
    
    audio_player::play_sound(general_assets.sound_effects.get("block").unwrap());
    let dir_to_push = if player_hitting.is_airborne {
        player_hitting.direction_at_jump_time
    } else {
        player_hitting.facing_dir
    };
    player_hit.knock_back(attack.push_back * dir_to_push.signum() as f64, time);
}

fn hit_particles(point: naPoint<f32, U2>, hit_particle: &str, general_assets: &CommonAssets, game: &mut Game) {
    let texture_id = &general_assets.hit_effect_animations.get(hit_particle).unwrap().sprites[0].1;
    let TextureQuery { width, height, .. } = general_assets
                            .hit_effect_textures
                            .get(texture_id)
                            .unwrap()
                            .query();

    let texture_width = width * 2;
    let texture_height = height * 2;
    //^ * 2 above is to make the sprite bigger, and the hardcoded - 80 and -100 is because the sprite is not centered
    //this will have issues with other vfx
    game.spawn_vfx(
        Rect::new(
            point.x as i32,
            point.y as i32 - texture_height as i32 / 2,
            texture_width,
            texture_height,
        ),
        false,
        hit_particle.to_string(),
        Some(Color::GREEN),
    );
}

fn did_sucessfully_block(point: naPoint<f32, U2>, attack: &Attack, player_blocking: &Player) -> bool{
    
    let facing_correct_dir = (point.x > player_blocking.position.x as f32 && player_blocking.facing_dir > 0) || 
    (point.x < player_blocking.position.x as f32 && !player_blocking.facing_dir > 0);

    player_blocking.is_blocking && facing_correct_dir
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

        game.player.init_colliders();

        let p1_width = game
            .player.colliders
            .iter()
            .filter(|&c| c.collider_type == ColliderType::Pushbox)
            .last()
            .unwrap()
            .aabb
            .half_extents()
            .x;
        game.player.character_width = p1_width as f64;


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

                if game.player.state != PlayerState::Dead
                {
                    if !self.p1_inputs.input_new_frame.is_empty() {
                        game.player.apply_input(&p1_anims, &p1_data, &mut self.p1_inputs);
                    }

                    game.player.apply_input_state(&self.p1_inputs.action_history);
                }
                
                self.p1_inputs.update_inputs_reset_timer();

                game.player.character_width = match game
                    .player.colliders
                    .iter()
                    .filter(|&c| c.collider_type == ColliderType::Pushbox)
                    .last()
                {
                    Some(point) => point.aabb.half_extents().x as f64,
                    None => { game.player.character_width },
                };

                game.player.state_update(&p1_anims, &p1_assets.texture_data);
                game.player.update(
                    &game.camera,
                    logic_timestep,
                    game.player.character_width as i32,
                );

                update_animations_enemies(&mut game.enemies);

                if let Some(ability) = game.player.curr_special_effect {
                    if ability.0 == game.player.animator.sprite_shown {
                        ability.1(&mut game, 1, &p1_anims);
                        game.player.curr_special_effect = None;
                    }
                }
                
                let start_p1_pos = game.player.position.clone();

                /*
                detect_push(
                    &mut game.player,
                    LEVEL_WIDTH,
                    logic_timestep,
                );
                */

                /*
                if !game.player.has_hit {
                    match detect_hit(&game.player.colliders, &game.player2.colliders) {
                        Some((point, name)) => {
                            game.player1.has_hit = true;
                            let attack = p1_data
                                .attacks
                                .get(&name.replace("?", ""))
                                .unwrap();
                            if !did_sucessfully_block(point, attack, &game.player1){
                                hit_opponent(
                                    attack,
                                    logic_timestep,
                                    &general_assets, 
                                    &mut game.player1, &mut game.player2, &p2_anims, &p2_assets);
                                hit_particles(point, "special_hit", &general_assets, &mut game);
                                hit_stop = 10;
                            } else {
                                opponent_blocked(
                                    attack,
                                    logic_timestep,
                                    &general_assets, 
                                    &mut game.player1, &mut game.player2, &p2_anims);
                                hit_particles(point, "block", &general_assets, &mut game);
                                hit_stop = 5;
                            }
                        }
                        None => {}
                    }
                }

                for i in 0..game.projectiles.len(){
                    if game.projectiles[i].player_owner == 1 && game.projectiles[i].colliders.len() > 0 {
                        match detect_hit(&game.projectiles[i].colliders, &game.player2.colliders) {
                            Some((point, name)) => {
                                let attack = &game.projectiles[i].attack;
                                if !did_sucessfully_block(point, attack, &game.player2) {
                                    hit_opponent(
                                        attack,
                                        logic_timestep,
                                        &general_assets, 
                                        &mut game.player1, &mut game.player2, &p2_anims, &p2_assets);
                                    hit_particles(point, "special_hit", &general_assets, &mut game);
                                    hit_stop = 10;
                                } else {
                                    opponent_blocked(
                                        attack,
                                        logic_timestep,
                                        &general_assets, 
                                        &mut game.player1, &mut game.player2, &p2_anims);
                                    hit_particles(point, "block", &general_assets, &mut game);
                                    hit_stop = 5;
                                }
                                (game.projectiles[i].on_hit)(point, &mut game.projectiles[i], &p1_anims);
                                break;
                            }
                            None => {}
                        }
                    } 
                }
                */

                //TODO probably doesnt need to run unless there is a collision
                game.projectiles.retain(|p| p.is_alive);
                
                for i in 0..game.projectiles.len(){
                    if game.projectiles[i].player_owner == 2 {
                        match detect_hit(&game.projectiles[i].colliders, &game.player.colliders) {
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
