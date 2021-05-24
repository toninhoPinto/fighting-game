use std::{collections::HashMap, fs};

use sdl2::rect::Rect;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub key: String,
    pub coords: Coords,
    pub size: Size,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub fn load_spritesheet(dir: String) -> HashMap<String, Rect>{
    println!("load sprite_sheet {}", dir);
    let json_string = fs::read_to_string(dir.clone()).unwrap();
    let v = &serde_json::from_str::<Vec<Root>>(&json_string).unwrap();
    
    let mut map = HashMap::new();

    for sprite in v {
        let sprite_rect = Rect::new(sprite.coords.x,sprite.coords.y,sprite.size.width,sprite.size.height);
        map.insert(sprite.key.clone(),sprite_rect);
    }
    
    map
}