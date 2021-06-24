use std::collections::HashMap;

use parry2d::na::Vector2;
use rand::{Rng, prelude::SmallRng};
use sdl2::{rect::{Point, Rect}, render::Texture};

use crate::asset_management::{asset_holders::ItemAssets, rng_tables::LootTable};

use super::{characters::Character, effects::Effect};

pub mod item_effects;
pub mod loot_table_effects;

#[derive(PartialEq, Clone)]
pub enum ItemType {
    ActivePart,
    CombatPart,
    PassivePart,
}

#[derive(Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub asset_id: String,
    pub price: u32,
    pub effects: Vec<Effect>,
    pub chance_mod: Option<Chance>
}

#[derive(Clone)]
pub struct ItemGround {
    pub position: Vector2<f64>,
    pub item: Item
}

#[derive(Clone)]
pub struct Chance {
    pub modifier: fn(Vec<i32>, i32, &Character, &mut HashMap<String, LootTable>),
    pub item_ids: Vec<i32>,
    pub chance_mod: i32
}

impl ItemGround {
    pub fn render<'a>(&'a mut self, assets: &'a ItemAssets<'a>) -> (&'a Texture<'a>, Rect, Point, bool, i32) {
        let key = &self.item.asset_id;

        let sprite_data = &assets.spritesheet;
        
        let src_rect = assets.src_rects.get(key).unwrap();
        
        let pos_to_render = Point::new(self.position.x as i32, self.position.y as i32 );
        (sprite_data, src_rect.clone(), pos_to_render, false, self.position.y as i32)
    }
}

pub trait Pickup {
    fn grab(&mut self);
}

pub fn get_random_item(loot_table: &LootTable, rng: &mut SmallRng) -> i64 {
    let mut random = (rng.gen::<f64>() * loot_table.acc as f64) as u64;
    
    for item in loot_table.items.iter() {
        if random < item.rarity {
            return item.item_id;
        } else {
            random -= item.rarity;
        }
    }
    return loot_table.items[0].item_id;
}