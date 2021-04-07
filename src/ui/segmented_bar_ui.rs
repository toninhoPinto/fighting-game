use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::cmp::min;

pub struct SegmentedBar<'a> {
    pub rects: Vec<Rect>,
    pub segmentations: i32,
    pub curr_value: f32,
    pub step: f32,
    pub color: Option<Color>,
    pub sprite: Option<&'a Texture<'a>>,
}

impl<'a> SegmentedBar<'a> {
    pub fn new(
        pos_x: i32,
        pos_y: i32,
        width: u32,
        height: u32,
        segmentations: i32,
        color: Option<Color>,
        sprite: Option<&'a Texture<'a>>,
    ) -> Self {
        //TODO return an error if both color and sprite are is_some or both are is_none
        let step = width as f32 / segmentations as f32;
        let gap = 10.0;
        let mut rects = Vec::new();
        for i in 0..segmentations {
            rects.push(Rect::new(
                pos_x + ((gap as f32 + step) * i as f32) as i32,
                pos_y,
                step as u32,
                height,
            ));
        }
        Self {
            rects: rects,
            segmentations,
            curr_value: segmentations as f32,
            step,
            color,
            sprite,
        }
    }

    //if has 3 bars
    //if current value = 3.0 -> index should be 2 and width should be 1 * step
    //if current value = 2.6 -> index should be 2 and width should be 0.6 * step
    //if current value = 2.0 -> index should be 2 and width should be 0 * step
    //if current value = 0.6 -> index should be 0 and width should be 0.6 * step
    pub fn update(&mut self, curr_value: f32) {

        self.curr_value = (curr_value.clamp(0.0, self.segmentations as f32) * 10.0).round() / 10.0;
        let index = min(self.curr_value as usize, self.segmentations as usize - 1);
        
        let decimal_only = self.curr_value - index as f32;
        let new_width = min((decimal_only * self.step) as u32, self.step as u32);
        
        for i in 0..self.rects.len() {
             if i < index {
                self.rects[i].set_width(self.step as u32);
             } else if i == index {
                self.rects[i].set_width(new_width);
                break;
             }
        }
    }

    pub fn render(&self) -> Vec<Rect> {
        let index = min(self.curr_value.ceil() as usize, self.segmentations as usize);
        self.rects[0..index].to_vec()
    }
}
