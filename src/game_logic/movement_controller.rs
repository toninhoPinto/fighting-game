use std::rc::Rc;

use parry2d::na::Vector2;

use crate::{asset_management::{asset_holders::EntityAnimations, common_assets::CommonAssets, sound::audio_player}, engine_types::animator::Animator, rendering::camera::Camera};

use super::characters::{Character, player::EntityState};

use crate::utils::math_sign::Sign;

#[derive(Clone)]
pub struct MovementController {
    pub walking_dir: Vector2<i8>,
    pub ground_height: i32,
    pub velocity_y: f64,

    pub direction_at_jump_time: i8,
    pub jump_initial_velocity: f64,
    pub can_double_jump: bool,
    pub is_double_jumping: bool,
    pub can_air_dash: bool,

    pub facing_dir: i8,
    pub state: EntityState,
    pub animations: Rc<EntityAnimations>,
    pub is_attacking: bool,
    pub is_blocking: bool,
    pub is_airborne: bool,
    pub is_falling: bool,
    pub has_hit: bool,
    pub combo_counter: i32,
        
    pub knock_back_distance: f64,
    pub mid_jump_pos: f64,
}

impl MovementController {

    pub fn new(character: &Character, starting_pos: Vector2<f64>, player_pos: Vector2<f64>, animations: Rc<EntityAnimations>) -> Self {
        Self {
            walking_dir: Vector2::new(0,0),
            ground_height: starting_pos.y as i32,
            velocity_y: 0f64,
        
            direction_at_jump_time: 0,
            jump_initial_velocity: 4.0 * character.jump_height,
            can_double_jump: character.can_double_jump,
            can_air_dash: character.can_air_dash,
        
            facing_dir: (player_pos.x - starting_pos.x).sign() as i8,
            state: EntityState::Idle,
            animations,
            is_attacking: false,
            is_blocking: false,
            is_airborne: false,
            is_falling: false,
            is_double_jumping: false,
            combo_counter: 0,
            has_hit: false,
            knock_back_distance: 0f64,
        
            mid_jump_pos: 0f64,
        }
    }

    pub fn set_velocity(&mut self, vec: Vector2<i8>, animator: &mut Animator) {
        if vec.x != 0 || vec.y != 0 {
            if vec.x != 0 {
                self.facing_dir = vec.x;
            }
            self.set_entity_state(EntityState::Walking, animator);
        } else {
            self.set_entity_state(EntityState::Idle, animator);
        }
        self.walking_dir = vec;
    }

    pub fn set_velocity_x(&mut self, x: i8, animator: &mut Animator) {
        if x != 0 {
            if x != 0 {
                self.facing_dir = x;
            }
            self.set_entity_state(EntityState::Walking, animator);
        } else {
            self.set_entity_state(EntityState::Idle, animator);
        }
        self.walking_dir.x = x;
    }

    pub fn can_dash_attack(&self) -> bool {
        !((self.is_attacking && !self.has_hit)
            || self.state == EntityState::Jump
            || self.state == EntityState::Dead)
    }

    pub fn can_attack(&self) -> bool {
        !((self.is_attacking && !self.has_hit)
            || self.state == EntityState::Jump
            || self.state == EntityState::Dashing
            || self.state == EntityState::Dead)
    }

    pub fn can_move(&self) -> bool {
        !((self.is_attacking && !self.has_hit)
            || self.is_airborne
            || self.knock_back_distance.abs() > 0.0
            || self.state == EntityState::Dead
            || self.state == EntityState::Dashing)
    }

