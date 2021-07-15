use std::rc::Rc;

use crate::{GameStateData, Transition, asset_management::asset_loader::asset_loader::load_texture, game_logic::{factories::{character_factory::{load_character, load_character_animations}, enemy_factory::load_enemy_ryu_animations}, items::loot_table_effects::stop_attack_spawn}, input::{self, input_devices::InputDevices, translated_inputs::TranslatedInput}, rendering::renderer_ui::{text_gen, text_gen_wrapped}};
use rand::{Rng, SeedableRng, prelude::SmallRng};
use sdl2::{EventPump, event::Event, pixels::Color, rect::{Point, Rect}, render::{Canvas, TextureCreator, TextureQuery}, surface::Surface, ttf::Font, video::{Window, WindowContext}};

//character select
use crate::engine_traits::scene::Scene;


use crate::rendering::renderer_ui::render_cursor_ui;
use super::overworld_scene::OverworldScene;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MenuScreen {
    MainMenu,
    Settings,
    Credits,
}

pub struct MenuScene {
    pub curr_screen: MenuScreen,
    pub prev_screen: Option<MenuScreen>,
    pub selected_btn: i32,
}


impl MenuScene {
    pub fn new_main_menu() -> Self {
        Self {
            curr_screen: MenuScreen::MainMenu,
            prev_screen: None,
            selected_btn: 0,
        }
    }
}

fn start_game(screen_res: (u32, u32), game_state_data: & mut GameStateData) -> Transition {
    game_state_data.enemy_animations.insert("player".to_string(), Rc::new(load_character_animations("foxgirl")));
    game_state_data.player = Some(load_character(
        "foxgirl",
        Point::new(200, 50),
        1,
        Rc::clone(game_state_data.enemy_animations.get("player").unwrap())
    ));

    let hp_bars = crate::hp_bar_init(
        screen_res,
        game_state_data.player.as_ref().unwrap().character.hp,
        game_state_data.player.as_ref().unwrap().hp.0,
    );

    let energy_bars = crate::energy_bar_init(
        screen_res,
        0,
        0,
    );

    game_state_data.hp_bar = Some(hp_bars);
    game_state_data.energy_bar = Some(energy_bars);

    game_state_data.enemy_animations.insert("ryu".to_string(), Rc::new(load_enemy_ryu_animations()));

    let mut overworld = OverworldScene::new();
    
    let seed = 123234223481761;
    
    game_state_data.seed = Some(seed);
    game_state_data.map_rng = Some(SmallRng::seed_from_u64(seed));
    overworld.init(screen_res, false, game_state_data.map_rng.as_mut().unwrap());
    
    stop_attack_spawn(vec![4,5,6,7,8,9,10,11,12,15], 0, &game_state_data.player.as_ref().unwrap().character, &mut game_state_data.general_assets.loot_tables);

    return Transition::Change(Box::new(overworld));
} 

impl Scene for MenuScene {
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

        let main_menu_background = if self.curr_screen == MenuScreen::MainMenu {
            Some(load_texture(texture_creator, "assets/stages/main_menu.png"))
        } else {
            None
        };

        let btn_text = vec!["New Game", "Load Game", "Settings", "Credits", "Quit"];

        for text in btn_text.iter() {
            let btn_text_texture = text_gen(
                text.to_string(),
                texture_creator, 
                game_state_data.general_assets.fonts.get("main_menu_font").unwrap(), 
                Color::WHITE);


            let TextureQuery { width, height, .. } = btn_text_texture.query();
            let target = Rect::new( 70, (screen_res.1 * 5 / 10) as i32 + offset, width, height);
            text_buttons.push((btn_text_texture, target));
            offset += 35;
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

                let mouse_pos = input::handle_mouse_click::rcv_mouse_pos(&event);
                let mouse_click = input::handle_mouse_click::rcv_mouse_input(&event);

                if let Some(mouse_pos) = mouse_pos {
                    for (i, (_, btn_rect)) in text_buttons.iter().enumerate(){
                        if input::handle_mouse_click::check_mouse_within_rect(mouse_pos, &btn_rect) {
                            self.selected_btn = i as i32;
                        }
                    }
                }

                if let Some((is_click_down, _)) = mouse_click {
                    if !is_click_down {
                        return start_game(screen_res, game_state_data);
                    }
                }

                if raw_input.is_some() {
                    let (_id, translated_input, is_pressed) = raw_input.unwrap();
                   
                    if translated_input == TranslatedInput::Vertical(1) && is_pressed {
                        self.selected_btn = (((self.selected_btn - 1)
                            % btn_text.len() as i32)
                            + btn_text.len() as i32)
                            % btn_text.len() as i32;
                    } else if translated_input == TranslatedInput::Vertical(-1) && is_pressed  {
                        self.selected_btn = (self.selected_btn + 1) % btn_text.len() as i32;
                    
                    } else if translated_input == TranslatedInput::Punch {
                        //confirm
                        if !is_pressed {
                            if self.selected_btn == 0 {
                                return start_game(screen_res, game_state_data);
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

            if let Some(ref main_menu_background) = main_menu_background {
                let TextureQuery { width, height, .. } = main_menu_background.query();
                canvas.copy(main_menu_background, Rect::new(0,0, width, height), Rect::new(0,0, screen_res.0, screen_res.1)).unwrap();
            }
            

            for i in 0..text_buttons.len() {
                if i == (self.selected_btn as usize) {
                    render_cursor_ui(canvas, &game_state_data.ui_assets, &text_buttons[i].1);
                } 
                canvas
                    .copy(&text_buttons[i].0, None, Some(text_buttons[i].1))
                    .unwrap();
            }

            canvas.present();
        }
    }
}
