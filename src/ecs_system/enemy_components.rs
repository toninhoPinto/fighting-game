use parry2d::na::Vector2;
use sdl2::rect::Rect;

use crate::{asset_management::asset_holders::EntityAnimations, collision::collider_manager::ColliderManager, engine_types::animator::Animator, game_logic::{characters::player::Player,movement_controller::MovementController}};

#[derive(Clone)]
pub struct Health(pub i32);

pub struct Position(pub Vector2<f64>);

pub struct Renderable {
    pub flipped: bool,
    pub rect: Rect,
}

pub(crate) type Behaviour = fn(&Player, &Position, &mut MovementController, &mut Animator, collision_manager: &mut ColliderManager) -> ();
