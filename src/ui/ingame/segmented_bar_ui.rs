use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::cmp::min;

pub struct SegmentedBar<'a> {
    pub point: Point,
    pub dimensions: (u32, u32),
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
            point: Point::new(pos_x, pos_y),
            dimensions: (width, height),
            rects,
            curr_value: value,
            max_value,
            color,
            step: value_step,
            sprite,
        }
    }

    fn create_rects(
        &mut self,
        max_value: i32,
    ){
        let step = max_value / self.step;
        let step_width = self.dimensions.0 / (max_value / self.step) as u32;
        let gap = 5;
        let mut rects = Vec::new();
        for i in 0..step {
            rects.push(Rect::new(
                self.point.x + ((gap + step_width) * i as u32) as i32,
                self.point.y,
                step_width,
                self.dimensions.1,
            ));
        }
        self.rects = rects;
    }

    pub fn update(&mut self, max_value: i32, curr_value: i32) {
        if self.max_value != max_value {
            self.max_value = max_value;
            self.create_rects(self.max_value);
        }
        self.curr_value = curr_value;
    }

    pub fn render(&self) -> Vec<Rect> {
        let index = min((self.curr_value / self.step) as usize, (self.max_value / self.step) as usize);
        self.rects[0..index].to_vec()
    }
}
