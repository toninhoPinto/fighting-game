use std::collections::HashMap;

use parry2d::bounding_volume::AABB;
use sdl2::rect::Point;

use crate::game_logic::characters::player::Player;

use super::transformation::Transformation;
pub struct ColliderAnimation {
    pub colliders: Vec<Collider>,
    pub pos_animations: HashMap<String, HashMap<i32, Transformation>> 
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ColliderType {
    Hitbox,  //attacking collider
    Hurtbox, //take damage
    Pushbox, //push character
    Grabbox,
}
#[derive(Debug)]
pub struct Collider {
    pub collider_type: ColliderType,
    pub name: String,
    pub aabb: AABB
}

impl ColliderAnimation {

    pub fn init(&self, current_colliders: &mut Vec<Collider>, player: &Player) {
        for i in 0..self.colliders.len() {
            if i < current_colliders.len() {
                //modify current
                current_colliders[i].collider_type = self.colliders[i].collider_type;
                current_colliders[i].name = self.colliders[i].name.clone();
                current_colliders[i].aabb = self.colliders[i].aabb;
            } else {
                //push
                current_colliders.push(Collider {
                    collider_type: self.colliders[i].collider_type,
                    name: self.colliders[i].name.clone(),
                    aabb: self.colliders[i].aabb
                });
            }
        }
        //TODO perhaps instead of truncating, add a bool that makes it enabled or disabled
        //TODO this way, eventually the player will get all the colliders it needs in the vec
        current_colliders.truncate(self.colliders.len());
    }

    // update offsets by player position 
    pub fn update(current_colliders: &mut Vec<Collider>, player: &Player) {
        for i in 0..current_colliders.len() {
            let aabb = &mut current_colliders[i].aabb;
        
            let left_player_pos = player.position.x as f32 - player.character.sprite.width() as f32 / 2.0;
                        
            aabb.mins.coords[0] = left_player_pos;
            aabb.mins.coords[1] = player.position.y as f32;
            aabb.maxs.coords[0] = left_player_pos;
            aabb.maxs.coords[1] = player.position.y as f32;
        }
    }

    //render offsets by frame index
    pub fn render(&self, current_colliders: &mut Vec<Collider>, player: &Player) {
        for i in 0..current_colliders.len() {
            let collider = &mut current_colliders[i];
            let aabb = &mut collider.aabb;
            let original_aabb = self.colliders[i].aabb;

            let position_at_frame = self.pos_animations.get(&self.colliders[i].name).unwrap();

            match position_at_frame.get(&(player.animator.animation_index as i32)) {
                Some(transformation) => {
                    let offset_x = transformation.pos.x as f32 * 2.0;
                    let offset_y = transformation.pos.y as f32 * 2.0;
                             
                    if player.flipped {
                        aabb.mins.coords[0] = (player.position.x as f32 + player.character.sprite.width() as f32 / 2.0)  - (offset_x + original_aabb.maxs.x * 2.0  * transformation.scale.0);
                        aabb.maxs.coords[0] = (player.position.x as f32 + player.character.sprite.width() as f32 / 2.0) - offset_x;
                    } else {
                        aabb.mins.coords[0] += offset_x;
                        aabb.maxs.coords[0] += offset_x + original_aabb.maxs.x * 2.0 * transformation.scale.0;
                    }
                                            
                    aabb.mins.coords[1] += offset_y;
                    aabb.maxs.coords[1] += offset_y + original_aabb.maxs.y * 2.0 * transformation.scale.1;
                },
                 //collider doesnt exist at this frame
                None => {}
            }



        }
    }

}
            

