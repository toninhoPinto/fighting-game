use sdl2::{pixels::Color, rect::{Point, Rect}, render::{Texture, TextureCreator}, ttf::Font, video::WindowContext};

pub struct Button<'a> {
    pub rect: Rect,
    pub is_pressed: bool,
    pub position: Point,
    pub text: Option<Texture<'a>>, //Option this
    pub sprite: String,
    pub pressed_sprite: Option<String>,
    pub on_press: Box<dyn Fn() -> ()>,
}

impl<'a> Button<'a> {
    pub fn new(rect: Rect, 
        position: Point, 
        texture_creator: &'a TextureCreator<WindowContext>, 
        button_tex: String, 
        text: Option<&'a str>, 
        text_color: Color, 
        font: &Font,
        on_press: Box<dyn Fn() -> ()>,
    ) -> Self {

        let text_texture = if let Some(text) = text {
            let text_surface = font
                .render(text)
                .blended(text_color)
                .map_err(|e| e.to_string())
                .unwrap();

            Some(texture_creator
                .create_texture_from_surface(&text_surface)
                .map_err(|e| e.to_string())
                .unwrap())
        } else {
            None
        };

        Self {
            rect,
            is_pressed: false,
            position,
            text: text_texture,
            sprite: button_tex,
            pressed_sprite: None,
            on_press,
        }
    }
}