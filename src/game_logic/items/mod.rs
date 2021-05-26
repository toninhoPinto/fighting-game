use parry2d::na::Vector2;
use sdl2::{rect::{Point, Rect}, render::Texture};

use crate::asset_management::asset_holders::ItemAssets;

use super::effects::Effect;

pub mod item_effects;

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
    pub effects: Vec<Effect>,
}

#[derive(Clone)]
pub struct ItemGround {
    pub position: Vector2<f64>,
    pub item: Item
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
