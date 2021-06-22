use std::{collections::HashMap, fs};

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "normal_table")]
    pub normal_table: Vec<Item>,
    #[serde(rename = "boss_table")]
    pub boss_table: Vec<Item>,
    #[serde(rename = "store_table")]
    pub store_table: Vec<Item>,
    #[serde(rename = "event_table")]
    pub event_table: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub item_id: i64,
    pub rarity: u64,
}

#[derive(Debug, Clone)]
pub struct LootTable {
    pub acc: u64,
    pub items: Vec<Item>,
}


pub fn load_item_table(dir: String) -> HashMap<String, LootTable> {
    println!("load loot tables {}", dir);
    let json_string = fs::read_to_string(dir.clone()).unwrap();
    let item_tables = &serde_json::from_str::<Root>(&json_string).unwrap();
    
    let mut loot_tables = HashMap::new();

    //sort these by rarity descending
    let mut normal_table_items = item_tables.normal_table.clone();
    normal_table_items.sort_by(|a,b| a.rarity.cmp(&b.rarity));

    let mut boss_table_items = item_tables.boss_table.clone();
    boss_table_items.sort_by(|a,b| a.rarity.cmp(&b.rarity));

    let mut store_table_items = item_tables.store_table.clone();
    store_table_items.sort_by(|a,b| a.rarity.cmp(&b.rarity));

    let mut event_table_items = item_tables.event_table.clone();
    event_table_items.sort_by(|a,b| a.rarity.cmp(&b.rarity));


    loot_tables.insert("normal_table".to_string(), LootTable{acc: normal_table_items.iter().map(|i|{i.rarity}).sum(), items: normal_table_items});
    loot_tables.insert("boss_table".to_string(), LootTable{acc: boss_table_items.iter().map(|i|{i.rarity}).sum(), items: boss_table_items});
    loot_tables.insert("store_table".to_string(), LootTable{acc: store_table_items.iter().map(|i|{i.rarity}).sum(), items: store_table_items});
    loot_tables.insert("event_table".to_string(), LootTable{acc: event_table_items.iter().map(|i|{i.rarity}).sum(), items: event_table_items});

    loot_tables
}