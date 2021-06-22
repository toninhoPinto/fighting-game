use sdl2::{pixels::Color, rect::Rect, render::{Texture, TextureCreator, TextureQuery}, ttf::Font, video::WindowContext};

pub struct PopUp {
    pub popup: Rect, 
    pub contents: Vec<Rect>,
    pub alpha: f32,
}

pub fn new_item_popup(screen_res: (u32, u32)) -> PopUp {

    let popup_width = (screen_res.0 as f32 * 0.25f32) as u32;
    let popup = Rect::new(50, 64, popup_width, 32);

    let title_rect = Rect::new(popup.x() + (popup_width as f32 * 0.5f32)  as i32, popup.y(), 0,popup.height()/2);
    let desc_rect =  Rect::new(popup.x() + 5, popup.y() + title_rect.height() as i32, popup_width, popup.height()/2);

    PopUp{
        popup,
        contents: vec![title_rect, desc_rect],
        alpha: 0f32,
    }
}

pub fn render_popup<'a>(texture_creator: &'a TextureCreator<WindowContext>, title: &String, description: &String, font: &Font, popup: &mut PopUp) -> Vec<Texture<'a>>{
    let title_surface = font
        .render(&title)
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())
        .unwrap();

    let title_texture = texture_creator
                .create_texture_from_surface(&title_surface)
                .map_err(|e| e.to_string())
                .unwrap();

    let TextureQuery { width, .. } = title_texture.query();

    popup.contents[0].set_width(width);
    popup.contents[0].set_x(popup.popup.x() + (popup.popup.width() as f32 * 0.5f32)  as i32 - (width as i32/2));

    let desc_surface = font
    .render(&description)
    .blended(Color::WHITE)
    .map_err(|e| e.to_string())
    .unwrap();

    let desc_texture = texture_creator
            .create_texture_from_surface(&desc_surface)
            .map_err(|e| e.to_string())
            .unwrap();

            
    let TextureQuery { width, .. } = desc_texture.query();

    popup.contents[1].set_width(width);
    popup.alpha = 255f32;

    vec![title_texture, desc_texture]
}

pub fn popup_fade<'a>(popup_item: &mut PopUp, popup_content: &mut Option<Vec<Texture<'a>>>, logic_timestep: f64) {
    if popup_item.alpha > 0f32 {
        let new_alpha = popup_item.alpha - (logic_timestep * 90f64) as f32;
        popup_item.alpha = new_alpha;

        if let Some(ref mut popup_content) = popup_content {
            for i in 0..popup_content.len() {
                popup_content[i].set_alpha_mod(new_alpha as u8);
            }
        } 
    }
}