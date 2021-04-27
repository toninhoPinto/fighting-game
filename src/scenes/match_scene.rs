use sdl2::{rect::Rect, render::TextureQuery};
use std::{
    collections::{HashMap, VecDeque},
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

use crate::{
    asset_management::collider::ColliderType, game_logic::{character_factory::{CharacterAnimations, load_character, load_character_anim_data, load_stage}, 
    characters::{Attack, AttackHeight, player::{Player, PlayerState}}, 
    game::Game, 
    inputs::{apply_inputs::{apply_input, apply_input_state},
     input_cycle::AllInputManagement, 
     process_inputs::{released_joystick_reset_directional_state, update_directional_state}}, saved_game::SavedGame}, 
    ui::ingame::end_match_ui::EndMatch,
};
use crate::asset_management::sound::audio_player;
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
const FRAME_AMOUNT_CAN_ROLLBACK: i16 = 7;

const LEVEL_WIDTH: i32 = 2560;
const LEVEL_HEIGHT: i32 = 720;

//Screen dimension constants
const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

pub struct Match {
    pub is_single_player: bool,
    pub is_local_versus: bool,
    pub p1_character: String,
    pub p2_character: String,

    p1_input_history: VecDeque<AllInputManagement>,
    p1_inputs: AllInputManagement,

    p2_inputs: AllInputManagement,

    game_rollback: VecDeque<SavedGame>,
}

impl Match {
    pub fn new(
        is_single_player: bool,
        is_local_versus: bool,
        p1_character: String,
        p2_character: String,
    ) -> Self {
        Self {
            is_single_player,
            is_local_versus,
            p1_character,
            p2_character,
            p1_input_history: VecDeque::new(),
            p1_inputs: AllInputManagement::new(),
            p2_inputs: AllInputManagement::new(),
            game_rollback: VecDeque::new(),
        }
    }

    pub fn reset(
        &mut self,
        is_single_player: bool,
        is_local_versus: bool,
        p1_character: String,
        p2_character: String,
    ) {
        self.is_single_player = is_single_player;
        self.is_local_versus = is_local_versus;
        self.p1_character = p1_character;
        self.p2_character = p2_character;
        self.p1_input_history.clear();
        self.p1_inputs = AllInputManagement::new();
        self.p2_inputs = AllInputManagement::new();
        self.game_rollback.clear();
    }

    fn hp_bars_init<'a>(screen_res: (u32, u32), p1_hp: i32, p2_hp: i32) -> [Bar<'a>; 2] {
        let p1_health_bar = Bar::new(
            10,
            20,
            screen_res.0 / 2 - 20,
            50,
            p1_hp,
            Some(Color::RGB(255, 100, 100)),
            None,
        );
        let p2_health_bar = Bar::new(
            screen_res.0 as i32 / 2 + 10,
            20,
            screen_res.0 / 2 - 20,
            50,
            p2_hp,
            Some(Color::RGB(255, 100, 100)),
            None,
        );

        [p1_health_bar, p2_health_bar]
    }

    fn special_bars_init<'a>(
        screen_res: (u32, u32),
        p1_special: i32,
        p2_special: i32,
    ) -> [SegmentedBar<'a>; 2] {
        let special_bar_width = 150;
        let p1_special_bar = SegmentedBar::new(
            10,
            screen_res.1 as i32 - 30,
            special_bar_width,
            10,
            p1_special,
            Some(Color::RGB(20, 250, 250)),
            None,
        );
        let p2_special_bar = SegmentedBar::new(
            screen_res.0 as i32 - (special_bar_width as i32 + 10 * p2_special),
            screen_res.1 as i32 - 30,
            special_bar_width,
            10,
            p2_special,
            Some(Color::RGB(20, 250, 250)),
            None,
        );

        [p1_special_bar, p2_special_bar]
    }
}


fn hit_opponent(attack: &Attack, time: f64, general_assets: &CommonAssets, 
    player_hitting: &mut Player, player_hit: &mut Player, player_hit_anims: &CharacterAnimations){
    
    audio_player::play_sound(general_assets.sound_effects.get("hit").unwrap());
    player_hit.take_damage(attack.damage);
    player_hit.state_update(&player_hit_anims);
    let dir_to_push = if player_hitting.is_airborne {
        player_hitting.direction_at_jump_time
    } else {
        player_hitting.dir_related_of_other
    };
    player_hit.knock_back(attack.push_back * dir_to_push.signum() as f64, time);
}

