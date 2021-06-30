use std::cmp;

use rand::{Rng, SeedableRng, prelude::SmallRng};
use sdl2::{rect::Rect, render::Texture};

use crate::{GameStateData, Transition, asset_management::rng_tables::LootTable, game_logic::items::get_random_item, ui::{ingame::{segmented_bar_ui::SegmentedBar, wrapping_list_ui::WrappingList}, menus::{button_trait::Button, nav_button::{NavButton}, store_button::StoreButton}}};

use super::items::Item;


pub struct StoreUI {
    pub background: Rect,
    pub selected_item: usize,
    pub item_buttons: Vec<StoreButton>,
    pub items: Vec<i64>,
    pub prices: Vec<Rect>,
    pub store_keeper: Rect,
    pub back_button: NavButton,
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

        let nav_button =         NavButton::new(
            Rect::new(store_x + (store_width as f32 * 0.6f32) as i32, 
            store_y + (store_height as f32 * 0.8f32) as i32, 
                            128, 64),
            "button".to_string(),
            Some("back".to_string()),
            || {Transition::Pop}
        );

        Self {
            background: Rect::new(store_x, store_y, store_width, store_height),
            selected_item: 0,
            item_buttons: Vec::new(),
            items: Vec::new(),
            prices: price_rects,
            store_keeper: Rect::new(store_x - 120, store_y, shoop_keeper_width as u32, shoop_keeper_height),
            back_button: nav_button,
        }
    }


}

pub fn get_store_item_list(seed: u64, loot_table: &LootTable) -> Vec<i64> {
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

pub fn init_buttons<'a>(store: &mut StoreUI, 
    store_item_prices: &mut Option<Vec<Texture<'a>>>, 
    item_list: &mut WrappingList, 
    game_state_data: &mut GameStateData) {

    let item_width: u32 = 64;
    let item_height: u32 = 64;

    let item_between_space = 70;
    let item_y_start = 100;

    let buttons: Vec<StoreButton> = (0..4)
    .map(|i| { 
        let mut bought_item = game_state_data.items.get(&(store.items[i as usize] as i32)).unwrap().clone();
        
        StoreButton::new(
            Rect::new(store.background.x() + (store.background.width() as f32 * 0.6f32) as i32 + ((item_width + item_between_space) * (i % 2)) as i32, 
            store.background.y() + item_y_start + item_height as i32 + ((item_width + item_between_space) * (i / 2)) as i32, 
            item_width, item_height),
            bought_item.asset_id,
        None,
    Box::new(|| {buy_item_button(store, bought_item, store_item_prices, item_list, game_state_data)}),
        )
    }).collect();

}

pub fn buy_item_button<'a>(store_ui: &mut StoreUI, bought_item: Item, store_item_prices: &mut Option<Vec<Texture<'a>>>, item_list: &mut WrappingList, game_state_data: &mut GameStateData) {
  
    store_ui.items.remove(store_ui.selected_item);
    store_ui.item_buttons.remove(store_ui.selected_item);
    store_ui.prices.remove(store_ui.selected_item);
    if let Some(ref mut store_item_prices) = store_item_prices {
        store_item_prices.remove(store_ui.selected_item);
    }

    let new_selected = if store_ui.selected_item == 0 {store_ui.selected_item} else {store_ui.selected_item-1};
    store_ui.selected_item = cmp::max(0,cmp::min(store_ui.items.len(), new_selected));

    let player = game_state_data.player.as_mut().unwrap();
    player.currency = cmp::max(0, player.currency - bought_item.price);
    player.equip_item(&mut bought_item, &game_state_data.effects, game_state_data.energy_bar.as_mut().unwrap());

    if let Some(chance_mod) = &bought_item.chance_mod {
        (chance_mod.modifier)(chance_mod.item_ids.clone(), chance_mod.chance_mod, &game_state_data.player.as_ref().unwrap().character, &mut game_state_data.general_assets.loot_tables);
    } else {
        for (_key, val) in game_state_data.general_assets.loot_tables.iter_mut() {
            val.items.retain(|x| x.item_id as i32 != bought_item.id);
            val.acc = val.items.iter().map(|i|{i.rarity}).sum();
        }
    }

    if game_state_data.player.as_ref().unwrap().items.len() != item_list.rects.len() {
        item_list.update(game_state_data.player.as_ref().unwrap().items.iter()
            .map(|_| {Rect::new(0,0,32,32)})
            .collect::<Vec<Rect>>()
        );
    }
}