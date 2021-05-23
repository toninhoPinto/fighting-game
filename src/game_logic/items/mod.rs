use super::effects::Effect;

pub mod item_effects;

#[derive(PartialEq)]
pub enum ItemType {
    ActivePart,
    CombatPart,
    PassivePart,
}

pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub asset_id: i32,
    pub effects: Vec<Effect>,
}

