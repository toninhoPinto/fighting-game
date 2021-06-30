use sdl2::rect::Point;

use crate::ui::menus::nav_button::NavButton;
pub struct VerticalList {
    pub position: Point,
    pub buttons: Vec<NavButton>,
}


impl<'a> VerticalList {

    pub fn new(position: Point, mut buttons: Vec<NavButton>, offset: i32) -> Self {
        VerticalList::init(offset, &mut buttons);
        Self {
            position,
            buttons,
        } 
    }

    fn init(offset: i32, buttons: &mut Vec<NavButton>) {
        for i in 0..buttons.len() {
            buttons[i].ui.rect.y += offset + buttons[i].ui.rect.height() as i32;
        }
    }
}