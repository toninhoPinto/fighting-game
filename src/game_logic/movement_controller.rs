use parry2d::na::Vector2;

use crate::{asset_management::asset_holders::EntityAnimations, ecs_system::enemy_components::Position, engine_types::animator::Animator, rendering::camera::Camera};

use super::characters::{Attack, Character, player::EntityState};

use crate::utils::math_sign::Sign;

#[derive(Clone)]
pub struct MovementController {
    pub walking_dir: Vector2<i8>,
    pub ground_height: i32,
    pub velocity_y: f64,

    pub direction_at_jump_time: i8,
    pub jump_initial_velocity: f64,
    pub extra_gravity: Option<f64>,

    pub facing_dir: i8,
    pub state: EntityState,
    pub is_attacking: bool,
    pub is_blocking: bool,
    pub is_airborne: bool,
    pub combo_counter: i32,
    pub has_hit: bool,
    pub knock_back_distance: f64,

    pub mid_jump_pos: f64,
}

impl MovementController {

    pub fn new(character: &Character, starting_pos: Vector2<f64>, player_pos: Vector2<f64>) -> Self {
        Self {
            walking_dir: Vector2::new(0,0),
            ground_height: starting_pos.y as i32,
            velocity_y: 0f64,
        
            direction_at_jump_time: 0,
            jump_initial_velocity: 4.0 * character.jump_height,
            extra_gravity: None,
        
            facing_dir: (player_pos.x - starting_pos.x).sign() as i8,
            state: EntityState::Idle,
            is_attacking: false,
            is_blocking: false,
            is_airborne: false,
            combo_counter: 0,
            has_hit: false,
            knock_back_distance: 0f64,
        
            mid_jump_pos: 0f64,
        }
    }

    pub fn set_velocity(&mut self, vec: Vector2<i8>, animator: &mut Animator, assets: &EntityAnimations) {
        if vec.x != 0 || vec.y != 0 {
            if vec.x != 0 {
                self.facing_dir = vec.x;
            }
            self.set_entity_state(EntityState::Walking, animator, assets);
        } else {
            self.set_entity_state(EntityState::Idle, animator, assets);
        }
        self.walking_dir = vec;
    }

    pub fn set_velocity_x(&mut self, x: i8, animator: &mut Animator, assets: &EntityAnimations) {
        if x != 0 {
            if x != 0 {
                self.facing_dir = x;
            }
            self.set_entity_state(EntityState::Walking, animator, assets);
        } else {
            self.set_entity_state(EntityState::Idle, animator, assets);
        }
        self.walking_dir.x = x;
    }

