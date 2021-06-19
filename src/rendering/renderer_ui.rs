use sdl2::{rect::Rect, render::WindowCanvas};

use crate::{asset_management::asset_holders::ItemAssets, game_logic::characters::player::Player, ui::ingame::{segmented_bar_ui::SegmentedBar, wrapping_list_ui::WrappingList}};

pub fn active_item_ui() -> Rect{
    Rect::new(10, 0 , 64, 64)
}

pub fn render_ui(canvas: &mut WindowCanvas, 
    player: &Player,
    hp_bars: &SegmentedBar,
    item_list: &WrappingList,
    item_assets: &ItemAssets,
    ) {

        if let Some(active_item) = &player.active_item_key {
            let src_rect = item_assets.src_rects.get(active_item).unwrap();
            canvas.copy(&item_assets.spritesheet, src_rect.clone(), active_item_ui()).unwrap();
        }
    
        if hp_bars.curr_value > 0 {
            canvas.set_draw_color(hp_bars.color.unwrap());
            for hp_rect in hp_bars.render() {
                canvas.draw_rect(hp_rect).unwrap();
                canvas.fill_rect(hp_rect).unwrap();
            }
        }
    
        let item_list = item_list.render();
        if player.items.len() > 0 {
            for i in 0..player.items.len() {
                let src_rect = item_assets.src_rects.get(&player.items[i]).unwrap();
                let dst_rect = item_list[i];
                canvas.copy(&item_assets.spritesheet, src_rect.clone(), dst_rect).unwrap();
            }
        }

    }