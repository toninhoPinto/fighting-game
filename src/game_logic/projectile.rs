use parry2d::na::Vector2;
use sdl2::rect::{Point, Rect};
use std::string::String;

#[derive(Clone, PartialEq, Debug)]
pub struct Projectile {
    pub position: Vector2<f64>,
    pub sprite: Rect,
    pub direction: Point,
    pub target_position: Option<Vector2<f64>>,
    pub speed: i32,
    pub damage: i32,
    pub flipped: bool,
    pub animation_index: f32,
    pub animation_name: String,
    pub player_owner: i32,
    pub is_alive: bool,
}

impl Projectile {
    pub fn new(player_owner: i32, spawn_point: Vector2<f64>) -> Self {
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
                if self.position.x <= self.target_position.unwrap().x
                    && self.position.y <= self.target_position.unwrap().y
                {
                    self.position += Vector2::new(self.speed as f64, 0.0);
                }
            }
            None => {
                self.position += Vector2::new(self.speed as f64, 0.0);
            }
        }
    }
}
