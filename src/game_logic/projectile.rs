use parry2d::na::Vector2;
use sdl2::{rect::{Point, Rect}, render::Texture};

use crate::asset_management::{animation::{Animation, Animator}, collider::Collider};

use super::character_factory::CharacterAssets;

pub struct Projectile {
    pub position: Vector2<f64>,
    pub sprite: Rect,
    pub direction: Point,
    pub target_position: Option<Vector2<f64>>,
    pub colliders: Vec<Collider>,
    pub speed: i32,
    pub damage: i32,
    pub flipped: bool,
    pub animator: Animator,
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
            colliders: Vec::new(),
            damage: 20,
            flipped: false,
            animator: Animator::new(),
            player_owner,
            is_alive: true,
        }
    }

    pub fn init(&mut self, animation: Animation) {
        self.animator.play(animation, 1.0,false);
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

    pub fn render<'a>(&'a self, assets: &'a CharacterAssets<'a>) -> &'a Texture {
        assets.textures.get(&self.animator.render()).unwrap()
    }
}
