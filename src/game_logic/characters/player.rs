use parry2d::na::Vector2;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use std::{collections::{HashMap, VecDeque}, fmt};

use crate::{asset_management::{animation::ColliderAnimation, animator::Animator, collider::Collider, sprite_data::SpriteData}, game_logic::{character_factory::{CharacterAssets, CharacterData}, characters::{AttackType, Character}, inputs::{game_inputs::GameAction, input_cycle::AllInputManagement}}, input::translated_inputs::TranslatedInput};
use crate::{
    asset_management::animation::AnimationState, game_logic::character_factory::CharacterAnimations,
    rendering::camera::Camera,
};

use super::Ability;

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
    pub walking_dir: Vector2<i8>,
    pub ground_height: i32,
    pub velocity_y: f64,

    pub direction_at_jump_time: i8,
    pub jump_initial_velocity: f64,
    pub extra_gravity: Option<f64>,

    pub facing_dir: i8,
    pub state: PlayerState,
    pub is_attacking: bool,
    pub is_blocking: bool,
    pub is_airborne: bool,
    pub is_pushing: bool,
    pub knock_back_distance: f64,

    pub animator: Animator,
    pub animation_state: Option<AnimationState>,
    pub character_width: f64,
    pub character: Character,

    pub mid_jump_pos: f64,
    pub curr_special_effect: Option<(i32, Ability)>,

    pub colliders: Vec<Collider>,
}

impl Player {
    pub fn new(id: i32, character: Character, spawn_position: Point) -> Self {
        Self {
            id,
            position: Vector2::new(spawn_position.x as f64, spawn_position.y  as f64),
            ground_height: spawn_position.y,

            direction_at_jump_time: 0,
            jump_initial_velocity: 2.0 * character.jump_height,
            mid_jump_pos: 0.0,
            velocity_y: 0.0,
            extra_gravity: None,

            facing_dir: 1,
            walking_dir: Vector2::new(0,0),
            state: PlayerState::Standing,
            animator: Animator::new(),
            animation_state: None,
            is_attacking: false,
            is_airborne: false,
            is_blocking: false,
            is_pushing: false,
            knock_back_distance: 0.0,
            character_width: 0.0,
            character,

            curr_special_effect: None,

            colliders: Vec::new(),
        }
    }

    pub fn set_velocity_x(&mut self, vec_x: i8) {
        if vec_x != 0 {
            self.facing_dir = vec_x;
        }
        self.walking_dir.x = vec_x;
    }

    pub fn take_damage(&mut self, damage: i32) {
        if self.character.hp > 0 {
            self.character.hp -= damage;
            self.state = PlayerState::Hurt;
        }

        if self.character.hp <= 0 {
            self.state = PlayerState::Dead;
        }
    }

    pub fn change_special_meter(&mut self, special: f32) {
        self.character.special_curr = ((self.character.special_curr + special)
            .clamp(0.0, self.character.special_max as f32)
            * 10.0)
            .round()
            / 10.0;
    }

    pub fn player_can_attack(&self) -> bool {
        !(self.is_attacking
            || self.state == PlayerState::Dashing
            || self.state == PlayerState::Dead)
    }

    pub fn player_can_move(&self) -> bool {
        !(self.is_attacking
            || self.is_airborne
            || self.knock_back_distance.abs() > 0.0
            || self.state == PlayerState::Dead
            || self.state == PlayerState::Dashing)
    }

    pub fn player_state_change(&mut self, new_state: PlayerState) {
        let is_interruptable = self.state != PlayerState::Dashing
            && self.state != PlayerState::Jumping
            && self.state != PlayerState::Jump;

        if is_interruptable && self.state != PlayerState::Dead {
            self.state = new_state;
        }
    }

    pub fn jump(&mut self) {
        if !self.is_airborne {
            self.player_state_change(PlayerState::Jump);
        }
    }

