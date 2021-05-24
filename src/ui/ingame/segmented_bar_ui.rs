use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::cmp::min;

pub struct SegmentedBar<'a> {
    pub rects: Vec<Rect>,
    pub curr_value: i32,
    pub max_value: i32,
    pub step: i32,
    pub color: Option<Color>,
    pub sprite: Option<&'a Texture<'a>>,
}

impl<'a> SegmentedBar<'a> {
    pub fn new(
        pos_x: i32,
        pos_y: i32,
        width: u32,
        height: u32,
        max_value: i32,
        value: i32,
        value_step: i32,
        color: Option<Color>,
        sprite: Option<&'a Texture<'a>>,
    ) -> Self {
        //TODO return an error if both color and sprite are is_some or both are is_none
        let step = max_value / value_step;
        let step_width = width / (max_value / value_step) as u32;
        let gap = 5;
        let mut rects = Vec::new();
        for i in 0..step {
            rects.push(Rect::new(
                pos_x + ((gap + step_width) * i as u32) as i32,
                pos_y,
                step_width,
                height,
            ));
        }
        Self {
            rects,
            curr_value: value,
            max_value,
            color,
            step: value_step,
            sprite,
        }
    }

    pub fn update(&mut self, curr_value: i32) {
        self.curr_value = curr_value;
    }

    pub fn render(&self) -> Vec<Rect> {
        let index = min((self.curr_value / self.step) as usize, (self.max_value / self.step) as usize);
        self.rects[0..index].to_vec()
    }
}
