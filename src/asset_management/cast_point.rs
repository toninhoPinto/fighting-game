use parry2d::na::Vector2;

#[derive(Clone, Debug)]
pub struct CastPoint {
    pub frame: i64,
    pub point: Vector2<f64>,
    pub name: String,
}