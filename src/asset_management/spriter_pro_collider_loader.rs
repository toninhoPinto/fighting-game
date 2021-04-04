use parry2d::bounding_volume::AABB;
use parry2d::math::Point as aabbPoint;

use std::{collections::HashMap, fs};

use sdl2::rect::Point;

use super::{collider::{Collider, ColliderType}, transformation::Transformation};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseJson {
    pub animation: Vec<Animation>,
    pub obj_info: Vec<ObjectInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    pub mainline: Mainline,
    pub timeline: Vec<HitboxKeyframes>,
    pub interval: i32,
    pub length: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mainline {
    pub key: Vec<MainlineKey>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainlineKey {
    pub id: u8,
    pub time: Option<i32>,
    pub object_ref: Vec<ObjectRef>

}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectRef {
    pub id: u8,
    pub key: u8,
    pub timeline: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectInfo {
    pub h: f64,
    pub name: String,
    #[serde(rename = "type")] 
    pub object_type: String,
    pub w: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HitboxKeyframes {
    pub id: u8,
    pub name: String,
    pub key: Vec<KeyframeHitbox>,
    pub obj: u8,
    pub object_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyframeHitbox {
    pub id: u8,
    pub object: ObjectHitbox,
    pub time: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectHitbox {
    pub x: f64,
    pub y: f64,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
}

pub fn load_hitboxes(file: std::string::String) -> (Vec<Collider>, HashMap<String, HashMap<i32, Transformation>>) {

    let json_string = fs::read_to_string(file).unwrap();
    let v = serde_json::from_str::<BaseJson>(&json_string).unwrap();
    let timeline = &v.animation[0].timeline;
    let mainline = &v.animation[0].mainline;
    let boxes = v.obj_info;

    let mut colliders: Vec<Collider> = Vec::new();
    for j in 0..boxes.len() {
        let min = aabbPoint::new(0.0, 0.0);
        let max = aabbPoint::new(boxes[j].w as f32, boxes[j].h as f32);

        let collider_type = if boxes[j].name.contains("hit") {
            ColliderType::Hitbox
        } else if boxes[j].name.contains("push")  {
            ColliderType::Pushbox
        } else {
            ColliderType::Hurtbox
        };

        let collider = Collider{
            aabb: AABB::new(min, max),
            collider_type: collider_type,
            name: boxes[j].name.clone(),
        };
        colliders.push(collider);
    }

    

    let mut time_keys = HashMap::new();
    for i in 0..mainline.key.len() {
        let time = if mainline.key[i].time.is_some() {mainline.key[i].time.unwrap()} else {0};
        time_keys.insert(time, i);
    }

    // for string - name of collider object -- hold a map of frame animation id and position at that frame
    let mut final_positions: HashMap<String, HashMap<i32, Transformation>> = HashMap::new();
    for i in 0..timeline.len() { //for each  collider object
        let name = timeline[i].name.clone();
        let mut positions: HashMap<i32, Transformation> = HashMap::new();

        for j in 0..timeline[i].key.len() { //for each frame of the specific object
            let key_time = &timeline[i].key[j];
            let time = if key_time.time.is_some() {key_time.time.unwrap()} else {0};

            let scale_x = if timeline[i].key[j].object.scale_x.is_some() {timeline[i].key[j].object.scale_x.unwrap()} else {1.0};
            let scale_y = if timeline[i].key[j].object.scale_y.is_some() {timeline[i].key[j].object.scale_y.unwrap().abs()} else {1.0};
            let transformation_frame = Transformation {
                pos: Point::new(timeline[i].key[j].object.x as i32, timeline[i].key[j].object.y as i32),
                scale: (scale_x as f32, scale_y as f32),
            };
            if time_keys.contains_key(&time) {
                positions.insert(*time_keys.get(&time).unwrap() as i32, transformation_frame);
            }
        }

        final_positions.insert(name, positions);
    }

    (colliders, final_positions)
}
