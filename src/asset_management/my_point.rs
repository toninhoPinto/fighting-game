use sdl2::rect::Point;
use super::custom_serialization::sdl2_point_serial;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct MyPoint {
    #[serde(with = "sdl2_point_serial")]
    pub p : sdl2::rect::Point
}

impl MyPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { p: Point::new(x, y) }
    }
}