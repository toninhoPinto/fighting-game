use parry2d::{bounding_volume::AABB, na::Vector2};
use parry2d::math::Point as aabbPoint;

use std::{collections::HashMap, fs};

use sdl2::rect::Point;

use crate::asset_management::{animation::{ColliderAnimation, Transformation}, cast_point::CastPoint, collider::{Collider, ColliderType}, sprite_data::SpriteData};
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub entity: Vec<Entity>,
    pub folder: Vec<Folder>,
    pub generator: String,
    #[serde(rename = "generator_version")]
    pub generator_version: String,
    #[serde(rename = "scon_version")]
    pub scon_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub animation: Vec<Animation>,
    #[serde(rename = "character_map")]
    pub character_map: Vec<::serde_json::Value>,
    pub id: i64,
    pub name: String,
    #[serde(rename = "obj_info")]
    pub obj_info: Vec<ObjInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    pub id: i64,
    pub interval: i64,
    pub length: i64,
    pub mainline: Mainline,
    pub name: String,
    pub timeline: Vec<Timeline>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mainline {
    pub key: Vec<Key>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    #[serde(rename = "bone_ref")]
    pub bone_ref: Vec<::serde_json::Value>,
    pub id: i64,
    #[serde(rename = "object_ref")]
    pub object_ref: Vec<ObjectRef>,
    pub time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectRef {
    pub id: i64,
    pub key: i64,
    pub timeline: String,
    #[serde(rename = "z_index")]
    pub z_index: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeline {
    pub id: i64,
    pub key: Vec<Key2>,
    pub name: String,
    pub obj: Option<i64>,
    #[serde(rename = "object_type")]
    pub object_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key2 {
    pub id: i32,
    pub object: Object,
    pub spin: i64,
    pub time: Option<i64>,
    #[serde(rename = "scale_x")]
    pub scale_x: Option<f64>,
    #[serde(rename = "scale_y")]
    pub scale_y: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub angle: Option<f64>,
    pub file: Option<i64>,
    pub folder: Option<i64>,
    #[serde(rename = "pivot_x")]
    pub pivot_x: Option<f64>,
    #[serde(rename = "pivot_y")]
    pub pivot_y: Option<f64>,
    #[serde(rename = "scale_x")]
    pub scale_x: Option<f64>,
    #[serde(rename = "scale_y")]
    pub scale_y: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjInfo {
    pub h: f64,
    pub name: String,
    #[serde(rename = "pivot_x")]
    pub pivot_x: f64,
    #[serde(rename = "pivot_y")]
    pub pivot_y: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub w: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub file: Vec<File>,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub height: u32,
    pub id: i64,
    pub name: String,
    #[serde(rename = "pivot_x")]
    pub pivot_x: f64,
    #[serde(rename = "pivot_y")]
    pub pivot_y: f64,
    pub width: u32,
}

const FREQUENCY_OF_FPS: i64 = 16;

pub fn load_frame_data(file: std::path::PathBuf) -> Vec<SpriteData> {
    let mut sprites: Vec<SpriteData> = Vec::new();
    
    let json_string = fs::read_to_string(file).unwrap();
    let json = serde_json::from_str::<Root>(&json_string).unwrap();
    
    for file in &json.folder[0].file {
        sprites.push(SpriteData {
                sprite_name: file.name.clone().replace(".png", ""),
                height: file.height,
                width: file.width,
                pivot_x: file.pivot_x,
                pivot_y: file.pivot_y,
            }
        );
    }

    sprites
}


pub fn load_animation_data(
    file: std::path::PathBuf,
) -> (Vec<i64>, ColliderAnimation, HashMap<i32, Transformation>, HashMap<i64, CastPoint>, i64) {

    println!("Loading animation data: {:?}", file);
    let json_string = fs::read_to_string(file).unwrap();
    let v = &serde_json::from_str::<Root>(&json_string).unwrap().entity[0];
    let timeline = &v.animation[0].timeline;
    let mainline = &v.animation[0].mainline;
    let boxes = &v.obj_info;
    let duration = &v.animation[0].length / FREQUENCY_OF_FPS;

    let mut colliders: Vec<Collider> = Vec::new();
    let mut cast_points: HashMap<i64, CastPoint> =  HashMap::new();
    for j in 0..boxes.len() {
        let min = aabbPoint::new(0.0, 0.0);
        let max = aabbPoint::new(boxes[j].w as f32, boxes[j].h as f32);

        let collider_type = if boxes[j].name.contains("hit") {
            ColliderType::Hitbox
        } else if boxes[j].name.contains("push") {
            ColliderType::Pushbox
        } else {
            ColliderType::Hurtbox
        };

        let mut name = boxes[j].name.clone();
        let split_offset = name.find('_').unwrap_or(name.len());

        let collider = Collider {
            aabb: AABB::new(min, max),
            collider_type,
            name: name.drain(..split_offset).collect(),
            enabled: false,
        };
        colliders.push(collider);
    }

    //TODO this is only used for the render to show the pushbox under the other colliders, it does not change the hit detection
    colliders.sort_by(|a, b| a.collider_type.partial_cmp(&b.collider_type).unwrap());

    let mut time_keys = HashMap::new();
    let mut time_vec = Vec::new();
    for i in 0..mainline.key.len() {
        let time = if mainline.key[i].time.is_some() {
            mainline.key[i].time.unwrap()
        } else {
            0
        };
        time_keys.insert(time/FREQUENCY_OF_FPS, i);
        time_vec.push(time/FREQUENCY_OF_FPS);
    }

    let mut sprite_transformations: HashMap<i32, Transformation> = HashMap::new();
    // for string - name of collider object -- hold a map of frame animation id and position at that frame
    let mut final_transformations: HashMap<String, HashMap<i32, Transformation>> = HashMap::new();
    for i in 0..timeline.len() {
        //for each  collider object
        let mut name = timeline[i].name.clone();
        let split_offset = name.find('_').unwrap_or(name.len());
        
        let mut transformations_of_frame: HashMap<i32, Transformation> = HashMap::new();

        match &timeline[i].object_type {
            std::option::Option::Some(obj_type) => {
                if obj_type == "box" {
                    for j in 0..timeline[i].key.len() {
                        //for each frame of the specific object
            
                        let key_time = &timeline[i].key[j];
                        let time = if key_time.time.is_some() {
                            key_time.time.unwrap()
                        } else {
                            0
                        };
            
                        let scale_x = if timeline[i].key[j].object.scale_x.is_some() {
                            timeline[i].key[j].object.scale_x.unwrap().abs()
                        } else {
                            1.0
                        };
                        let scale_y = if timeline[i].key[j].object.scale_y.is_some() {
                            timeline[i].key[j].object.scale_y.unwrap().abs()
                        } else {
                            1.0
                        };

                        let x = if let Some(x) = timeline[i].key[j].object.x {
                            x
                        } else {
                            0f64
                        };
    
                        let y = if let Some(y) = timeline[i].key[j].object.y {
                            y
                        } else {
                            0f64
                        };

                        let transformation_frame = Transformation {
                            pos: Vector2::new(
                                x,
                                y,
                            ),
                            scale: (scale_x as f32, scale_y as f32),
                        };
                        if time_keys.contains_key(&(time / FREQUENCY_OF_FPS)) {
                            transformations_of_frame
                                .insert(*time_keys.get(&(time / FREQUENCY_OF_FPS)).unwrap() as i32, transformation_frame);
                        }
                    }
                } else if obj_type == "point" {
                    for j in 0..timeline[i].key.len() {
                        //for each frame of the specific object
            
                        let key_time = &timeline[i].key[j];
                        let time = if key_time.time.is_some() {
                            key_time.time.unwrap()
                        } else {
                            0
                        };

                        let point = CastPoint {
                            frame: time,
                            name: timeline[i].name.clone(),
                            point: Vector2::new(key_time.object.x.unwrap(), key_time.object.y.unwrap().abs()),
                        };
                        cast_points.insert(time / FREQUENCY_OF_FPS, point);
                    }
                } else {
                    println!("Bones are not supported yet")
                }
            }
            std::option::Option::None => {
                //sprite data here 
                for sprite_frame in &timeline[i].key {

                    let scale_x = if sprite_frame.object.scale_x.is_some() {
                        sprite_frame.object.scale_x.unwrap().abs()
                    } else {
                        1.0
                    };
                    let scale_y = if sprite_frame.object.scale_y.is_some() {
                        sprite_frame.object.scale_y.unwrap().abs()
                    } else {
                        1.0
                    };

                    let x = if let Some(x) = sprite_frame.object.x {
                        x
                    } else {
                        0f64
                    };

                    let y = if let Some(y) = sprite_frame.object.y {
                        y
                    } else {
                        0f64
                    };

                    let transformation_frame = Transformation {
                        pos: Vector2::new(
                            x,
                            y,
                        ),
                        scale: (scale_x as f32, scale_y as f32),
                    };

                    sprite_transformations.insert(sprite_frame.id, transformation_frame);
                }
            }
        }
        final_transformations.insert(name.drain(..split_offset).collect(), transformations_of_frame);
    }

    (time_vec,
    ColliderAnimation {
        colliders: colliders,
        pos_animations: final_transformations,
    },
     sprite_transformations,
     cast_points,
     duration)
}