    fn update_state(&mut self, new_state: EntityState, animator: &mut Animator) {
        self.state = new_state;
        let character_animation = &self.animations.animations;
        match self.state {
            EntityState::Idle => {
                animator
                    .play(character_animation.get("idle").unwrap().clone(), 1.0, false);
            }

            EntityState::Walking => {
                animator
                    .play(character_animation.get("walk").unwrap().clone(), 1.0, false);
            }

            EntityState::Dead => {
                animator
                    .play_once(character_animation.get("dead").unwrap().clone(), 1.0, false);
            }

            EntityState::Jump => {
                animator
                    .play_once(character_animation.get("crouch").unwrap().clone(), 3.0, false);
            }

            EntityState::Jumping => {
                animator
                    .play_once(character_animation.get("neutral_jump").unwrap().clone(), 1.0, false);
            }

            EntityState::Landing => {
                animator
                    .play_once(character_animation.get("crouch").unwrap().clone(), 3.0, true);
            }

            EntityState::Dashing => {
                if !self.is_airborne {
                    animator
                        .play_once(character_animation.get("dash").unwrap().clone(), 1.0, false);
                } else {
                    animator
                        .play_once(character_animation.get("air-dash").unwrap().clone(), 1.0, false);
                }
            }
            EntityState::Hurt => {
                animator
                    .play_once(character_animation.get("take_damage").unwrap().clone(), 1.0, false);
            }
            EntityState::Knocked | EntityState::Dropped => {
                if self.is_airborne {
                    animator
                    .play_once(character_animation.get("launched").unwrap().clone(), 1.0, false);
                }
            }
            EntityState::KnockedLanding | EntityState::DroppedLanding => {
                let animation = character_animation.get("knock_land");
                if let Some(animation) = animation {
                    animator.play_once(animation.clone(), 1.0, false);
                }
            }
        }
    }

    pub fn set_entity_state(&mut self, new_state: EntityState, animator: &mut Animator) {
        
        let can_idle = 
            self.state == EntityState::Idle ||
            self.state == EntityState::Walking || 
            self.state == EntityState::Landing ||
            (self.state == EntityState::Dashing && animator.is_finished) || 
            self.state == EntityState::Hurt ||
            self.state == EntityState::KnockedLanding || 
            self.state == EntityState::DroppedLanding;

        let can_walk = 
            self.state == EntityState::Idle ||
            self.state == EntityState::Walking ||
            (self.state == EntityState::Landing && animator.is_finished) ||
            (self.state == EntityState::Dashing && animator.is_finished) || 
            (self.state == EntityState::Hurt && animator.is_finished) ||
            (self.state == EntityState::KnockedLanding && animator.is_finished) ||
            (self.state == EntityState::DroppedLanding && animator.is_finished);

        let can_jump = 
            (self.is_attacking && self.has_hit) ||
            self.state == EntityState::Idle ||
            self.state == EntityState::Jumping ||
            self.state == EntityState::Walking;

        let interrupt_attack = new_state == EntityState::Landing || 
            new_state == EntityState::Hurt;

        let cancel_attack = (self.is_attacking && self.has_hit) && (
            new_state == EntityState::Jump ||
            new_state == EntityState::Dashing 
            );
        
        let can_dash = !self.is_airborne && 
        (
            self.state == EntityState::Idle ||
            self.state == EntityState::Walking ||
            (self.state == EntityState::Landing && animator.is_finished) ||
            (self.state == EntityState::Dashing && animator.is_finished) || 
            (self.state == EntityState::Hurt && animator.is_finished) ||
            (self.state == EntityState::KnockedLanding && animator.is_finished) ||
            (self.state == EntityState::DroppedLanding && animator.is_finished)
        );

        let can_air_dash =(self.is_airborne && self.can_air_dash) && self.state != EntityState::Knocked;
        let got_hurt = new_state == EntityState::Hurt || new_state == EntityState::Knocked || new_state == EntityState::Dropped || new_state == EntityState::Dead;
        
        if (!self.is_attacking || (self.is_attacking && (interrupt_attack || got_hurt)) || cancel_attack )  && self.state != EntityState::Dead {
            match new_state {
                EntityState::Idle => {
                    if can_idle {
                        self.update_state(new_state, animator);
                    }
                },
                EntityState::Walking => {
                    if can_walk {
                        self.update_state(new_state, animator);
                    }
                },
                EntityState::Jump => {
                    if can_jump {
                        self.update_state(new_state, animator);
                    }
                },
                EntityState::Dashing => {
                    if can_dash || can_air_dash{
                        self.update_state(new_state, animator);
                    }
                },
                _ => {
                    self.update_state(new_state, animator);
                }
            }
        }
    }

