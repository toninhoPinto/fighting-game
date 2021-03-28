use std::fs;

use parry2d::bounding_volume::AABB;

use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::{Point};
use sdl2::video::WindowContext;
use sdl2::image::{LoadTexture};

use super::spriter_pro_collider_loader;

pub fn load_hitboxes(file: std::string::String) -> (Vec<AABB>, Vec<Vec<Point>>)  {
    spriter_pro_collider_loader::load_hitboxes(file)
}

pub fn load_anim_from_dir<'a>(tex_creator: &'a TextureCreator<WindowContext>, dir: &'_ str) -> Vec<Texture<'a>> {
    let paths = fs::read_dir(dir).unwrap();

    let mut vec: Vec<Texture> = Vec::new();

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "png"  {
            vec.push(tex_creator.load_texture(path).unwrap());
        }
    }
    vec
}

pub fn load_single_sprite(tex_creator: &TextureCreator<WindowContext>, file_path: std::string::String) -> Texture {
    tex_creator.load_texture(file_path).unwrap()
}