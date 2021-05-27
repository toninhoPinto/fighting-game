use parry2d::na::Vector2;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use std::{collections::{HashMap, VecDeque}, fmt};

use crate::{asset_management::asset_holders::{EntityAnimations, EntityAssets, EntityData}, collision::collider_manager::ColliderManager, ecs_system::enemy_components::Health, engine_types::{animator::Animator, sprite_data::SpriteData}, game_logic::{effects::{Effect, ItemEffects, events_pub_sub::{CharacterEvent, EventsPubSub}}, inputs::{game_inputs::GameAction, input_cycle::AllInputManagement}, items::{Item, ItemType}, movement_controller::MovementController}, rendering::camera::Camera};

use super::Character;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EntityState {
    Idle,
    Walking,
    Jump,
    Jumping,
    Landing,
    Dashing,
    Hurt,
    Knocked,
    KnockedLanding,
    Dead,
}
impl fmt::Display for EntityState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Player {
    pub id: i32,
    pub position: Vector2<f64>,
    pub hp: Health,

    pub controller: MovementController,

    pub animator: Animator,
    pub character_width: f64,
    pub character: Character,

    pub collision_manager: ColliderManager,

    pub events: EventsPubSub,
    pub items: Vec<String>,
    pub active_item: Option<(CharacterEvent, Effect)>
}

impl Player {
    pub fn new(id: i32, character: Character, spawn_position: Point) -> Self {
        let pos_as_vec = Vector2::new(spawn_position.x as f64, spawn_position.y  as f64);
        Self {
            id,
            hp: Health(character.hp),
            position: pos_as_vec,

            controller: MovementController::new(&character, pos_as_vec, pos_as_vec),

            animator: Animator::new(),

            character_width: 0.0,
            character,

            collision_manager: ColliderManager::new(),

            events: EventsPubSub::new(),
            items: Vec::new(),

            active_item: None,
        }
    }

    pub fn equip_item(&mut self, item: &mut Item, hash_effects: &HashMap<i32, ItemEffects>){

        //check type of item
            //if active, replace with active item
            //if combat, apply effect function
            //if passive, apply effect function
        if item.item_type != ItemType::ActivePart {
            self.items.push(item.asset_id.clone());
            for effect in item.effects.iter_mut() {
                if let Some(apply_effect) = hash_effects.get(&effect.effect_id) {
                    apply_effect(self, effect);
                }
            }
        } else {
            
        }
    }

    pub fn attack(&mut self, character_assets: &EntityAnimations, _character_data: &EntityData, attack_animation: String) {
        if self.controller.can_attack() {
            self.controller.is_attacking = true;
            self.controller.combo_counter += 1;

            self.collision_manager.collisions_detected.clear();
            self.controller.has_hit = false;

            if let Some(attack_anim) = character_assets.animations.get(&attack_animation) { 
                self.animator.play_animation(attack_anim.clone(),1.0, false, true, true);
            }

            if let Some(_) = self.animator.current_animation.as_ref().unwrap().collider_animation {
                self.collision_manager.init_colliders(&self.animator);
            }
        }
    }

    pub fn apply_input_state(&mut self, inputs: &mut AllInputManagement, character_anims: &EntityAnimations, character_data: &EntityData) {
        if let Some(&last_action) = inputs.action_history.back() {
            if GameAction::is_pressed(last_action, GameAction::Right) { //1
                self.controller.set_velocity_x(1, &mut self.animator, character_anims);
            }
            if GameAction::is_pressed(last_action, GameAction::Left) { //-1
                self.controller.set_velocity_x(-1, &mut self.animator, character_anims);
            }

            if GameAction::is_pressed(last_action, GameAction::Jump) { 
                self.controller.jump(&mut self.animator, character_anims);
            }
        }

        let mut occupied = (self.controller.is_attacking && !self.controller.has_hit) ||
            self.controller.state == EntityState::Hurt ||
            self.controller.state == EntityState::Landing || 
            self.controller.state == EntityState::Dashing;

        let action_history = inputs.action_history.clone();

        inputs.input_buffer.retain(|&buffered_input| {

            occupied = (self.controller.is_attacking && !self.controller.has_hit) ||
            self.controller.state == EntityState::Hurt ||
            self.controller.state == EntityState::Landing || 
            self.controller.state == EntityState::Dashing;

            if !occupied {
                self.process_input(buffered_input, character_anims, character_data, &action_history);
            }

            occupied
        });
            
    }

