use super::transformation::Transformation;
use crate::game_logic::characters::player::Player;
use parry2d::bounding_volume::AABB;
use std::collections::HashMap;
#[derive(Debug)]
pub struct ColliderAnimation {
    pub colliders: Vec<Collider>,
    pub pos_animations: HashMap<String, HashMap<i32, Transformation>>,
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum ColliderType {
    Hitbox,  //attacking collider
    Hurtbox, //take damage
    Grabbox,
    Grabbablebox,
    Pushbox, //push character
}
#[derive(Debug, Clone)]
pub struct Collider {
    pub collider_type: ColliderType,
    pub name: String,
    pub aabb: AABB,
    pub enabled: bool,
}

impl ColliderAnimation {
    pub fn init(&self, current_colliders: &mut Vec<Collider>) {
        for i in 0..self.colliders.len() {
            if i < current_colliders.len() {
                //modify current
                current_colliders[i].collider_type = self.colliders[i].collider_type;
                current_colliders[i].name = self.colliders[i].name.clone();
                current_colliders[i].aabb = self.colliders[i].aabb;
                current_colliders[i].enabled = self.colliders[i].enabled;
            } else {
                //push
                current_colliders.push(Collider {
                    collider_type: self.colliders[i].collider_type,
                    name: self.colliders[i].name.clone(),
                    aabb: self.colliders[i].aabb,
                    enabled: self.colliders[i].enabled,
                });
            }
        }
        current_colliders.truncate(self.colliders.len());
    }

    // update offsets by player position
    pub fn update(&self, player: &mut Player) {
        for i in 0..player.colliders.len() {
            let aabb = &mut player.colliders[i].aabb;

            let left_player_pos =
                player.position.x as f32 - player.character.sprite.width() as f32 / 2.0;

            aabb.mins.coords[0] = left_player_pos;
            aabb.mins.coords[1] = player.position.y as f32;
            aabb.maxs.coords[0] = left_player_pos;
            aabb.maxs.coords[1] = player.position.y as f32;
            self.sync_with_character_animation(i, player);
        }
    }

    //render offsets by frame index
    fn sync_with_character_animation(
        &self,
        collider_index: usize,
        player: &mut Player,
    ) {
        let current_collider = &mut player.colliders[collider_index];
        let aabb = &mut current_collider.aabb;
        let original_collider = &self.colliders[collider_index];
        let original_aabb = original_collider.aabb;

        let position_at_frame = self.pos_animations.get(&original_collider.name).unwrap();

        match position_at_frame.get(&(player.animator.sprite_shown)) {
            Some(transformation) => {
                current_collider.enabled = true;
                let offset_x = transformation.pos.x as f32 * 2.0;
                let offset_y = transformation.pos.y as f32 * 2.0;

                if player.flipped {
                    aabb.mins.coords[0] = (player.position.x as f32
                        + player.character.sprite.width() as f32 / 2.0)
                        - (offset_x + original_aabb.maxs.x * 2.0 * transformation.scale.0);
                    aabb.maxs.coords[0] = (player.position.x as f32
                        + player.character.sprite.width() as f32 / 2.0)
                        - offset_x;
                } else {
                    aabb.mins.coords[0] += offset_x;
                    aabb.maxs.coords[0] +=
                        offset_x + original_aabb.maxs.x * 2.0 * transformation.scale.0;
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
