use sdl2::{pixels::Color, rect::Rect};

#[derive(Clone, Debug)]
pub struct Particle {
    pub active :bool,
    pub sprite: Rect,
    pub name: String,
    pub animation_index: i32,
    pub tint: Option<Color>,
}