use sdl2::{rect::{Point, Rect}, render::WindowCanvas};

use crate::{asset_management::asset_holders::OverworldAssets, game_logic::events::Event};

use super::renderer::world_to_screen;



pub fn render_event(canvas: &mut WindowCanvas,
    assets: &OverworldAssets,
    event: &Event
    ) {

    let (w, h) = canvas.output_size().unwrap();

    let rect_screen_pos = world_to_screen(Rect::new(0,0, 450, 582), Point::new(0,0), (w, h), None);
    let texture = &assets.portraits.get("portrait").unwrap();
    canvas.copy(texture, Rect::new(0,0, 900, 1165), rect_screen_pos).unwrap();

    let rect_screen_pos = world_to_screen(Rect::new(0,0, 450, 582), Point::new(w as i32-450,0), (w, h), None);
    let texture = &assets.portraits.get(&event.portrait_id).unwrap();
    canvas.copy_ex(texture, Rect::new(0,0, 900, 1165), rect_screen_pos, 0., Point::new(0,0), true, false).unwrap();
}