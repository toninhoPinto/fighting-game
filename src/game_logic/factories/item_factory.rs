use std::{collections::HashMap, fs};

use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{asset_management::{asset_holders::ItemAssets, asset_loader::{asset_loader, my_spritesheet_format::load_spritesheet}, rng_tables::LootTable}, game_logic::{characters::Character, effects::Effect, items::{Chance, Item, ItemType, loot_table_effects::{change_spawn_item, stop_attack_spawn, stop_spawn_item}}}};

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
    pub price: u32,
    pub effects: Vec<JsonEffect>,
    #[serde(rename = "chance_mod")]
    pub chance_mod: Option<ChanceMod>,
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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChanceMod {
    #[serde(rename = "other_items")]
    #[serde(default)]
    pub other_items: Option<Vec<i32>>,
    #[serde(rename = "rarity_mod")]
    pub rarity_mod: Option<i32>,
    #[serde(rename = "fn_id")]
    pub fn_id: Option<i32>,
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
            price: item.price,
            effects: item.effects.iter().map(|ef| {make_effect(ef)}).collect::<Vec<Effect>>(),
            chance_mod: if let Some(chance_mod) = item.chance_mod{ Some(make_chance_modifier(item.id, &chance_mod)) } else {None}
        };
        map.insert(item.id, item_to_add);
    }
    
    map
}

fn make_chance_modifier(item_id: i32, chance_mod: &ChanceMod) -> Chance {
    let item_ids = if let Some(items) = chance_mod.other_items.clone() { items } else {vec![item_id]};
    Chance {
        modifier: match chance_mod.fn_id { 
            Some(0) => {stop_spawn_item},
            Some(1) => {stop_attack_spawn},
            Some(_) => {change_spawn_item},
            None => {stop_spawn_item},
        },
        item_ids,
        chance_mod: if let Some(rarity_mod) = chance_mod.rarity_mod {rarity_mod} else {0}
    }
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