use std::collections::HashMap;

use sdl2::{rect::Rect, render::{TextureQuery, WindowCanvas}};

use crate::{asset_management::asset_holders::{ItemAssets, OverworldAssets}, game_logic::{items::Item, store::StoreUI}};


pub fn render_store(canvas: &mut WindowCanvas, 
    assets: &OverworldAssets, 
    store: &StoreUI, 
    item_assets: &ItemAssets,
    items: &HashMap<i32, Item>) {

    canvas.copy(&assets.backgrounds[0], Rect::new(0,0,store.background.width(), store.background.height()), store.background).unwrap();

    let items_ui = store.items.iter().zip(store.item_rects.iter());

    for (item_id, item_rect) in items_ui {
        let src_rect = item_assets.src_rects.get(&items.get(item_id).unwrap().asset_id).unwrap();
        let dst_rect = item_rect;
        canvas.copy(&item_assets.spritesheet, src_rect.clone(), dst_rect.clone()).unwrap();
    }

    let shoop_keeper = &assets.portraits.get("shop_keeper").unwrap();
    let TextureQuery { width, height, .. } = shoop_keeper.query();

    canvas.copy(shoop_keeper, Rect::new(0,0, width, height), store.store_keeper).unwrap();


}