    pub fn player_can_attack(&self) -> bool {
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

    fn update_state(&mut self, new_state: EntityState, animator: &mut Animator, assets: &EntityAnimations) {
        self.state = new_state;
        let character_animation = &assets.animations;
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
                animator
                    .play_once(character_animation.get("dash").unwrap().clone(), 1.0, false);
            }
            EntityState::Hurt => {
                animator
                    .play_once(character_animation.get("take_damage").unwrap().clone(), 1.0, false);
            }
            EntityState::Knocked => {
                if self.is_airborne {
                    animator
                    .play_once(character_animation.get("launched").unwrap().clone(), 1.0, false);
                }
            }
            EntityState::KnockedLanding => {
                let animation = character_animation.get("knock_land");
                if let Some(animation) = animation {
                    animator.play_once(animation.clone(), 1.0, false);
                }
            }
        }
    }

    pub fn set_entity_state(&mut self, new_state: EntityState, animator: &mut Animator, assets: &EntityAnimations) {
        
        let can_idle = 
            self.state == EntityState::Idle ||
            self.state == EntityState::Walking || 
            self.state == EntityState::Landing ||
            (self.state == EntityState::Dashing && animator.is_finished) || 
            self.state == EntityState::Hurt ||
            self.state == EntityState::KnockedLanding;

        let can_walk = 
            self.state == EntityState::Idle ||
            self.state == EntityState::Walking ||
            (self.state == EntityState::Landing && animator.is_finished) ||
            (self.state == EntityState::Dashing && animator.is_finished) || 
            (self.state == EntityState::Hurt && animator.is_finished) ||
            (self.state == EntityState::KnockedLanding && animator.is_finished);

        let can_jump = 
            (self.is_attacking && self.has_hit) ||
            self.state == EntityState::Idle ||
            self.state == EntityState::Walking;

        let interrupt_attack = new_state == EntityState::Landing || 
            new_state == EntityState::Hurt;

        let cancel_attack = (self.is_attacking && self.has_hit) && (
            new_state == EntityState::Jump ||
            new_state == EntityState::Dashing 
            );
        
        let can_dash = !self.is_airborne && (
            self.state == EntityState::Idle ||
        self.state == EntityState::Walking ||
        (self.state == EntityState::Landing && animator.is_finished) ||
        (self.state == EntityState::Dashing && animator.is_finished) || 
        (self.state == EntityState::Hurt && animator.is_finished) ||
        (self.state == EntityState::KnockedLanding && animator.is_finished)
        );
        
        
        if (!self.is_attacking || (self.is_attacking && interrupt_attack) || cancel_attack )  && self.state != EntityState::Dead{

            match new_state {
                EntityState::Idle => {
                    if can_idle {
                        self.update_state(new_state, animator, assets);
                    }
                },
                EntityState::Walking => {
                    if can_walk {
                        self.update_state(new_state, animator, assets);
                    }
                },
                EntityState::Jump => {
                    if can_jump {
                        self.update_state(new_state, animator, assets);
                    }
                },
                EntityState::Dashing => {
                    if can_dash {
                        self.update_state(new_state, animator, assets);
                    }
                },
                _ => {
                    self.update_state(new_state, animator, assets);
                }
            }
        }
    }

    pub fn jump(&mut self, animator: &mut Animator, assets: &EntityAnimations) {
        if !self.is_airborne {
            self.set_entity_state(EntityState::Jump, animator, assets);
        }
    }

    pub fn launch(&mut self, attack: &Attack, animator: &mut Animator, assets: &EntityAnimations) {
        self.is_airborne = true;
        self.set_entity_state(EntityState::Knocked, animator, assets);
        self.velocity_y = self.jump_initial_velocity;
        self.direction_at_jump_time = 0;
    }

    pub fn knock_back(&mut self, pos: &mut Position, amount: f64, dt: f64) {
        pos.0 += Vector2::new(amount * dt, 0.0);
        self.knock_back_distance = amount - (amount * 10.0 * dt);
    }

    pub fn state_update(&mut self, animator: &mut Animator, assets: &EntityAnimations, debug: bool) {

        if animator.is_finished && self.state != EntityState::Dead {

            if self.is_attacking {
                self.is_attacking = false;
                self.combo_counter = 0;
                if self.walking_dir.x != 0 || self.walking_dir.y != 0  {
                    self.set_entity_state(EntityState::Walking, animator, assets);
                } else {
                    self.set_entity_state(EntityState::Idle, animator, assets);
                }
            }

            if self.state == EntityState::Jump {
                self.set_entity_state(EntityState::Jumping, animator, assets);
            }

            if self.state == EntityState::Landing {
                if self.walking_dir.x != 0 || self.walking_dir.y != 0 {
                    self.set_entity_state(EntityState::Walking, animator, assets);
                } else {
                    self.set_entity_state(EntityState::Idle, animator, assets);
                }
                
            }

            if self.state == EntityState::Hurt {
                if self.walking_dir.x != 0 || self.walking_dir.y != 0  {
                    self.set_entity_state(EntityState::Walking, animator, assets);
                } else {
                    self.set_entity_state(EntityState::Idle, animator, assets);
                }
            }

            if self.state == EntityState::Dashing {
                if self.walking_dir.x != 0 || self.walking_dir.y != 0  {
                    self.set_entity_state(EntityState::Walking, animator, assets);
                } else {
                    self.set_entity_state(EntityState::Idle, animator, assets);
                }
            }

            if self.state == EntityState::KnockedLanding {
                if self.walking_dir.x != 0 || self.walking_dir.y != 0  {
                    self.set_entity_state(EntityState::Walking, animator, assets);
                } else {
                    self.set_entity_state(EntityState::Idle, animator, assets);
                }
            }
        }

    }


    pub fn update(
        &mut self,
        position: &mut Vector2<f64>,
        character: &Character,
        animator: &mut Animator,
        anims: &EntityAnimations,
        camera: &Camera,
        dt: f64,
        character_width: i32,
    ) {
        if self.state == EntityState::Jump {
            self.ground_height = position.y as i32;
            self.velocity_y = self.jump_initial_velocity;
            self.direction_at_jump_time = self.walking_dir.x.sign();
        }
   
        if self.state == EntityState::Jumping {
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
            let gravity = match self.extra_gravity {
                Some(extra_g) => extra_g,
                None => { 
                    if self.state == EntityState::Knocked {
                        -4.0 *self.jump_initial_velocity
                    } else {
                        -4.0 * self.jump_initial_velocity
                    } 
                } 
            };
    
            let ground = self.ground_height;
            let should_land = position.y < ground as f64;
    
            if !should_land {
                let position_offset_x = self.direction_at_jump_time as f64
                    * character.jump_distance
                    * dt;
    
                self.velocity_y += gravity * dt;
                let position_offset_y = self.velocity_y * dt + 0.5 * gravity * dt * dt; //pos += vel * delta_time + 1/2 gravity * delta time * delta time
                *position += Vector2::new(position_offset_x, position_offset_y);
            }
    
            //reset position back to ground height
            let should_land = position.y < ground as f64;
            if should_land {
                position.y = self.ground_height as f64;
                if self.state == EntityState::Jumping {
                    self.set_entity_state(EntityState::Landing, animator, anims);
                    self.is_attacking = false;
                    self.combo_counter = 0;
                }
                if self.state == EntityState::Knocked {
                    self.set_entity_state(EntityState::KnockedLanding, animator, anims);
                }
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
                    *position += Vector2::new( self.facing_dir as f64 * offset.x, offset.y) * dt
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

}
