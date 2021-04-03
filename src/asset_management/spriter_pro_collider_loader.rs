use parry2d::bounding_volume::AABB;
use parry2d::math::Point as aabbPoint;

use std::fs;

use sdl2::rect::Point;

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseJson {
    pub timeline: Vec<HitboxKeyframes>,
    pub obj_info: Vec<AnimationHitbox>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationHitbox {
    pub h: f64,
    pub name: String,
    pub object_type: String,
    pub w: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HitboxKeyframes {
    pub id: u8,
    pub name: String,
    pub key: Vec<KeyframeHitbox>,
    pub object_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyframeHitbox {
    pub id: u8,
    pub object: ObjectHitbox
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectHitbox {
    pub x: f64,
    pub y: f64
}

pub fn load_hitboxes(file: std::string::String) -> (Vec<AABB>, Vec<Vec<Point>>) {

    let json_string = fs::read_to_string(file).unwrap();
    let v: BaseJson = serde_json::from_str(&json_string).unwrap();
    let timeline = v.timeline;
    let boxes = v.obj_info;

    let mut colliders: Vec<AABB> = Vec::new();
    for j in 0..boxes.len() {
        let min = aabbPoint::new(0.0, 0.0);
        let max = aabbPoint::new(boxes[j].w as f32, boxes[j].h as f32);
        println!("id {} min {} max{}", j, min, max);
        colliders.push(AABB::new(min, max));
    }

    let mut positions: Vec<Vec<Point>> = Vec::new();

    for i in 0.. timeline[0].key.len() {
        let mut positions_per_frame: Vec<Point> = Vec::new();
        for j in 0..timeline.len() {
            positions_per_frame.push(Point::new(timeline[j].key[i].object.x as i32, timeline[j].key[i].object.y as i32));
        }
        positions.push(positions_per_frame);
    }

    (colliders, positions)
}
