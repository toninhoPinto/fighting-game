use sdl2::{pixels::Color, rect::Rect};

#[derive(Clone, Debug)]
pub struct Particle {
    pub active: bool,
    pub sprite: Rect,
    pub name: String,
    pub animation_index: i32,
    pub sprite_shown: i32,
    pub flipped: bool,
    pub tint: Option<Color>,
}
