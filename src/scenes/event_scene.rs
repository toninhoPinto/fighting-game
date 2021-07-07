use std::{cmp, time::Instant};

use sdl2::{EventPump, event::Event, pixels::Color, rect::Rect, render::{Canvas, Texture, TextureCreator}, video::{Window, WindowContext}};

use crate::{GameStateData, Transition, engine_traits::scene::Scene, game_logic::{events::EventType, factories::{item_factory::{load_item_assets, load_items}, world_factory::load_overworld_assets}, items::Item, store::{StoreUI, get_store_item_list}}, hp_bar_init, input::{self, input_devices::InputDevices, translated_inputs::TranslatedInput}, item_list_init, overworld::{node::{WorldNode, WorldNodeType}, overworld_generation, overworld_change_connections}, rendering::{renderer_event::render_event, renderer_overworld::render_overworld, renderer_store::render_store, renderer_ui::{render_ui, text_gen, text_gen_wrapped}}, ui::{ingame::popup_ui::{PopUp, new_item_popup, popup_fade}, menus::button_ui::Button}};

use super::match_scene::{MAX_UPDATES_AVOID_SPIRAL_OF_DEATH, MatchScene};

use crate::game_logic::events::Event as Level_Event;

pub struct EventScene {
    pub event_id: u32,

    pub has_refused: bool,
    pub has_succeeded: bool,
    pub has_failed: bool,
}

impl EventScene {

    pub fn new(id: u32) -> Self {
        Self {
            event_id: id,

            has_refused: false,
            has_succeeded: false,
            has_failed: false,
        }
    }

    pub fn accept_btn_trade(&mut self, event_id: u32, game_state_data: &mut GameStateData) -> Option<Transition>{
        let event =  game_state_data.events.get(&event_id).unwrap();
        game_state_data.player.as_mut().unwrap().hp.0 -= event.cost.as_ref().unwrap().health;
        game_state_data.player.as_mut().unwrap().currency -= event.cost.as_ref().unwrap().currency as u32;
        self.has_succeeded = true;
        return None;
    }

    pub fn accept_btn(&mut self, _: u32, _: &mut GameStateData) -> Option<Transition> {
        let scene = Box::new(MatchScene::new("foxgirl".to_string()));
        return Some(Transition::Push(scene));
    }

    pub fn refuse_btn(&mut self, _: u32, _: &mut GameStateData) -> Option<Transition> {
        self.has_refused = true;
        return None;
    }

    pub fn continue_btn(&mut self, _: u32, _: &mut GameStateData) -> Option<Transition> {
        return Some(Transition::Pop);
    }

    pub fn init_btn_callbacks(&self,
            event: &Level_Event) 
            -> Vec<fn(&mut EventScene, u32, &mut GameStateData) -> Option<Transition>>  {
        
        println!("type {:?}", event.event_type);
        if event.event_type == EventType::Challenge {
            if self.has_failed || self.has_refused || self.has_succeeded {
                vec![EventScene::continue_btn]
            } else {
                vec![EventScene::accept_btn, EventScene::refuse_btn]
            }
        } else if event.event_type == EventType::TradeOffer {
            if self.has_failed || self.has_refused || self.has_succeeded {
                vec![EventScene::continue_btn]
            } else {
                vec![EventScene::accept_btn_trade, EventScene::refuse_btn]
            }
        } else if event.event_type == EventType::LevelMod {
            vec![EventScene::continue_btn]
        } else {
            vec![EventScene::continue_btn]
        }
        
    }


    pub fn init_buttons<'a>(&self,
        event: &Level_Event,
         texture_creator: &'a TextureCreator<WindowContext>,
         game_state_data: &GameStateData) -> Vec<Button<'a>> {
        
        return if event.event_type == EventType::Challenge {
            if self.has_failed || self.has_refused || self.has_succeeded {
                vec!["Continue"]
            } else {
                vec!["Accept", "Refuse"]
            }
        } else if event.event_type == EventType::TradeOffer {
            if self.has_failed || self.has_refused || self.has_succeeded {
                vec!["Continue"]
            } else {
                vec!["Accept", "Refuse"]
            }
        } else if event.event_type == EventType::LevelMod {
            vec!["Continue"]
        } else {
            vec!["Continue"]
        }.iter().enumerate().map(|(i, option)| {
            let button_rect = Rect::new(
                350 - 100 + 600 as i32/ 2, 
                400 + 100 * i as i32, 
            200, 50);

            Button::new(button_rect,
                texture_creator, 
                "grey_button".to_string(),
                Some("pressed_grey_button".to_string()),
                Some(option), 
                Color::WHITE, 
                game_state_data.general_assets.fonts.get("event_font").unwrap()
            )

        }).collect::<Vec<Button<'_>>>()

    }
}

