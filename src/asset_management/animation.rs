use std::collections::HashMap;
use parry2d::na::Vector2;
use super::{cast_point::CastPoint, collider::Collider};
use sdl2::rect::Point;

#[derive(Clone)]
pub enum AnimationState {
    Startup,
    Active,
    Recovery,
}

#[derive(Clone)]
pub struct Animation {
    pub name: String,
    pub length: i64,
    pub sprites: Vec<(i64, String)>,
    pub offsets: Option<Vec<Vector2<f64>>>,
    pub cast_point: HashMap<i64, CastPoint>,
    pub collider_animation: Option<ColliderAnimation>,
}

#[derive(Clone, Debug)]
pub struct ColliderAnimation {
    pub colliders: Vec<Collider>,
    //Collider name -> transformations per frame
    pub pos_animations: HashMap<String, HashMap<i32, Transformation>>,
}

#[derive(Debug, Clone)]
pub struct Transformation {
    pub pos: Point,
    pub scale: (f32, f32),
}

impl Animation {
    pub fn new(sprites: Vec<(i64, String)>, length: i64, name: String, offsets: Option<Vec<Vector2<f64>>>) -> Self {
        Self {
            name,
            length,
            sprites,
            offsets,
            cast_point: HashMap::new(),
            collider_animation: None,
        }
    }

    pub fn new_with_data(sprites: Vec<(i64, String)>, length: i64, name: String, offsets: Option<Vec<Vector2<f64>>>, cast_point: HashMap<i64, CastPoint>, collider_animation: Option<ColliderAnimation>) -> Self {
        Self {
            name,
            length,
            sprites,
            offsets,
            cast_point,
            collider_animation,
        }
    }
}


