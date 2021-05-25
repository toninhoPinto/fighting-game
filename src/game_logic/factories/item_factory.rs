use std::{collections::HashMap, fs};

use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{asset_management::{asset_holders::ItemAssets, asset_loader::{asset_loader, my_spritesheet_format::load_spritesheet}}, game_logic::{effects::Effect, items::{Item, ItemType}}};

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[serde(rename = "item_type")]
    pub item_type: String,
    #[serde(rename = "asset_id")]
    pub asset_id: String,
    pub effects: Vec<JsonEffect>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonEffect {
    pub id: i32,
    pub change: Option<i32>,
    #[serde(rename = "add_attack")]
    pub add_attack: Option<String>,
    #[serde(default)]
    pub stat: Vec<String>,
    #[serde(rename = "apply_at_every")]
    pub apply_at_every: Option<i32>,
    pub duration: Option<i32>,
}


pub fn load_items(dir: String) -> HashMap<i32, Item>{
    println!("loading {}", dir);
    let json_string = fs::read_to_string(dir.clone()).unwrap();
    let items = serde_json::from_str::<Vec<Root>>(&json_string).unwrap();
    
    let mut map = HashMap::new();

    for item in items {
        let item_to_add = Item {
            id: item.id,
            name: item.name,
            description: item.description,
            item_type: match &item.item_type as &str {
                "Active" => {ItemType::ActivePart}
                "Combat" => {ItemType::CombatPart}
                "Passive" => {ItemType::PassivePart}
                _ => {ItemType::PassivePart}
            },
            asset_id: item.asset_id,
            effects: item.effects.iter().map(|ef| {make_effect(ef)}).collect::<Vec<Effect>>(),
        };
        map.insert(item.id, item_to_add);
    }
    
    map
}

fn make_effect(json_effect: &JsonEffect) -> Effect {
    Effect {
        duration: json_effect.duration,
        change: json_effect.change,
        add_attack: if let Some(add_attack) = &json_effect.add_attack {Some(add_attack.to_string())} else {None},
        effect_id: json_effect.id,
        stat: if json_effect.stat.len() == 0 {None} else {Some(json_effect.stat.clone())},
        time_elapsed: 0,
        apply_at_every: json_effect.apply_at_every,
    }
}

pub fn load_item_assets(texture_creator: &TextureCreator<WindowContext>) -> ItemAssets {
    let spritesheet = asset_loader::load_texture(&texture_creator, "assets/items/items.png");

    let mapping = load_spritesheet("assets/items/spritesheet_mapping.json".to_string());
    
    ItemAssets {
        spritesheet,
        src_rects: mapping,
    }
}