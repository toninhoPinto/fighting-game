use parry2d::na::Vector2;


#[derive(Debug, Clone)]
pub struct Transform {
    pub pos: Vector2<f64>,
    pub scale: (f32, f32),
}
