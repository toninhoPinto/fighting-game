use parry2d::na::Vector2;
use sdl2::rect::Rect;

use crate::{collision::collider_manager::ColliderManager, engine_types::animator::Animator, game_logic::{characters::player::Player,movement_controller::MovementController}};

#[derive(Clone)]
pub struct Health(pub i32);

pub struct Position(pub Vector2<f64>);

pub struct Renderable {
    pub flipped: bool,
    pub rect: Rect,
}


pub trait Behaviour {
    fn act(&mut self, player: &Player, pos: &Position, mov: &mut MovementController, anim: &mut Animator, collision_manager: &mut ColliderManager, delta: f64);
}