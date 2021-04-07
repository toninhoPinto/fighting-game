use sdl2::rect::Rect;
use std::string::String;

pub mod foxgirl;
pub mod keetar;
pub mod player;

#[derive(Debug, Clone)]
pub struct Character {
    //visual
    pub sprite: Rect,

    //stats
    pub name: String,
    pub hp: i32,
    pub special_max: i32,
    pub special_curr: f32,
    pub speed: f64,
    pub dash_speed: f64,
    pub dash_back_speed: f64,
    hit_stunned_duration: i32,
    // hit_stunned_duration was intended to give a small break on the dash animation
    // but actually ryu dash has 6 sprites that run over 26 frames and not spread equally, 11 of which have movement
    pub jump_height: f64,
    pub jump_distance: f64,
}

impl Character {
    pub fn new(
        name: String,
        height: u32,
        width: u32,
        hp: i32,
        special_max: i32,
        speed: f64,
        dash_speed: f64,
        dash_back_speed: f64,
        jump_height: f64,
        jump_distance: f64,
    ) -> Self {
        Self {
            name,
            sprite: Rect::new(0, 0, height, width),
            speed,
            dash_speed,
            dash_back_speed,
            hp,
            special_max,
            special_curr: special_max as f32,

            hit_stunned_duration: 5,
            jump_height,
            jump_distance,
        }
    }
}
