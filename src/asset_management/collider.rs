use parry2d::bounding_volume::AABB;
use sdl2::rect::Point;

use crate::game_logic::characters::player::Player;
pub struct ColliderAnimation {
    pub colliders: Vec<Collider>,
    pub pos_animations: Vec<Vec<Point>> // outside of Vec is for n frames, the inside Vec is for m colliders
}

pub enum ColliderType {
    Hitbox,  //attacking collider
    Hurtbox, //take damage
    Pushbox, //push character
    Grabbox,
}

pub struct Collider {
    pub collider_type: ColliderType,
    pub name: String,
    pub aabb: AABB
}

impl ColliderAnimation {

    pub fn init(&self, current_colliders: &mut Vec<AABB>, player: &Player) {
        for i in 0..self.colliders.len() {
            let mut aabb = self.colliders[i].aabb;
            let offset_x = self.pos_animations[0][i].x as f32 ;
            let offset_y = self.pos_animations[0][i].y as f32;
    
            let left_player_pos = player.position.x as f32 - player.character.sprite.width() as f32 / 2.0;
            aabb.mins.coords[0] = left_player_pos + offset_x * 2.0;
            aabb.mins.coords[1] = offset_y * 2.0 + player.position.y as f32;
            aabb.maxs.coords[0] = left_player_pos + (aabb.maxs.x + offset_x) * 2.0;
            aabb.maxs.coords[1] = (aabb.maxs.y + offset_y) * 2.0  + player.position.y as f32;
    
            current_colliders.push(aabb);
        }
    }

    // update offsets by player position 
    pub fn update(current_colliders: &mut Vec<AABB>, player: &Player) {
        for i in 0..current_colliders.len() {
            let aabb = &mut current_colliders[i];
        
            let left_player_pos = player.position.x as f32 - player.character.sprite.width() as f32 / 2.0;
                        
            aabb.mins.coords[0] = left_player_pos;
            aabb.mins.coords[1] = player.position.y as f32;
            aabb.maxs.coords[0] = left_player_pos;
            aabb.maxs.coords[1] = player.position.y as f32;
        }
    }

    //render offsets by frame index
    pub fn render(&self, current_colliders: &mut Vec<AABB>, player: &Player) {
        for i in 0..current_colliders.len() {
            let aabb = &mut current_colliders[i];
            let original_aabb = self.colliders[i].aabb;
            let offset_x = self.pos_animations[player.animator.animation_index as usize][i].x as f32 ;
            let offset_y = self.pos_animations[player.animator.animation_index as usize][i].y as f32;
                    
            aabb.mins.coords[0] += offset_x * 2.0;
            aabb.mins.coords[1] += offset_y * 2.0;
            aabb.maxs.coords[0] += (original_aabb.maxs.x + offset_x) * 2.0;
            aabb.maxs.coords[1] += (original_aabb.maxs.y + offset_y) * 2.0;
        }
    }

}
            


