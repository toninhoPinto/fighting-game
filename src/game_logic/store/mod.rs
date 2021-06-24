use rand::{Rng, SeedableRng, prelude::SmallRng};
use sdl2::rect::Rect;

use crate::{asset_management::rng_tables::LootTable, game_logic::items::get_random_item};


pub struct StoreUI {
    pub background: Rect,
    pub selected_item: usize,
    pub item_rects: Vec<Rect>,
    pub items: Vec<i64>,
    pub prices: Vec<Rect>,
    pub store_keeper: Rect,
    pub back_button: Rect,
}

impl StoreUI {
    pub fn new(screen_res: (u32, u32)) -> Self {

        let store_width = (screen_res.0 as f32 * 0.85f32) as u32;
        let store_height = (screen_res.1 as f32 * 0.85f32) as u32;

        let store_x = (screen_res.0 as f32 * 0.1f32) as i32;
        let store_y = (screen_res.1 as f32 * 0.1f32) as i32;

        let item_width: u32 = 64;
        let item_height: u32 = 64;

        let item_between_space = 70;

        let item_y_start = 100;

        let item_rects: Vec<Rect> = (0..4)
            .map(|i| { 
                Rect::new(store_x + (store_width as f32 * 0.6f32) as i32 + ((item_width + item_between_space) * (i % 2)) as i32, 
                store_y + item_y_start + item_height as i32 + ((item_width + item_between_space) * (i / 2)) as i32, 
                item_width, 
                item_height)})
            .collect();

        let price_offset_x = item_width / 2 - 8;
        let price_offset_y = 70;
        let price_rects: Vec<Rect> = (0..4)
            .map(|i| { Rect::new(store_x + (store_width as f32 * 0.6f32) as i32 + ((item_width + item_between_space) * (i % 2)) as i32 + price_offset_x as i32, 
                store_y + item_y_start + item_height as i32 + ((item_width + item_between_space) * (i / 2)) as i32 + price_offset_y, 
                16, 
                16)})
            .collect();

        let shoop_keeper_width = 500;
        let shoop_keeper_height = 650;
        Self {
            background: Rect::new(store_x, store_y, store_width, store_height),
            selected_item: 0,
            item_rects,
            items: Vec::new(),
            prices: price_rects,
            store_keeper: Rect::new(store_x - 120, store_y, shoop_keeper_width as u32, shoop_keeper_height),
            back_button: Rect::new(store_x + (store_width as f32 * 0.6f32) as i32, store_y + (store_height as f32 * 0.8f32) as i32, 128, 64)
        }
    }


}

pub fn get_store_item_list(seed: u64, loot_table: &LootTable) -> Vec<i64>{
    let mut rng = SmallRng::seed_from_u64(seed);

    let mut store_clone = (*loot_table).clone();

    let mut four_random_indexes: Vec<i64> = Vec::new();

    let random_item = get_random_item(&store_clone, &mut rng);
    four_random_indexes.push(random_item);
    store_clone.items.retain(|i| {i.item_id != random_item});

    let random_item = get_random_item(&store_clone, &mut rng);
    four_random_indexes.push(random_item);
    store_clone.items.retain(|i| {i.item_id != random_item});

    let random_item = get_random_item(&store_clone, &mut rng);
    four_random_indexes.push(random_item);
    store_clone.items.retain(|i| {i.item_id != random_item});

    let random_item = get_random_item(&store_clone, &mut rng);
    four_random_indexes.push(random_item);
    store_clone.items.retain(|i| {i.item_id != random_item});


    four_random_indexes
}

