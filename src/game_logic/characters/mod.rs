use sdl2::rect::Rect;
use std::string::String;

pub mod player;

#[derive(Debug)]
pub struct Character {
    //visual
    pub sprite: Rect,

    //stats
    pub name: String,
    pub hp: i32,
    pub speed: i32,
    pub dash_speed: i32,
    pub dash_back_speed: i32,
    hit_stunned_duration: i32,
    // hit_stunned_duration was intended to give a small break on the dash animation
    // but actually ryu dash has 6 sprites that run over 26 frames and not spread equally, 11 of which have movement
}

impl Character {
    pub fn new(name: String, height: u32, width: u32, speed: i32, dash_speed: i32, dash_back_speed: i32, hp: i32) -> Self {
        Self {
            name,
            sprite: Rect::new(0, 0, height, width),
            speed,
            dash_speed,
            dash_back_speed,
            hp,

            hit_stunned_duration: 5,
        }
    }
}