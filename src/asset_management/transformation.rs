use sdl2::rect::Point;

#[derive(Debug)]
pub struct Transformation {
    pub pos: Point,
    pub scale: (f32, f32)
}