    fn should_pause_gravity(&self) -> bool {
        !self.is_attacking && self.state != EntityState::Hurt && self.state != EntityState::Dashing
    }

    pub fn jump(&mut self, animator: &mut Animator) {
        if !self.is_airborne || (self.is_airborne && self.can_double_jump && !self.is_double_jumping) {
            if self.is_airborne && self.can_double_jump {
                self.is_double_jumping = true;
            }
            self.set_entity_state(EntityState::Jump, animator);
        }
    }

    pub fn launch(&mut self, animator: &mut Animator) {
        self.is_airborne = true;
        self.is_attacking = false;
        self.set_entity_state(EntityState::Knocked, animator);
        self.velocity_y = self.jump_initial_velocity * 0.75f64;
        self.direction_at_jump_time = 0;
    }

    pub fn dropped(&mut self, animator: &mut Animator) {
        self.set_entity_state(EntityState::Dropped, animator);
        self.velocity_y = -self.jump_initial_velocity * 2f64;
        self.direction_at_jump_time = 0;
    }

    pub fn knock_back(&mut self, pos: &mut Vector2<f64>, amount: f64, dt: f64) {
        *pos += Vector2::new(amount * dt, 0.0);
        self.knock_back_distance = amount - (amount * 10.0 * dt);
    }

    pub fn state_update(&mut self, animator: &mut Animator, debug: bool) {

        if animator.is_finished && self.state != EntityState::Dead {

            if self.is_attacking {
                self.is_attacking = false;
                self.combo_counter = 0;
                if self.walking_dir.x != 0 || self.walking_dir.y != 0  {
                    self.set_entity_state(EntityState::Walking, animator);
                } else {
                    self.set_entity_state(EntityState::Idle, animator);
                }
            }

            if self.state == EntityState::Jump {
                self.set_entity_state(EntityState::Jumping, animator);
            }

            if self.state == EntityState::Landing {
                if self.walking_dir.x != 0 || self.walking_dir.y != 0 {
                    self.set_entity_state(EntityState::Walking, animator);
                } else {
                    self.set_entity_state(EntityState::Idle, animator);
                }
                
            }

            if self.state == EntityState::Hurt {
                if self.walking_dir.x != 0 || self.walking_dir.y != 0  {
                    self.set_entity_state(EntityState::Walking, animator);
                } else {
                    self.set_entity_state(EntityState::Idle, animator);
                }
            }

            if self.state == EntityState::Dashing {
                if self.walking_dir.x != 0 || self.walking_dir.y != 0  {
                    self.set_entity_state(EntityState::Walking, animator);
                } else {
                    self.set_entity_state(EntityState::Idle, animator);
                }
            }

            if self.state == EntityState::KnockedLanding || self.state == EntityState::DroppedLanding  {
                if self.walking_dir.x != 0 || self.walking_dir.y != 0  {
                    self.set_entity_state(EntityState::Walking, animator);
                } else {
                    self.set_entity_state(EntityState::Idle, animator);
                }
            }
        }

    }

