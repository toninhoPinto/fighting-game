use sdl2::{rect::{Point, Rect}, render::Texture, ttf::Font};
use sdl2::pixels::Color;

use crate::ui::menus::button_ui::Button;

use super::vertical_list_ui::VerticalList;

pub struct EndMatch<'a> {
    pub rect: Rect,
    pub position: Point,
    pub vertical_list_ui: VerticalList<'a>,
    pub sprite: Option<&'a Texture<'a>>,
} 

impl EndMatch<'_> {

    pub fn new(rect: Rect, position: Point, font: &Font) -> Self {
        let text_color = Color::RGB(0, 0, 0);
        let rematch = Button::new(Rect::new(0, 0, 50, 50), Point::new(0, 0), "rematch", text_color, font);
        let exit_to_main_menu = Button::new(Rect::new(0, 0, 50, 50), Point::new(0, 0), "rematch", text_color, font);
        let exit_to_character_menu = Button::new(Rect::new(0, 0, 50, 50), Point::new(0, 0), "rematch", text_color, font);
        let bttns = VerticalList::new(Point::new(0, 0), vec![rematch, exit_to_character_menu, exit_to_main_menu], 50);

        Self {
            rect,
            position,
            vertical_list_ui: bttns,
            sprite: None,
        } 
    }
}