    pub fn apply_input(&mut self,   
        character_anims: &EntityAnimations,
        character_data: &EntityData,
        inputs: &mut AllInputManagement) {
            
        let mut inputs_for_current_frame = if let Some(&last_action) = inputs.action_history.back() {last_action} else {0};
        inputs_for_current_frame ^= inputs.input_new_frame;

        if inputs_for_current_frame & GameAction::Dash as i32 > 0 {
            inputs_for_current_frame ^= GameAction::Dash as i32;
        }

        if Player::check_for_dash_inputs(inputs_for_current_frame, &inputs.action_history) {
            inputs_for_current_frame |= GameAction::Dash as i32;
        }

        let occupied = (self.controller.is_attacking && !self.controller.has_hit) ||
        self.controller.state == EntityState::Hurt ||
        self.controller.state == EntityState::Landing || 
        self.controller.state == EntityState::Dashing ;
        
        if inputs_for_current_frame != 0 && occupied {
            if inputs.input_buffer.len() < 1 {
                let mut input_to_buffer = inputs_for_current_frame;
                let dashing = inputs_for_current_frame & GameAction::Dash as i32 > 0;
                if !dashing {
                    if input_to_buffer & GameAction::Right as i32 > 0 {
                        input_to_buffer ^= GameAction::Right as i32;
                    }
                    if input_to_buffer & GameAction::Left as i32 > 0 {
                        input_to_buffer ^= GameAction::Left as i32;
                    }
                    if input_to_buffer & GameAction::Up as i32 > 0 {
                        input_to_buffer ^= GameAction::Up as i32;
                    }
                    if input_to_buffer & GameAction::Down as i32 > 0 {
                        input_to_buffer ^= GameAction::Down as i32;
                    }
                }
                inputs.input_buffer.push_front(input_to_buffer);
                inputs.input_buffer_reset_time.push(0);
            }
            
            inputs.action_history.push_back(inputs_for_current_frame);
            inputs.input_reset_timer.push(0);
            inputs.input_new_frame = 0;
            return;
        }

        self.process_input(inputs_for_current_frame, character_anims, character_data, &inputs.action_history);

        inputs.action_history.push_back(inputs_for_current_frame);
        inputs.input_reset_timer.push(0);
        inputs.input_new_frame = 0;
    }

    fn process_input(&mut self, 
        inputs_for_current_frame: i32, 
        character_anims: &EntityAnimations,
        character_data: &EntityData, 
        action_history: &VecDeque<i32>) {

        //println!("run inputs {:?}", GameAction::debug_i32(inputs_for_current_frame));

        let x = if inputs_for_current_frame & GameAction::Right as i32 > 0 {
            1i8
        } else if inputs_for_current_frame & GameAction::Left as i32 > 0 {
            -1i8
        } else {
            0i8
        };

        let y = if inputs_for_current_frame & GameAction::Up as i32 > 0 {
            1i8
        } else if inputs_for_current_frame & GameAction::Down as i32 > 0 {
            -1i8
        } else {
            0i8
        };

        self.controller.set_velocity(Vector2::new(x, y), &mut self.animator, character_anims);

        if inputs_for_current_frame & GameAction::Jump as i32 > 0 {
            self.controller.jump(&mut self.animator, character_anims);
        }
        if inputs_for_current_frame & GameAction::Punch as i32 > 0 {
            self.check_attack_inputs(
                character_anims,
                character_data,
                action_history,
                GameAction::Punch,
                "light_punch".to_string(),
            );
        }
        if inputs_for_current_frame & GameAction::Kick as i32 > 0 {
            self.check_attack_inputs(
                character_anims,
                character_data,
                action_history,
                GameAction::Kick,
                "light_kick".to_string(),
            );
        }
        if inputs_for_current_frame & GameAction::Block as i32 > 0 { self.controller.is_blocking = true }

        if inputs_for_current_frame & GameAction::Dash as i32 > 0 {
            self.controller.set_entity_state(EntityState::Dashing, &mut self.animator, character_anims);
        }

        if inputs_for_current_frame & GameAction::Slide as i32 > 0 {}
    }

    fn check_for_dash_inputs(current_actions: i32, last_inputs: &VecDeque<i32>) -> bool {
        let len = last_inputs.len();
        if len >= 2 {

            let repeated_actions = current_actions & last_inputs[len-2];
            let gap_frame_actions = repeated_actions & last_inputs[len-1];
            if repeated_actions > 0 {

                let avoid_dash_combo = !GameAction::is_pressed(last_inputs[len-2], GameAction::Dash);

                let dir_not_pressed = !(GameAction::is_pressed(gap_frame_actions, GameAction::Right) || 
                    GameAction::is_pressed(gap_frame_actions, GameAction::Left));

                let dir_pressed = GameAction::is_pressed(repeated_actions, GameAction::Right) || 
                GameAction::is_pressed(repeated_actions, GameAction::Left);
                
                return dir_not_pressed && dir_pressed && avoid_dash_combo;
            }
        }
        return false;
    }
    
