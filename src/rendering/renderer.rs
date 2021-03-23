use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::rect::{Point, Rect};
use sdl2::video::WindowContext;
use sdl2::image::{self, LoadTexture, InitFlag};
use std::string::String;
use std::fs;
use std::path::Path;

use crate::game_logic::player::Player;

pub fn render(canvas: &mut WindowCanvas, color: Color, texture_1: &Texture, player1: &Player,texture_2: &Texture, player2: &Player) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let screen_position = player1.position + Point::new(width as i32 / 2, 4 * height as i32 / 5);
    let screen_rect = Rect::from_center(screen_position, player1.sprite.width(), player1.sprite.height());

    canvas.copy_ex(texture_1, player1.sprite, screen_rect, 0.0, None, player1.flipped, false)?;

    let screen_position_2 = player2.position + Point::new(width as i32 / 2, 4 * height as i32 / 5);
    let screen_rect_2 = Rect::from_center(screen_position_2, player2.sprite.width(), player2.sprite.height());
    canvas.copy_ex(texture_2, player2.sprite, screen_rect_2, 0.0, None, player2.flipped, false)?;

    canvas.present();

    Ok(())
}

pub fn load_anim_from_dir(tex_creator: &TextureCreator<WindowContext>, dir: std::string::String) -> Vec<Texture> {
    let paths = fs::read_dir(dir).unwrap();

    let mut vec: Vec<Texture> = Vec::new();

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            println!("Name: {}", path.display());
            vec.push(tex_creator.load_texture(path).unwrap());
        }
    }
    vec
}