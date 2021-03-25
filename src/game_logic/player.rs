use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use std::fmt;
use super::game_input::GameInputs;

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
    pub isAttacking: bool,
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
            isAttacking: false,
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

    pub fn update(&mut self) {

        if !self.isAttacking {
            if self.state == PlayerState::Standing {
                self.position = self.position.offset(self.direction * self.speed, 0);
            }

            let isDashing = self.state == PlayerState::DashingForward || self.state == PlayerState::DashingBackward;
            if  isDashing {
                if self.state == PlayerState::DashingForward {
                    self.position = self.position.offset(self.dir_related_of_other.signum() * self.dash_speed, 0);
                } else {
                    self.position = self.position.offset(-self.dir_related_of_other.signum() * self.dash_speed, 0);
                }
            }

        }
    }

    fn render(&mut self) {
        //self.animation_index = (self.animation_index + anim_speed) % p1_curr_anim.len() as f32;
    }
}