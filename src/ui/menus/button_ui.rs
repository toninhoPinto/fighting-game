use sdl2::{pixels::Color, rect::{Point, Rect}, render::{Texture, TextureCreator}, ttf::Font, video::WindowContext};

use crate::rendering::renderer_ui::text_gen;

pub struct Button<'a> {
    pub rect: Rect,
    pub is_pressed: bool,
    pub text: Option<Texture<'a>>, //Option this
    pub sprite: String,
    pub pressed_sprite: Option<String>,
    time_elapsed: f64,
}

impl<'a> Button<'a> {
    pub fn new(rect: Rect,
        texture_creator: &'a TextureCreator<WindowContext>, 
        button_tex: String, 
        button_pressed_tex: Option<String>, 
        text: Option<&'a str>, 
        text_color: Color, 
        font: &Font,
    ) -> Self {

        let text_texture = if let Some(text) = text {
            Some(text_gen(text.to_string(), texture_creator, font, text_color))
        } else {
            None
        };

        Self {
            rect,
            is_pressed: false,
            text: text_texture,
            sprite: button_tex,
            pressed_sprite: button_pressed_tex,
            time_elapsed: 0.,
        }
    }

    pub fn press(&mut self) {
        self.is_pressed = true;
    }

    pub fn update_btn_state(&mut self, time_elapsed: f64) {
        if self.is_pressed {
            self.time_elapsed += time_elapsed;
            if self.time_elapsed >= 0.1 {
                self.is_pressed = false;
                self.time_elapsed = 0f64;
            }
        }
        
    }

    pub fn get_curr_sprite(&self) -> String{
        if self.is_pressed {
            return match &self.pressed_sprite {
                Some(pressed) => {pressed.to_string()}
                None => {self.sprite.clone()}
            }
        } else {
            return self.sprite.clone();
        }
    }
}