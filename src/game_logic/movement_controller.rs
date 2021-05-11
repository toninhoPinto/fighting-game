use parry2d::na::Vector2;

use crate::{ecs_system::enemy_components::Position, engine_types::animator::Animator, rendering::camera::Camera};

use super::{characters::{Character, player::PlayerState}, factories::enemy_factory::EnemyAnimations};

pub struct MovementController {
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

    pub mid_jump_pos: f64,
}

impl MovementController {

    pub fn new(character: &Character, starting_pos: Vector2<f64>, player_pos: Vector2<f64>) -> Self {
        Self {
            walking_dir: Vector2::new(0,0),
            ground_height: starting_pos.y as i32,
            velocity_y: 0f64,
        
            direction_at_jump_time: 0,
            jump_initial_velocity: 2.0 * character.jump_height,
            extra_gravity: None,
        
            facing_dir: (player_pos.x - starting_pos.x).signum() as i8,
            state: PlayerState::Standing,
            is_attacking: false,
            is_blocking: false,
            is_airborne: false,
            is_pushing: false,
            knock_back_distance: 0f64,
        
            mid_jump_pos: 0f64,
        }
    }

    pub fn set_velocity_x(&mut self, vec_x: i8) {
        if vec_x != 0 {
            self.facing_dir = vec_x;
        }
        self.walking_dir.x = vec_x;
    }

    pub fn state_update(&mut self, animator: &mut Animator, position: &mut Position, assets: &EnemyAnimations) {
        let character_animation = &assets.animations;

        if animator.is_finished && self.state != PlayerState::Dead {
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
            position.0.y = self.ground_height as f64;
            self.is_attacking = false;
        }

        if !self.is_attacking {
            match self.state {
                PlayerState::Standing => {
                    if self.walking_dir.x != 0 || self.walking_dir.y != 0 {
                        animator
                            .play(character_animation.get("walk").unwrap().clone(), 1.0, false);
                    } else {
                        if animator.current_animation.as_ref().unwrap().name != "idle" {
                            animator
                            .play(character_animation.get("idle").unwrap().clone(), 1.0, false);
                        }
                        
                    }
                }

                PlayerState::Dead => {
                    animator
                        .play_once(character_animation.get("dead").unwrap().clone(), 1.0, false);
                }

                PlayerState::Jump => {
                    animator
                        .play_once(character_animation.get("crouch").unwrap().clone(), 3.0, false);
                }

                PlayerState::Jumping => {
                    animator
                        .play_once(character_animation.get("neutral_jump").unwrap().clone(), 1.0, false);
                }

                PlayerState::Landing => {
                    animator
                        .play_once(character_animation.get("crouch").unwrap().clone(), 3.0, true);
                }

                PlayerState::Dashing => {
                    animator
                        .play_once(character_animation.get("dash").unwrap().clone(), 1.0, false);
                }
                PlayerState::Hurt => {
                    animator
                        .play_once(character_animation.get("take_damage").unwrap().clone(), 1.0, false);
                }
            }
        }
    }


    pub fn update(
        &mut self,
        position: &mut Vector2<f64>,
        character: &Character,
        animator: &Animator,
        camera: &Camera,
        dt: f64,
        character_width: i32,
    ) {
        if self.state == PlayerState::Jump {
            self.ground_height = position.y as i32;
            self.velocity_y = self.jump_initial_velocity / 0.5;
            self.direction_at_jump_time = self.walking_dir.x;
        }
    
        if self.state == PlayerState::Jumping {
            self.is_airborne = true;
        }
    
        //TODO I KINDA HATE THIS
        if self.knock_back_distance.abs() > 0.0 {
            *position += Vector2::new(self.knock_back_distance as f64 * dt, 0.0);
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
            let should_land = position.y < ground as f64;
    
            if !should_land {
                let position_offset_x = self.direction_at_jump_time as f64
                    * character.jump_distance
                    * dt
                    * speed_mod;
    
                self.velocity_y += gravity * dt;
                let position_offset_y = self.velocity_y * dt + 0.5 * gravity * dt * dt; //pos += vel * delta_time + 1/2 gravity * delta time * delta time
                *position += Vector2::new(position_offset_x, position_offset_y);
            }
    
            //reset position back to ground height
            let should_land = position.y < ground as f64;
            if should_land {
                position.y = self.ground_height as f64;
                self.velocity_y = character.jump_height;
                if self.state == PlayerState::Jumping {
                    self.state = PlayerState::Landing;
                    self.is_attacking = false;
                }
                self.is_airborne = false;
            }
        }
    
        if self.can_move() {
            if self.state == PlayerState::Standing {
                self.ground_height = position.y as i32;
                let position_move = Vector2::new(
                    self.walking_dir.x as f64, 
                    self.walking_dir.y as f64
                );
                let normalized_movement = if position_move.magnitude() > 0f64 { position_move.normalize() } else {position_move};
                *position += normalized_movement * character.speed * dt * speed_mod;
            }
        } else {
            match &animator.current_animation.as_ref().unwrap().offsets {
                Some(offsets) => {
                    let offset = offsets[animator.sprite_shown as usize];
                    self.walking_dir.x = (self.facing_dir as f64 * offset.x).signum() as i8;
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


    pub fn can_move(&self) -> bool {
        !(self.is_attacking
            || self.is_airborne
            || self.knock_back_distance.abs() > 0.0
            || self.state == PlayerState::Dead
            || self.state == PlayerState::Dashing)
    }


}
