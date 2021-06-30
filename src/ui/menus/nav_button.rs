use sdl2::{rect::{Rect}};

use crate::Transition;

use super::{ButtonUI, button_trait::Button};

pub struct NavButton {
    pub ui: ButtonUI,
    pub on_press: fn() -> Transition,
}

impl NavButton {
    pub fn new(rect: Rect, 
        button_tex: String, 
        text: Option<String>,
        on_press: fn() -> Transition,
    ) -> Self {

        Self {
            ui: ButtonUI{
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

impl Button for NavButton {
    fn press_button(&mut self) {
        (self.on_press)();
    }
}