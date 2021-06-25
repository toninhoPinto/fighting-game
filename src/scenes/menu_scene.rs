use std::rc::Rc;

use crate::{GameStateData, Transition, game_logic::{factories::{character_factory::{load_character, load_character_animations}, enemy_factory::load_enemy_ryu_animations}, items::loot_table_effects::stop_attack_spawn}, input::{self, input_devices::InputDevices, translated_inputs::TranslatedInput}, rendering::renderer_ui::currency_text_gen};
use rand::{Rng, SeedableRng, prelude::SmallRng};
use sdl2::{EventPump, event::Event, pixels::Color, rect::{Point, Rect}, render::{Canvas, TextureCreator, TextureQuery}, surface::Surface, ttf::Font, video::{Window, WindowContext}};

//character select
use crate::engine_traits::scene::Scene;

use super::overworld_scene::OverworldScene;

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
        game_state_data: & mut GameStateData,
        texture_creator: & TextureCreator<WindowContext>,
        event_pump: &mut EventPump,
        input_devices: &mut InputDevices,
        canvas: &mut Canvas<Window>,
    ) -> Transition {
        let mut offset = 0;
        let mut text_buttons = Vec::new();

        let screen_res = canvas.output_size().unwrap();

        for text in self.text.iter() {
            let texture = texture_creator
                .create_texture_from_surface(&text)
                .map_err(|e| e.to_string())
                .unwrap();

            let TextureQuery { width, height, .. } = texture.query();
            let target = MenuScene::get_centered_rect(screen_res, width / 2, height / 2, offset);
            text_buttons.push((texture, target));
            offset += 80;
        }

        loop {
            //receive inputs for managing selecting menu options
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Transition::Quit,
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

                                game_state_data.enemy_animations.insert("player".to_string(), Rc::new(load_character_animations("foxgirl")));
                                game_state_data.player = Some(load_character(
                                    "foxgirl",
                                    Point::new(200, 50),
                                    1,
                                    Rc::clone(game_state_data.enemy_animations.get("player").unwrap())
                                ));

                                game_state_data.enemy_animations.insert("ryu".to_string(), Rc::new(load_enemy_ryu_animations()));

                                let mut overworld = OverworldScene::new();
                                
                                let seed = 1234567898761;
                                
                                game_state_data.seed = Some(seed);
                                game_state_data.map_rng = Some(SmallRng::seed_from_u64(seed));
                                overworld.init(screen_res, false, game_state_data.map_rng.as_mut().unwrap());
                                
                                stop_attack_spawn(vec![4,5,6,7,8,9,10,11,12,15], 0, &game_state_data.player.as_ref().unwrap().character, &mut game_state_data.general_assets.loot_tables);

                                println!("loot table {:?}", game_state_data.general_assets.loot_tables.get("normal_table").unwrap().items);
                                return Transition::Change(Box::new(overworld));
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
