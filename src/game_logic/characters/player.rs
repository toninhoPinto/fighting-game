use parry2d::na::Vector2;
use sdl2::rect::Point;
use sdl2::render::Texture;

use std::{collections::HashMap, fmt};

use crate::{asset_management::{animation::{Animation, ColliderAnimation}, animator::Animator, collider::Collider}, game_logic::{character_factory::{CharacterAssets, CharacterData}, characters::{AttackType, Character}}};
use crate::{
    asset_management::animation::AnimationState, game_logic::character_factory::CharacterAnimations,
    rendering::camera::Camera,
};

use super::Ability;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlayerState {
    Standing,
    Crouch,
    Crouching,
    UnCrouch,
    Jump,
    Jumping,
    Landing,
    DashingForward,
    DashingBackward,
    Grab,
    Grabbed,
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
    pub ground_height: i32,
    pub velocity_y: f64,

    pub direction_at_jump_time: i32,
    pub jump_initial_velocity: f64,
    pub extra_gravity: Option<f64>,

    pub prev_velocity_x: i32,
    pub velocity_x: i32,
    pub dir_related_of_other: i32,
    pub state: PlayerState,
    pub is_attacking: bool,
    pub is_blocking: bool,
    pub is_airborne: bool,
    pub is_pushing: bool,
    pub knock_back_distance: f64,

    pub animator: Animator,
    pub animation_state: Option<AnimationState>,
    pub flipped: bool,
    pub has_hit: bool,
    pub character_width: f64,
    pub character: Character,

    pub mid_jump_pos: f64,
    pub curr_special_effect: Option<(i32, Ability)>,

    pub colliders: Vec<Collider>,
}

