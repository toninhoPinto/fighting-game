use std::collections::{HashMap, HashSet};

use parry2d::na::Vector2;

use crate::engine_types::{animation::ColliderAnimation, animator::Animator, collider::Collider, sprite_data::SpriteData};

#[derive(Clone)]
pub struct ColliderManager {
    pub colliders: Vec<Collider>,
    pub collisions_detected: HashSet<i32>
}

impl ColliderManager {

    pub fn new() -> Self {
        Self {
            colliders: Vec::new(),
            collisions_detected: HashSet::new(),
        }
    }
    
    pub fn update_colliders(&mut self, flipped: bool, position: Vector2<f64>, animator: &Animator , sprite_data: &HashMap<String, SpriteData>) {
        if let Some(animation) = animator.current_animation.as_ref() {
            if let Some(_) = animation.collider_animation {
                let animation_id = animator.sprite_shown as usize;
                let sprite_handle = animation.sprites[animation_id].1.clone();
                if animator.is_starting {
                    self.init_colliders(animator);
                }
                
                self.update_colliders_pos(flipped, position, animator, sprite_data.get(&sprite_handle).unwrap());
            }
        }
    }

    pub fn init_colliders(&mut self, animator: &Animator) {
        if let Some(collider_animation) = animator.current_animation.as_ref().unwrap().collider_animation.as_ref() {
            for i in 0..collider_animation.colliders.len() {
                if i < self.colliders.len() {
                    //modify current
                    self.colliders[i].collider_type = collider_animation.colliders[i].collider_type;
                    self.colliders[i].name = collider_animation.colliders[i].name.clone();
                    self.colliders[i].aabb = collider_animation.colliders[i].aabb;
                    self.colliders[i].enabled = collider_animation.colliders[i].enabled;
                } else {
                    //push
                    self.colliders.push(Collider {
                        collider_type: collider_animation.colliders[i].collider_type,
                        name: collider_animation.colliders[i].name.clone(),
                        aabb: collider_animation.colliders[i].aabb,
                        enabled: collider_animation.colliders[i].enabled,
                    });
                }
            }
            self.colliders.truncate(collider_animation.colliders.len());
        } else {
            self.colliders.clear();
        }
    }
    
    // update offsets by player position
    pub fn update_colliders_pos(&mut self, flipped: bool, position: Vector2<f64>, animator: &Animator, _sprite_data: &SpriteData) {
        let collider_animation = animator.current_animation.as_ref().unwrap().collider_animation.as_ref().unwrap().clone();
        for i in 0..self.colliders.len() {
            let aabb = &mut self.colliders[i].aabb;
    
            aabb.mins.coords[0] = position.x as f32;
            aabb.maxs.coords[0] = position.x as f32;

            aabb.mins.coords[1] = position.y as f32;
            aabb.maxs.coords[1] = position.y as f32;

            self.sync_with_character_animation(flipped, position, animator, &collider_animation, i);
        }
    }
    
    //render offsets by frame index
    fn sync_with_character_animation(
        &mut self,
        flipped: bool,
        position: Vector2<f64>,
        animator: &Animator,
        collider_animation: &ColliderAnimation,
        collider_index: usize,
    ) {
        let current_collider = &mut self.colliders[collider_index];
        let aabb = &mut current_collider.aabb;
        let original_collider = &collider_animation.colliders[collider_index];
        let original_aabb = original_collider.aabb;
    
        let positions_at_frame = collider_animation.pos_animations.get(&original_collider.name).unwrap();

        match positions_at_frame.get(&(animator.sprite_shown)) {
            Some(transformation) => {
                current_collider.enabled = true;
                let offset_x = transformation.pos.x as f32 * 2.0;
                let offset_y = transformation.pos.y as f32 * 2.0;

                if flipped {
                    aabb.mins.coords[0] = position.x as f32 - (offset_x + original_aabb.maxs.x * 2.0 * transformation.scale.0);
                    aabb.maxs.coords[0] = position.x as f32 - offset_x;
                } else {
                    aabb.mins.coords[0] = position.x as f32 + offset_x;
                    aabb.maxs.coords[0] = position.x as f32 + offset_x + original_aabb.maxs.x * 2.0 * transformation.scale.0;
                }

                aabb.mins.coords[1] += offset_y;
                aabb.maxs.coords[1] +=
                    offset_y + original_aabb.maxs.y * 2.0 * transformation.scale.1;
            }
            //collider doesnt exist at this frame
            None => {
                current_collider.enabled = false;
            }
        }
    }

}