use std::collections::HashMap;

use parry2d::na::Vector2;
use sdl2::{keyboard::Keycode, ttf::Font};

use crate::game_logic::{game::Game, items::{Item, ItemGround}};

pub struct Console{
    pub up: bool,
    pub command: String,
}

impl Console {
    pub fn add(&mut self, keycode: Keycode) {
        if self.up {


            if keycode == Keycode::Space {
                self.command.push_str(" ");
            } else if keycode == Keycode::Backspace {
                self.command = self.command[0..self.command.len() - 1].to_string();
            } else {
                self.command.push_str(&keycode.name())
            }
        }
    }

    pub fn run(&mut self, game: &mut Game, items: &HashMap<i32, Item>) {
        println!("spawn {}", self.command);
        if self.up {
            let split = self.command.split(" ").collect::<Vec<&str>>();

            match split[0] {
                "I" => {
                    println!("spawn {}", self.command);
                    let item_id = split[1].parse::<i32>().unwrap();;
                    game.items_on_ground.push(ItemGround{ position: game.player.position + Vector2::new(200f64, 0f64), item: (*items.get(&item_id).unwrap()).clone() });
                },
                _ => {}
            }

            self.command.clear();
        }
    }

    pub fn toggle(&mut self){
        self.up = !self.up;
    }

}