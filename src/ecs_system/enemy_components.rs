use parry2d::na::Vector2;
use sdl2::rect::Rect;

use crate::game_logic::inputs::game_inputs::GameAction;



#[derive(Clone)]
pub struct Health(pub i32);

pub struct Position(pub Vector2<f64>);

pub struct Renderable {
    pub flipped: bool,
    pub rect: Rect,
}

pub trait Behaviour {
    fn act(&mut self, delta: f64) -> Option<GameAction>;
}

#[derive(PartialEq, Debug)]
pub enum AIType {
    Allied,
    Enemy,
    Neutral
}