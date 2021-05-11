use parry2d::na::Vector2;
use sdl2::rect::Rect;

use crate::game_logic::{characters::player::Player, factories::enemy_factory::EnemyAnimations, movement_controller::MovementController};

pub struct Health(pub i32);
pub struct Position(pub Vector2<f64>);

pub struct Renderable {
    pub flipped: bool,
    pub rect: Rect,
}

pub(crate) type Behaviour = fn(&Player, &Position, &mut MovementController, &EnemyAnimations) -> ();
