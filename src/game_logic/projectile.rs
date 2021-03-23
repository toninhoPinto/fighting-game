use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture};

pub struct Projectile<'a>{
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub damage: i32,
    pub animation_index: f32,
    pub current_animation: Vec<Texture<'a>>
}