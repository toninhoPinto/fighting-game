use std::collections::HashMap;

use parry2d::na::Vector2;
use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect, render::{Canvas, TextureCreator, TextureQuery}, ttf::Font, video::{Window, WindowContext}};

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

    pub fn render(&self, texture_creator: &TextureCreator<WindowContext>, canvas: &mut Canvas<Window>, font: &Font) {

        if self.up {
            let screen_res = canvas.output_size().unwrap();

            canvas.set_draw_color(Color::RGB(20, 20, 25));
            let position = (0, 50);
            let console_rect = Rect::new(position.0,position.1, screen_res.0, 50);

            canvas.draw_rect(console_rect).unwrap();
            canvas.fill_rect(console_rect).unwrap();

            let color = Color::RGB(255, 255, 255);
            canvas.set_draw_color(color);
            if self.command.len() > 0 {
                let surface = font
                    .render(&self.command)
                    .blended(color)
                    .map_err(|e| e.to_string())
                    .unwrap();

                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())
                    .unwrap();

                let TextureQuery { width, height, .. } = texture.query();
                canvas
                    .copy(&texture, None, Some(Rect::new(position.0 + 5,position.1, width, height)))
                    .unwrap();
            }
        }
    }

}