    pub fn knock_back(&mut self, amount: f64, dt: f64) {
        self.position += Vector2::new(amount * dt, 0.0);
        self.knock_back_distance = amount - (amount * 10.0 * dt);
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

    pub fn attack(&mut self, character_assets: &CharacterAnimations, character_data: &CharacterData, attack_animation: String) {
        println!("ATTACK {}", attack_animation);
        if self.player_can_attack() {
            self.is_attacking = true;
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
        self.state = PlayerState::Standing;
    }

    pub fn apply_input_state(&mut self, action_history: &VecDeque<i32>) {
        if action_history.len() > 0 && GameAction::check_if_pressed(action_history[action_history.len()-1], GameAction::Right as i32) {
            self.walking_dir.x = 1;
        }
        if action_history.len() > 0 && GameAction::check_if_pressed(action_history[action_history.len()-1], GameAction::Left as i32) {
            self.walking_dir.x = -1;
        }

        if action_history.len() > 0 && GameAction::check_if_pressed(action_history[action_history.len()-1], GameAction::Jump as i32) {
            self.jump();
        }
    }

    pub fn apply_input(&mut self,   
        character_anims: &CharacterAnimations,
        character_data: &CharacterData,
        inputs: &mut AllInputManagement) {
            
        let mut inputs_for_current_frame = if let Some(&last_action) = inputs.action_history.back() {last_action} else {0};

        if inputs_for_current_frame & GameAction::Dash as i32 > 0 {
            inputs_for_current_frame ^= GameAction::Dash as i32;
        }

        for &(recent_input, is_pressed) in inputs.input_new_frame.iter() {
            let recent_input_as_game_action = GameAction::from_translated_input(
                recent_input,
                inputs_for_current_frame,
                self.facing_dir,
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
            self.set_velocity_x(0);
        } else {
            if inputs_for_current_frame & GameAction::Right as i32 > 0 {
                self.set_velocity_x(1);
            }
            if inputs_for_current_frame & GameAction::Left as i32 > 0 {
                self.set_velocity_x(-1);
            }
        }
        //100 0010

        if inputs_for_current_frame & moving_vertically == 0 {
            self.walking_dir.y = 0;
        } else {
            if inputs_for_current_frame & GameAction::Up as i32 > 0 {
                self.walking_dir.y = 1;
            }
            if inputs_for_current_frame & GameAction::Down as i32 > 0 {
                self.walking_dir.y = -1;
            }
        }

        if inputs_for_current_frame & GameAction::Jump as i32 > 0 {
            self.jump();
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
        if inputs_for_current_frame & GameAction::Block as i32 > 0 { self.is_blocking = true }
        if inputs_for_current_frame & GameAction::Dash as i32 > 0 {
            self.player_state_change(PlayerState::Dashing);
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
        character_anims: &CharacterAnimations,
        character_data: &CharacterData,
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
            if !self.is_airborne {
                self.attack(character_anims, character_data, animation_name);
            } else if self.is_airborne {
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
        character_data: &CharacterData,
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
        character_data: &CharacterData,
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
        if self.state == PlayerState::Jump {
            self.ground_height = self.position.y as i32;
            self.velocity_y = self.jump_initial_velocity / 0.5;
            self.direction_at_jump_time = self.walking_dir.x;
        }

        if self.state == PlayerState::Jumping {
            self.is_airborne = true;
        }

        //TODO I KINDA HATE THIS
        if self.knock_back_distance.abs() > 0.0 {
            self.position += Vector2::new(self.knock_back_distance as f64 * dt, 0.0);
            self.knock_back_distance -= self.knock_back_distance * 10.0 * dt;
            if (self.knock_back_distance  * 100.0).round() / 100.0 <= 0.0 {
                self.knock_back_distance = 0.0;
            }
        }

        let speed_mod = if self.is_pushing { 0.5 } else { 1.0 };

        if self.is_airborne {
            let gravity = match self.extra_gravity {
                Some(extra_g) => extra_g,
                None => -2.0 * self.jump_initial_velocity / 0.25,
            };

            let ground = self.ground_height;
            let should_land = self.position.y < ground as f64;

            if !should_land {
                let position_offset_x = self.direction_at_jump_time as f64
                    * self.character.jump_distance
                    * dt
                    * speed_mod;

                self.velocity_y += gravity * dt;
                let position_offset_y = self.velocity_y * dt + 0.5 * gravity * dt * dt; //pos += vel * delta_time + 1/2 gravity * delta time * delta time
                self.position += Vector2::new(position_offset_x, position_offset_y);
            }

            //reset position back to ground height
            let should_land = self.position.y < ground as f64;
            if should_land {
                self.position.y = self.ground_height as f64;
                self.velocity_y = self.character.jump_height;
                if self.state == PlayerState::Jumping {
                    self.state = PlayerState::Landing;
                    self.is_attacking = false;
                }
                self.is_airborne = false;
            }
        }

        if self.player_can_move() {
            if self.state == PlayerState::Standing {
                self.ground_height = self.position.y as i32;
                let position_move = Vector2::new(
                    self.walking_dir.x as f64, 
                    self.walking_dir.y as f64
                );
                let normalized_movement = if position_move.magnitude() > 0f64 { position_move.normalize() } else {position_move};
                self.position += normalized_movement * self.character.speed * dt * speed_mod;
            }
        } else {
            match &self.animator.current_animation.as_ref().unwrap().offsets {
                Some(offsets) => {
                    let offset = offsets[self.animator.sprite_shown as usize];
                    self.walking_dir.x = (self.facing_dir as f64 * offset.x).signum() as i8;
                    self.position += Vector2::new( self.facing_dir as f64 * offset.x, offset.y) * dt
                }
                None => { }
            }
        }

        /*
        if (self.position.x  as i32 - character_width) < camera.rect.x() {
            self.position.x = (camera.rect.x() + character_width) as f64;
        }

        if (self.position.x as i32 + character_width) > (camera.rect.x() + camera.rect.width() as i32) {
            self.position.x = (camera.rect.x() + camera.rect.width() as i32 - character_width) as f64;
        }
        */  

    }

    pub fn state_update(&mut self, assets: &CharacterAnimations, sprite_data: &HashMap<String, SpriteData>) {
        let character_animation = &assets.animations;
        let prev_animation = self.animator.current_animation.as_ref().unwrap().name.clone();

        if self.animator.is_finished && self.state != PlayerState::Dead {
            self.walking_dir.x = 0;

            if self.is_attacking {
                self.is_attacking = false;
            }

            if self.state == PlayerState::Jump {
                self.state = PlayerState::Jumping;
            }

            if self.state == PlayerState::Landing {
                self.state = PlayerState::Standing;
            }

            if self.state == PlayerState::Hurt {
                self.state = PlayerState::Standing;
            }

            if self.state == PlayerState::Dashing
            {
                self.state = PlayerState::Standing;
            }
        }

        if self.state == PlayerState::Landing {
            self.position.y = self.ground_height as f64;
            self.is_attacking = false;
        }

        if !self.is_attacking {
            match self.state {
                PlayerState::Standing => {
                    if self.walking_dir.x != 0 || self.walking_dir.y != 0 {
                        self.animator
                            .play(character_animation.get("walk").unwrap().clone(), 1.0, false);
                    } else {
                        if self.animator.current_animation.as_ref().unwrap().name != "idle" {
                            self.animator
                            .play(character_animation.get("idle").unwrap().clone(), 1.0, false);
                        }
                        
                    }
                }

                PlayerState::Dead => {
                    self.animator
                        .play_once(character_animation.get("dead").unwrap().clone(), 1.0, false);
                }

                PlayerState::Jump => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap().clone(), 3.0, false);
                }

                PlayerState::Jumping => {
                    self.animator
                        .play_once(character_animation.get("neutral_jump").unwrap().clone(), 1.0, false);
                }

                PlayerState::Landing => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap().clone(), 3.0, true);
                }

                PlayerState::Dashing => {
                    self.animator
                        .play_once(character_animation.get("dash").unwrap().clone(), 1.0, false);
                }
                PlayerState::Hurt => {
                    self.animator
                        .play_once(character_animation.get("take_damage").unwrap().clone(), 1.0, false);
                }
            }
        }

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

                if self.facing_dir > 0 {
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



    pub fn render<'a>(&'a mut self, assets: &'a CharacterAssets<'a>) -> (&'a Texture, (Rect, (f64, f64))) {
        let key = &self.animator.render();

        let sprite_data = assets.texture_data.get(key);
        
        let rect = &mut self.character.sprite;
        let mut offset = (0f64, 0f64);

        if let Some(sprite_data) = sprite_data {
            rect.resize(sprite_data.width * 2 , sprite_data.height * 2 );

            let pivot_x_offset = if self.facing_dir > 0 {(1f64 - sprite_data.pivot_x)* 2.0 * sprite_data.width as f64} else {sprite_data.pivot_x * 2.0 * sprite_data.width as f64};
            let pivot_y_offset = sprite_data.pivot_y * 2.0 * sprite_data.height as f64;

            offset = if let Some(sprite_alignment) = self.animator.current_animation.as_ref().unwrap().sprite_alignments.get(&self.animator.sprite_shown) {
                (pivot_x_offset + self.facing_dir as f64 * sprite_alignment.pos.x * 2.0, pivot_y_offset + sprite_alignment.pos.y * 2.0)
            } else {
                (pivot_x_offset, pivot_y_offset)
            };

        }
        (assets.textures.get(key).unwrap(), (rect.clone(), offset))
    }
}