impl<'a> Scene for EventScene {
    
    fn run(
        &mut self,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump,
        input_devices: &mut InputDevices,
        canvas: &mut Canvas<Window>,
    ) -> Transition {

        let (w, h) = canvas.output_size().unwrap();

        let mut popup_item = new_item_popup((w,h));
        let mut popup_content: Option<Vec<Texture>> = None;

        let item_list = item_list_init(&game_state_data);

        self.event_id = 2;
        let event = game_state_data.events.get(&self.event_id).unwrap();

        let assets = load_overworld_assets(&texture_creator);

        let event_text =  text_gen_wrapped(event.text.clone(), texture_creator, game_state_data.general_assets.fonts.get("event_font").unwrap(), Color::WHITE, 450);
       
        let mut buttons = self.init_buttons(&event, texture_creator, game_state_data);
        let mut button_callbacks = self.init_btn_callbacks(&event);
        
        let mut selected_button: usize = 0;

        let mut previous_time = Instant::now();
        let logic_timestep: f64 = 0.016;
        let mut logic_time_accumulated: f64 = 0.0;
        let mut update_counter = 0;

        loop {
            //receive inputs for managing selecting menu options
            for input_event in event_pump.poll_iter() {
                match input_event {
                    Event::Quit { .. } => return Transition::Quit,
                    _ => {}
                };
                input::controller_handler::handle_new_controller(
                    &input_devices.controller,
                    &input_devices.joystick,
                    &input_event,
                    &mut input_devices.joys,
                );

                //needs also to return which controller/ which player
                let raw_input = input::input_handler::rcv_input(&input_event, &input_devices.controls);

                let mouse_pos = input::handle_mouse_click::rcv_mouse_pos(&input_event);
                let mouse_click = input::handle_mouse_click::rcv_mouse_input(&input_event);

                if let Some(mouse_pos) = mouse_pos {
                    for (i, btn) in buttons.iter_mut().enumerate(){
                        if input::handle_mouse_click::check_mouse_within_rect(mouse_pos, &btn.rect) {
                            selected_button = i;
                        }
                    }
                }

                if let Some((is_click_down, mouse_pos)) = mouse_click {
                    if is_click_down {
                        buttons[selected_button].press();
                    } else {
                        if let Some(transition) = (button_callbacks[selected_button])(self, self.event_id, game_state_data) {
                            return transition;
                        }
                    }
                }


                if raw_input.is_some() {
                    let (_id, translated_input, is_pressed) = raw_input.unwrap();
                    if is_pressed {
                        if let TranslatedInput::Horizontal(x) = translated_input {
                            selected_button = ((selected_button as i32 + (1 * x))  % 2).abs() as usize;
                        } else if let TranslatedInput::Vertical(y) = translated_input {
                            selected_button = ((selected_button as i32 + (1 * y))  % 2).abs() as usize;
                        }
                    }

                    if is_pressed {
                        if translated_input == TranslatedInput::Punch {
                            buttons[selected_button].press();
                        }
                    }

                    if !is_pressed {
                        if translated_input == TranslatedInput::Punch {
                            if let Some(transition) = (button_callbacks[selected_button])(self, self.event_id, game_state_data) {
                                return transition;
                            }
                        }
                    }
                }
                //end of input management
            }

            if (self.has_failed || self.has_refused || self.has_succeeded) && buttons.len() > 1 {
                buttons = self.init_buttons(&game_state_data.events.get(&self.event_id).unwrap(), texture_creator, game_state_data);
                button_callbacks = self.init_btn_callbacks(&game_state_data.events.get(&self.event_id).unwrap());
            }

            let current_time = Instant::now();
            let delta_time = current_time.duration_since(previous_time);
            let delta_time_as_nanos =
                delta_time.as_secs() as f64 + (delta_time.subsec_nanos() as f64 * 1e-9);

            previous_time = current_time;
                
            logic_time_accumulated += delta_time_as_nanos;
            //update
            while logic_time_accumulated >= logic_timestep {
                update_counter += 1;
                if update_counter >= MAX_UPDATES_AVOID_SPIRAL_OF_DEATH {
                    logic_time_accumulated = 0.0;
                }

                for btn in buttons.iter_mut() {
                    btn.update_btn_state(logic_timestep);
                }
                popup_fade(&mut popup_item, &mut popup_content, logic_timestep);

                logic_time_accumulated -= logic_timestep;
            }

            //render
            canvas.set_draw_color(Color::RGB(237, 158, 80));

            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 50));
            
            render_event(
                canvas, 
                &assets, 
                &game_state_data.ui_assets,
                &game_state_data.events.get(&self.event_id).unwrap(),
                &event_text,
                &buttons,
                selected_button
            );

            render_ui(canvas, 
                &game_state_data.player.as_ref().unwrap(),
                &game_state_data.hp_bar.as_ref().unwrap(),
                &game_state_data.energy_bar.as_ref().unwrap(),
                &item_list,
                &game_state_data.item_assets,
                Some(&popup_item),
                &popup_content
                );

            canvas.present();
        }
    }

}