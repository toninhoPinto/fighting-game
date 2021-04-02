use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct SegmentedBar<'a> {
    pub rect: Rect,
    pub max_value: i32,
    pub color: Option<Color>,
    pub sprite: Option<&'a Texture<'a>>
}

impl<'a> SegmentedBar<'a> {
    pub fn new(pos_x: i32, pos_y: i32, width: u32, height: u32, max_value: i32, color: Option<Color>, sprite: Option<&'a Texture<'a>>) -> Self{
        //TODO return an error if both color and sprite are is_some or both are is_none
        Self {
            rect: Rect::new(pos_x, pos_y, width, height),
            max_value,
            color,
            sprite,
        }
    }

    //modify width based on character owner health
    //add character owner id
    pub fn update(){
    }

}