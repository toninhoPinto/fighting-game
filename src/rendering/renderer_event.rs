use sdl2::{rect::{Point, Rect}, render::{Texture, WindowCanvas}};

use crate::{asset_management::asset_holders::{OverworldAssets, UIAssets}, game_logic::events::Event};

use super::{renderer::world_to_screen, renderer_ui::render_cursor_ui};

pub fn render_event(canvas: &mut WindowCanvas,
    assets: &OverworldAssets,
    ui_assets: &UIAssets,
    event: &Event,
    text: &Texture,
    options: &Vec<Texture>,
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


    canvas.copy(text, None, Rect::new(350 + 50,150, 400, 30)).unwrap();


    let mut button_rects = Rect::new(event_canvas.x() - 100 + event_canvas.width() as i32/ 2, 400, 200, 50);
    
    for (i, _) in event.options.iter().enumerate() {
        canvas.copy(&ui_assets.store_ui_sheet, ui_assets.store_ui_src_rects.get("grey_button").unwrap().clone(), button_rects).unwrap();

        let button_text_rect = Rect::new(button_rects.x() + button_rects.width() as i32 / 4, 
            button_rects.y() + button_rects.height() as i32 / 4, 
            button_rects.width() / 2, 
            button_rects.height() / 2);
        
        if i == selected_option {
            render_cursor_ui(canvas, ui_assets, &button_rects);
        }

        canvas.copy(&options[i], None, button_text_rect).unwrap();

        button_rects.set_y(button_rects.y() + 100);
    }
    
}