    pub fn update(
        &mut self,
        position: &mut Vector2<f64>,
        character: &Character,
        animator: &mut Animator,
        camera: &Camera,
        dt: f64,
        character_width: i32,
        common_assets: &CommonAssets,
    ) {
        if self.state == EntityState::Jump {
            if !self.is_airborne {
                self.ground_height = position.y as i32;
            }
            self.velocity_y = if !self.is_double_jumping {self.jump_initial_velocity * 0.9f64} else {self.jump_initial_velocity/2f64};
            self.direction_at_jump_time = self.walking_dir.x.sign();
        }
   
        if self.state == EntityState::Jumping {
            if !self.is_airborne || (self.is_airborne && self.is_double_jumping && animator.is_starting) {
                audio_player::play_sound(common_assets.sound_effects.get("jump").unwrap());
            }
            self.is_airborne = true;
        }
    
        if self.knock_back_distance.abs() > 0.0 {
            *position += Vector2::new(self.knock_back_distance as f64 * dt, 0.0);
            self.knock_back_distance -= self.knock_back_distance * 10.0 * dt;
            if (self.knock_back_distance  * 100.0).round() / 100.0 <= 0.0 {
                self.knock_back_distance = 0.0;
            }
        }
     
        if self.is_airborne {
            let gravity = if self.state == EntityState::Knocked {
                -1.5 * self.jump_initial_velocity
            } else {
                -3.0 * self.jump_initial_velocity
            }; 

            let ground = self.ground_height;
            let should_land = position.y < ground as f64;
    
            let is_going_up = self.velocity_y > 0f64;

            if !should_land {
                let position_offset_x = self.direction_at_jump_time as f64
                    * character.jump_distance
                    * dt;
    
                if self.should_pause_gravity() {
                    self.velocity_y += gravity * dt;
                    let position_offset_y = self.velocity_y * dt + 0.5 * gravity * dt * dt; //pos += vel * delta_time + 1/2 gravity * delta time * delta time

                    *position += Vector2::new(position_offset_x, position_offset_y);
                } else {
                    self.velocity_y = 0.0;
                }
            }
            self.is_falling = self.velocity_y <= 0f64;

            if is_going_up && self.is_falling && self.is_double_jumping {
                //TOP OF JUMP
            }
    
            //reset position back to ground height
            let should_land = position.y < ground as f64;
            if should_land {
                position.y = self.ground_height as f64;
                if self.state == EntityState::Jumping {
                    self.set_entity_state(EntityState::Landing, animator);
                    self.is_attacking = false;
                    self.combo_counter = 0;
                    audio_player::play_sound(common_assets.sound_effects.get("land").unwrap());
                }
                if self.state == EntityState::Knocked {
                    self.set_entity_state(EntityState::KnockedLanding, animator);
                    audio_player::play_sound(common_assets.sound_effects.get("land").unwrap());
                }
                if self.state == EntityState::Dropped {
                    self.set_entity_state(EntityState::DroppedLanding, animator);
                    audio_player::play_sound(common_assets.sound_effects.get("dropped").unwrap());
                }
                self.is_double_jumping = false;
                self.is_airborne = false;
            }
        }
    
        if self.can_move() {
            if self.state == EntityState::Idle || self.state == EntityState::Walking {
                self.ground_height = position.y as i32;
                let position_move = Vector2::new(
                    self.walking_dir.x as f64, 
                    self.walking_dir.y as f64
                );
                let normalized_movement = if position_move.magnitude() > 0f64 { position_move.normalize() } else {position_move};
                *position += normalized_movement * character.speed * dt;
            }
        } else {
            match &animator.current_animation.as_ref().unwrap().offsets {
                Some(offsets) => {
                    let offset = offsets[animator.sprite_shown as usize];
                    if self.state == EntityState::Dashing && offset.x > 0f64 && animator.sprite_shown > 0 && offsets[animator.sprite_shown as usize - 1].x == 0f64  {
                        audio_player::play_sound(common_assets.sound_effects.get("miss").unwrap());
                    }
                    *position += Vector2::new( self.facing_dir as f64 * offset.x, offset.y) * dt
                }
                None => { }
            }
        }
    
        
        if (position.x  as i32 - character_width) < camera.rect.x() {
            position.x = (camera.rect.x() + character_width) as f64;
        }
    
        if (position.x as i32 + character_width) > (camera.rect.x() + camera.rect.width() as i32) {
            position.x = (camera.rect.x() + camera.rect.width() as i32 - character_width) as f64;
        }
        
    
    }

}
