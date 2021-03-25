use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use std::fmt;
use super::game_input::GameInputs;
use super::character_factory::CharacterAnimationData;

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
pub struct Player{
    pub id: i32,
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub dash_speed: i32,
    pub dash_back_speed: i32,
    pub prev_direction: i32,
    pub direction: i32,
    pub hp: i32,
    pub dir_related_of_other: i32,
    pub state: PlayerState,
    pub is_attacking: bool,
    hit_stunned_duration: i32,
    pub animation_index: f32,
    pub current_animation: String,
    pub flipped: bool,
    pub last_directional_input_v: Option<GameInputs>,
    pub last_directional_input_h: Option<GameInputs>,
    pub last_directional_input: Option<GameInputs>
}

impl Player {

    pub fn new(id: i32, spawn_position: Point, flipped: bool) -> Player {
       Player {
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
            is_attacking: false,
            hit_stunned_duration: 0,
            animation_index: 0.0,
            current_animation: "idle".to_string(),
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

    pub fn update(&mut self, opponent: &Player) {

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

        self.dir_related_of_other = (opponent.position.x - self.position.x).signum();
    }

    pub fn render<'a>(&mut self, animations: &'a CharacterAnimationData<'a>) -> &Texture<'a> {
        //TODO 0.35 needs to change per current animation
        let mut curr_anim = animations.animations.get(&self.current_animation).unwrap();

        //TODO: trigger finished animation, instead make a function that can play an animation once and run callback at the end
        if (self.animation_index as f32 + 0.35 as f32) as usize >= curr_anim.len() {

            if self.is_attacking {
                self.is_attacking = false;
            }

            if self.state == PlayerState::DashingForward || self.state == PlayerState::DashingBackward {
                self.state = PlayerState::Standing;
                self.hit_stunned_duration = 5;
            }

            self.animation_index = 0.0;
        }

        if self.hit_stunned_duration > 0 {
            self.hit_stunned_duration -= 1;
        }

        if !self.is_attacking {

            if self.state == PlayerState::Standing {
                //TODO flip has a small animation i believe, also, have to take into account mixups
                //TODO needs to switch the FWD to BCK and vice versa when flipping

                //flipped true looks to the right, false looks to the left
                self.flipped = self.dir_related_of_other > 0;

                if self.direction * -self.dir_related_of_other < 0 {
                    self.current_animation = "walk".to_string();
                } else if self.direction * -self.dir_related_of_other > 0 {
                    self.current_animation = "walk_back".to_string();
                } else {
                    self.current_animation = "idle".to_string();
                }
            } else if self.state == PlayerState::Crouching {
                self.current_animation = "crouching".to_string();
            } else if self.state == PlayerState::DashingForward {
                self.current_animation = "dash".to_string();
            } else if self.state == PlayerState::DashingBackward {
                self.current_animation = "dash_back".to_string();
            }

            if self.state != PlayerState::DashingForward &&
                self.state != PlayerState::DashingBackward {
                if self.prev_direction != self.direction {
                    self.animation_index = 0.0;
                }
            }

            self.prev_direction = self.direction;
        }
        self.animation_index = (self.animation_index + 0.35) % curr_anim.len() as f32;

        curr_anim = animations.animations.get(&self.current_animation).unwrap();
        &curr_anim[self.animation_index as usize]
    }

}