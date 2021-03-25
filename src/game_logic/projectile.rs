use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture};
use std::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct Projectile{
    pub position: Point,
    pub sprite: Rect,
    pub direction: Point,
    pub target_position: Option<Point>,
    pub speed: i32,
    pub damage: i32,
    pub flipped: bool,
    pub animation_index: f32,
    pub animation_name: String,
    pub player_owner: i32
}

impl Projectile {
    pub fn update(&mut self) {
        match self.target_position {
            Some(target) => {
                if self.position.x <= self.target_position.unwrap().x &&
                    self.position.y <= self.target_position.unwrap().y
                {
                    self.position = self.position.offset(self.speed, 0);
                }
            }
            None => { self.position = self.position.offset(self.speed, 0); }
        }
    }

    fn render(&self) {
    }
}