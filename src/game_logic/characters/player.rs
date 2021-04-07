use sdl2::rect::Point;
use sdl2::render::Texture;

use std::fmt;

use crate::game_logic::character_factory::CharacterAssets;
use crate::game_logic::characters::Character;

use crate::asset_management::animation::Animator;

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
    KnockedOut,
    Hurt,
    Dead,
}
impl fmt::Display for PlayerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub struct Player<'a> {
    pub id: i32,
    pub position: Point,
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
    pub is_airborne: bool,

    pub animator: Animator<'a>,
    pub flipped: bool,
    pub has_hit: bool,

    pub character: Character,

    pub mid_jump_pos: f64,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, character: Character, spawn_position: Point, flipped: bool) -> Self {
        Self {
            id,
            position: spawn_position,
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
            is_attacking: false,
            is_airborne: false,
            has_hit: false,
            flipped,
            character,
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
        !(self.is_attacking || self.is_airborne || self.state == PlayerState::Dead)
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

    pub fn player_state_cancel(&mut self, _new_state: PlayerState) {
        self.state = PlayerState::Standing;
    }

    pub fn update(&mut self, dt: f64, opponent_position_x: i32) {
        if self.state == PlayerState::Jump {
            self.velocity_y = self.jump_initial_velocity / 0.5;
            self.direction_at_jump_time = self.velocity_x;
        }

        if self.state == PlayerState::Jumping {
            self.is_airborne = true;
        }

        if self.is_airborne {
            let gravity = match self.extra_gravity {
                Some(extra_g) => extra_g,
                None => -2.0 * self.jump_initial_velocity / 0.25,
            };

            if self.position.y >= self.ground_height {
                let position_offset_x =
                    self.direction_at_jump_time as f64 * self.character.jump_distance * dt;
                self.velocity_y += gravity * dt;
                let position_offset_y = self.velocity_y * dt + 0.5 * gravity * dt * dt; //pos += vel * delta_time + 1/2 gravity * delta time * delta time
                self.position = self
                    .position
                    .offset(position_offset_x as i32, position_offset_y as i32);
            }

            //reset position back to ground height
            if self.position.y < self.ground_height {
                self.position.y = 0;
                self.velocity_y = self.character.jump_height;
                if self.state == PlayerState::Jumping {
                    self.state = PlayerState::Landing;
                }
                self.is_airborne = false;
            }
        }

        if self.player_can_move() && self.character.hit_stunned_duration <= 0 {
            if self.state == PlayerState::Standing {
                self.position = self.position.offset(
                    (self.velocity_x as f64 * self.character.speed * dt) as i32,
                    0,
                );
            }

            let is_dashing = self.state == PlayerState::DashingForward
                || self.state == PlayerState::DashingBackward;
            if is_dashing {
                let dash_speed = (self.dir_related_of_other.signum() as f64
                    * self.character.dash_speed as f64
                    * dt) as i32;
                if self.state == PlayerState::DashingForward {
                    self.position = self.position.offset(dash_speed, 0);
                } else {
                    self.position = self.position.offset(-dash_speed, 0);
                }
            }
        }

        if opponent_position_x - self.position.x != 0 {
            self.dir_related_of_other = (opponent_position_x - self.position.x).signum();
        }
    
        if self.character.hit_stunned_duration > 0 {
            self.character.hit_stunned_duration -= 1;
        }
    }

    pub fn state_update(&mut self, character_data: &'a CharacterAssets) {
        let character_animation = &character_data.animations;

        if self.animator.is_finished && self.state != PlayerState::Dead {
            self.has_hit = false;

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

            if self.is_attacking {
                self.is_attacking = false;
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
                self.character.hit_stunned_duration = 5;
            }
        }
        
        if !self.is_attacking {
            match self.state {
                PlayerState::Standing => {
                    self.flipped = self.dir_related_of_other > 0;
                    
                    if self.velocity_x * -self.dir_related_of_other < 0 {
                        self.animator
                            .play(character_animation.get("walk").unwrap(), false);
                    } else if self.velocity_x * -self.dir_related_of_other > 0 {
                        if character_animation.contains_key("walk_back") {
                            self.animator
                                .play(character_animation.get("walk_back").unwrap(), false);
                        } else {
                            self.animator
                                .play(character_animation.get("walk").unwrap(), true);
                        }
                    } else {
                        self.animator
                            .play(character_animation.get("idle").unwrap(), false);
                    }
                }

                PlayerState::Dead => {
                    self.animator
                        .play_once(character_animation.get("dead").unwrap(), false);
                }

                PlayerState::KnockedOut => {}

                PlayerState::Jump => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap(), true);
                }

                PlayerState::Jumping => {
                    self.animator
                        .play_once(character_animation.get("neutral_jump").unwrap(), false);
                }

                PlayerState::Landing => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap(), false);
                }

                PlayerState::UnCrouch => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap(), true);
                }

                PlayerState::Crouch => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap(), false);
                }

                PlayerState::Crouching => {
                    self.animator
                        .play(character_animation.get("crouching").unwrap(), false);
                }

                PlayerState::DashingForward => {
                    self.animator
                        .play_once(character_animation.get("dash").unwrap(), false);
                }

                PlayerState::DashingBackward => {
                    self.animator
                        .play_once(character_animation.get("dash_back").unwrap(), false);
                }
                PlayerState::Grab => {
                    self.animator
                        .play_once(character_animation.get("grab").unwrap(), false);
                }
                PlayerState::Grabbed => {}
                PlayerState::Hurt => { 
                    self.animator
                        .play_once(character_animation.get("take_damage").unwrap(), false);
                }
            }

            self.prev_velocity_x = self.velocity_x;
        }
        self.animator.update();
    }

    pub fn render(&mut self) -> &Texture {
        self.animator.render()
    }
}
