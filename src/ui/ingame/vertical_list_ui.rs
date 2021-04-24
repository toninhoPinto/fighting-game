use sdl2::rect::Point;

use crate::ui::menus::button_ui::Button;
pub struct VerticalList<'a> {
    pub position: Point,
    pub buttons: Vec<Button<'a>>,
}


impl<'a> VerticalList<'a> {

    pub fn new(position: Point, mut buttons: Vec<Button<'a>>, offset: i32) -> Self {
        VerticalList::init(offset, &mut buttons);
        Self {
            position,
            buttons,
        } 
    }

    fn init(offset: i32, buttons: &mut Vec<Button<'a>>) {
        for i in 0..buttons.len() {
            buttons[i].position.y += offset + buttons[i].rect.height() as i32;
        }
    }
}