    fn check_attack_inputs(
        &mut self,
        character_anims: &EntityAnimations,
        character_data: &EntityData,
        action_history: &VecDeque<i32>,
        recent_input_as_game_action: GameAction,
        animation_name: String,
    ) {
        if action_history.len() > 0  {
            if let Some(directional_input) = self.check_directional_inputs(
                character_data,
                action_history.back().unwrap() | recent_input_as_game_action as i32) {
                self.attack(character_anims, character_data, directional_input);
            } else {
                let mut combo_id = 0;
                let mut current_combo_length = 0;
                
                if recent_input_as_game_action == GameAction::Punch {
                    combo_id = 0;
                    current_combo_length = self.character.punch_string_curr;
                }
                if recent_input_as_game_action == GameAction::Kick {
                    combo_id = 1;
                    current_combo_length = self.character.kick_string_curr;
                }
                if self.controller.is_airborne {
                    combo_id += 2;
                    if recent_input_as_game_action == GameAction::Punch {
                        current_combo_length = self.character.airborne_punch_string_curr;
                    }
                    if recent_input_as_game_action == GameAction::Kick {
                        current_combo_length = self.character.airborne_kick_string_curr;
                    }
                } 

                if let Some(combo) = character_data.auto_combo_strings.get(&(combo_id)) {
                    let curr_combo_length = std::cmp::min(combo.len(), current_combo_length as usize);
                    let combo_number = self.controller.combo_counter as usize % curr_combo_length;
                    self.attack(character_anims, character_data, combo[combo_number].to_string());
                }
            }
        } else {
            if !self.controller.is_airborne {
                self.attack(character_anims, character_data, animation_name);
            } else {
                self.attack(
                    character_anims,
                    character_data, 
                    format!("{}_{}", "airborne", animation_name),
                );
            }
        }
    }
    
    fn check_directional_inputs(
        &mut self,
        character_data: &EntityData,
        recent_inputs: i32
    ) -> Option<String> {
        for possible_combo in character_data.directional_variation_anims.iter() {
            let (mask,moves, name) = possible_combo;
             
            if mask & self.character.directional_attacks_mask_curr != 0 {
                if  GameAction::is_pressed(recent_inputs,moves.0) &&
                    GameAction::is_pressed(recent_inputs,moves.1) 
                {
                    return Some(name.to_string());
                }
            }
            
        }
        None
    }

    pub fn update(
        &mut self,
        camera: &Camera,
        assets: &EntityAnimations,
        dt: f64,
        character_width: i32,
    ) {
       self.controller.update(&mut self.position, &self.character, &mut self.animator, assets, camera, dt, character_width);
    }

    pub fn state_update(&mut self, assets: &EntityAnimations, sprite_data: &HashMap<String, SpriteData>) {
        if self.animator.is_finished {
            self.collision_manager.collisions_detected.clear();
            self.controller.has_hit = false;
        }

        self.controller.state_update(&mut self.animator, &assets, true);

        self.collision_manager.update_colliders(self.controller.facing_dir > 0, 
            self.position,  &self.animator, sprite_data)
    }
    
    pub fn render<'a>(&'a mut self, assets: &'a EntityAssets<'a>) -> (&'a Texture<'a>, Rect, Point, bool, i32) {
        let key = &self.animator.render();

        let sprite_data = assets.texture_data.get(key);
        
        let rect = &mut self.character.sprite;
        let mut offset = (0f64, 0f64);

        if let Some(sprite_data) = sprite_data {
            rect.resize(sprite_data.width * 2 , sprite_data.height * 2 );

            let pivot_x_offset = if self.controller.facing_dir >= 0 {(1f64 - sprite_data.pivot_x)* 2.0 * sprite_data.width as f64} else {sprite_data.pivot_x * 2.0 * sprite_data.width as f64};
            let pivot_y_offset = sprite_data.pivot_y * 2.0 * sprite_data.height as f64;

            offset = if let Some(sprite_alignment) = self.animator.current_animation.as_ref().unwrap().sprite_alignments.get(&self.animator.sprite_shown) {
                (pivot_x_offset + self.controller.facing_dir as f64 * sprite_alignment.pos.x * 2.0, pivot_y_offset + sprite_alignment.pos.y * 2.0)
            } else {
                (pivot_x_offset, pivot_y_offset)
            };

        }
        
        let pos_to_render = Point::new((self.position.x - offset.0) as i32, (self.position.y - offset.1 )as i32 );
        (assets.textures.get(key).unwrap(), rect.clone(), pos_to_render, self.controller.facing_dir >= 0 , self.controller.ground_height)
    }
}