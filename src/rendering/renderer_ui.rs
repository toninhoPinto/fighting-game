use sdl2::{pixels::Color, rect::Rect, render::{Texture, TextureCreator, WindowCanvas}, ttf::Font, video::WindowContext};

use crate::{asset_management::asset_holders::{ItemAssets, UIAssets}, game_logic::{characters::player::Player, combo_string::manage_combo_resources::Combo}, ui::{ingame::{popup_ui::PopUp, segmented_bar_ui::SegmentedBar, wrapping_list_ui::WrappingList}, menus::button_ui::Button}};

pub fn active_item_ui() -> Rect{
    Rect::new(10, 0 , 64, 64)
}

pub fn text_gen<'a>(value: String, texture_creator: &'a TextureCreator<WindowContext>, font: &Font, color: Color) -> Texture<'a> {

    let title_surface = font
                .render(&value)
                .blended(color)
                .map_err(|e| e.to_string())
                .unwrap();

               

    texture_creator
                .create_texture_from_surface(&title_surface)
                .map_err(|e| e.to_string())
                .unwrap()
}

pub fn render_button<'a> (canvas: &mut WindowCanvas, button: &Button, assets: &UIAssets) {
    canvas.copy(&assets.store_ui_sheet, assets.store_ui_src_rects.get(&button.sprite).unwrap().clone(), button.rect).unwrap();

    if let Some(text) = &button.text {
        canvas.copy(text, None, button.rect.clone()).unwrap();
    }
    
}

pub fn render_combo(canvas: &mut WindowCanvas, combo: &Combo) {
    if let Some((_, tex, outline)) = &combo.curr_combo_texture {
        canvas.copy(outline, None, Rect::new(combo.combo_rect.x()-2, combo.combo_rect.y()-5, combo.combo_rect.width()+2, combo.combo_rect.height()+2)).unwrap();
        canvas.copy(outline, None, Rect::new(combo.combo_rect.x()-2, combo.combo_rect.y()+5, combo.combo_rect.width()+2, combo.combo_rect.height()+2)).unwrap();
        canvas.copy(outline, None, Rect::new(combo.combo_rect.x()+9, combo.combo_rect.y()-5, combo.combo_rect.width()+2, combo.combo_rect.height()+2)).unwrap();
        canvas.copy(outline, None, Rect::new(combo.combo_rect.x()+9, combo.combo_rect.y()+5, combo.combo_rect.width()+2, combo.combo_rect.height()+2)).unwrap();

        canvas.copy(tex, None, combo.combo_rect).unwrap();
    }
}

pub fn render_ui<'a>(canvas: &mut WindowCanvas, 
    player: &Player,
    hp_bars: &SegmentedBar,
    energy_bars: &SegmentedBar,
    item_list: &WrappingList,
    item_assets: &ItemAssets,
    popups: Option<&PopUp>,
    popup_content: &Option<Vec<Texture<'a>>>
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

        if energy_bars.curr_value > 0 {
            canvas.set_draw_color(energy_bars.color.unwrap());
            for energy_rect in energy_bars.render() {
                canvas.draw_rect(energy_rect).unwrap();
                canvas.fill_rect(energy_rect).unwrap();
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

        //canvas.copy(texture, None, Rect)

        if let (Some(popups), Some(popup_content)) = (popups, popup_content) {
            if popups.alpha > 0f32 {
                canvas.set_draw_color(Color::RGBA(50, 50, 50, popups.alpha as u8));

                canvas.draw_rect(popups.popup).unwrap();
                canvas.fill_rect(popups.popup).unwrap();

                for i in 0..popup_content.len() {
                    canvas
                        .copy(&popup_content[i], None, popups.contents[i])
                        .unwrap();
                }
            }

        }

        
    }