use sdl2::{rect::{Point, Rect}, render::{Texture, WindowCanvas}};

use crate::{asset_management::asset_holders::{OverworldAssets, UIAssets}, game_logic::events::Event, ui::menus::button_ui::Button};

use super::{renderer::world_to_screen, renderer_ui::render_cursor_ui};

pub fn render_event(canvas: &mut WindowCanvas,
    assets: &OverworldAssets,
    ui_assets: &UIAssets,
    event: &Event,
    text: &Texture,
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

    
    for (i, btn) in options.iter().enumerate() {
        if btn.is_pressed{
            println!("curr butn sprite {}", &btn.get_curr_sprite());
        }
        
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