fn opponent_blocked(attack: &Attack, time: f64, general_assets: &CommonAssets, 
    player_hitting: &mut Player, player_hit: &mut Player, _player_hit_assets: &CharacterAnimations){
    
    audio_player::play_sound(general_assets.sound_effects.get("block").unwrap());
    let dir_to_push = if player_hitting.is_airborne {
        player_hitting.direction_at_jump_time
    } else {
        player_hitting.dir_related_of_other
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

    let texture_width =width * 2;
    let texture_height = height * 2;
    //^ * 2 above is to make the sprite bigger, and the hardcoded - 80 and -100 is because the sprite is not centered
    //this will have issues with other vfx
    game.spawn_vfx(
        Rect::new(
            point.x as i32 - texture_width as i32 / 2 - 80,
            point.y as i32 - texture_height as i32 / 2 - 100,
            texture_width,
            texture_height,
        ),
        hit_particle.to_string(),
        Some(Color::GREEN),
    );
}

fn did_sucessfully_block(point: naPoint<f32, U2>, attack: &Attack, player_blocking: &Player) -> bool{
    
    let blocked_low = attack.attack_height == AttackHeight::LOW && (player_blocking.state == PlayerState::Crouch || player_blocking.state == PlayerState::Crouching);

    let blocked_middle = attack.attack_height == AttackHeight::MIDDLE;

    let blocked_high = attack.attack_height == AttackHeight::HIGH && !(player_blocking.state == PlayerState::Crouch || player_blocking.state == PlayerState::Crouching);

    let blocked_all = attack.attack_height == AttackHeight::ALL;

    let facing_correct_dir = (point.x > player_blocking.position.x as f32 && player_blocking.flipped) || 
    (point.x < player_blocking.position.x as f32 && !player_blocking.flipped);

    player_blocking.is_blocking && (blocked_all || blocked_low || blocked_middle || blocked_high) && facing_correct_dir
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

        let (p1_assets, p1_anims, p1_data) = load_character_anim_data(texture_creator, &self.p1_character);
        let (p2_assets, p2_anims, p2_data) = load_character_anim_data(texture_creator, &self.p2_character);

        let stage = load_stage(texture_creator);
        let stage_rect = Rect::new(0, 0, LEVEL_WIDTH as u32, LEVEL_HEIGHT as u32);

        let camera: Camera = Camera::new(
            LEVEL_WIDTH as i32 / 2 - SCREEN_WIDTH as i32 / 2,
            0,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        );

        let player1 = load_character(
            &self.p1_character,
            Point::new(camera.rect.center().x - 200, 0),
            false,
            1,
        );
        let player2 = load_character(
            &self.p2_character,
            Point::new(camera.rect.center().x + 200, -50),
            true,
            2,
        );

        let mut game = Game::new(player1.clone(), player2.clone(), camera);

        game.player1
            .animator
            .play(p1_anims.animations.get("idle").unwrap().clone(), 1.0,false);
        game.player2
            .animator
            .play(p2_anims.animations.get("idle").unwrap().clone(), 1.0,false);

        let collider_animation = p1_anims
            .collider_animations
            .get(&(&game.player1).animator.current_animation.as_ref().unwrap().name);
        collider_animation.unwrap().init(&mut game.player1.colliders);

        let collider_animation = p2_anims
            .collider_animations
            .get(&(&game.player2).animator.current_animation.as_ref().unwrap().name);
        collider_animation.unwrap().init(&mut game.player2.colliders);

        let p1_width = game
            .player1.colliders
            .iter()
            .filter(|&c| c.collider_type == ColliderType::Pushbox)
            .last()
            .unwrap()
            .aabb
            .half_extents()
            .x;
        game.player1.character_width = p1_width as f64;

        let p2_width = game
            .player2.colliders
            .iter()
            .filter(|&c| c.collider_type == ColliderType::Pushbox)
            .last()
            .unwrap()
            .aabb
            .half_extents()
            .x;
            game.player2.character_width = p2_width as f64;

        let screen_res = canvas.output_size().unwrap();
        let mut hp_bars = Match::hp_bars_init(
            screen_res,
            game.player1.character.hp,
            game.player2.character.hp,
        );
        let mut special_bars = Match::special_bars_init(
            screen_res,
            game.player1.character.special_max,
            game.player2.character.special_max,
        );

        let mut hit_stop = 0;

        let mut previous_time = Instant::now();
        let logic_timestep: f64 = 0.016;
        let mut logic_time_accumulated: f64 = 0.0;
        let mut update_counter = 0;

        let mut debug_pause = false;
        let mut should_rollback = false;
        let mut rollback = 0;

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
                            should_rollback ^= true;
                        }
                        if input == Keycode::P {
                            debug_pause ^= true
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

                //needs also to return which controller/ which player
                let raw_input = input::input_handler::rcv_input(&event, &controls);

                if raw_input.is_some() {
                    let (controller_id, translated_input, is_pressed) = raw_input.unwrap();

                    let is_p1_input = connected_controllers.selected_controllers[0].is_some()
                        && controller_id == connected_controllers.selected_controllers[0].unwrap();
                    let is_p2_input = connected_controllers.selected_controllers[1].is_some()
                        && controller_id == connected_controllers.selected_controllers[1].unwrap();

                    if is_p1_input {
                        self.p1_inputs
                            .input_new_frame
                            .push_back((translated_input, is_pressed));
                    } else if is_p2_input {
                        self.p2_inputs
                            .input_new_frame
                            .push_back((translated_input, is_pressed));
                    }

                    let is_directional_input =
                        TranslatedInput::is_directional_input(translated_input);
                    if is_directional_input {
                        if is_p1_input {
                            if !is_pressed {
                                if is_p1_input {
                                    released_joystick_reset_directional_state(
                                        translated_input,
                                        &mut self.p1_inputs.directional_state_input,
                                    );
                                }
                            }
                            update_directional_state(
                                translated_input,
                                is_pressed,
                                &mut self.p1_inputs.directional_state_input,
                            );
                        } else if is_p2_input {
                            if !is_pressed {
                                if is_p1_input {
                                    released_joystick_reset_directional_state(
                                        translated_input,
                                        &mut self.p2_inputs.directional_state_input,
                                    );
                                }
                            }
                            update_directional_state(
                                translated_input,
                                is_pressed,
                                &mut self.p2_inputs.directional_state_input,
                            );
                        }
                    }
                }
            }

            //Update
            while logic_time_accumulated >= logic_timestep || rollback > 0 {
                update_counter += 1;
                if update_counter > MAX_UPDATES_AVOID_SPIRAL_OF_DEATH && rollback == 0 {
                    logic_time_accumulated = 0.0;
                }

                if rollback > 0 {
                    rollback -= 1;
                    if rollback > 0 {
                        self.p1_inputs = self
                            .p1_input_history
                            .get((FRAME_AMOUNT_CAN_ROLLBACK - rollback) as usize)
                            .unwrap()
                            .clone();
                    }
                }
                if should_rollback && rollback == 0 {
                    rollback = FRAME_AMOUNT_CAN_ROLLBACK;
                    //TODO IM LEAVING THE ROLLBACK SERIALIZATION TO THE END SO IT IS EASIER AND I DONT NEED TO KEEP CHANGING IT
                    /*
                    self.game_rollback
                        .get(0)
                        .unwrap()
                        .load(&mut game, &p1_assets, &p2_assets);
                    */
                    self.p1_inputs = self.p1_input_history.get(0).unwrap().clone();
                    should_rollback = false;
                }
                if rollback == 0 {
                    self.game_rollback.push_back(SavedGame::save(&game));
                    self.p1_input_history.push_back(self.p1_inputs.clone());
                    if self.p1_input_history.len() as i16 > FRAME_AMOUNT_CAN_ROLLBACK {
                        self.p1_input_history.pop_front();
                        self.game_rollback.pop_front();
                    }
                }

                if hit_stop > 0 {
                    hit_stop -= 1;
                    logic_time_accumulated -= logic_timestep;
                    break;
                }

                game.current_frame += 1;

                if game.player1.state != PlayerState::Dead
                    && game.player2.state != PlayerState::Dead
                {
                    if !self.p1_inputs.input_new_frame.is_empty() {
                        apply_input(&mut game.player1, &p1_anims, &p1_data, &mut self.p1_inputs);
                    }

                    if !self.p2_inputs.input_new_frame.is_empty() {
                        apply_input(&mut game.player2, &p2_anims, &p2_data, &mut self.p2_inputs);
                    }

                    apply_input_state(&mut game.player1, &self.p1_inputs.directional_state_input);
                    apply_input_state(&mut game.player2, &self.p2_inputs.directional_state_input);
                }
                
                self.p1_inputs.update_inputs_reset_timer();
                self.p1_inputs.update_special_inputs_reset_timer();

                self.p2_inputs.update_inputs_reset_timer();
                self.p2_inputs.update_special_inputs_reset_timer();

                game.player1.character_width = match game
                    .player1.colliders
                    .iter()
                    .filter(|&c| c.collider_type == ColliderType::Pushbox)
                    .last()
                {
                    Some(point) => point.aabb.half_extents().x as f64,
                    None => { game.player1.character_width },
                };

                game.player1.state_update(&p1_anims);
                game.player1.update(
                    &game.camera,
                    logic_timestep,
                    game.player1.character_width as i32,
                    game.player2.position.x,
                );

                game.player2.character_width = match game
                    .player2.colliders
                    .iter()
                    .filter(|&c| c.collider_type == ColliderType::Pushbox)
                    .last()
                {
                    Some(point) => point.aabb.half_extents().x as f64,
                    None => { game.player2.character_width },
                };

                game.player2.state_update(&p2_anims);
                game.player2.update(
                    &game.camera,
                    logic_timestep,
                    game.player2.character_width as i32,
                    game.player1.position.x,
                );

                if let Some(ability) = game.player1.curr_special_effect {
                    if ability.0 == game.player1.animator.sprite_shown {
                        ability.1(&mut game, 1, &p1_anims);
                        game.player1.curr_special_effect = None;
                    }
                }

                if let Some(ability) = game.player2.curr_special_effect {
                    if ability.0 == game.player2.animator.sprite_shown {
                        ability.1(&mut game, 2, &p2_anims);
                        game.player2.curr_special_effect = None;
                    }
                }
                
                Game::update_player_colliders(&mut game.player1,  &p1_anims);
                Game::update_player_colliders(&mut game.player2,  &p2_anims);

                let start_p1_pos = game.player1.position.clone();
                let start_p2_pos = game.player2.position.clone();

                detect_push(
                    &mut game.player1,
                    &mut game.player2,
                    LEVEL_WIDTH,
                    logic_timestep,
                );

                if !game.player1.has_hit {
                    match detect_hit(&game.player1.colliders, &game.player2.colliders) {
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
                                    &mut game.player1, &mut game.player2, &p2_anims);
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
                    if game.projectiles[i].player_owner == 1 {
                        match detect_hit(&game.projectiles[i].colliders, &game.player2.colliders) {
                            Some((point, name)) => {
                                let attack = &game.projectiles[i].attack;
                                if !did_sucessfully_block(point, attack, &game.player2) {
                                    hit_opponent(
                                        attack,
                                        logic_timestep,
                                        &general_assets, 
                                        &mut game.player1, &mut game.player2, &p2_anims);
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
                                (game.projectiles[i].on_hit)(&mut game.projectiles[i]);
                                break;
                            }
                            None => {}
                        }
                    } 
                }

                //TODO probably doesnt need to run unless there is a collision
                game.projectiles.retain(|p| p.is_alive);
                
                if !game.player2.has_hit {
                    match detect_hit(&game.player2.colliders, &game.player1.colliders) {
                        Some((point, name)) => {
                            game.player2.has_hit = true;
                            let attack = p2_data
                                .attacks
                                .get(&name.replace("?", ""))
                                .unwrap();

                            if !did_sucessfully_block(point, attack, &game.player1){
                                hit_opponent(
                                    attack,
                                    logic_timestep,
                                    &general_assets, 
                                    &mut game.player2, &mut game.player1, &p1_anims);
                                hit_particles(point, "special_hit", &general_assets, &mut game);
                                hit_stop = 10;
                            } else {
                                opponent_blocked(
                                    attack,
                                    logic_timestep,
                                    &general_assets, 
                                    &mut game.player2, &mut game.player1, &p1_anims);
                                hit_particles(point, "block", &general_assets, &mut game);
                                hit_stop = 5;
                            }
                        }
                        None => {}
                    }
                }
                
                for i in 0..game.projectiles.len(){
                    if game.projectiles[i].player_owner == 2 {
                        match detect_hit(&game.projectiles[i].colliders, &game.player1.colliders) {
                            Some((point, name)) => {

                                break;
                            }
                            None => {}
                        }
                    }
                }
                
                if game.player1.position != start_p1_pos {
                    println!("update position p1");
                    Game::update_player_colliders_position_only(&mut game.player1, start_p1_pos);
                }
                
                if game.player2.position != start_p2_pos {
                    println!("update position p2");
                    Game::update_player_colliders_position_only(&mut game.player2, start_p2_pos);
                }

                game.update_vfx(&general_assets);

                game.update_projectiles(&self.p1_inputs);

                game.camera.update(LEVEL_WIDTH, &game.player1, &game.player2);

                hp_bars[0].update(game.player1.character.hp);
                hp_bars[1].update(game.player2.character.hp);

                special_bars[0].update(game.player1.character.special_curr);
                special_bars[1].update(game.player2.character.special_curr);

                if rollback == 0 {
                    logic_time_accumulated -= logic_timestep;
                }
            }

            // Render
            if update_counter >= 0 {
                rendering::renderer::render(
                    canvas,
                    (&stage, stage_rect),
                    &mut game,
                    &p1_assets,
                    &p2_assets,
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
