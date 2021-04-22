use sdl2::{pixels::Color, rect::Rect};

use crate::asset_management::{
    collider::Collider, common_assets::CommonAssets, vfx::particle::Particle,
};

use super::{
    character_factory::CharacterAssets, characters::player::Player, projectile::Projectile,
};

const LIMIT_NUMBER_OF_VFX: usize = 5;
pub struct Game<'a> {
    pub current_frame: i32,
    pub player1: &'a mut Player<'a>,
    pub player2: &'a mut Player<'a>,

    pub projectiles: Vec<Projectile>,

    pub hit_vfx: Vec<Particle>,
}

impl<'a> Game<'a> {
    pub fn new(player1: &'a mut Player<'a>, player2: &'a mut Player<'a>) -> Self {
        Self {
            current_frame: 0,

            player1,
            player2,

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

    pub fn update_player_colliders(player: &mut Player, assets: &CharacterAssets) {
        let collider_animation = assets
            .collider_animations
            .get(&player.animator.current_animation.unwrap().name);

            if let Some(collider_anim) = collider_animation {
                collider_anim.update(player);
            }
    }

    pub fn update_projectiles(&mut self) {
        for i in 0..self.projectiles.len() {
            self.projectiles[i].update();
        }
    }

}
