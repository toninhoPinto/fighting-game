use sdl2::{pixels::Color, rect::Rect};

use crate::asset_management::{collider::Collider, common_assets::CommonAssets, vfx::particle::Particle};

use super::{character_factory::CharacterAssets, characters::player::Player, projectile::Projectile};

const LIMIT_NUMBER_OF_VFX: usize = 5;
pub struct Game<'a>{
    pub current_frame: i32,
    pub player1: &'a mut Player<'a>,
    pub player2: &'a mut Player<'a>,

    pub projectiles: Vec<Projectile>,

    pub p1_colliders: Vec<Collider>,
    pub p2_colliders: Vec<Collider>,

    pub hit_vfx: Vec<Particle>,
}

impl<'a> Game<'a>{
    pub fn new(player1: &'a mut Player<'a>, player2: &'a mut Player<'a>) -> Self{
        Self{
            current_frame: 0,

            player1,
            player2,

            projectiles: Vec::new(),

            p1_colliders: Vec::new(),
            p2_colliders: Vec::new(),

            hit_vfx: Vec::new(),
        }
    }

    pub fn spawn_vfx(&mut self, rect: Rect, type_of_animation: String, tint: Option<Color>){
        if self.hit_vfx.len() < LIMIT_NUMBER_OF_VFX {
            //push with bool as true
            self.hit_vfx.push( Particle {
                active: true,
                sprite: rect,
                name: type_of_animation,
                animation_index: 0,
                tint,
             } );
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

    pub fn update_vfx(&mut self, assets: &CommonAssets){
        for i in 0..self.hit_vfx.len() {
            if self.hit_vfx[i].active {
                self.hit_vfx[i].animation_index += 1; // multiply by dt and by animation speed i think, check animator code
                if self.hit_vfx[i].animation_index >= assets.hit_effect_animations.get(&self.hit_vfx[i].name).unwrap().length {
                    self.hit_vfx[i].active = false;
                    self.hit_vfx[i].animation_index = 0;
                }
            }
        }
    }

    pub fn update_collider_p1(&mut self, p1_assets: &CharacterAssets){
        let collider_animation1 = p1_assets.collider_animations.get(&self.player1.animator.current_animation.unwrap().name);
        if collider_animation1.is_some() {
            if collider_animation1.unwrap().colliders.len() != self.p1_colliders.len() {
                collider_animation1.unwrap().init(&mut self.p1_colliders);
            }
            collider_animation1.unwrap().update(&mut self.p1_colliders, &self.player1);
        }
    }

    pub fn update_collider_p2(&mut self, p2_assets: &CharacterAssets){
        let collider_animation2 = p2_assets.collider_animations.get(&self.player2.animator.current_animation.unwrap().name);
        if collider_animation2.is_some() {
            if collider_animation2.unwrap().colliders.len() != self.p2_colliders.len() {
                collider_animation2.unwrap().init(&mut self.p2_colliders);
            }
            collider_animation2.unwrap().update(&mut self.p2_colliders, &self.player2);
        }
    }

    pub fn update_projectiles(&mut self){
        for i in 0..self.projectiles.len() {
            self.projectiles[i].update();
        }
    }
}
