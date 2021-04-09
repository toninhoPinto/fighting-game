use sdl2::rect::{Point, Rect};
use std::string::String;
use crate::asset_management::{custom_serialization::{sdl2_point_serial,sdl2_rect_serial}, my_point::MyPoint};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Projectile {
    #[serde(with = "sdl2_point_serial")]
    pub position: Point,
    #[serde(with = "sdl2_rect_serial")]
    pub sprite: Rect,
    #[serde(with = "sdl2_point_serial")]
    pub direction: Point,
    pub target_position: Option<MyPoint>,
    pub speed: i32,
    pub damage: i32,
    pub flipped: bool,
    pub animation_index: f32,
    pub animation_name: String,
    pub player_owner: i32,
    pub is_alive: bool,
}

impl Projectile {
    pub fn new(player_owner: i32, spawn_point: Point) -> Self {
        Self {
            position: spawn_point,
            sprite: Rect::new(0, 0, 100, 110),
            speed: 10,
            direction: Point::new(0, 0),
            target_position: None,
            damage: 20,
            flipped: false,
            animation_index: 0.0,
            animation_name: "note".to_string(),
            player_owner,
            is_alive: true,
        }
    }

    pub fn update(&mut self) {
        match self.target_position {
            Some(_) => {
                if self.position.x <= self.target_position.unwrap().p.x
                    && self.position.y <= self.target_position.unwrap().p.y
                {
                    self.position = self.position.offset(self.speed, 0);
                }
            }
            None => {
                self.position = self.position.offset(self.speed, 0);
            }
        }
    }

    fn render(&self) {}
}
