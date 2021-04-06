use crate::asset_management::collider::Collider;

use super::{characters::player::Player, projectile::Projectile};

pub struct Game<'a>{
    pub player1: &'a mut Player<'a>,
    pub player2: &'a mut Player<'a>,

    pub projectiles: Vec<Projectile>,

    pub p1_colliders: Vec<Collider>,
    pub p2_colliders: Vec<Collider>,
}

impl<'a> Game<'a>{
    pub fn new(player1: &'a mut Player<'a>, player2: &'a mut Player<'a>) -> Self{
        Self{
            player1,
            player2,

            projectiles: Vec::new(),

            p1_colliders: Vec::new(),
            p2_colliders: Vec::new(),
        }
    }
}