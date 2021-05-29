use sdl2::{EventPump, event::Event, pixels::Color, rect::{Point, Rect}, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};

use crate::{GameStateData, Transition, engine_traits::scene::Scene, game_logic::factories::{item_factory::load_item_assets, world_factory::load_overworld_assets}, input::{self, input_devices::InputDevices, translated_inputs::TranslatedInput}, overworld::{node::{WorldNode, WorldNodeType}, overworld_generation}, rendering::renderer::{pos_world_to_screen, world_to_screen}, ui::ingame::{segmented_bar_ui::SegmentedBar, wrapping_list_ui::WrappingList}};

use super::match_scene::MatchScene;

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

    pub fn init(&mut self, (w, h): (u32, u32), full_conection: bool) {
        let map_area = Rect::new(400, 100, w-800, h-200);
        self.full_conection = full_conection;
        self.rect = map_area;
        self.nodes = overworld_generation(map_area, (5, 6), full_conection);
    }

    pub fn change_exploration_level(&mut self, full_conection: bool) {
        if self.full_conection != full_conection {
            self.full_conection = full_conection;
            self.nodes = overworld_generation(self.rect, (5, 6), full_conection);
        }
    }
}

pub fn hp_bar_init<'a>(screen_res: (u32, u32), max_hp: i32, curr_hp: i32) -> SegmentedBar<'a> {
    SegmentedBar::new(
        10,
        20,
        screen_res.0 / 2 - 50,
        25,
        max_hp,
        curr_hp,
        20,
        Some(Color::RGB(255, 100, 100)),
        None,
    )
}

pub fn item_list_init(game_state_data: &GameStateData) -> WrappingList {
    WrappingList::new(
        Point::new(10, 50),
        200,
        game_state_data.player.as_ref().unwrap().items.iter()
            .map(|item| {Rect::new(0,0,32,32)})
            .collect::<Vec<Rect>>(), 
        10
    )
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

        let mut hp_bars = hp_bar_init(
            (w, h),
            game_state_data.player.as_ref().unwrap().character.hp,
            game_state_data.player.as_ref().unwrap().hp.0,
        );

        let mut item_list = item_list_init(&game_state_data);

        self.connect_to_index = 0;
        let connecting_to = &self.nodes[self.player_node_pos as usize].connect_to;
        self.next_node = connecting_to
            .iter()
            .map(|&a| {a})
            .collect::<Vec<usize>>()[self.connect_to_index];

        let mut map_events =  game_state_data.player.as_ref().unwrap().events.on_overworld_map.clone();
        for map_event in map_events.iter_mut() {
            (map_event.0)(game_state_data.player.as_mut().unwrap(), self, &mut map_event.1);
        }
        game_state_data.player.as_mut().unwrap().events.on_overworld_map = map_events;

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
                            let connecting_to = &self.nodes[self.player_node_pos as usize].connect_to;
                            self.connect_to_index =  (1 + self.connect_to_index) % connecting_to.len();

                            self.next_node = connecting_to
                                .iter()
                                .map(|&a| {a})
                                .collect::<Vec<usize>>()[self.connect_to_index];
                        }
                    }
                    if !is_pressed {
                        if translated_input == TranslatedInput::Punch {
                            if let WorldNodeType::Level(_) = self.nodes[self.next_node].node_type {
                                self.player_node_pos = self.next_node;
                                return Transition::Push(Box::new(MatchScene::new("foxgirl".to_string())));
                            }
                        }
                    }
                }
                //end of input management
            }
            //update

            //render
            canvas.set_draw_color(Color::RGB(237, 158, 80));

            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 50));
            
            for node in self.nodes.iter() {
                for &connections in node.connect_to.iter() {
                    let origin_point = pos_world_to_screen(node.position + Point::new(30,30), (w, h), None);
                    let destination_point =  pos_world_to_screen(self.nodes[connections as usize].position + Point::new(30,30), (w, h), None);
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
                let src_rect;

                if let WorldNodeType::Level(_) = self.nodes[i].node_type  {
                    src_rect = assets.src_rects.get("camp").unwrap();
                    canvas.set_draw_color(Color::RGB(50, 255, 100));
                } else if self.nodes[i].node_type == WorldNodeType::Start {
                    src_rect = assets.src_rects.get("start").unwrap();
                    canvas.set_draw_color(Color::RGB(255, 255, 50));
                } else if self.nodes[i].node_type == WorldNodeType::Store {
                    src_rect = assets.src_rects.get("store").unwrap();
                    canvas.set_draw_color(Color::RGB(255, 255, 50));
                } else {
                    src_rect = assets.src_rects.get("boss_skull").unwrap();
                    canvas.set_draw_color(Color::RGB(200, 70, 70));
                }

                let node_rect = Rect::new(0,0, 60, 60);
                let rect_screen_pos = world_to_screen(node_rect, self.nodes[i].position, (w, h), None);
                canvas.set_draw_color(Color::RGBA(100, 50, 50, 50));

                canvas.copy(&assets.spritesheet, src_rect.clone(), rect_screen_pos).unwrap();
            }
            
            
            
            let src_pointer = assets.src_rects.get("arrow").unwrap();
            let pointer_screen = world_to_screen(Rect::new(0,0, 40, 40), self.nodes[self.next_node].position + Point::new(20,0), (w, h), None);
            canvas.copy_ex(&assets.spritesheet, src_pointer.clone(), pointer_screen, 90f64, Point::new(0,0), false, false).unwrap();
            
            let src_pointer = assets.src_rects.get("symbol").unwrap();
            let pointer_screen = world_to_screen(Rect::new(0,0, 40, 40), self.nodes[self.player_node_pos as usize].position - Point::new(20,0), (w, h), None);
            canvas.copy(&assets.spritesheet, src_pointer.clone(), pointer_screen).unwrap();
            

            let rect_screen_pos = world_to_screen(Rect::new(0,0, 300, 480), Point::new(0,0), (w, h), None);
            canvas.copy(&assets.portraits.get("portrait").unwrap(), Rect::new(0,0, 500, 870), rect_screen_pos).unwrap();
      
            if hp_bars.curr_value > 0 {
                canvas.set_draw_color(hp_bars.color.unwrap());
                for hp_rect in hp_bars.render() {
                    canvas.draw_rect(hp_rect).unwrap();
                    canvas.fill_rect(hp_rect).unwrap();
                }
            }

            let item_list = item_list.render();
            let player = game_state_data.player.as_ref().unwrap();
            if player.items.len() > 0 {
                for i in 0..player.items.len() {
                    let src_rect = game_state_data.item_sprites.src_rects.get(&player.items[i]).unwrap();
                    let dst_rect = item_list[i];
                    canvas.copy(&game_state_data.item_sprites.spritesheet, src_rect.clone(), dst_rect).unwrap();
                }
            }

            canvas.present();
        }
    }

}