impl Player {
    pub fn new(id: i32, character: Character, spawn_position: Point, flipped: bool) -> Self {
        Self {
            id,
            position: Vector2::new(spawn_position.x as f64, spawn_position.y  as f64),
            ground_height: spawn_position.y,

            direction_at_jump_time: 0,
            jump_initial_velocity: 2.0 * character.jump_height,
            mid_jump_pos: 0.0,
            velocity_y: 0.0,
            extra_gravity: None,

            prev_velocity_x: 0,
            velocity_x: 0,
            dir_related_of_other: 0,
            state: PlayerState::Standing,
            animator: Animator::new(),
            animation_state: None,
            is_attacking: false,
            is_airborne: false,
            is_blocking: false,
            has_hit: false,
            is_pushing: false,
            knock_back_distance: 0.0,
            flipped,
            character_width: 0.0,
            character,

            curr_special_effect: None,

            colliders: Vec::new(),
        }
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
            || self.state == PlayerState::DashingForward
            || self.state == PlayerState::DashingBackward
            || self.state == PlayerState::Dead)
    }

    pub fn player_can_move(&self) -> bool {
        !(self.is_attacking
            || self.is_airborne
            || self.knock_back_distance.abs() > 0.0
            || self.state == PlayerState::Dead
            || self.state == PlayerState::DashingForward
            || self.state == PlayerState::DashingBackward)
    }

    pub fn player_state_change(&mut self, new_state: PlayerState) {
        let is_interruptable = self.state != PlayerState::DashingForward
            && self.state != PlayerState::DashingBackward
            && self.state != PlayerState::Jumping
            && self.state != PlayerState::Jump;

        let already_crouching = (new_state == PlayerState::Crouch
            || new_state == PlayerState::Crouching)
            && (self.state == PlayerState::Crouch || self.state == PlayerState::Crouching);

        if is_interruptable && !already_crouching && self.state != PlayerState::Dead {
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

    pub fn update(
        &mut self,
        camera: &Camera,
        dt: f64,
        character_width: i32,
        opponent_position_x: f64,
    ) {
        if self.state == PlayerState::Jump {
            self.velocity_y = self.jump_initial_velocity / 0.5;
            self.direction_at_jump_time = self.velocity_x;
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

            let ground = if self.is_attacking && !self.has_hit {
                self.ground_height - 100
            } else {
                self.ground_height
            };
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
                self.position.y = self.ground_height as f64;
                self.position += Vector2::new(self.velocity_x as f64 * self.character.speed * dt * speed_mod, 0.0);
            }
        } else {
            match &self.animator.current_animation.as_ref().unwrap().offsets {
                Some(offsets) => {
                    let offset = offsets[self.animator.sprite_shown as usize];
                    self.velocity_x = (self.dir_related_of_other as f64 * offset.x).signum() as i32;
                    self.position += Vector2::new( self.dir_related_of_other as f64 * offset.x * dt, offset.y * dt)
                }
                None => { }
            }
            
        }

        //TODO float with != seems dangerous
        if opponent_position_x - self.position.x != 0.0 {
            self.dir_related_of_other = ((opponent_position_x - self.position.x) as i32).signum() ;
        }

        if (self.position.x  as i32 - character_width) < camera.rect.x() {
            self.position.x = (camera.rect.x() + character_width) as f64;
        }

        if (self.position.x as i32 + character_width) > (camera.rect.x() + camera.rect.width() as i32) {
            self.position.x = (camera.rect.x() + camera.rect.width() as i32 - character_width) as f64;
        }

        if self.velocity_x * self.dir_related_of_other < 0 {
            self.is_blocking = true;
        } else {
            self.is_blocking = false;
        }        
    }

    fn walk_anims(&mut self, character_animation: &HashMap<String, Animation>) {

        let walk_forward = self.velocity_x * -self.dir_related_of_other < 0;
        let changed_dir = self.prev_velocity_x != self.velocity_x;

        if walk_forward {
            self.animator
            .play_animation(character_animation.get("walk").unwrap().clone(), 1.0, false, false, changed_dir);
        } else {
            
            if character_animation.contains_key("walk_back") {
                self.animator
                    .play_animation(character_animation.get("walk_back").unwrap().clone(), 1.0, false, false, changed_dir);
            } else {
                self.animator
                    .play_animation(character_animation.get("walk").unwrap().clone(), 1.0, true, false, changed_dir);
            }
        }
    }

    pub fn state_update(&mut self, assets: &CharacterAnimations) {
        let character_animation = &assets.animations;
        let prev_animation = self.animator.current_animation.as_ref().unwrap().name.clone();

        if self.animator.is_finished && self.state != PlayerState::Dead {
            self.has_hit = false;
            self.flipped = self.dir_related_of_other > 0;

            if self.is_attacking {
                self.is_attacking = false;
            }

            if self.state == PlayerState::Jump {
                self.state = PlayerState::Jumping;
            }

            if self.state == PlayerState::Crouch {
                self.state = PlayerState::Crouching;
            }

            if self.state == PlayerState::Landing {
                self.state = PlayerState::Standing;
            }

            if self.state == PlayerState::UnCrouch {
                self.state = PlayerState::Standing;
            }

            if self.state == PlayerState::Grab {
                self.state = PlayerState::Standing;
            }

            if self.state == PlayerState::Hurt {
                self.state = PlayerState::Standing;
            }

            if self.state == PlayerState::DashingForward
                || self.state == PlayerState::DashingBackward
            {
                self.state = PlayerState::Standing;
            }
        }

        if self.has_hit && self.state == PlayerState::Landing {
            self.has_hit = false;
            self.position.y = self.ground_height as f64;
            self.is_attacking = false;
        }

        if !self.is_attacking {
            match self.state {
                PlayerState::Standing => {
                    self.flipped = self.dir_related_of_other > 0;
                    if self.velocity_x != 0 {
                        self.walk_anims(character_animation);
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
                        .play_once(character_animation.get("crouch").unwrap().clone(), 3.0, true);
                }

                PlayerState::Jumping => {
                    self.animator
                        .play_once(character_animation.get("neutral_jump").unwrap().clone(), 1.0, false);
                }

                PlayerState::Landing => {
                    self.flipped = self.dir_related_of_other > 0;
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap().clone(), 3.0, false);
                }

                PlayerState::UnCrouch => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap().clone(), 1.0, true);
                }

                PlayerState::Crouch => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap().clone(), 1.0, false);
                }

                PlayerState::Crouching => {
                    self.animator
                        .play(character_animation.get("crouching").unwrap().clone(), 1.0, false);
                }

                PlayerState::DashingForward => {
                    self.animator
                        .play_once(character_animation.get("dash").unwrap().clone(), 1.0, false);
                }

                PlayerState::DashingBackward => {
                    self.animator
                        .play_once(character_animation.get("dash_back").unwrap().clone(), 1.0, false);
                }
                PlayerState::Grab => {
                    self.animator
                        .play_once(character_animation.get("grab").unwrap().clone(), 1.0, false);
                }
                PlayerState::Grabbed => {}
                PlayerState::Hurt => {
                    self.animator
                        .play_once(character_animation.get("take_damage").unwrap().clone(), 1.0, false);
                }
            }
        }

        self.prev_velocity_x = self.velocity_x;
        self.animator.update();

        if let Some(_) = self.animator.current_animation.as_ref().unwrap().collider_animation {
            if prev_animation != self.animator.current_animation.as_ref().unwrap().name {
                self.init_colliders();
            }
            self.update_colliders();
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
    pub fn update_colliders(&mut self) {
        let left_player_pos =
        self.position.x as f32 - self.character.sprite.width() as f32 / 2.0;
    
        let collider_animation = self.animator.current_animation.as_ref().unwrap().collider_animation.as_ref().unwrap().clone();

        for i in 0..self.colliders.len() {
            let aabb = &mut self.colliders[i].aabb;
    
            aabb.mins.coords[0] = left_player_pos;
            aabb.mins.coords[1] = self.position.y as f32;
            aabb.maxs.coords[0] = left_player_pos;
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
    
                if self.flipped {
                    aabb.mins.coords[0] = (self.position.x as f32
                        + self.character.sprite.width() as f32 / 2.0)
                        - (offset_x + original_aabb.maxs.x * 2.0 * transformation.scale.0);
                    aabb.maxs.coords[0] = (self.position.x as f32
                        + self.character.sprite.width() as f32 / 2.0)
                        - offset_x;
                } else {
                    aabb.mins.coords[0] += offset_x;
                    aabb.maxs.coords[0] +=
                        offset_x + original_aabb.maxs.x * 2.0 * transformation.scale.0;
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



    pub fn render<'a>(&'a self, assets: &'a CharacterAssets<'a>) -> &'a Texture {
        assets.textures.get(&self.animator.render()).unwrap()
    }
}