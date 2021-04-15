use std::{collections::{HashMap, VecDeque}, time::Instant};
use sdl2::{rect::Rect, render::TextureQuery};

use parry2d::bounding_volume::BoundingVolume;
use sdl2::{EventPump, GameControllerSubsystem, JoystickSubsystem, event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};

use crate::{GameStateData, asset_management::{collider::ColliderType, common_assets::CommonAssets}, engine_traits::scene::Scene, input::{self, controller_handler::Controller, translated_inputs::TranslatedInput}, rendering, ui::ingame::{bar_ui::Bar, segmented_bar_ui::SegmentedBar}};
use crate::asset_management::sound::audio_player;

use super::{character_factory::{load_character, load_character_anim_data}, game::{Game, SavedGame}, inputs::{apply_inputs::apply_input, input_cycle::AllInputManagement}};
use super::inputs::process_inputs::{released_joystick_reset_directional_state, update_directional_state};
use super::inputs::apply_inputs::apply_input_state;

const FRAME_WINDOW_BETWEEN_INPUTS: i32 = 20;
const MAX_UPDATES_AVOID_SPIRAL_OF_DEATH: i32 = 4;
const FRAME_AMOUNT_CAN_ROLLBACK: i16 = 7;

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
        Self{
            is_single_player,
            is_local_versus,
            p1_character,
            p2_character,
            p1_input_history:  VecDeque::new(),
            p1_inputs: AllInputManagement::new(),
            p2_inputs: AllInputManagement::new(),
            game_rollback: VecDeque::new(),
        }
    }

    pub fn reset(&mut self,
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

    fn special_bars_init<'a>(screen_res: (u32, u32), p1_special: i32, p2_special: i32) -> [SegmentedBar<'a>; 2]{
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

impl Scene for Match {
    fn run(
        &mut self,
        game_state_stack: &mut Vec<Box<dyn Scene>>,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump, joystick: &JoystickSubsystem,
        controller: &GameControllerSubsystem,
        controls: &HashMap<String, TranslatedInput>,
        connected_controllers: &mut Controller,
        canvas: &mut Canvas<Window>) {

        let mut general_assets = CommonAssets::load(&texture_creator);

        let p1_assets = load_character_anim_data(texture_creator, &self.p1_character);
        let p2_assets = load_character_anim_data(texture_creator, &self.p2_character);

        let mut player1 = load_character(&self.p1_character, Point::new(400, 0), false, 1);
        let mut player2 = load_character(&self.p2_character, Point::new(700, -50), true, 2);

        let mut game = Game::new(&mut player1, &mut player2);

        game.player1
            .animator
            .play(p1_assets.animations.get("idle").unwrap(), false);
        game.player2
            .animator
            .play(p2_assets.animations.get("idle").unwrap(), false);

        let screen_res = canvas.output_size().unwrap();
        let mut hp_bars = Match::hp_bars_init(screen_res, game.player1.character.hp, game.player2.character.hp);
        let mut special_bars = Match::special_bars_init(screen_res, game.player1.character.special_max, game.player2.character.special_max);
        
        let mut previous_time = Instant::now();
        let logic_timestep: f64 = 0.016;
        let mut logic_time_accumulated: f64 = 0.0;
        let mut update_counter = 0;

        let mut debug_pause = false;
        let mut debug_rollback = false;
        let mut rollback = 0;

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
                            debug_rollback ^= true;
                        }
                        if input == Keycode::P {
                            debug_pause ^= true
                        }
                        if input == Keycode::Right && debug_pause {
                            logic_time_accumulated += logic_timestep;
                        }
                    },
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
                    
                    let is_p1_input = connected_controllers.selected_controllers[0].is_some() && controller_id == connected_controllers.selected_controllers[0].unwrap();
                    let is_p2_input = connected_controllers.selected_controllers[1].is_some() && controller_id == connected_controllers.selected_controllers[1].unwrap();

                    if is_p1_input {
                        self.p1_inputs.input_new_frame.push_back((translated_input, is_pressed));
                    } else if is_p2_input {
                        self.p2_inputs.input_new_frame.push_back((translated_input, is_pressed));
                    } 
                    
                    let is_directional_input = TranslatedInput::is_directional_input(translated_input);
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
                        }
                        else if is_p2_input {
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


                //end of input management
            }

            //Update
            while logic_time_accumulated >= logic_timestep || rollback > 0 {
                update_counter +=1;
                if update_counter > MAX_UPDATES_AVOID_SPIRAL_OF_DEATH && rollback == 0 {
                    logic_time_accumulated = 0.0;
                }

                if rollback > 0 {
                    rollback -= 1;
                    if rollback > 0 {
                        self.p1_inputs = self.p1_input_history.get((FRAME_AMOUNT_CAN_ROLLBACK - rollback) as usize).unwrap().clone();
                    }
                } 


                if debug_rollback {             
                    if rollback == 0 {
                        rollback = FRAME_AMOUNT_CAN_ROLLBACK;
                        game.load(&self.game_rollback.get(0).unwrap(), &p1_assets, &p2_assets);
                        self.p1_inputs = self.p1_input_history.get(0).unwrap().clone();
                        debug_rollback = false;
                    }
                }

                game.current_frame += 1;

                if rollback == 0 {
                    self.game_rollback.push_back(game.save());
                    self.p1_input_history.push_back(self.p1_inputs.clone());
                    if self.p1_input_history.len() as i16 > FRAME_AMOUNT_CAN_ROLLBACK {
                        self.p1_input_history.pop_front();
                        self.game_rollback.pop_front();
                    }
                }

                if !self.p1_inputs.input_new_frame.is_empty() {
                    apply_input(&mut game.player1, &p1_assets, 
                        &self.p1_inputs.directional_state_input,
                        &mut self.p1_inputs.input_new_frame, 
                        &mut self.p1_inputs.input_processed, &mut self.p1_inputs.input_processed_reset_timer,
                        &mut self.p1_inputs.action_history, &mut self.p1_inputs.special_reset_timer);
                }
                if !self.p2_inputs.input_new_frame.is_empty() {
                    apply_input(&mut game.player2, &p2_assets, 
                        &self.p2_inputs.directional_state_input,
                        &mut self.p2_inputs.input_new_frame, 
                        &mut self.p2_inputs.input_processed, &mut self.p2_inputs.input_processed_reset_timer,
                        &mut self.p2_inputs.action_history, &mut self.p2_inputs.special_reset_timer);
                }

                apply_input_state(&mut game.player1, &self.p1_inputs.directional_state_input);
                apply_input_state(&mut game.player2, &self.p2_inputs.directional_state_input);

                for i in 0..self.p1_inputs.input_processed_reset_timer.len() {
                    self.p1_inputs.input_processed_reset_timer[i] += 1;
                    if self.p1_inputs.input_processed_reset_timer[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                        self.p1_inputs.input_processed.pop_front();
                    }
                }
                self.p1_inputs.input_processed_reset_timer.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);


                for i in 0..self.p1_inputs.special_reset_timer.len() {
                    self.p1_inputs.special_reset_timer[i] += 1;
                    if self.p1_inputs.special_reset_timer[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                        if self.p1_inputs.action_history.len() > 1 {
                            self.p1_inputs.action_history.pop_front();
                        }
                    }
                }
                self.p1_inputs.special_reset_timer.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);





                for i in 0..self.p2_inputs.input_processed_reset_timer.len() {
                    self.p2_inputs.input_processed_reset_timer[i] += 1;
                    if self.p2_inputs.input_processed_reset_timer[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                        self.p2_inputs.input_processed.pop_front();
                    }
                }
                self.p2_inputs.input_processed_reset_timer.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);


                for i in 0..self.p2_inputs.special_reset_timer.len() {
                    self.p2_inputs.special_reset_timer[i] += 1;
                    if self.p2_inputs.special_reset_timer[i] > FRAME_WINDOW_BETWEEN_INPUTS {
                        if self.p2_inputs.action_history.len() > 1 {
                            self.p2_inputs.action_history.pop_front();
                        }
                    }
                }
                self.p2_inputs.special_reset_timer.retain(|&i| i <= FRAME_WINDOW_BETWEEN_INPUTS);

                hp_bars[0].update(game.player1.character.hp);
                hp_bars[1].update(game.player2.character.hp);

                special_bars[0].update(game.player1.character.special_curr);
                special_bars[1].update(game.player2.character.special_curr);

                game.player1.update(logic_timestep, game.player2.position.x);
                game.player2.update(logic_timestep, game.player1.position.x);

                game.player1.state_update(&p1_assets);
                game.player2.state_update(&p2_assets);

                if  game.player1.is_attacking {
                    //println!("is attacking {} -> {}", game.player1.animator.current_animation.unwrap().name,  game.player1.animator.animation_index);
                }

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

                
                let mut collision_point = None;
                //TODO, this cant be right, instead of iterating like this, perhaps use a quadtree? i think Parry2d has SimdQuadTree
                //TODO probably smartest is to record the hits, and then have a separate function to handle if there is a trade between characters??
                {
                    game.player1.is_pushing = false;
                    game.player2.is_pushing = false;
                    for collider in game.p1_colliders
                        .iter()
                        .filter(|&c| c.collider_type == ColliderType::Pushbox)
                    {
                        for collider_to_take_dmg in game.p2_colliders
                            .iter()
                            .filter(|&c| c.collider_type == ColliderType::Pushbox)
                        {
                            if collider.aabb.intersects(&collider_to_take_dmg.aabb) {
                                //println!("PUSH OR BE PUSHED");
                                if game.player1.velocity_x != 0 && game.player1.velocity_x.signum() == game.player1.dir_related_of_other {
                                    game.player2.push(game.player1.velocity_x, game.player1, logic_timestep);
                                    game.player1.is_pushing = true;
                                }

                                if game.player1.is_airborne {
                                    game.player2.push(game.player1.dir_related_of_other, game.player1, logic_timestep);
                                    game.player1.is_pushing = true;
                                }

                                if game.player2.velocity_x != 0 && game.player2.velocity_x.signum() == game.player2.dir_related_of_other {
                                    game.player1.push(game.player2.velocity_x, game.player2, logic_timestep);
                                    game.player2.is_pushing = true;
                                }

                                if game.player2.is_airborne {
                                    game.player1.push(game.player2.dir_related_of_other, game.player2, logic_timestep);
                                    game.player2.is_pushing = true;
                                }
                            }
                        }   
                    }

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
                                    audio_player::play_sound(general_assets.sound_effects.get("hit").unwrap());
                                    
                                    let attack = p1_assets.attacks.get(&game.player1.animator.current_animation.unwrap().name).unwrap();
                                    game.player1.has_hit = true;
                                    game.player2.take_damage(attack.damage);
                                    game.player2.state_update(&p2_assets);
                                    game.player2.knock_back(attack.push_back);
                                    
                                    let mut center = collider_to_take_dmg.aabb.center();
                                    let mut left = center;
                                    left.x -= collider_to_take_dmg.aabb.half_extents().x;
                                    let mut right = center;
                                    right.x += collider_to_take_dmg.aabb.half_extents().x;

                                    collision_point = Some(
                                        collider.aabb.clip_segment(
                                            &left,
                                            &right
                                        ).unwrap().a
                                    );
                                    println!("left{:?} right{:?} collision_point{:?}", left, right, collision_point.unwrap());
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
                                    println!("TAKE DMG");
                                    audio_player::play_sound(general_assets.sound_effects.get_mut("hit").unwrap());
                                    let attack = p2_assets.attacks.get(&game.player2.animator.current_animation.unwrap().name).unwrap();
                                    game.player2.has_hit = true;
                                    game.player1.take_damage(attack.damage);
                                    game.player1.state_update(&p1_assets);
                                    game.player1.knock_back(attack.push_back);
                                }
                            }
                        }
                    } 
                }
                
                if collision_point.is_some()  {
                    let point = collision_point.unwrap();
                    let TextureQuery { width, height, .. } = general_assets.hit_effect_animations.get("normal_hit").unwrap().sprites[0].query();

                    let texture_width = width * 2;
                    let texture_height = height * 2;
                    //^ * 2 above is to make the sprite bigger, and the hardcoded - 80 and -100 is because the sprite is not centered
                    //this will have issues with other vfx

                    game.spawn_vfx(
                        Rect::new(point.x as i32 - texture_width as i32 / 2 - 80, 
                            point.y as i32 - texture_height as i32 / 2 - 100, 
                            texture_width, texture_height), 
            "special_hit".to_string(), Some(Color::GREEN));
                }
                
                game.update_vfx(&general_assets);


                //Handle projectile movement
                //TODO maybe have a function inside game that does this, like update_vfx
                for i in 0..game.projectiles.len() {
                    game.projectiles[i].update();
                }

                if rollback == 0 {
                    logic_time_accumulated -= logic_timestep;
                }
            }

            // Render
            if update_counter >= 0 {
                rendering::renderer::render(
                    canvas,
                    Color::RGB(70, 70, 70),
                    game.player1,
                    &p1_assets,
                    game.player2,
                    &p2_assets,
                    &game.projectiles,
                    &mut game.hit_vfx,
                    &mut general_assets,
                    &mut game.p1_colliders,
                    &mut game.p2_colliders,
                    &hp_bars[0],
                    &hp_bars[1],
                    &special_bars[0],
                    &special_bars[1],
                    true,
                ).unwrap();

                update_counter = 0;
            }
        }
    }







}