use parry2d::na::Vector2;
use sdl2::rect::Rect;

pub struct Health(pub i32);
pub struct Position(pub Vector2<f64>);

pub struct Renderable {
    pub flipped: bool,
    pub rect: Rect,
}

//Controller need to refactor the code away from the player (both the component and the system)
//Colliders