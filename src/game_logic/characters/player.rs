use parry2d::na::Vector2;
use sdl2::rect::Point;
use sdl2::render::Texture;

use std::{collections::HashMap, fmt};

use crate::{asset_management::animation::Animation, game_logic::characters::Character};
use crate::{
    asset_management::animation::AnimationState, game_logic::character_factory::CharacterAssets,
    rendering::camera::Camera,
};

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
    pub is_airborne: bool,
    pub is_pushing: bool,
    pub knock_back_distance: i32,

    pub animator: Animator<'a>,
    pub animation_state: Option<AnimationState>,
    pub flipped: bool,
    pub has_hit: bool,
    pub character: Character,

    pub mid_jump_pos: f64,
}

impl<'a> Player<'a> {
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
            has_hit: false,
            is_pushing: false,
            knock_back_distance: 0,
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
        !(self.is_attacking
            || self.is_airborne
            || self.knock_back_distance > 0
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

    pub fn knock_back(&mut self, amount: i32) {
        self.knock_back_distance = amount;
    }

    pub fn attack(&mut self, character_anims: &'a CharacterAssets, attack_animation: String) {
        println!("ATTACK {}", attack_animation);
        if self.player_can_attack() {
            self.is_attacking = true;
            if let Some(attack) = character_anims.animations.get(&attack_animation) {
                self.animator.play_once(attack, 1.0, false);
            };
        }
    }

    pub fn player_state_cancel(&mut self, _new_state: PlayerState) {
        self.state = PlayerState::Standing;
    }

    pub fn push(&mut self, dir: i32, player_pushing: &Player, player_width: f32, dt: f64) {
        let speed = if player_pushing.state == PlayerState::DashingForward {
            player_pushing.character.dash_speed / 2.0
        } else if player_pushing.is_airborne {
            let offset = if (player_pushing.position.x - self.position.x).abs() < 10.0 {
                player_width as f64
            } else {
                player_width as f64 - (player_pushing.position.x - self.position.x).abs() as f64
            };
            offset * 20.0
        } else {
            player_pushing.character.speed / 2.0
        };
        self.position += Vector2::new(dir as f64 * speed * dt, 0.0);
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

        //TODO im just moving by an int instead of multiplying by dt, not sure if this is bad
        if self.knock_back_distance != 0 {
            self.position += Vector2::new(self.knock_back_distance as f64, 0.0);
            self.knock_back_distance = 0;
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
                println!("LANDED");
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
            match &self.animator.current_animation.unwrap().offsets {
                Some(offsets) => {
                    let offset = offsets[self.animator.sprite_shown as usize];
                    self.position += Vector2::new( self.dir_related_of_other as f64 * offset.x * dt, offset.y * dt)
                }
                None => { }
            }
            
        }

        //TODO float wiht != seems dangerous
        if opponent_position_x - self.position.x != 0.0 {
            self.dir_related_of_other = ((opponent_position_x - self.position.x) as i32).signum() ;
        }

        if (self.position.x  as i32 - character_width) < camera.rect.x() {
            self.position.x = (camera.rect.x() + character_width) as f64;
        }

        if (self.position.x as i32 + character_width) > (camera.rect.x() + camera.rect.width() as i32) {
            self.position.x = (camera.rect.x() + camera.rect.width() as i32 - character_width) as f64;
        }
    }

    fn walk_anims(&mut self, character_animation: &'a HashMap<String, Animation>) {

        let walk_forward = self.velocity_x * -self.dir_related_of_other < 0;
        let changed_dir = self.prev_velocity_x != self.velocity_x;

        if walk_forward {
            self.animator
            .play_animation(character_animation.get("walk").unwrap(), 1.0, false, false, changed_dir);
        } else {
            
            if character_animation.contains_key("walk_back") {
                self.animator
                    .play_animation(character_animation.get("walk_back").unwrap(), 1.0, false, false, changed_dir);
            } else {
                self.animator
                    .play_animation(character_animation.get("walk").unwrap(), 1.0, true, false, changed_dir);
            }
        }
    }

    pub fn state_update(&mut self, character_data: &'a CharacterAssets) {
        let character_animation = &character_data.animations;

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
                        if self.animator.current_animation.unwrap().name != "idle" {
                            self.animator
                            .play(character_animation.get("idle").unwrap(), 1.0, false);
                        }
                        
                    }
                }

                PlayerState::Dead => {
                    self.animator
                        .play_once(character_animation.get("dead").unwrap(), 1.0, false);
                }

                PlayerState::Jump => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap(), 3.0, true);
                }

                PlayerState::Jumping => {
                    self.animator
                        .play_once(character_animation.get("neutral_jump").unwrap(), 1.0, false);
                }

                PlayerState::Landing => {
                    self.flipped = self.dir_related_of_other > 0;
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap(), 3.0, false);
                }

                PlayerState::UnCrouch => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap(), 1.0, true);
                }

                PlayerState::Crouch => {
                    self.animator
                        .play_once(character_animation.get("crouch").unwrap(), 1.0, false);
                }

                PlayerState::Crouching => {
                    self.animator
                        .play(character_animation.get("crouching").unwrap(), 1.0, false);
                }

                PlayerState::DashingForward => {
                    self.animator
                        .play_once(character_animation.get("dash").unwrap(), 1.0, false);
                }

                PlayerState::DashingBackward => {
                    self.animator
                        .play_once(character_animation.get("dash_back").unwrap(), 1.0, false);
                }
                PlayerState::Grab => {
                    self.animator
                        .play_once(character_animation.get("grab").unwrap(), 1.0, false);
                }
                PlayerState::Grabbed => {}
                PlayerState::Hurt => {
                    self.animator
                        .play_once(character_animation.get("take_damage").unwrap(), 1.0, false);
                }
            }
        }

        self.prev_velocity_x = self.velocity_x;
        self.animator.update();
    }

    pub fn render(&mut self) -> &Texture {
        self.animator.render()
    }
}
