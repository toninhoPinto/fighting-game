use parry2d::na::Vector2;
use sdl2::{pixels::Color, rect::Rect};

use crate::{asset_management::{
    common_assets::CommonAssets, vfx::particle::Particle,
}, rendering::camera::Camera};

use super::{character_factory::CharacterAnimations, characters::player::Player, inputs::input_cycle::AllInputManagement, projectile::Projectile};

const LIMIT_NUMBER_OF_VFX: usize = 5;
pub struct Game {
    pub current_frame: i32,
    pub player1: Player,
    pub player2: Player,
    pub camera: Camera,

    pub projectiles: Vec<Projectile>,

    pub hit_vfx: Vec<Particle>,
}

impl Game {
    pub fn new(player1: Player, player2: Player, camera: Camera) -> Self {
        Self {
            current_frame: 0,

            player1,
            player2,
            camera,

            projectiles: Vec::new(),

            hit_vfx: Vec::new(),
        }
    }

    pub fn spawn_vfx(&mut self, rect: Rect, type_of_animation: String, tint: Option<Color>) {
        if self.hit_vfx.len() < LIMIT_NUMBER_OF_VFX {
            //push with bool as true
            self.hit_vfx.push(Particle {
                active: true,
                sprite: rect,
                name: type_of_animation,
                animation_index: 0,
                tint,
            });
        } else {
            let mut disabled_index = None;
            for i in 0..self.hit_vfx.len() {
                if !self.hit_vfx[i].active {
                    disabled_index = Some(i);
                    break;
                }
            }
            if disabled_index.is_some() {
                self.hit_vfx[disabled_index.unwrap()].active = true;
                self.hit_vfx[disabled_index.unwrap()].sprite = rect;
                self.hit_vfx[disabled_index.unwrap()].name = type_of_animation;
                self.hit_vfx[disabled_index.unwrap()].animation_index = 0;
                self.hit_vfx[disabled_index.unwrap()].tint = tint;
            }
        }
    }

    pub fn update_vfx(&mut self, assets: &CommonAssets) {
        for i in 0..self.hit_vfx.len() {
            if self.hit_vfx[i].active {
                self.hit_vfx[i].animation_index += 1;
                if self.hit_vfx[i].animation_index
                    >= assets
                        .hit_effect_animations
                        .get(&self.hit_vfx[i].name)
                        .unwrap()
                        .sprites.len() as i32
                {
                    self.hit_vfx[i].active = false;
                    self.hit_vfx[i].animation_index = 0;
                }
            }
        }
    }


    //TODO change player and projectile to &Vec<Collider> and fuse both functions
    pub fn update_player_colliders_position_only(player: &mut Player, prev_pos: Vector2<f64>) {
        let offset = player.position - prev_pos;
        for i in 0..player.colliders.len() {
            let aabb = &mut player.colliders[i].aabb;

            aabb.mins.coords[0] += offset.x as f32;
            aabb.mins.coords[1] += offset.y as f32;
            aabb.maxs.coords[0] += offset.x as f32;
            aabb.maxs.coords[1] += offset.y as f32;
        }
    }

    pub fn update_projectile_colliders_position_only(projectile: &mut Projectile, prev_pos: Vector2<f64>) {
        let offset = projectile.position - prev_pos;
        for i in 0..projectile.colliders.len() {
            let aabb = &mut projectile.colliders[i].aabb;

            aabb.mins.coords[0] += offset.x as f32;
            aabb.mins.coords[1] += offset.y as f32;
            aabb.maxs.coords[0] += offset.x as f32;
            aabb.maxs.coords[1] += offset.y as f32;
        }
    }

    pub fn update_projectiles(&mut self, inputs: &AllInputManagement, p1_anims: &CharacterAnimations, p2_anims: &CharacterAnimations) {
        for i in 0..self.projectiles.len() {
            let prev_pos =  self.projectiles[i].position;
            self.projectiles[i].update(&self.camera);
            Game::update_projectile_colliders_position_only(&mut self.projectiles[i], prev_pos);

            let animations = if self.projectiles[i].player_owner == 1 { p1_anims } else { p2_anims };

            if let Some(on_update) = self.projectiles[i].on_update {
                on_update(inputs, animations, &mut self.projectiles[i]);
            }
        }
    }

}
