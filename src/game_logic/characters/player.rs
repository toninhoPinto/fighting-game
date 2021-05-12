use parry2d::na::Vector2;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use std::{collections::{HashMap, VecDeque}, fmt};

use crate::{asset_management::asset_holders::{EntityAnimations, EntityAssets, EntityData}, ecs_system::enemy_components::Health, engine_types::{animation::{AnimationState, ColliderAnimation}, animator::Animator, collider::Collider, sprite_data::SpriteData}, game_logic::{characters::AttackType, factories::character_factory::{CharacterAnimations, CharacterAssets, CharacterData}, inputs::{game_inputs::GameAction, input_cycle::AllInputManagement}, movement_controller::MovementController}, rendering::camera::Camera};

use super::{Ability, Character};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlayerState {
    Standing,
    Jump,
    Jumping,
    Landing,
    Dashing,
    Hurt,
    Dead,
}
impl fmt::Display for PlayerState {
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
    pub animation_state: Option<AnimationState>,
    pub character_width: f64,
    pub character: Character,

    pub curr_special_effect: Option<(i32, Ability)>,

    pub colliders: Vec<Collider>,
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
            animation_state: None,

            character_width: 0.0,
            character,

            curr_special_effect: None,

            colliders: Vec::new(),
        }
    }


    pub fn change_special_meter(&mut self, special: f32) {
        self.character.special_curr = ((self.character.special_curr + special)
            .clamp(0.0, self.character.special_max as f32)
            * 10.0)
            .round()
            / 10.0;
    }

    pub fn push(&self, level_width: i32, push_vec: Vector2<f64>) -> Vector2<f64>{
        if (self.position.x + push_vec.x - self.character_width) < 0.0 {
            Vector2::new( - (self.position.x - self.character_width), 0.0)
        } else if (self.position.x + push_vec.x + self.character_width) > level_width as f64 {
            Vector2::new(level_width  as f64 - (self.position.x + self.character_width), 0.0)
        } else {
            push_vec * 0.5
        }
        
    }

    pub fn attack(&mut self, character_assets: &EntityAnimations, character_data: &EntityData, attack_animation: String) {
        println!("ATTACK {}", attack_animation);
        if self.controller.player_can_attack() {
            self.controller.is_attacking = true;
            let special_effect = character_data.attack_effects.get(&attack_animation);
            if let Some(&special_effect) = special_effect {
                self.curr_special_effect = Some(special_effect);
            }

            if let Some(attack) = character_data.attacks.get(&attack_animation) {
                if attack.attack_type == AttackType::Special {
                    self.change_special_meter(-1.0); 
                }
            }

            if let Some(attack_anim) = character_assets.animations.get(&attack_animation) { 
                self.animator.play_once(attack_anim.clone(), 1.0, false);
            }

            if let Some(_) = self.animator.current_animation.as_ref().unwrap().collider_animation {
                self.init_colliders();
            }
        }
    }

    pub fn player_state_cancel(&mut self, _new_state: PlayerState) {
        self.controller.state = PlayerState::Standing;
    }

    pub fn apply_input_state(&mut self, action_history: &VecDeque<i32>) {
        if action_history.len() > 0 && GameAction::check_if_pressed(action_history[action_history.len()-1], GameAction::Right as i32) {
            self.controller.walking_dir.x = 1;
        }
        if action_history.len() > 0 && GameAction::check_if_pressed(action_history[action_history.len()-1], GameAction::Left as i32) {
            self.controller.walking_dir.x = -1;
        }

        if action_history.len() > 0 && GameAction::check_if_pressed(action_history[action_history.len()-1], GameAction::Jump as i32) {
            self.controller.jump();
        }
    }

    pub fn apply_input(&mut self,   
        character_anims: &EntityAnimations,
        character_data: &EntityData,
        inputs: &mut AllInputManagement) {
            
        let mut inputs_for_current_frame = if let Some(&last_action) = inputs.action_history.back() {last_action} else {0};

        if inputs_for_current_frame & GameAction::Dash as i32 > 0 {
            inputs_for_current_frame ^= GameAction::Dash as i32;
        }

        for &(recent_input, is_pressed) in inputs.input_new_frame.iter() {
            let recent_input_as_game_action = GameAction::from_translated_input(
                recent_input,
                inputs_for_current_frame,
                self.controller.facing_dir,
            )
            .unwrap();
            inputs_for_current_frame = GameAction::update_state(inputs_for_current_frame, (recent_input_as_game_action, is_pressed));
        }

        if Player::check_for_dash_inputs(inputs_for_current_frame, &inputs.action_history) {
            inputs_for_current_frame |= GameAction::Dash as i32;
        }
    
        let moving_horizontally = GameAction::Right as i32 | GameAction::Left as i32;
        let moving_vertically = GameAction::Up as i32 | GameAction::Down as i32;
        
        if inputs_for_current_frame & moving_horizontally == 0 {
            self.controller.set_velocity_x(0);
        } else {
            if inputs_for_current_frame & GameAction::Right as i32 > 0 {
                self.controller.set_velocity_x(1);
            }
            if inputs_for_current_frame & GameAction::Left as i32 > 0 {
                self.controller.set_velocity_x(-1);
            }
        }
        //100 0010

        if inputs_for_current_frame & moving_vertically == 0 {
            self.controller.walking_dir.y = 0;
        } else {
            if inputs_for_current_frame & GameAction::Up as i32 > 0 {
                self.controller.walking_dir.y = 1;
            }
            if inputs_for_current_frame & GameAction::Down as i32 > 0 {
                self.controller.walking_dir.y = -1;
            }
        }

        if inputs_for_current_frame & GameAction::Jump as i32 > 0 {
            self.controller.jump();
        }
        if inputs_for_current_frame & GameAction::Punch as i32 > 0 {
            self.check_attack_inputs(
                character_anims,
                character_data,
                GameAction::Punch,
                "light_punch".to_string(),
                &inputs.action_history,
            );
        }
        if inputs_for_current_frame & GameAction::Kick as i32 > 0 {
            self.check_attack_inputs(
                character_anims,
                character_data,
                GameAction::Kick,
                "light_kick".to_string(),
                &inputs.action_history,
            );
            }
        if inputs_for_current_frame & GameAction::Block as i32 > 0 { self.controller.is_blocking = true }
        if inputs_for_current_frame & GameAction::Dash as i32 > 0 {
            self.controller.player_state_change(PlayerState::Dashing);
        }
        if inputs_for_current_frame & GameAction::Slide as i32 > 0 {}

        inputs.action_history.push_back(inputs_for_current_frame);
        inputs.input_reset_timer.push(0);
        inputs.input_new_frame.clear();
    }

    //TODO kinda yikes but should work for now
    fn check_for_dash_inputs(current_actions: i32, last_inputs: &VecDeque<i32>) -> bool {
        let len = last_inputs.len();
        if len > 3 {
            let curr_input_right = GameAction::check_if_pressed(current_actions, GameAction::Right as i32);
            let curr_input_left = GameAction::check_if_pressed(current_actions, GameAction::Left as i32);
            if !(curr_input_right || curr_input_left) {
                return false;
            }

            if !(GameAction::check_if_pressed(last_inputs[len-1], GameAction::Right as i32) || 
            GameAction::check_if_pressed(last_inputs[len-1], GameAction::Left as i32)){
                
                return GameAction::check_if_pressed(last_inputs[len-2], GameAction::Right as i32) || 
                     GameAction::check_if_pressed(last_inputs[len-2], GameAction::Left as i32);
            }
        }
        return false;
    }
    
    fn check_attack_inputs(
        &mut self,
        character_anims: &EntityAnimations,
        character_data: &EntityData,
        recent_input_as_game_action: GameAction,
        animation_name: String,
        action_history: &VecDeque<i32>,
    ) {
        if let Some(special_input) = self.check_special_inputs(character_data, action_history) {
            self.attack(character_anims, character_data, special_input);
        } else if let Some(directional_input) = self.check_directional_inputs(
            character_data,
            action_history[action_history.len() - 1],
        ) {
            self.attack(character_anims, character_data, directional_input);
        } else {
            self.change_special_meter(0.1);
            if !self.controller.is_airborne {
                self.attack(character_anims, character_data, animation_name);
            } else if self.controller.is_airborne {
                self.attack(
                    character_anims,
                    character_data, 
                    format!("{}_{}", "airborne", animation_name),
                );
            } else {
                self.attack(
                    character_anims,
                    character_data, 
                    format!("{}_{}", "crouched", animation_name),
                );
            }
        }
    }
    
    fn check_special_inputs(
        &mut self,
        character_data: &EntityData,
        action_history: &VecDeque<i32>,
    ) -> Option<String> {
        //iterate over last inputs starting from the end
        //check of matches against each of the player.input_combination_anims
        //if no match
        // iterate over last inputs starting from the end -1
        //etc
        //if find match, play animation and remove that input from array
        let cleaned_history: VecDeque<i32> =
            action_history.iter().cloned().filter(|&z| z > 0).collect();
        for possible_combo in character_data.input_combination_anims.iter() {
            let size_of_combo = possible_combo.0.len();
            let size_of_history = cleaned_history.len();
            let mut j = 0;
            //TODO change special meter price per ability
            if self.character.special_curr >= 1.0 {
                if size_of_combo <= size_of_history {
                    for i in (size_of_history - size_of_combo)..cleaned_history.len() {
                        if cleaned_history[i] & possible_combo.0[j] > 0 {
                            j += 1;
                        } else {
                            break;
                        }
    
                        if j == size_of_combo {
                            println!("SPECIAL ATTACK");
                            return Some(possible_combo.1.clone());
                        }
                    }
                }
            }
        }
        None
    }
    
    fn check_directional_inputs(
        &mut self,
        character_data: &EntityData,
        recent_inputs: i32
    ) -> Option<String> {
        for possible_combo in character_data.directional_variation_anims.iter() {
            let (moves, name) = possible_combo;
    
            if GameAction::check_if_pressed(recent_inputs,moves.0  as i32) &&
                GameAction::check_if_pressed(recent_inputs,moves.1  as i32) 
            {
                return Some(name.to_string());
            }
        }
        None
    }

    pub fn update(
        &mut self,
        camera: &Camera,
        dt: f64,
        character_width: i32,
    ) {
       self.controller.update(&mut self.position, &self.character, &self.animator, camera, dt, character_width);
    }

    pub fn state_update(&mut self, assets: &EntityAnimations, sprite_data: &HashMap<String, SpriteData>) {
        let prev_animation = self.animator.current_animation.as_ref().unwrap().name.clone();

        self.controller.state_update(&mut self.animator, &mut self.position, &assets);
        self.animator.update();

        if let Some(animation) = self.animator.current_animation.as_ref() {
            if let Some(_) = animation.collider_animation {
                let animation_id = self.animator.sprite_shown as usize;
                let sprite_handle = animation.sprites[animation_id].1.clone();
                if prev_animation != self.animator.current_animation.as_ref().unwrap().name {
                    self.init_colliders();
                }
                
                self.update_colliders(sprite_data.get(&sprite_handle).unwrap());
            }
        }
    
    }


    pub fn init_colliders(&mut self) {
        let collider_animation = self.animator.current_animation.as_ref().unwrap().collider_animation.as_ref().unwrap();
        for i in 0..collider_animation.colliders.len() {
            if i < self.colliders.len() {
                //modify current
                self.colliders[i].collider_type = collider_animation.colliders[i].collider_type;
                self.colliders[i].name = collider_animation.colliders[i].name.clone();
                self.colliders[i].aabb = collider_animation.colliders[i].aabb;
                self.colliders[i].enabled = collider_animation.colliders[i].enabled;
            } else {
                //push
                self.colliders.push(Collider {
                    collider_type: collider_animation.colliders[i].collider_type,
                    name: collider_animation.colliders[i].name.clone(),
                    aabb: collider_animation.colliders[i].aabb,
                    enabled: collider_animation.colliders[i].enabled,
                });
            }
        }
        self.colliders.truncate(collider_animation.colliders.len());
    }
    
    // update offsets by player position
    pub fn update_colliders(&mut self, sprite_data: &SpriteData) {
        let collider_animation = self.animator.current_animation.as_ref().unwrap().collider_animation.as_ref().unwrap().clone();

        for i in 0..self.colliders.len() {
            let aabb = &mut self.colliders[i].aabb;
    
            aabb.mins.coords[0] = self.position.x as f32;
            aabb.maxs.coords[0] = self.position.x as f32;

            aabb.mins.coords[1] = self.position.y as f32;
            aabb.maxs.coords[1] = self.position.y as f32;

            self.sync_with_character_animation(&collider_animation, i);
        }
    }
    
    //render offsets by frame index
    fn sync_with_character_animation(
        &mut self,
        collider_animation: &ColliderAnimation,
        collider_index: usize,
    ) {
        let current_collider = &mut self.colliders[collider_index];
        let aabb = &mut current_collider.aabb;
        let original_collider = &collider_animation.colliders[collider_index];
        let original_aabb = original_collider.aabb;
    
        let positions_at_frame = collider_animation.pos_animations.get(&original_collider.name).unwrap();

        match positions_at_frame.get(&(self.animator.sprite_shown)) {
            Some(transformation) => {
                current_collider.enabled = true;
                let offset_x = transformation.pos.x as f32 * 2.0;
                let offset_y = transformation.pos.y as f32 * 2.0;

                if self.controller.facing_dir > 0 {
                    aabb.mins.coords[0] = self.position.x as f32 - (offset_x + original_aabb.maxs.x * 2.0 * transformation.scale.0);
                    aabb.maxs.coords[0] = self.position.x as f32 - offset_x;
                } else {
                    aabb.mins.coords[0] = self.position.x as f32 + offset_x;
                    aabb.maxs.coords[0] = self.position.x as f32 + offset_x + original_aabb.maxs.x * 2.0 * transformation.scale.0;
                }

                aabb.mins.coords[1] += offset_y;
                aabb.maxs.coords[1] +=
                    offset_y + original_aabb.maxs.y * 2.0 * transformation.scale.1;
            }
            //collider doesnt exist at this frame
            None => {
                current_collider.enabled = false;
            }
        }
    }



    pub fn render<'a>(&'a mut self, assets: &'a EntityAssets<'a>) -> (&'a Texture, (Rect, (f64, f64))) {
        let key = &self.animator.render();

        let sprite_data = assets.texture_data.get(key);
        
        let rect = &mut self.character.sprite;
        let mut offset = (0f64, 0f64);

        if let Some(sprite_data) = sprite_data {
            rect.resize(sprite_data.width * 2 , sprite_data.height * 2 );

            let pivot_x_offset = if self.controller.facing_dir > 0 {(1f64 - sprite_data.pivot_x)* 2.0 * sprite_data.width as f64} else {sprite_data.pivot_x * 2.0 * sprite_data.width as f64};
            let pivot_y_offset = sprite_data.pivot_y * 2.0 * sprite_data.height as f64;

            offset = if let Some(sprite_alignment) = self.animator.current_animation.as_ref().unwrap().sprite_alignments.get(&self.animator.sprite_shown) {
                (pivot_x_offset + self.controller.facing_dir as f64 * sprite_alignment.pos.x * 2.0, pivot_y_offset + sprite_alignment.pos.y * 2.0)
            } else {
                (pivot_x_offset, pivot_y_offset)
            };

        }
        (assets.textures.get(key).unwrap(), (rect.clone(), offset))
    }
}