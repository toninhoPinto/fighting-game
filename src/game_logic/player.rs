use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use std::fmt;
use super::game_input::GameInputs;
use super::character_factory::CharacterAssets;

use crate::asset_management::animation::AnimationPlayer;

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
    KnockedOut,
    Dead
}
impl fmt::Display for PlayerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//TODO might have redundant data
pub struct Player<'a>{
    pub id: i32,
    pub position: Point,
    pub sprite: Rect,
    pub prev_direction: i32,
    pub direction: i32,
    pub dir_related_of_other: i32,
    pub state: PlayerState,
    pub is_attacking: bool,

    hit_stunned_duration: i32,

    pub animation_manager: AnimationPlayer<'a>,
    pub flipped: bool,
    pub last_directional_input_v: Option<GameInputs>,
    pub last_directional_input_h: Option<GameInputs>,
    pub last_directional_input: Option<GameInputs>,

    pub hp: i32,
    pub speed: i32,
    pub dash_speed: i32,
    pub dash_back_speed: i32,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, spawn_position: Point, flipped: bool) -> Self {
        Self {
            id,
            position: spawn_position,
            sprite: Rect::new(0, 0, 580, 356),
            speed: 5,
            dash_speed: 10,
            dash_back_speed: 7,
            prev_direction: 0,
            direction: 0,
            hp: 100,
            dir_related_of_other: 0,
            state: PlayerState::Standing,
            animation_manager: AnimationPlayer::new(),
            is_attacking: false,
            hit_stunned_duration: 0,
            flipped,
            last_directional_input: None,
            last_directional_input_v: None,
            last_directional_input_h: None
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp <= 0 {
            self.state = PlayerState::Dead;
        }
    }

    pub fn update(&mut self, opponent_position_x: i32) {

        if !self.is_attacking && self.hit_stunned_duration <= 0 {
            if self.state == PlayerState::Standing {
                self.position = self.position.offset(self.direction * self.speed, 0);
            }

            let is_dashing = self.state == PlayerState::DashingForward || self.state == PlayerState::DashingBackward;
            if  is_dashing {
                if self.state == PlayerState::DashingForward {
                    self.position = self.position.offset(self.dir_related_of_other.signum() * self.dash_speed, 0);
                } else {
                    self.position = self.position.offset(-self.dir_related_of_other.signum() * self.dash_speed, 0);
                }
            }
        }

        self.dir_related_of_other = (opponent_position_x - self.position.x).signum();
    }

    pub fn render(&mut self, character_data: &'a CharacterAssets) -> &Texture {
        let curr_anim = self.animation_manager.current_animation.unwrap();
        let character_animation = &character_data.animations;

        //TODO: trigger finished animation, instead make a function that can play an animation once and run callback at the end
        if !self.animation_manager.is_playing {

            if self.state == PlayerState::Jump {
                self.state == PlayerState::Jumping;
            }

            if self.state == PlayerState::Crouch {
                self.state = PlayerState::Crouching;
            }

            if self.state == PlayerState::UnCrouch {
                self.state = PlayerState::Standing;
            }

            if self.is_attacking {
                self.is_attacking = false;
            }

            if self.state == PlayerState::DashingForward || self.state == PlayerState::DashingBackward {
                self.state = PlayerState::Standing;
                self.hit_stunned_duration = 5;
            }
            self.animation_manager.animation_index = 0.0;
        }

        if self.hit_stunned_duration > 0 {
            self.hit_stunned_duration -= 1;
        }


        if !self.is_attacking {

            match self.state {
                PlayerState::Standing => {
                    self.flipped = self.dir_related_of_other > 0;
                    if self.direction * -self.dir_related_of_other < 0 {
                        self.animation_manager.play(character_animation.get("walk").unwrap());
                    } else if self.direction * -self.dir_related_of_other > 0 {
                        self.animation_manager.play(character_animation.get("walk_back").unwrap());
                    } else {
                        self.animation_manager.play(character_animation.get("idle").unwrap());
                    }
                }

                PlayerState::Dead => {

                }

                PlayerState::KnockedOut => {

                }

                PlayerState::Jump => {
                    //self.state = PlayerState::Jumping;
                }

                PlayerState::Jumping => {
                    self.animation_manager.play_once(character_animation.get("neutral_jump").unwrap(), false);
                }

                PlayerState::Landing => {
                    self.animation_manager.play_once(character_animation.get("crouch").unwrap(), false);
                }

                PlayerState::UnCrouch => {
                    self.animation_manager.play_once(character_animation.get("crouch").unwrap(), true);
                }

                PlayerState::Crouch => {
                    self.animation_manager.play_once(character_animation.get("crouch").unwrap(), false);
                }

                PlayerState::Crouching => {
                    self.animation_manager.play(character_animation.get("crouching").unwrap());
                }

                PlayerState::DashingForward => {
                    self.animation_manager.play_once(character_animation.get("dash").unwrap(), false);
                }

                PlayerState::DashingBackward => {
                    self.animation_manager.play_once(character_animation.get("dash_back").unwrap(), false);
                }
            }

            self.prev_direction = self.direction;
        }

        if self.id == 1 {
            self.animation_manager.render(true)
        } else {
            self.animation_manager.render(false)
        }

    }

}