use std::time::Instant;

use rand::prelude::SmallRng;
use sdl2::{EventPump, event::Event, pixels::Color, rect::Rect, render::{Canvas, Texture, TextureCreator}, ttf::Font, video::{Window, WindowContext}};

use crate::{GameStateData, Transition, asset_management::{sound::audio_player::play_sound}, engine_traits::scene::Scene, game_logic::{effects::hash_effects, factories::{item_factory::load_item_assets, world_factory::load_overworld_assets}, items::Item, store::{StoreUI}}, hp_bar_init, input::{self, input_devices::InputDevices, translated_inputs::TranslatedInput}, item_list_init, overworld::{node::{WorldNode, WorldNodeType}, overworld_generation, overworld_change_connections}, rendering::{renderer_overworld::render_overworld, renderer_store::render_store, renderer_ui::{currency_text_gen, render_ui}}, ui::ingame::popup_ui::{PopUp, new_item_popup, popup_fade}};


use super::{match_scene::{MAX_UPDATES_AVOID_SPIRAL_OF_DEATH, MatchScene}, store_scene::StoreScene};

pub struct OverworldScene {
    pub rect: Rect,
    pub full_conection: bool,
    pub nodes: Vec<WorldNode>,
    pub player_node_pos: usize,
    pub next_node: usize,
    pub connect_to_index: usize,
}

impl OverworldScene {
    pub fn new() -> Self { 
        Self {
            rect: Rect::new(0,0,0,0),
            full_conection: false,
            nodes: Vec::new(),
            player_node_pos: 0,
            next_node: 0,
            connect_to_index: 0,
        }
    } 

    pub fn init(&mut self, (w, h): (u32, u32), full_conection: bool, seeded_rng: &mut SmallRng) {
        let map_area = Rect::new(400, 100, w-800, h-200);
        self.full_conection = full_conection;
        self.rect = map_area;
        self.nodes = overworld_generation(map_area, (5, 6), full_conection, seeded_rng);
    }

    pub fn change_exploration_level(&mut self, rng: &mut SmallRng, full_conection: bool) {
        if self.full_conection != full_conection {
            self.full_conection = full_conection;
            overworld_change_connections(self, rng, full_conection);
        }
    }

        
    fn iterate_over_levels(&mut self, game_state_data: &mut GameStateData) {
        let connecting_to = &self.nodes[self.player_node_pos as usize].connect_to;
        let old_connect_to = self.connect_to_index;
        self.connect_to_index =  (1 + self.connect_to_index) % connecting_to.len();

        if old_connect_to != self.connect_to_index {
            play_sound(game_state_data.general_assets.sound_effects.get("scroll_level").unwrap());
        }
        
        self.next_node = connecting_to
            .iter()
            .map(|&a| {a})
            .collect::<Vec<usize>>()[self.connect_to_index];
    }
}

impl<'a> Scene for OverworldScene {
    
    fn run(
        &mut self,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump,
        input_devices: &mut InputDevices,
        canvas: &mut Canvas<Window>,
    ) -> Transition {

        let (w, h) = canvas.output_size().unwrap();
        let map_area = Rect::new(400, 100, w-800, h-200);

        let assets = load_overworld_assets(&texture_creator);
        let item_assets = load_item_assets(&texture_creator);
        
        //game_state_data.text_cache.insert("currency".to_string(), currency_text_gen(game_state_data.player.as_ref().unwrap(), texture_creator, &game_state_data.general_assets.font));

        let mut popup_item = new_item_popup((w,h));
        let mut popup_content: Option<Vec<Texture>> = None;

        let mut item_list = item_list_init(&game_state_data);

        self.connect_to_index = 0;
        let connecting_to = &self.nodes[self.player_node_pos as usize].connect_to;
        self.next_node = connecting_to
            .iter()
            .map(|&a| {a})
            .collect::<Vec<usize>>()[self.connect_to_index];

        let mut map_events =  game_state_data.player.as_ref().unwrap().events.on_overworld_map.clone();
        for map_event in map_events.iter_mut() {
            (map_event.0)(game_state_data.player.as_mut().unwrap(), self, game_state_data.map_rng.as_mut().unwrap(), &mut map_event.1);
        }
        game_state_data.player.as_mut().unwrap().events.on_overworld_map = map_events;

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
                        if translated_input == TranslatedInput::Vertical(1) {
                            self.iterate_over_levels(game_state_data);
                        }
                    }
                    if !is_pressed {
                        if translated_input == TranslatedInput::Punch {
                            play_sound(game_state_data.general_assets.sound_effects.get("select_level").unwrap());
                            if let WorldNodeType::Level(_) = self.nodes[self.next_node].node_type {
                                self.player_node_pos = self.next_node;
                                game_state_data.curr_level = self.player_node_pos as i32;
                                return Transition::Push(Box::new(MatchScene::new("foxgirl".to_string())));
                            }

                            if let WorldNodeType::Store = self.nodes[self.next_node].node_type {
                                self.player_node_pos = self.next_node;
                                game_state_data.curr_level = self.player_node_pos as i32;
                                return Transition::Push(Box::new(StoreScene{}));
                            }

                            if let WorldNodeType::Event(_) = self.nodes[self.next_node].node_type {
                            }
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
            

            render_overworld(canvas, 
                &assets,
                self.player_node_pos,
                self.next_node,
                &self.nodes, 
                &map_area);

            render_ui(canvas, 
                &game_state_data.player.as_ref().unwrap(),
                &game_state_data.hp_bar.as_ref().unwrap(),
                &game_state_data.energy_bar.as_ref().unwrap(),
                &item_list,
                &item_assets,
                Some(&popup_item),
                &popup_content
                );

            canvas.present();
        }
    }

}