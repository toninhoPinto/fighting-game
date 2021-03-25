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

}