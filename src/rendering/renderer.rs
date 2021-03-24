use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::rect::{Point, Rect};
use sdl2::video::WindowContext;
use sdl2::image::{LoadTexture};
use std::string::String;
use std::fs;

use crate::game_logic::player::Player;
use crate::game_logic::projectile::Projectile;
use crate::game_logic::character_factory::CharacterAnimationData;

pub fn render(canvas: &mut WindowCanvas, color: Color,
              texture_1: Option<&Texture>, player1: &Player, p1_anims: &CharacterAnimationData,
              texture_2: Option<&Texture>, player2: &Player, p2_anims: &CharacterAnimationData,
              projectiles: &Vec<Projectile>) -> Result<(), String> {

    canvas.set_draw_color(color);
    canvas.clear();

    match texture_1 {
        Some(texture) => {
            let (width, height) = canvas.output_size()?;
            let screen_position = player1.position + Point::new(width as i32 / 2, height as i32 / 2);
            let screen_rect = Rect::from_center(screen_position, player1.sprite.width(), player1.sprite.height());
            canvas.copy_ex(texture, player1.sprite, screen_rect, 0.0, None, player1.flipped, false)?;
        },
        None => ()
    }

    match texture_2 {
        Some(texture) => {
            let (width, height) = canvas.output_size()?;
            let screen_position_2 = player2.position + Point::new(width as i32 / 2, height as i32 / 2);
            let screen_rect_2 = Rect::from_center(screen_position_2, player2.sprite.width(), player2.sprite.height());
            canvas.copy_ex(texture, player2.sprite, screen_rect_2, 0.0, None, player2.flipped, false)?;
        },
        None => ()

    }

    for projectile in projectiles.iter() {
        let (width, height) = canvas.output_size()?;
        let screen_position_2 = projectile.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect_2 = Rect::from_center(screen_position_2, projectile.sprite.width(), projectile.sprite.height());

        if projectile.player_owner == 1 {
            canvas.copy_ex(&p1_anims.projectile_animation.get(&projectile.animation_name).unwrap()[projectile.animation_index as usize], projectile.sprite, screen_rect_2, 0.0, None, projectile.flipped, false)?;
        } else if  projectile.player_owner == 2 {
            canvas.copy_ex(&p2_anims.projectile_animation.get(&projectile.animation_name).unwrap()[projectile.animation_index as usize], projectile.sprite, screen_rect_2, 0.0, None, projectile.flipped, false)?;
        }
    }



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

pub fn load_single_sprite(tex_creator: &TextureCreator<WindowContext>, file_path: std::string::String) -> Texture {
    tex_creator.load_texture(file_path).unwrap()
}