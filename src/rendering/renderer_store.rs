use std::collections::HashMap;

use sdl2::{pixels::Color, rect::{Point, Rect}, render::{Texture, TextureQuery, WindowCanvas}};

use crate::{asset_management::asset_holders::{ItemAssets, OverworldAssets}, game_logic::{items::Item, store::StoreUI}};


pub fn render_store<'a>(canvas: &mut WindowCanvas, 
    assets: &OverworldAssets, 
    ui_assets: &HashMap<String, Texture<'a>>,
    store: &StoreUI,
    item_assets: &ItemAssets,
    items: &HashMap<i32, Item>,
    store_item_prices: &Option<Vec<Texture>>) {

    canvas.copy(&assets.backgrounds[0], Rect::new(0,0, store.background.width(), store.background.height()), store.background).unwrap();

    let items_ui = store.items.iter()
        .zip(store.item_rects.iter());
    let bought_item = Rect::new(352, 0, 32, 32);

    if let Some(store_item_prices) = store_item_prices {
        let prices_render = store_item_prices.iter().zip(store.prices.iter());
        for (text, rect) in prices_render {
            canvas.copy(text, None, rect.clone()).unwrap();
        }
    }

    for (item_id, item_rect) in items_ui {
        let src_rect = if *item_id >= 0 {
            item_assets.src_rects.get(&items.get(&(*item_id as i32)).unwrap().asset_id).unwrap()
        } else {
            &bought_item
        };
        let dst_rect = item_rect;
        canvas.copy(&item_assets.spritesheet, src_rect.clone(), dst_rect.clone()).unwrap();
    }

    let shoop_keeper = &assets.portraits.get("shop_keeper").unwrap();
    let TextureQuery { width, height, .. } = shoop_keeper.query();

    let scaler = 0.9f32;
    let shop_keeper = Rect::new(store.store_keeper.x(), store.store_keeper.y(), (width as f32 * scaler) as u32, (height as f32 * scaler) as u32);
    canvas.copy(shoop_keeper, Rect::new(0,0, width, height), shop_keeper).unwrap();

    let src_pointer = assets.src_rects.get("square").unwrap();

    let mut selected_item: Rect;
    if store.selected_item < store.item_rects.len() {
        selected_item = store.item_rects[store.selected_item];
    } else {
        selected_item = store.back_button;
    }
    selected_item.x = selected_item.x + 50;
    canvas.copy_ex(&assets.spritesheet, src_pointer.clone(), selected_item, 90f64, Point::new(0,0), false, false).unwrap();

    if let Some(store_item_prices) = store_item_prices {
        for _ in store_item_prices {
            canvas.copy_ex(&assets.spritesheet, src_pointer.clone(), selected_item, 90f64, Point::new(0,0), false, false).unwrap();
        }
    }

    canvas.set_draw_color(Color::RGB(200, 50, 50));
    canvas.draw_rect(store.back_button).unwrap();
    canvas.fill_rect(store.back_button).unwrap();

    let rect = Rect::new(store.back_button.x() + (store.back_button.width() / 4) as i32, store.back_button.y() + (store.back_button.height() / 4) as i32, store.back_button.width() / 2,store.back_button.height() / 2);
    canvas.copy(ui_assets.get("back").unwrap(), None, rect).unwrap();
}