use std::collections::HashMap;
use parry2d::na::Vector2;
use crate::{asset_management::cast_point::CastPoint, engine_types::transform::Transform};

use super::{collider::Collider};

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
    pub sprite_alignments: HashMap<i32, Transform>,
    pub offsets: Option<Vec<Vector2<f64>>>,
    pub cast_point: HashMap<i64, CastPoint>,
    pub collider_animation: Option<ColliderAnimation>,
}

#[derive(Clone, Debug)]
pub struct ColliderAnimation {
    pub colliders: Vec<Collider>,
    pub pos_animations: HashMap<String, HashMap<i32, Transform>>,     //Collider name -> transformations per frame
}

impl Animation {
    pub fn new(sprites: Vec<(i64, String)>, length: i64, name: String, offsets: Option<Vec<Vector2<f64>>>) -> Self {
        Self {
            name,
            length,
            sprites,
            offsets,
            cast_point: HashMap::new(),
            sprite_alignments: HashMap::new(),
            collider_animation: None,
        }
    }

    pub fn new_with_data(sprites: Vec<(i64, String)>, 
    length: i64,
    name: String, 
    offsets: Option<Vec<Vector2<f64>>>, 
    cast_point: HashMap<i64, CastPoint>,
    sprite_alignments: HashMap<i32, Transform>,
    collider_animation: Option<ColliderAnimation>) -> Self {
        Self {
            name,
            length,
            sprites,
            offsets,
            cast_point,
            sprite_alignments,
            collider_animation,
        }
    }
}


