use sdl2::{pixels::Color, rect::{Point, Rect}, render::Texture, surface::Surface, ttf::Font};

pub struct Button<'a> {
    pub rect: Rect,
    pub position: Point,
    pub text: Surface<'a>,
    pub sprite: Option<&'a Texture<'a>>,
}

impl<'a> Button<'a> {
    pub fn new(rect: Rect, position: Point, text: &'a str, text_color: Color, font: &Font) -> Self {
        let text_surface = font
            .render("Campaign")
            .blended(text_color)
            .map_err(|e| e.to_string())
            .unwrap();
        Self {
            rect,
            position,
            text: text_surface,
            sprite: None,
        }
    }
}