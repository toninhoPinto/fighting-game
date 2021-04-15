use sdl2::rect::Rect;

pub struct Camera {
    pub rect: Rect,
}

impl Camera {
    pub fn new(x: i32, y: i32, width: u32, height:u32) -> Self {
        Self{
            rect: Rect::new(x, y, width, height)
        }
    }
}