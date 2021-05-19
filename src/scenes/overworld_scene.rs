

use std::collections::HashMap;

use sdl2::{EventPump, event::Event, pixels::Color, rect::Rect, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};

use crate::{GameStateData, engine_traits::scene::Scene, input::{self, input_devices::InputDevices, translated_inputs::TranslatedInput}, overworld::{node::{WorldNode, WorldNodeType}, overworld_generation}, rendering::renderer::{pos_world_to_screen, world_to_screen}};

use super::match_scene::Match;

pub struct OverworldScene {
    pub nodes: Vec<WorldNode>,
    pub player_node_pos: i32,
    pub next_node: usize,
}

impl OverworldScene{
    pub fn new() -> Self { 
        Self {
            nodes: Vec::new(),
            player_node_pos: 0,
            next_node: 0,
        }
    } 
}

impl<'a> Scene for OverworldScene {
    fn run(
        &mut self,
        game_state_stack: &mut Vec<Box<dyn Scene>>,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump,
        input_devices: &mut InputDevices,
        canvas: &mut Canvas<Window>,
    ) {

        let (w, h) = canvas.output_size().unwrap();
        let map_area = Rect::new(200, 100, w-400, h-200);
        self.nodes = overworld_generation(map_area, (5, 6), false);

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
                    if !is_pressed {
                        if translated_input == TranslatedInput::Punch {
                            //must leave and make main use match scene instead
                            game_state_stack.push(Box::new(Match::new(
                                "foxgirl".to_string(),
                            )));
                            return;
                        }
                    }
                }
                //end of input management
            }
            //update

            //render
            canvas.set_draw_color(Color::RGB(0, 85, 200));

            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 50));
            for node in self.nodes.iter() {
                for &connections in node.connect_to.iter() {
                    let origin_point = pos_world_to_screen(node.position, (w, h), None);
                    let destination_point =  pos_world_to_screen(self.nodes[connections as usize].position, (w, h), None);
                    canvas.draw_line(origin_point, destination_point).unwrap();
                }
            }

            let rect_screen_pos = world_to_screen(
                Rect::new(0,0, map_area.width(), map_area.height()), 
                map_area.top_left(), (w, h), None);
            canvas.set_draw_color(Color::RGBA(100, 50, 50, 50));
            canvas.draw_rect(rect_screen_pos).unwrap();
            canvas.fill_rect(rect_screen_pos).unwrap();

            for i in 0..self.nodes.len() {
                if self.nodes[i].node_type == WorldNodeType::Level {
                    canvas.set_draw_color(Color::RGB(50, 255, 100));
                } else if self.nodes[i].node_type == WorldNodeType::Start {
                    canvas.set_draw_color(Color::RGB(255, 255, 50));
                } else {
                    canvas.set_draw_color(Color::RGB(200, 70, 70));
                }
                let debug_rect = Rect::new(0,0, 10, 10);
                let rect_screen_pos = world_to_screen(debug_rect, self.nodes[i].position, (w, h), None);
                canvas.draw_rect(rect_screen_pos).unwrap();
                canvas.fill_rect(rect_screen_pos).unwrap();
                canvas.set_draw_color(Color::RGBA(100, 50, 50, 50));
            }
      
            canvas.present();
        }
    }

}