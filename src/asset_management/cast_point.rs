use parry2d::na::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct CastPoint {
    pub frame: i32,
    pub point: Vector2<f64>
}