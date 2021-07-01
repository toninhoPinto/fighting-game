use std::{cmp, time::Instant};

use sdl2::{EventPump, event::Event, pixels::Color, rect::Rect, render::{Canvas, Texture, TextureCreator}, ttf::Font, video::{Window, WindowContext}};

use crate::{GameStateData, Transition, engine_traits::scene::Scene, game_logic::{effects::hash_effects, factories::{item_factory::{load_item_assets, load_items}, world_factory::load_overworld_assets}, items::Item, store::{StoreUI, get_store_item_list}}, hp_bar_init, input::{self, input_devices::InputDevices, translated_inputs::TranslatedInput}, item_list_init, overworld::{node::{WorldNode, WorldNodeType}, overworld_generation, overworld_change_connections}, rendering::{renderer_overworld::render_overworld, renderer_store::render_store, renderer_ui::render_ui}, ui::ingame::popup_ui::{PopUp, new_item_popup, popup_fade}};

use super::match_scene::{MAX_UPDATES_AVOID_SPIRAL_OF_DEATH};

pub struct StoreScene {

}

impl StoreScene {

    fn iterate_over_store_items(store: &mut StoreUI, direction: i32) {
        if store.items.len() > 0 {
            store.selected_item = (store.selected_item as i32 + (1 * direction)) as usize;
            store.selected_item %= store.items.len() + 1;
        }
    }

    pub fn create_price_textures<'a>(texture_creator: &'a TextureCreator<WindowContext>, font: &Font, store: &StoreUI, items: &std::collections::HashMap<i32, Item>) -> Vec<Texture<'a>>{
        store.items.iter().map(|item_id| {
           
            let mut price_text = items.get(&(*item_id as i32)).unwrap().price.to_string();
            price_text.push_str("$");

            let title_surface = font
                .render(&price_text)
                .blended(Color::WHITE)
                .map_err(|e| e.to_string())
                .unwrap();

            texture_creator
                        .create_texture_from_surface(&title_surface)
                        .map_err(|e| e.to_string())
                        .unwrap()
                })
        .collect::<Vec<Texture<'a>>>()
    }
}

impl<'a> Scene for StoreScene {
    
    fn run(
        &mut self,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump,
        input_devices: &mut InputDevices,
        canvas: &mut Canvas<Window>,
    ) -> Transition {

        let (w, h) = canvas.output_size().unwrap();

        let mut store_struct = StoreUI::new((w, h));
        let item_room_seed = game_state_data.seed.unwrap() + (game_state_data.curr_level as u64);
        store_struct.items = get_store_item_list(item_room_seed, game_state_data.general_assets.loot_tables.get("store_table").unwrap());

        let mut store_item_prices = Some(StoreScene::create_price_textures(texture_creator, 
            &game_state_data.general_assets.fonts.get("basic_font").unwrap(), 
            &store_struct, 
            &game_state_data.items)
        );

        let mut store = Some(store_struct);

        let assets = load_overworld_assets(&texture_creator);
        let item_assets = load_item_assets(&texture_creator);

        let mut popup_item = new_item_popup((w,h));
        let mut popup_content: Option<Vec<Texture>> = None;

        let mut item_list = item_list_init(&game_state_data);

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
                            StoreScene::iterate_over_store_items(&mut store.as_mut().unwrap(), x)
                        }
                    }

                    if !is_pressed {
                        if translated_input == TranslatedInput::Punch {
                            if let Some(ref mut store_ui) = store {

                                if store_ui.selected_item < store_ui.items.len() {
                                    if store_ui.items.len() > 0 {

                                        let mut bought_item = game_state_data.items.get(&(store_ui.items[store_ui.selected_item] as i32)).unwrap().clone();

                                        if game_state_data.player.as_ref().unwrap().currency >= bought_item.price {

                                            store_ui.items.remove(store_ui.selected_item);
                                            store_ui.item_rects.remove(store_ui.selected_item);
                                            store_ui.prices.remove(store_ui.selected_item);
                                            if let Some(ref mut store_item_prices) = store_item_prices {
                                                store_item_prices.remove(store_ui.selected_item);
                                            }

                                            let new_selected = if store_ui.selected_item == 0 {store_ui.selected_item} else {store_ui.selected_item-1};
                                            store_ui.selected_item = cmp::max(0,cmp::min(store_ui.items.len(), new_selected));

                                            let player = game_state_data.player.as_mut().unwrap();
                                            player.currency = cmp::max(0, player.currency - bought_item.price);
                                            player.equip_item(&mut bought_item, &game_state_data.effects, &mut game_state_data.energy_bar.as_mut().unwrap());
                                    
                                            popup_content = Some(crate::ui::ingame::popup_ui::render_popup(texture_creator, 
                                                &bought_item.name, 
                                                &bought_item.description, 
                                                &game_state_data.general_assets.fonts.get("basic_font").unwrap(), 
                                                &mut popup_item));

                                            if let Some(chance_mod) = &bought_item.chance_mod {
                                                (chance_mod.modifier)(chance_mod.item_ids.clone(), chance_mod.chance_mod, &game_state_data.player.as_ref().unwrap().character, &mut game_state_data.general_assets.loot_tables);
                                            } else {
                                                for (_key, val) in game_state_data.general_assets.loot_tables.iter_mut() {
                                                    val.items.retain(|x| x.item_id as i32 != bought_item.id);
                                                    val.acc = val.items.iter().map(|i|{i.rarity}).sum();
                                                }
                                            }

                                            if game_state_data.player.as_ref().unwrap().items.len() != item_list.rects.len() {
                                                item_list.update(game_state_data.player.as_ref().unwrap().items.iter()
                                                    .map(|_| {Rect::new(0,0,32,32)})
                                                    .collect::<Vec<Rect>>()
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    return Transition::Pop;
                                }

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
            
            render_store(canvas, 
                &assets,
                &game_state_data.ui_assets.ui_text,
                &store.as_ref().unwrap(), 
                &item_assets,
                &game_state_data.items, 
                &store_item_prices);

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