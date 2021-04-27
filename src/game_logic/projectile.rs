use parry2d::na::Vector2;
use sdl2::{rect::{Point, Rect}, render::Texture};

use crate::{asset_management::{animation::{Animation, Animator}, collider::{Collider, ColliderAnimation}}, rendering::camera::Camera};

use super::{character_factory::CharacterAssets, characters::Attack};

pub struct Projectile {
    pub position: Vector2<f64>,
    pub sprite: Rect,
    pub direction: Point,
    pub target_position: Option<Vector2<f64>>,
    pub dissapear_if_offscreen: bool,
    pub colliders: Vec<Collider>,
    pub speed: i32,
    pub attack: Attack,
    pub flipped: bool,
    pub animator: Animator,
    pub player_owner: i32,
    pub is_alive: bool,
    pub die_out_of_camera: bool,
    pub on_hit: fn(&mut Projectile) -> (),
    pub on_update: Option<fn(&mut Projectile) -> ()>,
    pub on_death: Option<fn(&mut Projectile) -> ()>,
}

impl Projectile {
    pub fn new(player_owner: i32, spawn_point: Vector2<f64>, attack: Attack) -> Self {
        let on_hit_die = |projectile: &mut Projectile| {projectile.is_alive = false};
        Self {
            position: spawn_point,
            sprite: Rect::new(0, 0, 100, 110),
            speed: 0,
            direction: Point::new(0, 0),
            target_position: None,
            colliders: Vec::new(),
            dissapear_if_offscreen: false,
            attack,
            flipped: false,
            animator: Animator::new(),
            player_owner,
            is_alive: true,
            die_out_of_camera: true,
            on_hit: on_hit_die,
            on_update:None,
            on_death: None,
        }
    }

    pub fn init(&mut self, animation: Animation, collider_data: &ColliderAnimation) {
        self.animator.play(animation, 1.0,false);

        self.colliders = collider_data.colliders.clone();

        let projectile_pos = self.position;
        self.colliders.iter_mut().for_each(|c| {
            c.enabled = true;

            let aabb = &mut c.aabb;
            aabb.mins.coords[0] += projectile_pos.x as f32;
            aabb.mins.coords[1] += projectile_pos.y as f32;
            aabb.maxs.coords[0] += projectile_pos.x as f32;
            aabb.maxs.coords[1] += projectile_pos.y as f32;
        });
    }

    pub fn update(&mut self, camera: &Camera) {
        match self.target_position {
            Some(target) => {

                if self.position.x < camera.rect.x as f64 || self.position.x > (camera.rect.x as u32 + camera.rect.width()) as f64 {
                    self.is_alive = false;
                }

                let mut position_directionless = self.position;
                position_directionless.x *= self.direction.x as f64;
                position_directionless.y *= self.direction.y as f64;

                let mut target_directionless = target;
                target_directionless.x *= self.direction.x as f64;
                target_directionless.y *= self.direction.y as f64;

                if position_directionless.x <= target_directionless.x
                    && position_directionless.y <= target_directionless.y
                {
                    self.position += Vector2::new((self.direction.x * self.speed) as f64, 0.0);
                }
            }
            None => {
                self.position += Vector2::new((self.direction.x * self.speed) as f64, 0.0);
            }
        }
    }

    pub fn render<'a>(&'a self, assets: &'a CharacterAssets<'a>) -> &'a Texture {
        assets.textures.get(&self.animator.render()).unwrap()
    }
}
