use sdl2::{rect::{Rect}};

use super::{ButtonUI, button_trait::Button};

pub struct StoreButton {
    pub ui: ButtonUI,
    pub on_press:Box<dyn FnMut()>,
}

impl<'a> StoreButton {
    pub fn new(rect: Rect, 
        button_tex: String, 
        text: Option<String>,
        on_press: Box<dyn FnMut()>,
    ) -> Self {

        Self {
            ui: ButtonUI {
                rect,
                is_pressed: false,
                text: text,
                sprite: button_tex,
                pressed_sprite: None,
            },
            on_press,
        }
    }
}




impl Button for StoreButton {
    fn press_button(&mut self) {
        (self.on_press)();
    }
}