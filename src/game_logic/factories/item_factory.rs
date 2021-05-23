use std::{collections::HashMap, fs};

use crate::game_logic::{effects::Effect, items::{Item, ItemType}};

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[serde(rename = "item_type")]
    pub item_type: String,
    #[serde(rename = "asset_id")]
    pub asset_id: i32,
    pub effects: Vec<JsonEffect>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonEffect {
    pub id: i32,
    pub duration: Option<i32>,
    #[serde(rename = "add_attack")]
    pub add_attack: Option<String>,
    pub change: Option<i32>,
}

fn make_effect(json_effect: &JsonEffect) -> Effect {
    Effect {
        duration: json_effect.duration,
        change: json_effect.change,
        add_attack: if let Some(add_attack) = &json_effect.add_attack {Some(add_attack.to_string())} else {None} ,
        effect_id: json_effect.id,
        time_elapsed: 0,
    }
}

pub fn load_items(dir: String) -> HashMap<i32, Item>{
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