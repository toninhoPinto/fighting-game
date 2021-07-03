use std::{cmp, time::Instant};

use sdl2::{EventPump, event::Event, pixels::Color, render::{Canvas, Texture, TextureCreator}, video::{Window, WindowContext}};

use crate::{GameStateData, Transition, engine_traits::scene::Scene, game_logic::{factories::{item_factory::{load_item_assets, load_items}, world_factory::load_overworld_assets}, items::Item, store::{StoreUI, get_store_item_list}}, hp_bar_init, input::{self, input_devices::InputDevices, translated_inputs::TranslatedInput}, item_list_init, overworld::{node::{WorldNode, WorldNodeType}, overworld_generation, overworld_change_connections}, rendering::{renderer_event::render_event, renderer_overworld::render_overworld, renderer_store::render_store, renderer_ui::render_ui}, ui::ingame::popup_ui::{PopUp, new_item_popup, popup_fade}};

use super::match_scene::{MAX_UPDATES_AVOID_SPIRAL_OF_DEATH};

pub struct EventScene {
    pub event_id: u32,
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

        let event = game_state_data.events.get(&self.event_id).unwrap();

        let assets = load_overworld_assets(&texture_creator);

        let mut previous_time = Instant::now();
        let logic_timestep: f64 = 0.016;
        let mut logic_time_accumulated: f64 = 0.0;
        let mut update_counter = 0;

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
                    if is_pressed {
                        if let TranslatedInput::Horizontal(x) = translated_input {
                        }
                    }

                    if !is_pressed {
                        if translated_input == TranslatedInput::Punch {
    
                        }
                    }
                }
                //end of input management
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

                popup_fade(&mut popup_item, &mut popup_content, logic_timestep);

                logic_time_accumulated -= logic_timestep;
            }

            //render
            canvas.set_draw_color(Color::RGB(237, 158, 80));

            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 50));
            
            render_event(canvas, &assets, &event);

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