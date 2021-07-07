use std::collections::HashMap;

use sdl2::{rect::{Point, Rect}, render::{Texture, TextureQuery, WindowCanvas}};

use crate::{asset_management::asset_holders::{ItemAssets, OverworldAssets, UIAssets}, game_logic::{events::Event, items::Item}, ui::menus::button_ui::Button};

use super::{renderer::world_to_screen, renderer_ui::render_cursor_ui};

pub fn render_event(canvas: &mut WindowCanvas,
    assets: &OverworldAssets,
    ui_assets: &UIAssets,
    item_assets: &ItemAssets,
    items: &HashMap<i32, Item>,
    event: &Event,
    text: &Texture,
    req_text: &Option<Vec<(Texture, String)>>,
    options: &Vec<Button>,
    selected_option: usize,
    ) {

    let (w, h) = canvas.output_size().unwrap();

    let event_canvas = Rect::new(350,50, 600, 700);
    canvas.copy(&assets.backgrounds[0], Rect::new(0,0, 500, 700), event_canvas).unwrap();

    let rect_screen_pos = world_to_screen(Rect::new(0,0, 450, 582), Point::new(0,0), (w, h), None);
    let texture = &assets.portraits.get("portrait").unwrap();
    canvas.copy(texture, Rect::new(0,0, 900, 1165), rect_screen_pos).unwrap();

    let rect_screen_pos = world_to_screen(Rect::new(0,0, 450, 500), Point::new(w as i32-450,0), (w, h), None);
    let texture = &assets.portraits.get(&event.portrait_id).unwrap();
    canvas.copy(texture, Rect::new(0,0, 900, 1000), rect_screen_pos).unwrap();

    canvas.copy(text, None, Rect::new(350 + 50,150, 500, 80)).unwrap();

    if let Some(req_text) = req_text {
        for (i, (text, handle)) in req_text.iter().enumerate(){
            let TextureQuery { width, height, .. } = text.query();
            canvas.copy(&text, None, Rect::new(500,270 + 50 * i as i32, width, height)).unwrap();

            if handle.contains("item_") {
            let item_id = handle.split("item_").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
            let asset_rect = items.get(&item_id).unwrap().asset_id.clone();
            canvas.copy(&item_assets.spritesheet, 
                item_assets.src_rects.get(&asset_rect).unwrap().clone(), 
                Rect::new(500 + width as i32 + 10, 270 + 50 * i as i32, 30, 30)).unwrap();
            }

        }
    }

    
    for (i, btn) in options.iter().enumerate() {
       
        canvas.copy(&ui_assets.store_ui_sheet, 
            ui_assets.store_ui_src_rects.get(&btn.get_curr_sprite()).unwrap().clone(), 
            btn.rect).unwrap();
        
        if i == selected_option {
            render_cursor_ui(canvas, ui_assets, &btn.rect);
        }

        if let Some(text) = &btn.text {

            let btn_pressed_displacement = if btn.is_pressed {3} else {0};

            let button_text_rect = Rect::new(btn.rect.x() + btn.rect.width() as i32 / 4, 
            btn.rect.y() + btn.rect.height() as i32 / 4 + btn_pressed_displacement, 
            btn.rect.width() / 2, 
            btn.rect.height() / 2);
    
            canvas.copy(text, None, button_text_rect).unwrap();

        }

    }
    
}