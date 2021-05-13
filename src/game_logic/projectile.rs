use parry2d::{math::{Point, Real}, na::Vector2};
use sdl2::{rect::Rect, render::Texture};

use crate::{asset_management::asset_holders::{EntityAnimations, EntityAssets}, engine_types::{animation::Animation, animator::Animator, collider::Collider}, rendering::camera::Camera};

use super::{characters::Attack, inputs::input_cycle::AllInputManagement};

pub struct Projectile {
    pub position: Vector2<f64>,
    pub sprite: Rect,
    pub direction: Vector2<f64>,
    pub target_position: Option<Vector2<f64>>,
    pub has_reached_target: bool,
    pub dissapear_if_offscreen: bool,
    pub colliders: Vec<Collider>,
    pub speed: i32,
    pub attack: Attack,
    pub flipped: bool,
    pub animator: Animator,
    pub player_owner: i32,
    pub kill_at_animation_end: bool,
    pub is_alive: bool,
    pub die_out_of_camera: bool,
    pub on_hit: fn(Point<Real>, &mut Projectile, &EntityAnimations) -> (),
    pub on_update: Option<fn(&AllInputManagement, &EntityAnimations, &mut Projectile) -> ()>,
    pub on_death: Option<fn(&mut Projectile) -> ()>,
}

impl Projectile {
    pub fn new(player_owner: i32, spawn_point: Vector2<f64>, attack: Attack) -> Self {
        let on_hit_die = |hit_point: Point<Real>, projectile: &mut Projectile, animations: &EntityAnimations| {
            if let Some(hit_anim) = animations.projectile_animation.get("hit") {
                projectile.animator.play_once(hit_anim.clone(), 1.0, false);
                projectile.colliders.clear();
            }
            projectile.position.x = hit_point.x as f64;
            projectile.position.y = hit_point.y as f64;
            projectile.sprite.set_width(100);
            projectile.sprite.set_height(80);
            projectile.direction = Vector2::new(0.0, 0.0);
            projectile.target_position = None;
            projectile.kill_at_animation_end = true
        };
        Self {
            position: spawn_point,
            sprite: Rect::new(0, 0, 50, 70),
            speed: 0,
            direction: Vector2::new(0.0, 0.0),
            target_position: None,
            has_reached_target: false,
            colliders: Vec::new(),
            dissapear_if_offscreen: false,
            attack,
            flipped: false,
            animator: Animator::new(),
            player_owner,
            is_alive: true,
            kill_at_animation_end: false,
            die_out_of_camera: true,
            on_hit: on_hit_die,
            on_update:None,
            on_death: None,
        }
    }

    pub fn init(&mut self, animation: Animation) {
        if let Some(cd) = &animation.collider_animation {
            self.colliders = cd.colliders.clone();
        }

        self.animator.play(animation, 1.0,false);

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
                    self.position += Vector2::new(self.direction.x * self.speed as f64, 0.0);
                } else {
                    self.has_reached_target = true;
                }
            }
            None => {
                self.position += Vector2::new(self.direction.x * self.speed as f64, self.direction.y * self.speed as f64);
            }
        }
        if self.kill_at_animation_end && self.animator.is_finished {
            self.is_alive = false;
        }
        self.animator.update();
    }

    pub fn render<'a>(&'a self, assets: &'a EntityAssets<'a>) -> &'a Texture {
        assets.textures.get(&self.animator.render()).unwrap()
    }
}
