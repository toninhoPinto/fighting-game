use parry2d::na::Vector2;

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
    pub asset_id: String,
    pub effects: Vec<Effect>,
}

pub struct ItemGround {
    pub position: Vector2<f64>,
    pub item: Item
}

pub trait Pickup {
    fn grab(&mut self);
}
