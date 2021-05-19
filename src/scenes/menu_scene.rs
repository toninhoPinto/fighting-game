//main menu
//story mode *recommended*
//arcade
//versus
//local
//online
//training mode *recommended only for experts*
//settings
//credits
//quit

use crate::{GameStateData, input::{self, input_devices::InputDevices, translated_inputs::TranslatedInput}};
use sdl2::{
    event::Event,
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator, TextureQuery},
    surface::Surface,
    ttf::Font,
    video::{Window, WindowContext},
    EventPump,
};

//character select
//stage select
use crate::engine_traits::scene::Scene;

use super::{match_scene::Match, overworld_scene::OverworldScene};

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[derive(Copy, Clone, Debug)]
pub enum MenuScreen {
    MainMenu,
    Settings,
    Credits,
}

pub struct MenuScene<'a> {
    pub curr_screen: MenuScreen,
    pub prev_screen: Option<MenuScreen>,
    pub text: Vec<Surface<'a>>,
    pub selected_btn: i32,
}


impl<'a> MenuScene<'a> {
    pub fn new_main_menu(font: &Font) -> Self {
        let color = Color::RGB(200, 70, 70);
        let surface = font
            .render("New Game")
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();

        let surface2 = font
            .render("Load Game")
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();

        let surface6 = font
            .render("Settings")
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();

        let surface7 = font
            .render("Credits")
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();

        let surface8 = font
            .render("Quit")
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();

        Self {
            curr_screen: MenuScreen::MainMenu,
            prev_screen: None,
            text: vec![
                surface, surface2, surface6, surface7, surface8,
            ],
            selected_btn: 0,
        }
    }

    pub fn get_centered_rect(
        _screen_res: (u32, u32),
        rect_width: u32,
        rect_height: u32,
        offset: u32,
    ) -> Rect {
        let cx = 20;
        let cy = 20 + offset;
        rect!(cx, cy, rect_width, rect_height)
    }
}

impl<'a> Scene for MenuScene<'a> {
    fn run(
        &mut self,
        game_state_stack: &mut Vec<Box<dyn Scene>>,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump,
        input_devices: &mut InputDevices,
        canvas: &mut Canvas<Window>,
    ) {
        let mut offset = 0;
        let mut text_buttons = Vec::new();
        for text in self.text.iter() {
            let texture = texture_creator
                .create_texture_from_surface(&text)
                .map_err(|e| e.to_string())
                .unwrap();

            let TextureQuery { width, height, .. } = texture.query();
            let screen_res = canvas.output_size().unwrap();
            let target = MenuScene::get_centered_rect(screen_res, width / 2, height / 2, offset);
            text_buttons.push((texture, target));
            offset += 80;
        }

        loop {
            //receive inputs for managing selecting menu options
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return,
                    _ => {}
                };
                input::controller_handler::handle_new_controller(
                    &input_devices.controller,
                    &input_devices.joystick,
                    &event,
                    &mut input_devices.joys,
                );

                //needs also to return which controller/ which player
                let raw_input = input::input_handler::rcv_input(&event, &input_devices.controls);

                if raw_input.is_some() {
                    let (_id, translated_input, is_pressed) = raw_input.unwrap();
                    if translated_input == TranslatedInput::Vertical(1) && is_pressed {
                        self.selected_btn = (((self.selected_btn - 1)
                            % self.text.len() as i32)
                            + self.text.len() as i32)
                            % self.text.len() as i32;
                    } else if translated_input == TranslatedInput::Vertical(-1) && is_pressed  {
                        self.selected_btn = (self.selected_btn + 1) % self.text.len() as i32;
                    } else if translated_input == TranslatedInput::Punch {
                        //confirm
                        if !is_pressed {
                            if self.selected_btn == 0 {
                                //must leave and make main use match scene instead
                                game_state_stack.push(Box::new(OverworldScene::new()));
                                return;
                            }
                        }
                    } else if translated_input == TranslatedInput::Kick {
                        //go back
                        if self.prev_screen.is_some() {
                            self.curr_screen = self.prev_screen.unwrap();
                        }
                    }
                }
                //end of input management
            }
            //update

            //render
            canvas.set_draw_color(Color::RGB(0, 85, 200));

            canvas.clear();
            for i in 0..text_buttons.len() {
                if i == (self.selected_btn as usize) {
                    text_buttons[i].0.set_color_mod(50, 255, 100);
                } else {
                    text_buttons[i].0.set_color_mod(200, 70, 70);
                }
                canvas
                    .copy(&text_buttons[i].0, None, Some(text_buttons[i].1))
                    .unwrap();
            }

            canvas.present();
        }
    }
}
