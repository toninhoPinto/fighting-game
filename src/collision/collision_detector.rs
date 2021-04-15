use parry2d::{bounding_volume::BoundingVolume, math::Real, math::Point};

use crate::asset_management::collider::{Collider, ColliderType};
use crate::game_logic::characters::player::Player;

//TODO, this cant be right, instead of iterating like this, perhaps use a quadtree? i think Parry2d has SimdQuadTree
//TODO probably smartest is to record the hits, and then have a separate function to handle if there is a trade between characters??

pub fn detect_p1_hit_p2(player1: &mut Player, 
    p1_colliders: &Vec<Collider>, p2_colliders: &Vec<Collider>) -> Option<Point<Real>> {

    if !player1.has_hit {
        for collider in p1_colliders
            .iter()
            .filter(|&c| c.collider_type == ColliderType::Hitbox)
        {
            for collider_to_take_dmg in p2_colliders
                .iter()
                .filter(|&c| c.collider_type == ColliderType::Hurtbox)
            {
                if collider.aabb.intersects(&collider_to_take_dmg.aabb) {

                    let mut center = collider_to_take_dmg.aabb.center();
                    let mut left = center;
                    left.x -= collider_to_take_dmg.aabb.half_extents().x;
                    let mut right = center;
                    right.x += collider_to_take_dmg.aabb.half_extents().x;

                    return Some(
                        collider.aabb.clip_segment(
                            &left,
                            &right
                        ).unwrap().a
                    )
                }
            }   
        }
    }
        
    None
}

pub fn detect_p2_hit_p1(player2: &mut Player, 
    p1_colliders: &Vec<Collider>, p2_colliders: &Vec<Collider>) -> Option<Point<Real>>{
    
    if !player2.has_hit {
        for collider in p2_colliders
            .iter()
            .filter(|&c| c.collider_type == ColliderType::Hitbox)
        {
            for collider_to_take_dmg in p1_colliders
                .iter()
                .filter(|&c| c.collider_type == ColliderType::Hurtbox)
            {
                if collider.aabb.intersects(&collider_to_take_dmg.aabb) {

                    let mut center = collider_to_take_dmg.aabb.center();
                    let mut left = center;
                    left.x -= collider_to_take_dmg.aabb.half_extents().x;
                    let mut right = center;
                    right.x += collider_to_take_dmg.aabb.half_extents().x;

                    return Some(
                        collider.aabb.clip_segment(
                            &left,
                            &right
                        ).unwrap().a
                    )
                }
            }
        }
    } 
    None
}


pub fn detect_push(player1: &mut Player, player2: &mut Player, 
    p1_colliders: &Vec<Collider>, p2_colliders: &Vec<Collider>, logic_timestep: f64){
    
    player1.is_pushing = false;
    player2.is_pushing = false;

    for p1_collider in p1_colliders
    .iter()
    .filter(|&c| c.collider_type == ColliderType::Pushbox)
    {
        for p2_collider in p2_colliders
            .iter()
            .filter(|&c| c.collider_type == ColliderType::Pushbox)
        {
            if p1_collider.aabb.intersects(&p2_collider.aabb) {
                let p1_width = p1_collider.aabb.half_extents().y;
                let p2_width = p2_collider.aabb.half_extents().y;
                if player1.velocity_x != 0 && player1.velocity_x.signum() == player1.dir_related_of_other {
                    player2.push(player1.velocity_x, player1, p2_width, logic_timestep);
                    player1.is_pushing = true;
                }

                if player1.is_airborne {
                    player2.push(player1.dir_related_of_other, player1, p2_width, logic_timestep);
                    player1.is_pushing = true;
                }

                if player2.velocity_x != 0 && player2.velocity_x.signum() == player2.dir_related_of_other {
                    player1.push(player2.velocity_x, player2, p1_width,logic_timestep);
                    player2.is_pushing = true;
                }

                if player2.is_airborne {
                    player1.push(player2.dir_related_of_other, player2, p1_width,logic_timestep);
                    player2.is_pushing = true;
                }
            }
        }   
    }
}