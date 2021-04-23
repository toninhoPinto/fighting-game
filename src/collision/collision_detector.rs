use parry2d::{bounding_volume::BoundingVolume, math::Point, math::Real, na::Isometry2, query::{self, Contact}, shape::Cuboid};

use crate::asset_management::collider::{Collider, ColliderType};
use crate::game_logic::characters::player::Player;

//TODO, this cant be right, instead of iterating like this, perhaps use a quadtree? i think Parry2d has SimdQuadTree
//TODO probably smartest is to record the hits, and then have a separate function to handle if there is a trade between characters??

pub fn detect_hit(player_hitting: &mut Player, player_hit_colliders: &Vec<Collider>) -> Option<(Point<Real>, String)>{
    
    if !player_hitting.has_hit {
        for collider in player_hitting.colliders
            .iter()
            .filter(|&c| c.collider_type == ColliderType::Hitbox && c.enabled)
        {
            for collider_to_take_dmg in player_hit_colliders
                .iter()
                .filter(|&c| c.collider_type == ColliderType::Hurtbox && c.enabled)
            {
                if collider.aabb.intersects(&collider_to_take_dmg.aabb) {
                    let contact = contact(collider, collider_to_take_dmg);
                    
                    return if let Some(contact) = contact {
                        player_hitting.has_hit = true;
                        Some((contact.point2, collider.name.clone()))
                    } else {
                        None
                    };
                }
            }
        }
    }
    None
}


fn contact(p1_collider: &Collider, p2_collider: &Collider) -> Option<Contact> {
    let p1_width = p1_collider.aabb.half_extents().y;
    let p2_width = p2_collider.aabb.half_extents().y;

    let cuboid1 = Cuboid::new(p1_collider.aabb.half_extents());
    let cuboid2 = Cuboid::new(p2_collider.aabb.half_extents());
    let prediction = 1.0;

    let cuboid1_pos = Isometry2::translation(
        p1_collider.aabb.center().x,
        p1_collider.aabb.center().y,
    );
    let cuboid2_pos = Isometry2::translation(
        p2_collider.aabb.center().x,
        p2_collider.aabb.center().y,
    );


    query::contact(&cuboid1_pos, &cuboid1, &cuboid2_pos, &cuboid2, prediction)
        .unwrap()
}

pub fn detect_push(
    player1: &mut Player,
    player2: &mut Player,
    logic_timestep: f64,
) {
    player1.is_pushing = false;
    player2.is_pushing = false;

    for p1_collider in player1.colliders
        .iter()
        .filter(|&c| c.collider_type == ColliderType::Pushbox)
    {
        for p2_collider in player2.colliders
            .iter()
            .filter(|&c| c.collider_type == ColliderType::Pushbox)
        {
            if p1_collider.aabb.intersects(&p2_collider.aabb) {
                let p1_width = p1_collider.aabb.half_extents().y;
                let p2_width = p2_collider.aabb.half_extents().y;

                let penetrating = contact(p1_collider, p2_collider).unwrap().dist;

                if player1.velocity_x != 0
                    && player1.velocity_x.signum() == player1.dir_related_of_other
                {
                    player1.position.x =
                        player1.position.x + player1.velocity_x as f64 * penetrating as f64;
                    player2.position += player2.push(player1.velocity_x, &*player1, p2_width, logic_timestep);
                    player1.is_pushing = true;
                }

                if player1.is_airborne {
                    player2.position += player2.push(
                        player1.dir_related_of_other,
                        player1,
                        p2_width,
                        logic_timestep,
                    );
                    player1.is_pushing = true;
                }

                if player2.velocity_x != 0
                    && player2.velocity_x.signum() == player2.dir_related_of_other
                {
                    player2.position.x =
                        player2.position.x + player2.velocity_x as f64 * penetrating as f64;
                        player1.position += player1.push(player2.velocity_x, player2, p1_width, logic_timestep);
                    player2.is_pushing = true;
                }

                if player2.is_airborne {
                    player1.position += player1.push(
                        player2.dir_related_of_other,
                        player2,
                        p1_width,
                        logic_timestep,
                    );
                    player2.is_pushing = true;
                }
            }
        }
    }
}
