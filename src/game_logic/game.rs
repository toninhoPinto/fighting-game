use parry2d::na::Vector2;
use sdl2::{pixels::Color, rect::Rect, render::TextureQuery};

use crate::{asset_management::{cast_point::CastPoint, common_assets::CommonAssets, vfx::particle::Particle}, rendering::camera::Camera};

use super::{character_factory::CharacterAnimations, characters::player::Player, inputs::input_cycle::AllInputManagement, projectile::Projectile};

const LIMIT_NUMBER_OF_VFX: usize = 5;
pub struct Game {
    pub current_frame: i32,
    pub player: Player,
    pub camera: Camera,

    pub projectiles: Vec<Projectile>,

    pub hit_vfx: Vec<Particle>,
}

impl Game {
    pub fn new(player: Player, camera: Camera) -> Self {
        Self {
            current_frame: 0,

            player,
            camera,

            projectiles: Vec::new(),

            hit_vfx: Vec::new(),
        }
    }

    pub fn spawn_vfx(&mut self, rect: Rect, flipped: bool, type_of_animation: String, tint: Option<Color>) {
        if self.hit_vfx.len() < LIMIT_NUMBER_OF_VFX {
            //push with bool as true
            self.hit_vfx.push(Particle {
                active: true,
                sprite: rect,
                name: type_of_animation,
                animation_index: 0,
                sprite_shown: 0,
                flipped,
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
                self.hit_vfx[disabled_index.unwrap()].sprite_shown = 0;
                self.hit_vfx[disabled_index.unwrap()].flipped = flipped;
                self.hit_vfx[disabled_index.unwrap()].tint = tint;
            }
        }
    }

    pub fn update_vfx(&mut self, assets: &CommonAssets) {
        for i in 0..self.hit_vfx.len() {
            let vfx = &mut self.hit_vfx[i];

            if vfx.active {
                vfx.animation_index += 1;
                
                let curr_animation = assets
                .hit_effect_animations
                .get(&vfx.name)
                .unwrap();

                if vfx.animation_index >= curr_animation.sprites[vfx.sprite_shown as usize].0 as i32
                {
                    vfx.sprite_shown += 1;
                }
                let time_over_animation_length = vfx.animation_index >= curr_animation.length as i32;
                let time_to_switch_to_next_sprite = vfx.sprite_shown < (curr_animation.sprites.len() - 1) as i32 
                && (curr_animation.sprites[vfx.sprite_shown as usize + 1].0 ) <= vfx.animation_index as i64;

                if time_over_animation_length || time_to_switch_to_next_sprite {
                    vfx.sprite_shown += 1;
                }


                if vfx.sprite_shown >= curr_animation.sprites.len() as i32 {
                    vfx.active = false;
                    vfx.animation_index = 0;
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

    pub fn update_projectiles(&mut self, inputs: &AllInputManagement, p1_anims: &CharacterAnimations) {
        for i in 0..self.projectiles.len() {
            let prev_pos =  self.projectiles[i].position;
            self.projectiles[i].update(&self.camera);
            Game::update_projectile_colliders_position_only(&mut self.projectiles[i], prev_pos);

            if let Some(on_update) = self.projectiles[i].on_update {
                on_update(inputs, p1_anims, &mut self.projectiles[i]);
            }
        }
    }

    pub fn fx(&mut self, general_assets: &CommonAssets) {
        
        let spawn = |cast_point: &mut CastPoint, flipped: bool, game: &mut Game, general_assets: &CommonAssets| {
            let texture_id = &general_assets.hit_effect_animations.get(&cast_point.name.replace("?", "")).unwrap().sprites[0].1;
            let TextureQuery { width, height, .. } = general_assets
                                    .hit_effect_textures
                                    .get(texture_id)
                                    .unwrap()
                                    .query();
        
            let texture_width = width * 2;
            let texture_height = height * 2;
            //^ * 2 above is to make the sprite bigger

            let rect = Rect::new(
                cast_point.point.x as i32,
                cast_point.point.y as i32,
                texture_width,
                texture_height,
            );
            
            game.spawn_vfx(
                rect,
                flipped,
                cast_point.name.to_string(),
                Some(Color::GREEN),
            );
            
        };

        let process_point_offset = |player: &Player, point: &CastPoint| -> Vector2<f64> {
            let mut final_pos = player.position;
            if player.facing_dir > 0 {
                final_pos.x -= player.character.sprite.width() as f64 / 2.0;
                final_pos.x -= point.point.x * 2.0;
                final_pos.y += point.point.y * 2.0;
            } else {
                final_pos.x -= player.character.sprite.width() as f64 / 2.0;
                final_pos += point.point * 2.0;
            }
            final_pos
        };

        let mut points = Vec::new();
        let hash_points = &self.player.animator.current_animation.as_ref().unwrap().cast_point;

        if hash_points.keys().len() > 0 {
            match hash_points.get(&(self.player.animator.animation_index as i64 -1)) {
                Some(point) => {
                    let mut point_position_fixed = point.clone();
                    point_position_fixed.point = process_point_offset(&self.player, &point_position_fixed);
                    points.push((point_position_fixed, self.player.facing_dir));
                }
                None => {}
            }
        }

        for point in &mut points {
            spawn(&mut point.0, point.1 > 0, self, general_assets);
        }
        
    }
}
