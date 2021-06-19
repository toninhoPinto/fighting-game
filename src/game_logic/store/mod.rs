use rand::{Rng, SeedableRng, prelude::SmallRng};
use sdl2::rect::Rect;

use crate::asset_management::rng_tables::LootTable;


pub struct StoreUI {
    pub background: Rect,
    pub item_rects: Vec<Rect>,
    pub items: Vec<i32>,
    pub prices: Vec<Rect>,
    pub store_keeper: Rect,
}

impl StoreUI {
    pub fn new(screen_res: (u32, u32)) -> Self {

        let store_width = (screen_res.0 as f32 * 0.75f32) as u32;
        let store_height = (screen_res.1 as f32 * 0.75f32) as u32;

        let store_x = (screen_res.0 as f32 * 0.1f32) as i32;
        let store_y = (screen_res.1 as f32 * 0.1f32) as i32;

        let item_width: u32 = 32;
        let item_height: u32 = 32;
        let item_rects: Vec<Rect> = (0..4)
            .map(|i| { Rect::new(store_x + (store_width as f32 + 0.1f32) as i32 + ((item_width + 10) * i) as i32, store_y, item_width, item_height)})
            .collect();

        let price_rects: Vec<Rect> = (0..4)
            .map(|i| { Rect::new(store_x + (store_width as f32 + 0.1f32) as i32 + ((item_width + 10) * i) as i32, store_y + item_height as i32 + 10, item_width, item_height)})
            .collect();

        Self {
            background: Rect::new(store_x, store_y, store_width, store_height),
            item_rects,
            items: Vec::new(),
            prices: price_rects,
            store_keeper: Rect::new(store_x + (store_width as f32 * 0.5f32) as i32 - 100, store_y, 200, 325),
        }
    }
}

pub fn get_store_item_list(seed: u64, loot_tables: &LootTable) -> Vec<i64>{
    let mut rng = SmallRng::seed_from_u64(seed);

    let four_random_indexes = vec![rng.gen::<u64>(),rng.gen::<u64>(),rng.gen::<u64>(),rng.gen::<u64>()];

    four_random_indexes.iter().map(|rng_i| { loot_tables.items[*rng_i as usize].item_id }).collect::<Vec<i64>>()
}